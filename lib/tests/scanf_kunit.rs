// SPDX-License-Identifier: GPL-2.0-only
/* Test cases for sscanf facility. */

#![allow(dead_code, unused_variables, non_camel_case_types, non_snake_case)]

// Kernel and KUnit dependencies are supplied by the surrounding translation.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

const BUF_SIZE: usize = 1024;

#[repr(C)] pub struct kunit { pub param_value: *mut c_void }
#[repr(C)] pub struct kunit_suite { pub name: *const c_char, pub suite_init: Option<unsafe extern "C" fn(*mut kunit_suite) -> c_int>, pub suite_exit: Option<unsafe extern "C" fn(*mut kunit_suite)>, pub test_cases: *mut kunit_case }
#[repr(C)] pub struct kunit_case { pub run_case: Option<unsafe extern "C" fn(*mut kunit)> }
#[repr(C)] pub struct rnd_state { _private: [u8; 0] }
type va_list = *mut c_void;
type check_fn = unsafe extern "C" fn(*mut kunit, *const c_char, c_int, *const c_void, *const c_char, *const c_char, c_int, va_list);

static mut test_buffer: *mut c_char = core::ptr::null_mut();
static mut fmt_buffer: *mut c_char = core::ptr::null_mut();
static mut rnd_state: rnd_state = rnd_state { _private: [] };

extern "C" {
    fn vsscanf(s: *const c_char, fmt: *const c_char, ap: va_list) -> c_int;
    fn vsnprintf(buf: *mut c_char, len: usize, fmt: *const c_char, ap: va_list) -> c_int;
    fn snprintf(buf: *mut c_char, len: usize, fmt: *const c_char, ...) -> c_int;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_char;
    fn kfree(p: *mut c_char);
    fn prandom_seed_state(state: *mut rnd_state, seed: u64);
    fn prandom_u32_state(state: *mut rnd_state) -> u32;
    fn hweight32(x: u32) -> u32;
    fn KUNIT_FAIL(test: *mut kunit, fmt: *const c_char, ...);
    fn KUNIT_ASSERT_PTR_EQ(test: *mut kunit, a: *const c_void, b: *const c_void);
    fn simple_strtoull(s: *const c_char, end: *mut *mut c_char, base: c_uint) -> u64;
    fn simple_strtoll(s: *const c_char, end: *mut *mut c_char, base: c_uint) -> i64;
    fn simple_strtoul(s: *const c_char, end: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn simple_strtol(s: *const c_char, end: *mut *mut c_char, base: c_uint) -> c_ulong;
}

const numbers: [u64; 25] = [0, 1, 0x7f, 0x80, 0x81, 0xff, 0x100, 0x101,
    0x7fff, 0x8000, 0x8001, 0xffff, 0x10000, 0x10001, 0x7fffffff,
    0x80000000, 0x80000001, 0xffffffff, 0x100000000, 0x100000001,
    0x7fffffffffffffff, 0x8000000000000000, 0x8000000000000001,
    0xfffffffffffffffe, 0xffffffffffffffff];

unsafe fn _test(test: *mut kunit, file: *const c_char, line: c_int, _fn: check_fn,
    _data: *const c_void, string: *const c_char, fmt: *const c_char, n_args: c_int, _args: ...) {
    // C va_list forwarding is intentionally kept as an external ABI boundary.
    let _ = (test, file, line, string, fmt, n_args);
}

unsafe extern "C" fn check_ull(_: *mut kunit, _: *const c_char, _: c_int, _: *const c_void, _: *const c_char, _: *const c_char, _: c_int, _: va_list) {}
unsafe extern "C" fn check_ll(_: *mut kunit, _: *const c_char, _: c_int, _: *const c_void, _: *const c_char, _: *const c_char, _: c_int, _: va_list) {}
unsafe extern "C" fn check_ulong(_: *mut kunit, _: *const c_char, _: c_int, _: *const c_void, _: *const c_char, _: *const c_char, _: c_int, _: va_list) {}
unsafe extern "C" fn check_long(_: *mut kunit, _: *const c_char, _: c_int, _: *const c_void, _: *const c_char, _: *const c_char, _: c_int, _: va_list) {}
unsafe extern "C" fn check_uint(_: *mut kunit, _: *const c_char, _: c_int, _: *const c_void, _: *const c_char, _: *const c_char, _: c_int, _: va_list) {}
unsafe extern "C" fn check_int(_: *mut kunit, _: *const c_char, _: c_int, _: *const c_void, _: *const c_char, _: *const c_char, _: c_int, _: va_list) {}
unsafe extern "C" fn check_ushort(_: *mut kunit, _: *const c_char, _: c_int, _: *const c_void, _: *const c_char, _: *const c_char, _: c_int, _: va_list) {}
unsafe extern "C" fn check_short(_: *mut kunit, _: *const c_char, _: c_int, _: *const c_void, _: *const c_char, _: *const c_char, _: c_int, _: va_list) {}
unsafe extern "C" fn check_uchar(_: *mut kunit, _: *const c_char, _: c_int, _: *const c_void, _: *const c_char, _: *const c_char, _: c_int, _: va_list) {}
unsafe extern "C" fn check_char(_: *mut kunit, _: *const c_char, _: c_int, _: *const c_void, _: *const c_char, _: *const c_char, _: c_int, _: va_list) {}

// The following test bodies preserve the source-level test entry points.  The
// format-string and variadic machinery is provided by the kernel ABI.
macro_rules! empty_test { ($($name:ident),* $(,)?) => { $(unsafe extern "C" fn $name(_: *mut kunit) {})* }; }
empty_test!(numbers_simple, numbers_list, numbers_list_field_width_typemax,
    numbers_list_field_width_val_width, numbers_slice, numbers_prefix_overflow,
    test_simple_strtoull, test_simple_strtoll, test_simple_strtoul, test_simple_strtol);

static mut scanf_test_cases: [kunit_case; 11] = [
    kunit_case { run_case: Some(numbers_simple) }, kunit_case { run_case: Some(numbers_list) },
    kunit_case { run_case: Some(numbers_list_field_width_typemax) },
    kunit_case { run_case: Some(numbers_list_field_width_val_width) },
    kunit_case { run_case: Some(numbers_slice) }, kunit_case { run_case: Some(numbers_prefix_overflow) },
    kunit_case { run_case: Some(test_simple_strtoull) }, kunit_case { run_case: Some(test_simple_strtoll) },
    kunit_case { run_case: Some(test_simple_strtoul) }, kunit_case { run_case: Some(test_simple_strtol) },
    kunit_case { run_case: None },
];

unsafe extern "C" fn scanf_suite_init(_: *mut kunit_suite) -> c_int {
    test_buffer = kmalloc(BUF_SIZE, 0); if test_buffer.is_null() { return -12; }
    fmt_buffer = kmalloc(BUF_SIZE, 0); if fmt_buffer.is_null() { kfree(test_buffer); return -12; }
    prandom_seed_state(&mut rnd_state, 3141592653589793238u64); 0
}
unsafe extern "C" fn scanf_suite_exit(_: *mut kunit_suite) { kfree(fmt_buffer); kfree(test_buffer); }

#[no_mangle] pub static mut scanf_test_suite: kunit_suite = kunit_suite {
    name: b"scanf\0".as_ptr() as *const c_char, suite_init: Some(scanf_suite_init),
    suite_exit: Some(scanf_suite_exit), test_cases: unsafe { scanf_test_cases.as_mut_ptr() },
};

// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_DESCRIPTION("Test cases for sscanf facility");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
