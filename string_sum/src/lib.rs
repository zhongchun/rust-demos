use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

// use std::collections::HashMap;

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
fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[pyfunction]
fn check_positive(x: i32) -> PyResult<()> {
    if x < 0 {
        Err(PyValueError::new_err("x is negative"))
    } else {
        Ok(())
    }
}

#[pyclass]
struct Number(i32);

#[pymethods]
impl Number {
    #[new]
    fn new(value: i32) -> Self {
        Number(value)
    }

    fn value(self_: PyRef<'_, Self>) -> PyResult<i32> {
        Ok(self_.0)
    }
}

// #[pyclass(extends=PyDict)]
// #[derive(Default)]
// struct DictWithCounter {
//     counter: HashMap<String, usize>,
// }

// #[pymethods]
// impl DictWithCounter {
//     #[new]
//     fn new() -> Self {
//         Self::default()
//     }

//     fn set(mut self_: PyRefMut<'_, Self>, key: String, value: &PyAny) -> PyResult<()> {
//         self_.counter.entry(key.clone()).or_insert(0);
//         let py = self_.py();
//         let dict: &PyDict = unsafe { py.from_borrowed_ptr_or_err(self_.as_ptr())? };
//         dict.set_item(key, value)
//     }
// }

/// A Python module implemented in Rust.
/// The name of this function must match the `lib.name` setting in the
/// `Cargo.toml`, else Python will not be able to import the module.
#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(double, m)?)?;
    m.add_function(wrap_pyfunction!(num_kwds, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(check_positive, m)?)?;
    m.add_class::<Number>()?;
    Ok(())
}
