mod input;
mod output;
use pyo3::prelude::*;
#[pymodule]

fn fastcp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(input::read_integers_per_line_tolist, m)?)?;
    m.add_function(wrap_pyfunction!(input::read_float_per_line_tolist, m)?)?;
    m.add_function(wrap_pyfunction!(input::read_separated_float_into_list, m)?)?;
    m.add_function(wrap_pyfunction!(
        input::read_separated_integers_into_list,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(input::read_separated_string_into_list, m)?)?;
    m.add_function(wrap_pyfunction!(input::read_string, m)?)?;
    m.add_function(wrap_pyfunction!(input::read_number, m)?)?;

    m.add_function(wrap_pyfunction!(output::fast_print_string, m)?)?;
    m.add_function(wrap_pyfunction!(output::fast_print_integer, m)?)?;
    m.add_function(wrap_pyfunction!(output::fast_print_float, m)?)?;
    Ok(())
}
