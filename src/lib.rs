use pyo3::{exceptions, prelude::*};

mod message;
mod server;
mod utils;

use crate::message::Message;

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
