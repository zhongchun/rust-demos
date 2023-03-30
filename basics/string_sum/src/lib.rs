use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

#[pyfunction]
#[pyo3(signature = (**kwds))]
fn num_kwds(kwds: Option<&PyDict>) -> usize {
    kwds.map_or(0, |dict| dict.len())
}

/// This function adds two unsigned 64-bit integers.
#[pyfunction]
#[pyo3(signature = (a, b=0, /))]
fn add(a:u64, b:u64) -> u64 {
    a + b
}

/// A Python module implemented in Rust.
/// The name of this function must match the `lib.name` setting in the
/// `Cargo.toml`, else Python will not be able to import the module.
#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(double, m)?)?;
    m.add_function(wrap_pyfunction!(num_kwds, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}