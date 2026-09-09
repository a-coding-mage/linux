// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/tests/comedi_example_test.c
 *  Example set of unit tests.
 *
 *  COMEDI - Linux Control and Measurement Device Interface
 *  Copyright (C) 2016 Spencer E. Olson <olsonse@umich.edu>
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

// C dependencies: <linux/module.h> and "unittest.h".

/* *** BEGIN fake board data *** */
#[repr(C)]
pub struct comedi_device {
    pub board_name: *const c_char,
    pub item: c_int,
}

static mut dev: comedi_device = comedi_device {
    board_name: b"fake_device\0".as_ptr() as *const c_char,
    item: 0,
};

/* *** END fake board data *** */

/* *** BEGIN fake data init *** */
unsafe fn init_fake() {
    dev.item = 10;
}

/* *** END fake data init *** */

// Supplied by unittest.h; the implementation and assertion semantics remain external.
pub type unittest_fptr = unsafe extern "C" fn();

unsafe extern "C" {
    fn unittest(condition: bool, message: *const c_char);
    fn exec_unittests(name: *const c_char, tests: *const unittest_fptr);
}

unsafe fn test0() {
    init_fake();
    unittest(dev.item != 11, b"negative result\n\0".as_ptr() as *const c_char);
    unittest(dev.item == 10, b"positive result\n\0".as_ptr() as *const c_char);
}

/* **** BEGIN simple module entry/exit functions **** */
unsafe extern "C" fn unittest_enter() -> c_int {
    static unit_tests: [Option<unittest_fptr>; 2] = [Some(test0), None];

    exec_unittests(
        b"example\0".as_ptr() as *const c_char,
        unit_tests.as_ptr() as *const unittest_fptr,
    );
    0
}

unsafe extern "C" fn unittest_exit() {}

// C module_init(unittest_enter) and module_exit(unittest_exit) registrations.
// MODULE_AUTHOR("Spencer Olson <olsonse@umich.edu>");
// MODULE_DESCRIPTION("Comedi unit-tests example");
// MODULE_LICENSE("GPL");
/* **** END simple module entry/exit functions **** */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
