use pyo3::prelude::*;

#[pyfunction]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
