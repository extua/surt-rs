use pyo3::prelude::*;
use surt_rs::generate_surt;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn surt(url: &str) -> PyResult<String> {
    Ok(generate_surt(url).unwrap_or_else(|_| url.to_string()))
}

/// A Python module implemented in Rust.
#[pymodule]
fn surt_rs_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(surt, m)?)?;
    Ok(())
}
