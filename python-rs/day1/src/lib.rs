extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn calculate(nums: Vec<i32>) -> PyResult<i32> {
    let sum = nums.iter().sum();
    Ok(sum)
}

#[pymodule]
fn calc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate, m)?)?;

    Ok(())
}
