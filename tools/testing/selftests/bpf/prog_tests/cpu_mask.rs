// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <bpf/btf.h>, "bpf/libbpf_internal.h"

use core::ffi::{c_char, c_int, c_void};

static mut duration: c_int = 0;

unsafe extern "C" {
    fn CHECK(condition: bool, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn parse_cpu_mask_str(s: *const c_char, mask: *mut *mut bool, n: *mut c_int) -> c_int;
    fn free(ptr: *mut c_void);
}

unsafe fn validate_mask(case_nr: c_int, exp: *const c_char, mask: *mut bool, n: c_int) {
    let mut i: c_int;

    i = 0;
    while *exp.offset(i as isize) != 0 {
        if *exp.offset(i as isize) as c_int == '1' as c_int {
            if CHECK(
                i + 1 > n,
                b"mask_short\0".as_ptr() as *const c_char,
                b"case #%d: mask too short, got n=%d, need at least %d\n\0".as_ptr()
                    as *const c_char,
                case_nr,
                n,
                i + 1,
            ) != 0
            {
                return;
            }
            CHECK(
                !*mask.offset(i as isize),
                b"cpu_not_set\0".as_ptr() as *const c_char,
                b"case #%d: mask differs, expected cpu#%d SET\n\0".as_ptr() as *const c_char,
                case_nr,
                i,
            );
        } else {
            CHECK(
                i < n && *mask.offset(i as isize),
                b"cpu_set\0".as_ptr() as *const c_char,
                b"case #%d: mask differs, expected cpu#%d UNSET\n\0".as_ptr() as *const c_char,
                case_nr,
                i,
            );
        }
        i += 1;
    }
    CHECK(
        i < n,
        b"mask_long\0".as_ptr() as *const c_char,
        b"case #%d: mask too long, got n=%d, expected at most %d\n\0".as_ptr() as *const c_char,
        case_nr,
        n,
        i,
    );
}

struct TestCase {
    cpu_mask: &'static [u8],
    expect: &'static [u8],
    fails: bool,
}

static test_cases: &[TestCase] = &[
    TestCase {
        cpu_mask: b"0\n\0",
        expect: b"1\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"0,2\n\0",
        expect: b"101\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"0-2\n\0",
        expect: b"111\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"0-2,3-4\n\0",
        expect: b"11111\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"0\0",
        expect: b"1\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"0-2\0",
        expect: b"111\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"0,2\0",
        expect: b"101\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"0,1-3\0",
        expect: b"1111\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"0,1,2,3\0",
        expect: b"1111\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"0,2-3,5\0",
        expect: b"101101\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"3-3\0",
        expect: b"0001\0",
        fails: false,
    },
    TestCase {
        cpu_mask: b"2-4,6,9-10\0",
        expect: b"00111010011\0",
        fails: false,
    },
    /* failure cases */
    TestCase {
        cpu_mask: b"\0",
        expect: b"\0",
        fails: true,
    },
    TestCase {
        cpu_mask: b"0-\0",
        expect: b"\0",
        fails: true,
    },
    TestCase {
        cpu_mask: b"0 \0",
        expect: b"\0",
        fails: true,
    },
    TestCase {
        cpu_mask: b"0_1\0",
        expect: b"\0",
        fails: true,
    },
    TestCase {
        cpu_mask: b"1-0\0",
        expect: b"\0",
        fails: true,
    },
    TestCase {
        cpu_mask: b"-1\0",
        expect: b"\0",
        fails: true,
    },
];

#[no_mangle]
pub unsafe extern "C" fn test_cpu_mask() {
    let mut i: usize;
    let mut err: c_int;
    let mut n: c_int = 0;
    let mut mask: *mut bool;

    i = 0;
    while i < test_cases.len() {
        mask = core::ptr::null_mut();
        err = parse_cpu_mask_str(
            test_cases[i].cpu_mask.as_ptr() as *const c_char,
            &mut mask,
            &mut n,
        );
        if test_cases[i].fails {
            CHECK(
                err == 0,
                b"should_fail\0".as_ptr() as *const c_char,
                b"case #%d: parsing should fail!\n\0".as_ptr() as *const c_char,
                i as c_int + 1,
            );
        } else {
            if CHECK(
                err != 0,
                b"parse_err\0".as_ptr() as *const c_char,
                b"case #%d: cpu mask parsing failed: %d\n\0".as_ptr() as *const c_char,
                i as c_int + 1,
                err,
            ) != 0
            {
                i += 1;
                continue;
            }
            validate_mask(
                i as c_int + 1,
                test_cases[i].expect.as_ptr() as *const c_char,
                mask,
                n,
            );
        }
        free(mask as *mut c_void);
        i += 1;
    }
}
