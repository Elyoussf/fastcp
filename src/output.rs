use pyo3::prelude::*;
use std::io::{self, Write};

#[pyfunction]
pub fn fast_print_string(s: String) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", s).unwrap();
}
#[pyfunction]
pub fn fast_print_integer(s: i64) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", s).unwrap();
}

#[pyfunction]
pub fn fast_print_float(s: f64) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", s).unwrap();
}
