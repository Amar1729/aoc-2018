use std::collections::HashSet;

extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn calculate(nums: Vec<i32>) -> PyResult<i32> {
    let mut freqs = HashSet::new();

    freqs.insert(0);

    let mut c = 0;
    loop {
        for i in &nums {
            c += i;
            if freqs.contains(&c) {
                return Ok(c);
            }
            freqs.insert(c);
        }
    }
}

#[pymodule]
fn calc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate, m)?)?;

    Ok(())
}
