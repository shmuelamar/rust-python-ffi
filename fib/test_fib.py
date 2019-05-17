import pytest

import fibpy
import fibpyo3
import fibrust_cpython
import fibc
import fibcython


@pytest.mark.parametrize('module,fn', [
    ('fibc', 'sum_fib_10k'),
    ('fibcython', 'sum_fib_10k'),
    ('fibpy', 'sum_fib_10k'),
    ('fibpy', 'sum_fib_10k_fun'),
    ('fibpyo3', 'sum_fib_10k'),
    ('fibpyo3', 'sum_fib_10k_fun'),
    ('fibrust_cpython', 'sum_fib_10k'),
    ('fibrust_cpython', 'sum_fib_10k_fun'),
])
def test_sum_fib_even_4m(module, fn, benchmark):
    max_num = 4_000_000

    module = globals()[module]
    fn = getattr(module, fn)
    assert fn(max_num) == 4613732
    benchmark(fn, max_num)
