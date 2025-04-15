# fastcp

A fast input reading library for Python, backed by Rust for performance.

## Overview

`fastcp` provides a set of functions to efficiently read various types of input from the standard input stream. Leveraging the speed of Rust, this library aims to accelerate input operations in your Python projects, especially in competitive programming or scenarios dealing with large input datasets.

## Installation

To use `fastcp`, you first need to have Rust installed on your system. You also need to have a Python environment set up with `pip`.

1.  **Install Rust:** Follow the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2.  **Install `maturin`:** `maturin` is a tool for building and publishing Rust-based Python packages. Install it using pip:
    ```bash
    pip install maturin
    ```

3.  **Build the `fastcp` package:** Navigate to the directory containing the `Cargo.toml` file (where the Rust code you provided is located) and build the Python package using `maturin`:
    ```bash
    maturin build --release
    ```
    This will create a `.whl` file in the `target/wheels` directory.



## Usage in Python

Once installed, you can import and use the functions provided by `fastcp` in your Python scripts.

```python
import fastcp

# Read a single number (integer or float)
success, number = fastcp.read_number()
if success:
    print(f"Read number: {number}")
else:
    print(f"Error reading number: {number}")

# Read N integers, one per line, into a list
n_integers = 3
integer_list = fastcp.read_integers_per_line_tolist(n_integers)
print(f"Read integers: {integer_list}")

# Read N floats, one per line, into a list
n_floats = 2
float_list = fastcp.read_float_per_line_tolist(n_floats)
print(f"Read floats: {float_list}")

# Read N strings, one per line, into a list
n_strings = 2
string_list = fastcp.read_string_per_line_tostring(n_strings)
print(f"Read strings: {string_list}")

# Read a single line as a string
success, single_string = fastcp.read_string()
if success:
    print(f"Read string: {single_string}")
else:
    print(f"Error reading string: {single_string}")

# Read a line of strings separated by a pattern into a list
separator = " "
separated_strings = fastcp.read_separated_string_into_list(separator)
print(f"Read separated strings: {separated_strings}")

# Read a line of integers separated by a pattern into a list
integer_separator = ","
separated_integers = fastcp.read_separated_integers_into_list(integer_separator)
print(f"Read separated integers: {separated_integers}")

# Read a line of floats separated by a pattern into a list
float_separator = ";"
separated_floats = fastcp.read_separated_float_into_list(float_separator)
print(f"Read separated floats: {separated_floats}")

## Functions

Here's a brief description of the functions provided by `fastcp`:

* `read_number()`: Reads a single line from standard input and attempts to parse it as either an integer (`i64`) or a float (`f64`). Returns a tuple `(success: bool, value: Union[int, float, str])`. If `success` is `False`, `value` will be an error message.
* `read_integers_per_line_tolist(n: int)`: Reads `n` lines from standard input and attempts to parse each line as an integer (`i64`). Returns a Python list of the parsed integers. Lines that cannot be parsed as integers are ignored.
* `read_float_per_line_tolist(n: int)`: Reads `n` lines from standard input and attempts to parse each line as a float (`f64`). Returns a Python list of the parsed floats. Lines that cannot be parsed as floats are ignored.
* `read_string_per_line_tostring(n: int)`: Reads `n` lines from standard input and returns a Python list where each element is a line read as a string (including the newline character).
* `read_string()`: Reads a single line from standard input as a string (including the newline character). Returns a tuple `(success: bool, value: str)`. If `success` is `False`, `value` will be an error message.
* `read_separated_string_into_list(pat: str)`: Reads a single line from standard input and splits it into a list of strings based on the provided delimiter `pat`.
* `read_separated_integers_into_list(pat: str)`: Reads a single line from standard input, splits it based on the delimiter `pat`, and attempts to parse each part as an integer (`i64`). Returns a Python list of the successfully parsed integers.
* `read_separated_float_into_list(pat: str)`: Reads a single line from standard input, splits it based on the delimiter `pat`, and attempts to parse each part as a float (`f64`). Returns a Python list of the successfully parsed floats.

## Contributing

Contributions to `fastcp` are welcome! Feel free to submit pull requests or open issues for bug fixes or new features.
