// SPDX-License-Identifier: GPL-2.0
// C source included <Python.h> for Py_Initialize.

extern "C" {
    fn Py_Initialize();
}

fn main() {
    unsafe {
        Py_Initialize();
    }
}

// C source ended with: #undef _GNU_SOURCE
