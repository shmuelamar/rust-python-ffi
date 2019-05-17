use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// xkcd-221 random generator written in Rust
#[pyfunction]
fn random4() -> PyResult<i32> { Ok(4) }

/// xkcd-221 module implemented in Rust
#[pymodule]
fn random4pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(random4))?;

    Ok(())
}
