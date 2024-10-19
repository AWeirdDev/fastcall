use std::{collections::HashMap, str::FromStr};

use pyo3::{
    exceptions,
    prelude::*,
    types::{PyDict, PyList},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Params {
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[derive(Serialize, Deserialize)]
#[pyclass]
struct Message {
    #[pyo3(get)]
    jsonrpc: String,
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    method: String,
    params: Params,
}

fn as_py(v: Value, py: Python) -> PyResult<Py<PyAny>> {
    match v {
        Value::Array(a) => {
            let list = PyList::empty_bound(py);
            for item in a {
                list.append(as_py(item, py)?)?;
            }
            Ok(list.into_py(py))
        }
        Value::Bool(b) => Ok(b.into_py(py)),
        Value::Number(n) => Ok({
            if n.is_i64() || n.is_u64() {
                n.as_i64().into_py(py)
            } else {
                n.as_f64().into_py(py)
            }
        }),
        Value::String(s) => Ok(s.into_py(py)),
        Value::Object(o) => {
            let dict = PyDict::new_bound(py);
            for (k, v) in o {
                dict.set_item(k, as_py(v, py)?)?;
            }
            Ok(dict.into())
        }
        Value::Null => Ok(py.None()),
    }
}

#[pymethods]
impl Message {
    #[new]
    #[pyo3(signature = (id, *, kw = true))]
    fn py_new(id: String, kw: bool) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            method: "method".into(),
            params: {
                if kw {
                    Params::Object(HashMap::new())
                } else {
                    Params::Array(vec![])
                }
            },
        }
    }

    fn set_param(&mut self, name: String, raw: &str) -> PyResult<()> {
        let binding = Value::from_str(raw);
        if let Err(e) = binding {
            return Err(exceptions::PyRuntimeError::new_err(format!(
                "Failed to parse JSON: {}",
                e
            )));
        };

        let value = binding.unwrap();

        match &mut self.params {
            Params::Object(o) => {
                o.insert(name, value);
            }
            Params::Array(a) => {
                a.push(value);
            }
            Params::String(ref s) => {
                self.params = Params::Array(vec![Value::String(s.to_string()), value]);
            }
        }

        Ok(())
    }

    #[getter]
    #[pyo3(name = "params")]
    fn py_params(&self, py: Python) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        let list = PyList::empty_bound(py);
        let dict = PyDict::new_bound(py);

        match &self.params {
            Params::String(s) => {
                list.append(s)?;
            }
            Params::Array(a) => {
                for item in a {
                    list.append(as_py(item.clone(), py)?)?;
                }
            }
            Params::Object(o) => {
                for (k, v) in o {
                    dict.set_item(k, as_py(v.clone(), py)?)?;
                }
            }
        }

        Ok((list.into(), dict.into()))
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "Message(jsonrpc={:?}, id={:?}, method={:?}, params={{{}}})",
            self.jsonrpc,
            self.id,
            self.method,
            {
                let (args, kwargs) = self.py_params(py)?;
                format!("args: {}, kwargs: {}", args.to_string(), kwargs.to_string())
            }
        ))
    }
}

#[pyfunction]
fn decode(data: &str) -> PyResult<Message> {
    match serde_json::from_str::<Message>(data) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(exceptions::PyRuntimeError::new_err(format!(
            "Failed to decode JSON: {}",
            e
        ))),
    }
}

#[pymodule]
fn fastcall(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add_class::<Message>()?;
    Ok(())
}
