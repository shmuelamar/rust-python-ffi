extern crate fibrust;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn sum_fib_10k(max_num: u64) -> PyResult<u64> { Ok(fibrust::sum_fib_10k(max_num)) }

#[pyfunction]
fn sum_fib_10k_fun(max_num: u64) -> PyResult<u64> {
    Ok(fibrust::sum_fib_10k_fun(max_num))
}

/// fibonacci series rust module
#[pymodule]
fn fibpyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_fib_10k))?;
    m.add_wrapped(wrap_pyfunction!(sum_fib_10k_fun))?;
    Ok(())
}
