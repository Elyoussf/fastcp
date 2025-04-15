mod input;

use pyo3::prelude::*;
#[pymodule]

fn fastcp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(input::read_integers_per_line_tolist, m)?)?;
    m.add_function(wrap_pyfunction!(input::read_number, m)?)?;

    Ok(())
}
