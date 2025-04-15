use std::io::{BufRead, BufReader, stdin};
// use std::error::Error;

use pyo3::{prelude::*, types::PyList};

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
