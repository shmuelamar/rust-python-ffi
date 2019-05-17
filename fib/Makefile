clean:
	mkdir -p ./builds
	rm -rf ./builds/*

clean-all:
	cd fibpyo3 && cargo clean
	cd fibrust && cargo clean
	cd fibrust_cpython && cargo clean

build: clean
	cd fibc && python setup.py build && cp build/lib.linux-*/fibc.cpython-*.so ../builds/
	cd fibpyo3 && cargo build --release && cp target/release/libfibpyo3.so ../builds/fibpyo3.so
	cd fibrust_cpython && cargo build --release && cp target/release/libfibrust_cpython.so ../builds/fibrust_cpython.so
	cd fibcython && python setup.py build_ext --inplace && cp fibcython.cpython-*.so ../builds/

test: build
	PYTHONPATH=./builds pytest -s -v test_fib.py
