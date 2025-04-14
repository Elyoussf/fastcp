mod input;
mod math;
use pyo3::prelude::*;
#[pymodule]
fn fastcp(_py: Python, m: &PyModule) -> PyResult<()> {
    // Register functions from submodules
    m.add_function(wrap_pyfunction!(math::add, m)?)?;
    m.add_function(wrap_pyfunction!(input::read_number, m)?)?;
    Ok(())
}
