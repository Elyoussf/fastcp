use std::io::{BufRead, BufReader, stdin};
// use std::error::Error;

use pyo3::{ffi::PyRun_String, prelude::*, types::PyList};

#[pyfunction]
pub fn read_number(py: Python) -> PyResult<(bool, PyObject)> {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut data = String::new();

    match reader.read_line(&mut data) {
        Ok(_) => {
            if data.trim().is_empty() {
                Ok((false, String::from("Empty input!").into_py(py)))
            } else if !data
                .trim()
                .chars()
                .all(|c| c.is_numeric() || c == '.' || c == '-')
            {
                Ok((false, String::from("Not a number").into_py(py)))
            } else {
                let res = match data.trim().parse::<i64>() {
                    Ok(num) => Ok((true, num.into_py(py))),
                    Err(_) => {
                        let r = match data.trim().parse::<f64>() {
                            Ok(num) => Ok((true, num.into_py(py))),
                            Err(_) => Ok((false, String::from("Not a valid number").into_py(py))),
                        };
                        r
                    }
                };
                res
            }
        }

        Err(_) => Ok((
            false,
            String::from("An internal error occured in Rust side").into_py(py),
        )),
    }
}
#[pyfunction]
pub fn read_integers_per_line_tolist(py: Python, n: i64) -> PyObject {
    let mut res = Vec::new();
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    for i in 0..n {
        let mut data = String::new();

        match reader.read_line(&mut data) {
            Ok(_) => {
                match data.trim().parse::<i64>() {
                    Ok(num) => {
                        res.push(num);
                    }
                    Err(_) => (),
                };
            }
            Err(_) => (),
        }
    }
    let list = PyList::new(py, res).into();
    list
}

#[pyfunction]
pub fn read_float_per_line_tolist(py: Python, n: i64) -> PyObject {
    let mut res = Vec::new();
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    for i in 0..n {
        let mut data = String::new();

        match reader.read_line(&mut data) {
            Ok(_) => {
                match data.trim().parse::<f64>() {
                    Ok(num) => {
                        res.push(num);
                    }
                    Err(_) => (),
                };
            }
            Err(_) => (),
        }
    }
    let list = PyList::new(py, res).into();
    list
}

#[pyfunction]
pub fn read_string_per_line_tostring(py: Python, n: i64) -> PyObject {
    let mut vec = Vec::new();
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    for i in 0..n {
        let mut data = String::new();

        match reader.read_line(&mut data) {
            Ok(_) => {
                vec.push(data);
            }
            Err(_) => (),
        }
    }
    PyList::new(py, vec).into()
}

#[pyfunction]
pub fn read_string(py: Python) -> PyResult<(bool, PyObject)> {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut data = String::new();

    match reader.read_line(&mut data) {
        Ok(_) => Ok((true, data.into_py(py))),

        Err(_) => Ok((
            false,
            String::from("An internal error occured in Rust side").into_py(py),
        )),
    }
}

#[pyfunction]
pub fn read_separated_string_into_list(py: Python, pat: String) -> PyObject {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut res = Vec::new();
    let mut data = String::new();
    match reader.read_line(&mut data) {
        Ok(_) => {
            for c in data.trim().split(pat.as_str()) {
                res.push(c);
            }
        }
        Err(_) => (),
    }
    PyList::new(py, res).into()
}

#[pyfunction]
pub fn read_separated_integers_into_list(py: Python, pat: String) -> PyObject {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut res = Vec::new();
    let mut data = String::new();
    match reader.read_line(&mut data) {
        Ok(_) => {
            for c in data.trim().split(pat.as_str()) {
                match c.trim().parse::<i64>() {
                    Ok(r) => res.push(r),
                    Err(_) => (),
                }
            }
        }
        Err(_) => (),
    }
    PyList::new(py, res).into()
}
#[pyfunction]
pub fn read_separated_float_into_list(py: Python, pat: String) -> PyObject {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut res = Vec::new();
    let mut data = String::new();
    match reader.read_line(&mut data) {
        Ok(_) => {
            for c in data.trim().split(pat.as_str()) {
                match c.parse::<f64>() {
                    Ok(r) => res.push(r),
                    Err(_) => (),
                }
            }
        }
        Err(_) => (),
    }
    PyList::new(py, res).into()
}
