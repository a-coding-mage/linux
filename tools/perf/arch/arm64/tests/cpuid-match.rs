// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/compiler.h>, "arch-tests.h", "tests/tests.h", "util/header.h"

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct test_suite {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn strcmp_cpuid_str(mapcpuid: *const c_char, idstr: *const c_char) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn test__cpuid_match(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;

    /* midr with no leading zeros matches */
    if strcmp_cpuid_str(
        b"0x410fd0c0\0".as_ptr() as *const c_char,
        b"0x00000000410fd0c0\0".as_ptr() as *const c_char,
    ) != 0
    {
        return -1;
    }
    /* Upper case matches */
    if strcmp_cpuid_str(
        b"0x410fd0c0\0".as_ptr() as *const c_char,
        b"0x00000000410FD0C0\0".as_ptr() as *const c_char,
    ) != 0
    {
        return -1;
    }
    /* r0p0 = r0p0 matches */
    if strcmp_cpuid_str(
        b"0x00000000410fd480\0".as_ptr() as *const c_char,
        b"0x00000000410fd480\0".as_ptr() as *const c_char,
    ) != 0
    {
        return -1;
    }
    /* r0p1 > r0p0 matches */
    if strcmp_cpuid_str(
        b"0x00000000410fd480\0".as_ptr() as *const c_char,
        b"0x00000000410fd481\0".as_ptr() as *const c_char,
    ) != 0
    {
        return -1;
    }
    /* r1p0 > r0p0 matches*/
    if strcmp_cpuid_str(
        b"0x00000000410fd480\0".as_ptr() as *const c_char,
        b"0x00000000411fd480\0".as_ptr() as *const c_char,
    ) != 0
    {
        return -1;
    }
    /* r0p0 < r0p1 doesn't match */
    if strcmp_cpuid_str(
        b"0x00000000410fd481\0".as_ptr() as *const c_char,
        b"0x00000000410fd480\0".as_ptr() as *const c_char,
    ) == 0
    {
        return -1;
    }
    /* r0p0 < r1p0 doesn't match */
    if strcmp_cpuid_str(
        b"0x00000000411fd480\0".as_ptr() as *const c_char,
        b"0x00000000410fd480\0".as_ptr() as *const c_char,
    ) == 0
    {
        return -1;
    }
    /* Different CPU doesn't match */
    if strcmp_cpuid_str(
        b"0x00000000410fd4c0\0".as_ptr() as *const c_char,
        b"0x00000000430f0af0\0".as_ptr() as *const c_char,
    ) == 0
    {
        return -1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
