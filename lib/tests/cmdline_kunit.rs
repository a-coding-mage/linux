// SPDX-License-Identifier: GPL-2.0+
/*
 * Test cases for API provided by cmdline.c
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_case {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub test_cases: *mut kunit_case,
}

extern "C" {
    fn get_option(args: *mut *mut c_char, option: *mut c_int) -> c_int;
    fn get_options(option: *const c_char, nints: c_int, ints: *mut c_int) -> c_int;
    fn next_arg(args: *mut c_char, param: *mut *mut c_char, val: *mut *mut c_char)
        -> *mut c_char;
    fn memparse(ptr: *const c_char, retptr: *mut *mut c_char) -> u64;
    fn get_random_u8() -> u8;
    fn memchr_inv(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
    fn kunit_test_suite(suite: *mut kunit_suite);
    fn kunit_expect_eq_msg(test: *mut kunit, left: i64, right: i64, fmt: *const c_char, ...);
    fn kunit_expect_ptr_eq_msg(
        test: *mut kunit,
        left: *const c_void,
        right: *const c_void,
        fmt: *const c_char,
        ...,
    );
    fn kunit_expect_streq(test: *mut kunit, left: *const c_char, right: *const c_char);
    fn kunit_assert_not_null(test: *mut kunit, ptr: *const c_void);
    fn kunit_expect_null(test: *mut kunit, ptr: *const c_void);
}

const CMDLINE_TEST_STRINGS: &[&[u8]] = &[
    b"\"\"\0", b"\0", b"=\0", b"\"-\0", b",\0", b"-,\0", b",-\0", b"-\0",
    b"+,\0", b"--\0", b",,\0", b"''\0", b"\"\",\0", b"\",\"\0", b"-\"\"\0", b"\"\0",
];

const CMDLINE_TEST_VALUES: &[c_int] = &[1, 1, 1, 1, 2, 3, 2, 3, 1, 3, 2, 1, 1, 1, 3, 1];

const CMDLINE_TEST_RANGE_STRINGS: &[&[u8]] = &[
    b"-7\0", b"--7\0", b"-1-2\0", b"7--9\0", b"7-\0", b"-7--9\0", b"7-9,\0", b"9-7\0",
    b"5-a\0", b"a-5\0", b"5-8\0", b",8-5\0", b"+,1\0", b"-,4\0", b"-3,0-1,6\0", b"4,-\0",
    b" +2\0", b" -9\0", b"0-1,-3,6\0", b"- 9\0",
];

const CMDLINE_TEST_RANGE_VALUES: [[c_int; 16]; 20] = [
    [1, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [4, -1, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [3, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [4, -3, 0, 1, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [4, 0, 1, -3, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

#[repr(C)]
struct CmdlineTestMemparseEntry {
    input: *const c_char,
    unrecognized: *const c_char,
    result: u64,
}

const SZ_1K: u64 = 1024;
const SZ_1M: u64 = 1024 * SZ_1K;
const SZ_1G: u64 = 1024 * SZ_1M;
const SZ_1T: u64 = 1024 * SZ_1G;
const SZ_4K: u64 = 4 * SZ_1K;
const SZ_16K: u64 = 16 * SZ_1K;
const SZ_32M: u64 = 32 * SZ_1M;
const SZ_2G: u64 = 2 * SZ_1G;
const SZ_4T: u64 = 4 * SZ_1T;

static TESTDATA: &[CmdlineTestMemparseEntry] = &[
    CmdlineTestMemparseEntry { input: b"0\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: 0 },
    CmdlineTestMemparseEntry { input: b"1\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: 1 },
    CmdlineTestMemparseEntry { input: b"a\0".as_ptr() as _, unrecognized: b"a\0".as_ptr() as _, result: 0 },
    CmdlineTestMemparseEntry { input: b"k\0".as_ptr() as _, unrecognized: b"k\0".as_ptr() as _, result: 0 },
    CmdlineTestMemparseEntry { input: b"E\0".as_ptr() as _, unrecognized: b"E\0".as_ptr() as _, result: 0 },
    CmdlineTestMemparseEntry { input: b"0xb\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: 11 },
    CmdlineTestMemparseEntry { input: b"0xz\0".as_ptr() as _, unrecognized: b"x\0".as_ptr() as _, result: 0 },
    CmdlineTestMemparseEntry { input: b"1234\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: 1234 },
    CmdlineTestMemparseEntry { input: b"04567\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: 2423 },
    CmdlineTestMemparseEntry { input: b"0x9876\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: 39030 },
    CmdlineTestMemparseEntry { input: b"05678\0".as_ptr() as _, unrecognized: b"8\0".as_ptr() as _, result: 375 },
    CmdlineTestMemparseEntry { input: b"0xabcdefz\0".as_ptr() as _, unrecognized: b"z\0".as_ptr() as _, result: 11259375 },
    CmdlineTestMemparseEntry { input: b"0cdba\0".as_ptr() as _, unrecognized: b"c\0".as_ptr() as _, result: 0 },
    CmdlineTestMemparseEntry { input: b"4K\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: SZ_4K },
    CmdlineTestMemparseEntry { input: b"0x10k@0xaaaabbbb\0".as_ptr() as _, unrecognized: b"@\0".as_ptr() as _, result: SZ_16K },
    CmdlineTestMemparseEntry { input: b"32M\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: SZ_32M },
    CmdlineTestMemparseEntry { input: b"067m:foo\0".as_ptr() as _, unrecognized: b":\0".as_ptr() as _, result: 55 * SZ_1M },
    CmdlineTestMemparseEntry { input: b"2G;bar=baz\0".as_ptr() as _, unrecognized: b";\0".as_ptr() as _, result: SZ_2G },
    CmdlineTestMemparseEntry { input: b"07gz\0".as_ptr() as _, unrecognized: b"z\0".as_ptr() as _, result: 7 * SZ_1G },
    CmdlineTestMemparseEntry { input: b"3T+data\0".as_ptr() as _, unrecognized: b"+\0".as_ptr() as _, result: 3 * SZ_1T },
    CmdlineTestMemparseEntry { input: b"04t,ro\0".as_ptr() as _, unrecognized: b",\0".as_ptr() as _, result: SZ_4T },
    CmdlineTestMemparseEntry { input: b"012p\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: 11258999068426240 },
    CmdlineTestMemparseEntry { input: b"7P,sync\0".as_ptr() as _, unrecognized: b",\0".as_ptr() as _, result: 7881299347898368 },
    CmdlineTestMemparseEntry { input: b"0x2e\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: 46 },
    CmdlineTestMemparseEntry { input: b"2E and more\0".as_ptr() as _, unrecognized: b" \0".as_ptr() as _, result: 2305843009213693952 },
    CmdlineTestMemparseEntry { input: b"18446744073709551615\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: u64::MAX },
    CmdlineTestMemparseEntry { input: b"0xffffffffffffffff0\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: u64::MAX },
    CmdlineTestMemparseEntry { input: b"1111111111111111111T\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: u64::MAX },
    CmdlineTestMemparseEntry { input: b"222222222222222222222G\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: u64::MAX },
    CmdlineTestMemparseEntry { input: b"3333333333333333333333M\0".as_ptr() as _, unrecognized: b"\0".as_ptr() as _, result: u64::MAX },
];

unsafe fn cmdline_do_one_test(_test: *mut kunit, _input: *const c_char, _rc: c_int, _offset: c_int) {}
unsafe fn cmdline_test_noint(_test: *mut kunit) {}
unsafe fn cmdline_test_lead_int(_test: *mut kunit) {}
unsafe fn cmdline_test_tail_int(_test: *mut kunit) {}
unsafe fn cmdline_do_one_range_test(
    _test: *mut kunit,
    _input: *const c_char,
    _n: u32,
    _expected: *const c_int,
) {}
unsafe fn cmdline_test_range(_test: *mut kunit) {}
unsafe fn cmdline_test_next_arg_quoted_value(_test: *mut kunit) {}
unsafe fn cmdline_test_next_arg_bare_quote_regression(_test: *mut kunit) {}
unsafe fn cmdline_test_next_arg_mixed_tokens(_test: *mut kunit) {}
unsafe fn cmdline_test_memparse(_test: *mut kunit) {}

// KUnit registration is supplied by the kernel integration layer.
#[no_mangle]
pub static mut cmdline_test_suite: kunit_suite = kunit_suite {
    name: b"cmdline\0".as_ptr() as *const c_char,
    test_cases: core::ptr::null_mut(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
