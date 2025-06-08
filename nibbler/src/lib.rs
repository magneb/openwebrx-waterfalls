use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn split_and_interleave_nibbles(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len() * 2);
    
    for &byte in input {
        let high = byte >> 4;
        let low = byte & 0x0F;
        // Interleave with low nibble first
        output.push(low);
        output.push(high);
    }
    
    output
}

/// A Python module implemented in Rust.
#[pymodule]
fn nibbler(m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(split_and_interleave_nibbles, &m)?)?;
    Ok(())
}
