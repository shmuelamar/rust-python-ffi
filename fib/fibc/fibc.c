#include <Python.h>

unsigned long sum_fib_even(unsigned long max_num) {
    unsigned long a = 0, b = 1, tmp;
    unsigned long sum = 0;

    while(a < max_num) {
        if(a % 2 == 0){
            sum += a;
        }
        tmp = a;
        a = b;
        b += tmp;
    }

    return sum;
}

unsigned long sum_fib_10k(unsigned long max_num) {
    unsigned long res;

    for (int i=0; i < 10000; i++) {
        res = sum_fib_even(max_num);
    }
    return res;
}

static PyObject* sum_fib_even_py(PyObject *self, PyObject *args) {
    unsigned long max_num;

    if (!PyArg_ParseTuple(args, "k", &max_num)) {
        return NULL;
    }

    unsigned long res = sum_fib_10k(max_num);
    return PyLong_FromLong(res);
}

static PyMethodDef module_methods[] = {
    {"sum_fib_10k", sum_fib_even_py, METH_VARARGS, "fibonacci series module"},
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef fibc_definition = {
    PyModuleDef_HEAD_INIT, "fibc", "fib module", -1, module_methods
};

PyMODINIT_FUNC PyInit_fibc(void) {
    Py_Initialize();
    return PyModule_Create(&fibc_definition);
}
