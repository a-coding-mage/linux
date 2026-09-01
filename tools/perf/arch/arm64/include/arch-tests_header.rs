/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct test_suite {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn test__cpuid_match(test: *mut test_suite, subtest: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub static mut arch_tests: *mut *mut test_suite;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
