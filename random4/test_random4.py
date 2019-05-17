import pytest

import random4py
import random4c
import random4cython
import random4pyo3
import random4rust_cpython


@pytest.mark.parametrize('module', [
    'random4py',
    'random4c',
    'random4pyo3',
    'random4rust_cpython',
    'random4cython',
])
def test_random4(module, benchmark):
    module = globals()[module]
    assert module.random4() == 4
    benchmark(module.random4)
