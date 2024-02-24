mod processor;
mod rules;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn process()  {
    let _ = processor::process();
}



/// A Python module implemented in Rust.
#[pymodule]
fn event_correlator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process, m)?)?;
    Ok(())
}
