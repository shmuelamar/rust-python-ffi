# Rust Python FFI

These repo contains materials for my talk titled **"Rust is the Next Python FFI"**.

# Dependencies

* cargo + rustc nightly (for PyO3)
* gcc
* make
* python3
* Cython
* pytest
* pytest-benchmarks


# Example Folders

## random4/

from [xkcd-221](https://xkcd.com/221/):

[![xkcd-221](https://imgs.xkcd.com/comics/random_number.png "xkcd-221")](https://xkcd.com/221/)

Implementation of a Python module contains a function that returns 4, implemented in:

* Python3.7+
* Cython
* C (#include <Python.h>)
* Rust (PyO3 crate)
* Rust (rust-cpython crate)

This example shows how to call a foreign function in other language from Python and the benchmarks 
shows the round trip time. 

#### run benchmarks:

```bash
$ make test
```


## fib/

Implementation of a Python module contains a function to sum the even fib numbers 
under the given num, implemented in:

* Python
* Cython
* C (#include <Python.h>)
* Rust (PyO3 crate)
* Rust (rust-cpython crate)


This example shows a heavy numeric calculation on a foreign language called from python.

#### run benchmarks:

```bash
$ make test
```