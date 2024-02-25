mod processor;
mod rules;

use pyo3::prelude::*;
use crate::rules::Rule;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn process()  {
    let rules: Vec<Rule> = Vec::new();
    let _ = processor::process(rules);
}



/// A Python module implemented in Rust.
#[pymodule]
fn event_correlator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process, m)?)?;
    Ok(())
}
