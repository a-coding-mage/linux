/* SPDX-License-Identifier: GPL-2.0+ */
/*
 *  comedi/drivers/tests/unittest.h
 *  Simple framework for unittests for comedi drivers.
 *
 *  COMEDI - Linux Control and Measurement Device Interface
 *  Copyright (C) 2016 Spencer E. Olson <olsonse@umich.edu>
 *  based of parts of drivers/of/unittest.c
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct unittest_results {
    pub passed: c_int,
    pub failed: c_int,
}

pub static mut unittest_results: unittest_results = unittest_results {
    passed: 0,
    failed: 0,
};

pub type unittest_fptr = Option<unsafe extern "C" fn()>;

/* The logging functions and their format-string dependencies are supplied externally. */
extern "C" {
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
}

macro_rules! c_string {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

#[macro_export]
macro_rules! unittest {
    ($result:expr, $fmt:literal $(, $args:expr)* $(,)?) => {{
        let failed = !($result);
        if failed {
            unsafe {
                $crate::unittest_results.failed =
                    $crate::unittest_results.failed.wrapping_add(1);
                $crate::pr_err(
                    c_string!("FAIL %s():%i " $fmt),
                    concat!(module_path!(), "\0").as_ptr(),
                    line!() as c_int
                    $(, $args)*,
                );
            }
        } else {
            unsafe {
                $crate::unittest_results.passed =
                    $crate::unittest_results.passed.wrapping_add(1);
                $crate::pr_debug(
                    c_string!("pass %s():%i " $fmt),
                    concat!(module_path!(), "\0").as_ptr(),
                    line!() as c_int
                    $(, $args)*,
                );
            }
        }
        failed
    }};
}

/**
 * Execute an array of unit tests.
 * @name: Name of set of unit tests--will be shown at INFO log level.
 * @unit_tests: A null-terminated list of unit tests to execute.
 */
pub unsafe fn exec_unittests(name: *const c_char, unit_tests: *const unittest_fptr) {
    pr_info(c_string!("begin comedi:\"%s\" unittests\n"), name);

    let mut unit_tests = unit_tests;
    while (*unit_tests).is_some() {
        ((*unit_tests).unwrap())();
        unit_tests = unit_tests.add(1);
    }

    pr_info(
        c_string!("end of comedi:\"%s\" unittests - %i passed, %i failed\n"),
        name,
        unittest_results.passed,
        unittest_results.failed,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
