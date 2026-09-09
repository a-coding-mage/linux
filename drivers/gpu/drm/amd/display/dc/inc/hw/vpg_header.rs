/*
 * Copyright 2024 Advanced Micro Devices, Inc.
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

// C forward declarations.
#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_info_packet {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vpg_funcs {
    pub update_generic_info_packet: Option<
        unsafe extern "C" fn(
            vpg: *mut vpg,
            packet_index: u32,
            info_packet: *const dc_info_packet,
            immediate_update: bool,
        ),
    >,

    pub vpg_poweron: Option<unsafe extern "C" fn(vpg: *mut vpg)>,

    pub vpg_powerdown: Option<unsafe extern "C" fn(vpg: *mut vpg)>,
}

#[repr(C)]
pub struct vpg {
    pub funcs: *const vpg_funcs,
    pub ctx: *mut dc_context,
    pub inst: core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
