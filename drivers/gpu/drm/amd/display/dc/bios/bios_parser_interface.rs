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

use core::ffi::c_int;

// Types and functions supplied by the corresponding BIOS parser interfaces.
#[repr(C)]
pub struct bp_init_data {
    _private: [u8; 0],
}

pub type dce_version = c_int;

#[repr(C)]
pub struct dc_bios {
    pub funcs: *mut dc_bios_funcs,
}

#[repr(C)]
pub struct dc_bios_funcs {
    pub bios_parser_destroy: Option<unsafe extern "C" fn(dcb: *mut *mut dc_bios)>,
}

extern "C" {
    fn firmware_parser_create(
        init: *mut bp_init_data,
        dce_version: dce_version,
    ) -> *mut dc_bios;
    fn bios_parser_create(
        init: *mut bp_init_data,
        dce_version: dce_version,
    ) -> *mut dc_bios;
}

pub unsafe extern "C" fn dal_bios_parser_create(
    init: *mut bp_init_data,
    dce_version: dce_version,
) -> *mut dc_bios {
    let mut bios: *mut dc_bios = core::ptr::null_mut();

    bios = firmware_parser_create(init, dce_version);

    /* Fall back to old bios parser for older asics */
    if bios.is_null() {
        bios = bios_parser_create(init, dce_version);
    }

    bios
}

pub unsafe extern "C" fn dal_bios_parser_destroy(dcb: *mut *mut dc_bios) {
    let bios: *mut dc_bios = *dcb;

    ((*(*bios).funcs).bios_parser_destroy.unwrap())(dcb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
