use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use serde_json::Value;

/// Converts a `serde_json::Value` into a `Py<PyAny>`.
///
/// # Example
///
/// ```rust
/// use serde_json::json;
///
/// as_py(json!([1, 2, 3]), py).unwrap();
/// ```
pub(crate) fn as_py(v: Value, py: Python) -> PyResult<Py<PyAny>> {
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
