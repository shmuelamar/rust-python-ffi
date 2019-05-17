#!/usr/bin/env python3
from distutils.core import setup, Extension

random4_module = Extension(
    'random4c',
    sources=['random4c.c'],
)

setup(
    name='random4c',
    version='0.1.0',
    description='A module written in C',
    ext_modules=[random4_module],
)
