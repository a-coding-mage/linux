/* SPDX-License-Identifier: GPL-2.0 */
/*
 * C++ stream style string builder used in KUnit for building messages.
 *
 * Copyright (C) 2019, Google LLC.
 * Author: Brendan Higgins <brendanhiggins@google.com>
 */

/* Dependencies supplied by the corresponding kernel headers are intentionally
 * referenced here rather than reimplemented in this translation unit. */

#[repr(C)]
pub struct string_stream_fragment {
    pub node: list_head,
    pub fragment: *mut ::core::ffi::c_char,
}

#[repr(C)]
pub struct string_stream {
    pub length: usize,
    pub fragments: list_head,
    /* length and fragments are protected by this lock */
    pub lock: spinlock_t,
    pub gfp: gfp_t,
    pub append_newlines: bool,
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

extern "C" {
    pub fn kunit_alloc_string_stream(test: *mut kunit, gfp: gfp_t) -> *mut string_stream;
    pub fn kunit_free_string_stream(test: *mut kunit, stream: *mut string_stream);

    pub fn alloc_string_stream(gfp: gfp_t) -> *mut string_stream;
    pub fn free_string_stream(stream: *mut string_stream);

    /* __printf(2, 3) */
    pub fn string_stream_add(
        stream: *mut string_stream,
        fmt: *const ::core::ffi::c_char,
        ...,
    ) -> ::core::ffi::c_int;

    /* __printf(2, 0) */
    pub fn string_stream_vadd(
        stream: *mut string_stream,
        fmt: *const ::core::ffi::c_char,
        args: va_list,
    ) -> ::core::ffi::c_int;

    pub fn string_stream_clear(stream: *mut string_stream);

    pub fn string_stream_get_string(
        stream: *mut string_stream,
    ) -> *mut ::core::ffi::c_char;

    pub fn string_stream_append(
        stream: *mut string_stream,
        other: *mut string_stream,
    ) -> ::core::ffi::c_int;

    pub fn string_stream_is_empty(stream: *mut string_stream) -> bool;

    pub fn string_stream_destroy(stream: *mut string_stream);
}

#[inline]
pub unsafe fn string_stream_set_append_newlines(
    stream: *mut string_stream,
    append_newlines: bool,
) {
    (*stream).append_newlines = append_newlines;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
