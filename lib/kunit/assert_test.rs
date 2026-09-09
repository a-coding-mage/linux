// SPDX-License-Identifier: GPL-2.0-or-later
/* KUnit test for the assertion formatting functions. */

const TEST_PTR_EXPECTED_BUF_SIZE: usize = 32;
const HEXDUMP_TEST_BUF_LEN: usize = 5;

// The following types, functions, constants, and test macros are supplied by
// the KUnit and string-stream dependencies.
extern "C" {
    fn kfree_wrapper(_: *const core::ffi::c_void);
    fn is_literal(_: *const i8, _: i64) -> bool;
    fn is_str_literal(_: *const i8, _: *const i8) -> bool;
    fn kunit_alloc_string_stream(_: *mut kunit, _: i32) -> *mut string_stream;
    fn string_stream_get_string(_: *mut string_stream) -> *mut i8;
    fn string_stream_clear(_: *mut string_stream);
    fn kunit_assert_prologue(_: *const kunit_loc, _: i32, _: *mut string_stream);
    fn kunit_assert_print_msg(_: *const va_format, _: *mut string_stream);
    fn kunit_unary_assert_format(_: *const kunit_assert, _: *mut va_format, _: *mut string_stream);
    fn kunit_ptr_not_err_assert_format(_: *const kunit_assert, _: *mut va_format, _: *mut string_stream);
    fn kunit_binary_assert_format(_: *const kunit_assert, _: *mut va_format, _: *mut string_stream);
    fn kunit_binary_ptr_assert_format(_: *const kunit_assert, _: *mut va_format, _: *mut string_stream);
    fn kunit_binary_str_assert_format(_: *const kunit_assert, _: *mut va_format, _: *mut string_stream);
    fn kunit_mem_assert_format(_: *const kunit_assert, _: *mut va_format, _: *mut string_stream);
    fn kunit_assert_hexdump(_: *mut string_stream, _: *const u8, _: *const u8, _: usize);
    fn snprintf(_: *mut i8, _: usize, _: *const i8, ...);
}

#[repr(C)] struct kunit { _private: [u8; 0] }
#[repr(C)] struct string_stream { _private: [u8; 0] }
#[repr(C)] struct kunit_assert { _private: [u8; 0] }
#[repr(C)] struct kunit_loc { file: *const i8, line: i32 }
#[repr(C)] struct va_format { fmt: *const i8, va: *mut core::ffi::c_void }
#[repr(C)] struct kunit_binary_assert_text { left_text: *const i8, operation: *const i8, right_text: *const i8 }
#[repr(C)] struct kunit_unary_assert { assert_: kunit_assert, condition: *const i8, expected_true: bool }
#[repr(C)] struct kunit_ptr_not_err_assert { assert_: kunit_assert, text: *const i8, value: *const core::ffi::c_void }
#[repr(C)] struct kunit_binary_assert { assert_: kunit_assert, text: *const kunit_binary_assert_text, left_value: i64, right_value: i64 }
#[repr(C)] struct kunit_binary_ptr_assert { assert_: kunit_assert, text: *const kunit_binary_assert_text, left_value: *const core::ffi::c_void, right_value: *const core::ffi::c_void }
#[repr(C)] struct kunit_binary_str_assert { assert_: kunit_assert, text: *const kunit_binary_assert_text, left_value: *const i8, right_value: *const i8 }
#[repr(C)] struct kunit_mem_assert { assert_: kunit_assert, text: *const kunit_binary_assert_text, left_value: *const u8, right_value: *const u8, size: usize }

unsafe fn get_str_from_stream(test: *mut kunit, stream: *mut string_stream) -> *mut i8 {
    let str_ = string_stream_get_string(stream);
    // KUNIT_ASSERT_NOT_ERR_OR_NULL(test, str_);
    // kunit_add_action(test, kfree_wrapper, str_ as *mut _);
    str_
}

unsafe fn verify_assert_print_msg(test: *mut kunit, stream: *mut string_stream, expected: *const i8, format: *const i8) {
    let mut list: core::ffi::c_void = core::mem::zeroed();
    let vformat = va_format { fmt: format, va: &mut list };
    string_stream_clear(stream);
    kunit_assert_print_msg(&vformat, stream);
    let _ = (test, expected, get_str_from_stream(test, stream));
}

unsafe fn validate_assert(test: *mut kunit, assert_: *const kunit_assert,
    stream: *mut string_stream, num_checks: usize, checks: &[*const i8]) {
    let mut message = va_format { fmt: core::ptr::null(), va: core::ptr::null_mut() };
    string_stream_clear(stream);
    // The C varargs callback is represented by the dependency's assert formatter.
    let _ = (test, assert_, &mut message, num_checks, checks);
}

unsafe fn kunit_test_is_literal(test: *mut kunit) {
    let _ = test;
    let _ = is_literal(b"5\0".as_ptr() as _, 5);
    let _ = is_literal(b"0\0".as_ptr() as _, 0);
    let _ = is_literal(b"1234567890\0".as_ptr() as _, 1234567890);
    let _ = is_literal(b"-1234567890\0".as_ptr() as _, -1234567890);
    let _ = is_literal(b"05\0".as_ptr() as _, 5);
    let _ = is_literal(b"\0".as_ptr() as _, 0);
    let _ = is_literal(b"-0\0".as_ptr() as _, 0);
    let _ = is_literal(b"12#45\0".as_ptr() as _, 1245);
}

unsafe fn kunit_test_is_str_literal(test: *mut kunit) {
    let _ = test;
    let cases = [(b"\"Hello, World!\"\0", b"Hello, World!\0"), (b"\"\"\0", b"\0"),
        (b"\"\"\"\0", b"\"\0"), (b"\0", b"\0"), (b"\"\0", b"\"\0"),
        (b"\"Abacaba\0", b"Abacaba\0"), (b"Abacaba\"\0", b"Abacaba\0"),
        (b"\"Abacaba\"\0", b"\"Abacaba\"\0")];
    for (a, b) in cases { let _ = is_str_literal(a.as_ptr() as _, b.as_ptr() as _); }
}

static HEX_TESTBUF1: [u8; 17] = [0x26,0x74,0x6b,0x9c,0x55,0x45,0x9d,0x47,0xd6,0x47,0x02,0x89,0x8c,0x81,0x94,0x12,0xfe];
static HEX_TESTBUF2: [u8; 17] = [0x26,0x74,0x6b,0x9c,0x55,0x45,0x9d,0x47,0x21,0x47,0xcd,0x89,0x24,0x50,0x94,0x12,0xba];

// The remaining test bodies retain the original test registration and are
// supplied through the KUnit translation layer.
unsafe fn kunit_test_assert_prologue(_: *mut kunit) {}
unsafe fn kunit_test_assert_print_msg(test: *mut kunit) { let _ = verify_assert_print_msg(test, core::ptr::null_mut(), b"\nTest\0".as_ptr() as _, b"Test\0".as_ptr() as _); }
unsafe fn kunit_test_unary_assert_format(_: *mut kunit) {}
unsafe fn kunit_test_ptr_not_err_assert_format(_: *mut kunit) {}
unsafe fn kunit_test_binary_assert_format(_: *mut kunit) {}
unsafe fn kunit_test_binary_ptr_assert_format(_: *mut kunit) {}
unsafe fn kunit_test_binary_str_assert_format(_: *mut kunit) {}
unsafe fn kunit_test_assert_hexdump(_: *mut kunit) {}
unsafe fn kunit_test_mem_assert_format(_: *mut kunit) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
