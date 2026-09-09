// SPDX-License-Identifier: GPL-2.0
/*
 * Assertion and expectation serialization API.
 *
 * Copyright (C) 2019, Google LLC.
 * Author: Brendan Higgins <brendanhiggins@google.com>
 */

// C dependencies supplied by the surrounding KUnit translation unit:
// kunit/assert.h, kunit/test.h, kunit/visibility.h, and string-stream.h.

pub unsafe fn kunit_assert_prologue(
    loc: *const kunit_loc,
    assertion_type: kunit_assert_type,
    stream: *mut string_stream,
) {
    let expect_or_assert: *const core::ffi::c_char = match assertion_type {
        KUNIT_EXPECTATION => c"EXPECTATION".as_ptr(),
        KUNIT_ASSERTION => c"ASSERTION".as_ptr(),
        _ => core::ptr::null(),
    };

    string_stream_add(
        stream,
        c"%s FAILED at %s:%d\n".as_ptr(),
        expect_or_assert,
        (*loc).file,
        (*loc).line,
    );
}

pub unsafe fn kunit_assert_print_msg(
    message: *const va_format,
    stream: *mut string_stream,
) {
    if !(*message).fmt.is_null() {
        string_stream_add(stream, c"\n%pV".as_ptr(), message);
    }
}

pub unsafe fn kunit_fail_assert_format(
    _assertion: *const kunit_assert,
    message: *const va_format,
    stream: *mut string_stream,
) {
    string_stream_add(stream, c"%pV".as_ptr(), message);
}

pub unsafe fn kunit_unary_assert_format(
    assertion: *const kunit_assert,
    message: *const va_format,
    stream: *mut string_stream,
) {
    let unary_assert = assertion as *const kunit_unary_assert;

    if (*unary_assert).expected_true {
        string_stream_add(stream, c"Expected %s to be true, but is false\n".as_ptr(), (*unary_assert).condition);
    } else {
        string_stream_add(stream, c"Expected %s to be false, but is true\n".as_ptr(), (*unary_assert).condition);
    }
    kunit_assert_print_msg(message, stream);
}

pub unsafe fn kunit_ptr_not_err_assert_format(
    assertion: *const kunit_assert,
    message: *const va_format,
    stream: *mut string_stream,
) {
    let ptr_assert = assertion as *const kunit_ptr_not_err_assert;

    if (*ptr_assert).value.is_null() {
        string_stream_add(stream, c"Expected %s is not null, but is\n".as_ptr(), (*ptr_assert).text);
    } else if is_err((*ptr_assert).value) {
        string_stream_add(stream, c"Expected %s is not error, but is: %ld\n".as_ptr(), (*ptr_assert).text, ptr_err((*ptr_assert).value));
    }
    kunit_assert_print_msg(message, stream);
}

/* Checks if `text` is a literal representing `value`, e.g. "5" and 5 */
pub unsafe fn is_literal(text: *const core::ffi::c_char, value: i64) -> bool {
    let len = snprintf_len(value);
    if strlen(text) != len { return false; }
    let buffer = kmalloc((len + 1) as usize, GFP_KERNEL);
    if buffer.is_null() { return false; }
    snprintf_value(buffer, len + 1, value);
    let ret = strncmp(buffer, text, len) == 0;
    kfree(buffer);
    ret
}

pub unsafe fn kunit_binary_assert_format(
    assertion: *const kunit_assert,
    message: *const va_format,
    stream: *mut string_stream,
) {
    let binary_assert = assertion as *const kunit_binary_assert;
    string_stream_add(stream, c"Expected %s %s %s, but\n".as_ptr(), (*binary_assert).text.left_text, (*binary_assert).text.operation, (*binary_assert).text.right_text);
    if !is_literal((*binary_assert).text.left_text, (*binary_assert).left_value) {
        string_stream_add(stream, c"%s == %lld (0x%llx)\n".as_ptr(), (*binary_assert).text.left_text, (*binary_assert).left_value, (*binary_assert).left_value);
    }
    if !is_literal((*binary_assert).text.right_text, (*binary_assert).right_value) {
        string_stream_add(stream, c"%s == %lld (0x%llx)".as_ptr(), (*binary_assert).text.right_text, (*binary_assert).right_value, (*binary_assert).right_value);
    }
    kunit_assert_print_msg(message, stream);
}

pub unsafe fn kunit_binary_ptr_assert_format(
    assertion: *const kunit_assert,
    message: *const va_format,
    stream: *mut string_stream,
) {
    let binary_assert = assertion as *const kunit_binary_ptr_assert;
    string_stream_add(stream, c"Expected %s %s %s, but\n".as_ptr(), (*binary_assert).text.left_text, (*binary_assert).text.operation, (*binary_assert).text.right_text);
    string_stream_add(stream, c"%s == %px\n".as_ptr(), (*binary_assert).text.left_text, (*binary_assert).left_value);
    string_stream_add(stream, c"%s == %px".as_ptr(), (*binary_assert).text.right_text, (*binary_assert).right_value);
    kunit_assert_print_msg(message, stream);
}

/* Checks if KUNIT_EXPECT_STREQ() args were string literals. */
pub unsafe fn is_str_literal(text: *const core::ffi::c_char, value: *const core::ffi::c_char) -> bool {
    let len = strlen(text);
    if len < 2 || *text != b'"' as i8 || *text.add(len - 1) != b'"' as i8 { return false; }
    strncmp(text.add(1), value, len - 2) == 0
}

pub unsafe fn kunit_binary_str_assert_format(assertion: *const kunit_assert, message: *const va_format, stream: *mut string_stream) {
    let binary_assert = assertion as *const kunit_binary_str_assert;
    string_stream_add(stream, c"Expected %s %s %s, but\n".as_ptr(), (*binary_assert).text.left_text, (*binary_assert).text.operation, (*binary_assert).text.right_text);
    if !is_str_literal((*binary_assert).text.left_text, (*binary_assert).left_value) { string_stream_add(stream, c"%s == \"%s\"\n".as_ptr(), (*binary_assert).text.left_text, (*binary_assert).left_value); }
    if !is_str_literal((*binary_assert).text.right_text, (*binary_assert).right_value) { string_stream_add(stream, c"%s == \"%s\"".as_ptr(), (*binary_assert).text.right_text, (*binary_assert).right_value); }
    kunit_assert_print_msg(message, stream);
}

pub unsafe fn kunit_assert_hexdump(stream: *mut string_stream, buf: *const core::ffi::c_void, compared_buf: *const core::ffi::c_void, len: usize) {
    let buf1 = buf as *const u8;
    let buf2 = compared_buf as *const u8;
    string_stream_add(stream, c"".as_ptr());
    for i in 0..len {
        if i % 16 == 0 && i != 0 { string_stream_add(stream, c"\n".as_ptr()); }
        if *buf1.add(i) != *buf2.add(i) { string_stream_add(stream, c"<%02x>".as_ptr(), *buf1.add(i)); }
        else { string_stream_add(stream, c" %02x ".as_ptr(), *buf1.add(i)); }
    }
}

pub unsafe fn kunit_mem_assert_format(assertion: *const kunit_assert, message: *const va_format, stream: *mut string_stream) {
    let mem_assert = assertion as *const kunit_mem_assert;
    if (*mem_assert).left_value.is_null() { string_stream_add(stream, c"Expected %s is not null, but is\n".as_ptr(), (*mem_assert).text.left_text); }
    else if (*mem_assert).right_value.is_null() { string_stream_add(stream, c"Expected %s is not null, but is\n".as_ptr(), (*mem_assert).text.right_text); }
    else {
        string_stream_add(stream, c"Expected %s %s %s, but\n".as_ptr(), (*mem_assert).text.left_text, (*mem_assert).text.operation, (*mem_assert).text.right_text);
        string_stream_add(stream, c"%s ==\n".as_ptr(), (*mem_assert).text.left_text);
        kunit_assert_hexdump(stream, (*mem_assert).left_value, (*mem_assert).right_value, (*mem_assert).size);
        string_stream_add(stream, c"\n".as_ptr());
        string_stream_add(stream, c"%s ==\n".as_ptr(), (*mem_assert).text.right_text);
        kunit_assert_hexdump(stream, (*mem_assert).right_value, (*mem_assert).left_value, (*mem_assert).size);
        kunit_assert_print_msg(message, stream);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
