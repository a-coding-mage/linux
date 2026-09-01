// SPDX-License-Identifier: GPL-2.0
// C dependencies: <inttypes.h>, <linux/compiler.h>, <linux/types.h>,
// <string.h>, "tests.h", "units.h", "debug.h"

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

pub type u64 = u64;

pub const TEST_FAIL: c_int = -1;
pub const TEST_OK: c_int = 0;

unsafe extern "C" {
    fn unit_number__scnprintf(buf: *mut c_char, size: usize, n: u64) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[repr(C)]
struct UnitNumberScnprintfTest {
    n: u64,
    str_: *const c_char,
}

unsafe extern "C" fn test__unit_number__scnprint(
    _t: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let test = [
        UnitNumberScnprintfTest {
            n: 1,
            str_: c"1B".as_ptr(),
        },
        UnitNumberScnprintfTest {
            n: 10 * 1024,
            str_: c"10K".as_ptr(),
        },
        UnitNumberScnprintfTest {
            n: 20 * 1024 * 1024,
            str_: c"20M".as_ptr(),
        },
        UnitNumberScnprintfTest {
            n: 30 * 1024 * 1024 * 1024u64,
            str_: c"30G".as_ptr(),
        },
        UnitNumberScnprintfTest {
            n: 0,
            str_: c"0B".as_ptr(),
        },
        UnitNumberScnprintfTest {
            n: 0,
            str_: core::ptr::null(),
        },
    ];
    let mut i: u32 = 0;

    while !test[i as usize].str_.is_null() {
        let mut buf = [0 as c_char; 100];

        unit_number__scnprintf(
            buf.as_mut_ptr(),
            core::mem::size_of_val(&buf),
            test[i as usize].n,
        );

        // C macro call preserved for dependency context:
        // pr_debug("n %" PRIu64 ", str '%s', buf '%s'\n",
        //          test[i].n, test[i].str, buf);

        if strcmp(test[i as usize].str_, buf.as_ptr()) != 0 {
            return TEST_FAIL;
        }

        i += 1;
    }

    TEST_OK
}

// C suite registration macro:
// DEFINE_SUITE("unit_number__scnprintf", unit_number__scnprint);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
