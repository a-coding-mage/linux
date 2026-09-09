/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency supplied by core_types.h in the original source.

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_log_buffer_ctx {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dcn10_clear_status_bits(dc: *mut dc, mask: ::core::ffi::c_uint);

    pub fn dcn10_log_hw_state(dc: *mut dc, log_ctx: *mut dc_log_buffer_ctx);

    pub fn dcn10_get_hw_state(
        dc: *mut dc,
        pBuf: *mut ::core::ffi::c_char,
        bufSize: ::core::ffi::c_uint,
        mask: ::core::ffi::c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
