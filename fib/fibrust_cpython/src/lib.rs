#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python};
use fibrust;


fn sum_fib_10k_py(_: Python, max_num: u64) -> PyResult<u64> { Ok(fibrust::sum_fib_10k(max_num)) }

fn sum_fib_10k_fun_py(_: Python, max_num: u64) -> PyResult<u64> { Ok(fibrust::sum_fib_10k_fun(max_num)) }

// add bindings to the generated python module
// N.B: names: "librust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(librust2py, initfibrust_cpython, PyInit_fibrust_cpython, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "sum_fib_10k", py_fn!(py, sum_fib_10k_py(max_num: u64)))?;
    m.add(py, "sum_fib_10k_fun", py_fn!(py, sum_fib_10k_fun_py(max_num: u64)))?;
    Ok(())
});
