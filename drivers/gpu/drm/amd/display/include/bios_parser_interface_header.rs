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
 *
 */

// Dependency supplied by dc_bios_types.h.

#[repr(C)]
pub struct bios_parser {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct dc_context {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct dc_bios {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct bp_init_data {
    pub ctx: *mut dc_context,
    pub bios: *mut u8,
}

// enum dce_version is supplied by dc_bios_types.h.
#[repr(C)]
pub enum dce_version {}

extern "C" {
    pub fn dal_bios_parser_create(
        init: *mut bp_init_data,
        dce_version: dce_version,
    ) -> *mut dc_bios;

    pub fn dal_bios_parser_destroy(dcb: *mut *mut dc_bios);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
