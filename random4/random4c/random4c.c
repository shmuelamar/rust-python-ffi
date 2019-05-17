#include <Python.h>

static PyObject* random4(PyObject *self, PyObject *args) {
    return PyLong_FromLong(4);
}

static PyMethodDef module_methods[] = {
    {"random4", random4, METH_NOARGS, "use to get pseudo random number"},
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef random4c_definition = {
    PyModuleDef_HEAD_INIT, "random4c", "random4 module", -1, module_methods
};

PyMODINIT_FUNC PyInit_random4c(void) {
    Py_Initialize();
    return PyModule_Create(&random4c_definition);
}
