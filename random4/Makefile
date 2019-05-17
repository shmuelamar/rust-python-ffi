clean:
	mkdir -p ./builds
	rm -rf ./builds/*

clean-all:
	cd random4pyo3 && cargo clean
	cd random4rust_cpython && cargo clean

build: clean
	cd random4c && python setup.py build && cp build/lib.linux-*/random4c.cpython-*.so ../builds/
	cd random4pyo3 && cargo build --release && cp target/release/librandom4pyo3.so ../builds/random4pyo3.so
	cd random4rust_cpython && cargo build --release && cp target/release/librandom4rust_cpython.so ../builds/random4rust_cpython.so
	cd random4cython && python setup.py build_ext --inplace && cp random4cython.cpython-*.so ../builds/

test: build
	PYTHONPATH=./builds pytest -s -v test_random4.py
