#![feature(proc_macro)]
#![feature(proc_macro_path_invoc)]

#[macro_use]
extern crate pyo3;

use pyo3::prelude::*;

#[py::modinit(_pyo3typeerror)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "loads")]
    fn loads(py: Python, s: &str) -> PyResult<PyObject> {
        Ok(s.to_object(py))
    }
    Ok(())
}
