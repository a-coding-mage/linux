// SPDX-License-Identifier: GPL-2.0
// C dependencies: <string.h>, "tests/tests.h", "arch-tests.h"

#[repr(C)]
pub struct test_suite {
    _unused: [u8; 0],
}

extern "C" {
    static suite__dwarf_unwind: test_suite;
    static suite__vectors_page: test_suite;
}

#[no_mangle]
pub static mut arch_tests: [*mut test_suite; 3] = [
    // Present in C when HAVE_DWARF_UNWIND_SUPPORT is defined.
    unsafe { &suite__dwarf_unwind as *const test_suite as *mut test_suite },
    unsafe { &suite__vectors_page as *const test_suite as *mut test_suite },
    core::ptr::null_mut(),
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
