/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Assertion and expectation serialization API.
 *
 * Copyright (C) 2019, Google LLC.
 * Author: Brendan Higgins <brendanhiggins@google.com>
 */

// Dependencies supplied by other translated headers are intentionally external.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct string_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct va_format {
    _private: [u8; 0],
}

/**
 * enum kunit_assert_type - Type of expectation/assertion.
 * @KUNIT_ASSERTION: Used to denote that a kunit_assert represents an assertion.
 * @KUNIT_EXPECTATION: Denotes that a kunit_assert represents an expectation.
 *
 * Used in conjunction with a &struct kunit_assert to denote whether it
 * represents an expectation or an assertion.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum kunit_assert_type {
    KUNIT_ASSERTION,
    KUNIT_EXPECTATION,
}

/**
 * struct kunit_loc - Identifies the source location of a line of code.
 * @line: the line number in the file.
 * @file: the file name.
 */
#[repr(C)]
pub struct kunit_loc {
    pub line: i32,
    pub file: *const c_char,
}

// C macro: expands to the current source file and line at each invocation.
#[macro_export]
macro_rules! KUNIT_CURRENT_LOC {
    () => {
        $crate::kunit_loc {
            file: concat!(file!(), "\0").as_ptr() as *const core::ffi::c_char,
            line: line!() as i32,
        }
    };
}

/**
 * struct kunit_assert - Data for printing a failed assertion or expectation.
 *
 * Represents a failed expectation/assertion. Contains all the data necessary to
 * format a string to a user reporting the failure.
 */
#[repr(C)]
pub struct kunit_assert {
    _private: [u8; 0],
}

pub type assert_format_t = Option<unsafe extern "C" fn(
    assert_: *const kunit_assert,
    message: *const va_format,
    stream: *mut string_stream,
)>;

unsafe extern "C" {
    pub fn kunit_assert_prologue(
        loc: *const kunit_loc,
        type_: kunit_assert_type,
        stream: *mut string_stream,
    );
}

/**
 * struct kunit_fail_assert - Represents a plain fail expectation/assertion.
 * @assert: The parent of this type.
 *
 * Represents a simple KUNIT_FAIL/KUNIT_FAIL_AND_ABORT that always fails.
 */
#[repr(C)]
pub struct kunit_fail_assert {
    pub assert: kunit_assert,
}

unsafe extern "C" {
    pub fn kunit_fail_assert_format(
        assert_: *const kunit_assert,
        message: *const va_format,
        stream: *mut string_stream,
    );
}

/** See the original C declaration for the full assertion semantics. */
#[repr(C)]
pub struct kunit_unary_assert {
    pub assert: kunit_assert,
    pub condition: *const c_char,
    pub expected_true: bool,
}

unsafe extern "C" {
    pub fn kunit_unary_assert_format(
        assert_: *const kunit_assert,
        message: *const va_format,
        stream: *mut string_stream,
    );
}

#[repr(C)]
pub struct kunit_ptr_not_err_assert {
    pub assert: kunit_assert,
    pub text: *const c_char,
    pub value: *const c_void,
}

unsafe extern "C" {
    pub fn kunit_ptr_not_err_assert_format(
        assert_: *const kunit_assert,
        message: *const va_format,
        stream: *mut string_stream,
    );
}

#[repr(C)]
pub struct kunit_binary_assert_text {
    pub operation: *const c_char,
    pub left_text: *const c_char,
    pub right_text: *const c_char,
}

#[repr(C)]
pub struct kunit_binary_assert {
    pub assert: kunit_assert,
    pub text: *const kunit_binary_assert_text,
    pub left_value: i64,
    pub right_value: i64,
}

unsafe extern "C" {
    pub fn kunit_binary_assert_format(
        assert_: *const kunit_assert,
        message: *const va_format,
        stream: *mut string_stream,
    );
}

#[repr(C)]
pub struct kunit_binary_ptr_assert {
    pub assert: kunit_assert,
    pub text: *const kunit_binary_assert_text,
    pub left_value: *const c_void,
    pub right_value: *const c_void,
}

unsafe extern "C" {
    pub fn kunit_binary_ptr_assert_format(
        assert_: *const kunit_assert,
        message: *const va_format,
        stream: *mut string_stream,
    );
}

#[repr(C)]
pub struct kunit_binary_str_assert {
    pub assert: kunit_assert,
    pub text: *const kunit_binary_assert_text,
    pub left_value: *const c_char,
    pub right_value: *const c_char,
}

unsafe extern "C" {
    pub fn kunit_binary_str_assert_format(
        assert_: *const kunit_assert,
        message: *const va_format,
        stream: *mut string_stream,
    );
}

#[repr(C)]
pub struct kunit_mem_assert {
    pub assert: kunit_assert,
    pub text: *const kunit_binary_assert_text,
    pub left_value: *const c_void,
    pub right_value: *const c_void,
    pub size: usize,
}

unsafe extern "C" {
    pub fn kunit_mem_assert_format(
        assert_: *const kunit_assert,
        message: *const va_format,
        stream: *mut string_stream,
    );
}

// These declarations are present when CONFIG_KUNIT is enabled.
#[cfg(feature = "CONFIG_KUNIT")]
unsafe extern "C" {
    pub fn kunit_assert_print_msg(message: *const va_format, stream: *mut string_stream);
    pub fn is_literal(text: *const c_char, value: i64) -> bool;
    pub fn is_str_literal(text: *const c_char, value: *const c_char) -> bool;
    pub fn kunit_assert_hexdump(
        stream: *mut string_stream,
        buf: *const c_void,
        compared_buf: *const c_void,
        len: usize,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
