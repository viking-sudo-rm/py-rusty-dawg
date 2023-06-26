use pyo3::prelude::*;

pub mod dawg;

use dawg::Dawg;

/// A Python module implemented in Rust.
#[pymodule]
fn py_rusty_dawg(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Dawg>()?;  // FIXME, generics?
    Ok(())
}