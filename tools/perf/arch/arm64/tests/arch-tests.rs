// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// #include <string.h>
// #include "tests/tests.h"
// #include "arch-tests.h"

extern "C" {
    static mut suite__dwarf_unwind: test_suite;
    static mut suite__cpuid_match: test_suite;
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

// DEFINE_SUITE("arm64 CPUID matching", cpuid_match);
// The suite object is provided by the translated equivalent of DEFINE_SUITE.

// Original C condition: #ifdef HAVE_DWARF_UNWIND_SUPPORT
#[cfg(HAVE_DWARF_UNWIND_SUPPORT)]
#[no_mangle]
pub static mut arch_tests: [*mut test_suite; 3] = [
    unsafe { &mut suite__dwarf_unwind as *mut test_suite },
    unsafe { &mut suite__cpuid_match as *mut test_suite },
    core::ptr::null_mut(),
];

#[cfg(not(HAVE_DWARF_UNWIND_SUPPORT))]
#[no_mangle]
pub static mut arch_tests: [*mut test_suite; 2] = [
    unsafe { &mut suite__cpuid_match as *mut test_suite },
    core::ptr::null_mut(),
];
