/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// C dependency: core_types.h

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource_pool {
    _private: [u8; 0],
}

// These types are defined by core_types.h in the C source.
pub type dc_status = i32;
pub type dc_validate_mode = i32;
pub struct dc_state;
pub struct dc_stream_state;

extern "C" {
    pub fn dce112_create_resource_pool(
        num_virtual_links: u8,
        dc: *mut dc,
    ) -> *mut resource_pool;

    pub fn dce112_validate_bandwidth(
        dc: *mut dc,
        context: *mut dc_state,
        validate_mode: dc_validate_mode,
    ) -> dc_status;

    pub fn dce112_add_stream_to_ctx(
        dc: *mut dc,
        new_ctx: *mut dc_state,
        dc_stream: *mut dc_stream_state,
    ) -> dc_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
