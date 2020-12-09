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

fn compare_words(w1: &String, w2: &String) -> Result<String, bool> {
    assert_eq!(w1.len(), w2.len());

    for i in 0..w1.len() {
        let w1_less: String = w1.char_indices().filter_map(|m| if m.0 != i { Some(m.1) } else { None }).collect();
        let w2_less: String = w2.char_indices().filter_map(|m| if m.0 != i { Some(m.1) } else { None }).collect();

        if w1_less == w2_less { return Ok(w1_less); }
    }

    Err(false)
}

#[pyfunction]
fn p2(ids: Vec<String>) -> PyResult<String> {
    for i in 0..ids.len()-1 {
        for j in i+1..ids.len() {
            if let Ok(k) = compare_words(&ids[i], &ids[j]) {
                return Ok(k)
            }
        }
    }

    return Ok(String::from(""));
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
    m.add_function(wrap_pyfunction!(p2, m)?)?;

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

    #[test]
    fn t5() {
        match compare_words(&String::from("abcd"), &String::from("abce")) {
            Ok(s) => assert_eq!(s, "abc"),
            Err(_) => unreachable!(),
        }
    }
}
