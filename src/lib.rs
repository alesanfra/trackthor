use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    let result = (a + b).to_string();
    Ok(result)
}

#[pyclass]
struct Box {
    #[pyo3(get)]
    center: (i32, i32),

    #[pyo3(get)]
    height: i32,

    #[pyo3(get)]
    width: i32,
}

#[pymethods]
impl Box {
    #[new]
    fn new(center: (i32, i32), height: i32, width: i32) -> Self {
        Box {
            center,
            height,
            width,
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn trackthor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Box>()?;

    Ok(())
}
