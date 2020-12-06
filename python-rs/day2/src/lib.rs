#![feature(map_into_keys_values)]
use std::collections::HashMap;

extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn word_count(id: String) -> (i32, i32) {
    let mut letters = HashMap::new();

    for c in id.chars() {
        let letter_c = letters.entry(c).or_insert(0);
        *letter_c += 1;
    }

    let values: Vec<i32> = letters.into_values().collect();
    let x = if values.iter().any(|&v| v == 2) { 1 } else { 0 };
    let y = if values.iter().any(|&v| v == 3) { 1 } else { 0 };

    (x, y)
}

#[pyfunction]
fn calculate(ids: Vec<String>) -> PyResult<i32> {
    let mut x = 0;
    let mut y = 0;
    
    for id in ids {
        let p = word_count(id);
        x += p.0;
        y += p.1;
    }

    Ok(x * y)
}

#[pymodule]
fn calc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!((0, 0), word_count("abcdef".to_string()));
    }

    #[test]
    fn t2() {
        assert_eq!((1, 1), word_count("bababc".to_string()));
    }

    #[test]
    fn t3() {
        assert_eq!((1, 0), word_count("abbcde".to_string()));
    }

    #[test]
    fn t4() {
        assert_eq!((0, 1), word_count("abcccd".to_string()));
    }
}
