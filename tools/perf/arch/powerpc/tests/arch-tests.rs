// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <string.h>
// #include "tests/tests.h"
// #include "arch-tests.h"

unsafe extern "C" {
    pub static mut suite__dwarf_unwind: test_suite;
}

// Supplied by the translated headers/dependencies corresponding to tests/tests.h.
pub type test_suite = core::ffi::c_void;

#[cfg(HAVE_DWARF_UNWIND_SUPPORT)]
pub static mut arch_tests: [*mut test_suite; 2] = [
    unsafe { core::ptr::addr_of_mut!(suite__dwarf_unwind) },
    core::ptr::null_mut(),
];

#[cfg(not(HAVE_DWARF_UNWIND_SUPPORT))]
pub static mut arch_tests: [*mut test_suite; 1] = [core::ptr::null_mut()];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
