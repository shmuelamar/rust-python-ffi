# Welcome to FibPyO3 Packge!

## Example Usage

```python
import fibpyo3

assert fibpyo3.sum_fib_even(4000000) == 4613732
assert fibpyo3.sum_fib_even_functional(4000000) == 4613732
```

## Build New Release

```bash
pyo3-pack build -i python3.7
```

## Install Wheel

```bash
pip install ./target/wheels/fibpyo3-0.1.0-cp37-cp37m-manylinux1_x86_64.whl
```

Have Fun!

