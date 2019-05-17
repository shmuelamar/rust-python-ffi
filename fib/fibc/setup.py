#!/usr/bin/env python3
from distutils.core import setup, Extension

fibc_module = Extension('fibc', sources=['fibc.c'])

setup(
    name='fibc',
    version='0.1.0',
    description='fibc module in c',
    ext_modules=[fibc_module],
)
