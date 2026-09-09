/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependency supplied by the original dc.h include.
pub struct dc;
pub struct dc_virtual_addr_space_config;

pub const MAX_VMID: i32 = 16;

#[repr(C)]
pub struct mod_vmid {
    pub dummy: ::core::ffi::c_int,
}

extern "C" {
    pub fn mod_vmid_get_for_ptb(
        mod_vmid: *mut mod_vmid,
        ptb: u64,
    ) -> u8;

    pub fn mod_vmid_reset(mod_vmid: *mut mod_vmid);

    pub fn mod_vmid_create(
        dc: *mut dc,
        num_vmid: ::core::ffi::c_uint,
        va_config: *mut dc_virtual_addr_space_config,
    ) -> *mut mod_vmid;

    pub fn mod_vmid_destroy(mod_vmid: *mut mod_vmid);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
