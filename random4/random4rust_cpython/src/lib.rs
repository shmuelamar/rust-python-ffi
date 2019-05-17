#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python};

fn random4(_: Python) -> PyResult<i32> {
    Ok(4)
}

py_module_initializer!(
    librust2py,
    initrandom4rust_cpython,
    PyInit_random4rust_cpython,
    |py, m| {
        m.add(py, "__doc__", "This module is implemented in Rust.")?;
        m.add(py, "random4", py_fn!(py, random4()))?;
        Ok(())
    }
);
