use numpy::{PyArray1, IntoPyArray};
use pyo3::prelude::*;

static STEP_SIZE_TABLE: [i32; 89] = [7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66, 73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449, 494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272, 2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493, 10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767];
static INDEX_TABLE: [i8; 16] = [-1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8];

/// Formats the sum of two numbers as string.
#[pyfunction]
fn decode(py: Python, input_nibbles: Vec<u8>) -> Py<PyArray1<i16>> {
    let mut pcm_samples: Vec<i16> = Vec::new();
    let mut predictor: i32 = 0;
    let mut index: i32 = 0;

    for &nibble in input_nibbles.iter() {
        // Calculate the step from the step size table
        let step = STEP_SIZE_TABLE[index as usize];

        // Initialize difference as half of step
        let mut diff = step >> 3;

        // Decode the nibble and adjust the difference
        if nibble & 4 != 0 { diff += step; }
        if nibble & 2 != 0 { diff += step >> 1; }
        if nibble & 1 != 0 { diff += step >> 2; }

        // Adjust predictor
        if nibble & 8 != 0 {
            predictor -= diff;
        } else {
            predictor += diff;
        }

        // Clamp predictor to 16-bit signed int
        if predictor > 32767 {
            predictor = 32767;
        } else if predictor < -32768 {
            predictor = -32768;
        }

        // Adjust the index
        index += INDEX_TABLE[nibble as usize] as i32;

        // Ensure index is within bounds
        if index < 0 { index = 0; }
        if index > 88 { index = 88; }

        // Append the predictor to the output samples
        pcm_samples.push(predictor as i16);
    }

    pcm_samples.into_pyarray(py).to_owned().into()
}


/// A Python module implemented in Rust.
#[pymodule]
fn radpcm(m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode, &m)?)?;
    Ok(())
}
