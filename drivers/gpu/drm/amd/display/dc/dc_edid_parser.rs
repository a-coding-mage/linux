/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

use core::ffi::c_void;

// Definitions are supplied by the corresponding C headers in the containing build.
#[repr(C)]
pub struct dc {
    pub res_pool: *mut dc_resource_pool,
}

#[repr(C)]
pub struct dc_resource_pool {
    pub dmcu: *mut dmcu,
}

#[repr(C)]
pub struct dmcu {
    pub funcs: *mut dmcu_funcs,
}

#[repr(C)]
pub struct dmcu_funcs {
    pub is_dmcu_initialized: Option<unsafe extern "C" fn(*mut dmcu) -> bool>,
    pub send_edid_cea: Option<unsafe extern "C" fn(*mut dmcu, i32, i32, *mut u8, i32) -> bool>,
    pub recv_edid_cea_ack: Option<unsafe extern "C" fn(*mut dmcu, *mut i32) -> bool>,
    pub recv_amd_vsdb: Option<unsafe extern "C" fn(*mut dmcu, *mut i32, *mut i32, *mut i32) -> bool>,
}

pub unsafe fn dc_edid_parser_send_cea(
    dc: *mut dc,
    offset: i32,
    total_length: i32,
    data: *mut u8,
    length: i32,
) -> bool {
    let dmcu = (*(*dc).res_pool).dmcu;

    if !dmcu.is_null() {
        let funcs = (*dmcu).funcs;
        if !funcs.is_null() {
            if let Some(is_initialized) = (*funcs).is_dmcu_initialized {
                if is_initialized(dmcu) {
                    if let Some(send_edid_cea) = (*funcs).send_edid_cea {
                        return send_edid_cea(dmcu, offset, total_length, data, length);
                    }
                }
            }
        }
    }

    false
}

pub unsafe fn dc_edid_parser_recv_cea_ack(dc: *mut dc, offset: *mut i32) -> bool {
    let dmcu = (*(*dc).res_pool).dmcu;

    if !dmcu.is_null() {
        let funcs = (*dmcu).funcs;
        if !funcs.is_null() {
            if let Some(is_initialized) = (*funcs).is_dmcu_initialized {
                if is_initialized(dmcu) {
                    if let Some(recv_edid_cea_ack) = (*funcs).recv_edid_cea_ack {
                        return recv_edid_cea_ack(dmcu, offset);
                    }
                }
            }
        }
    }

    false
}

pub unsafe fn dc_edid_parser_recv_amd_vsdb(
    dc: *mut dc,
    version: *mut i32,
    min_frame_rate: *mut i32,
    max_frame_rate: *mut i32,
) -> bool {
    let dmcu = (*(*dc).res_pool).dmcu;

    if !dmcu.is_null() {
        let funcs = (*dmcu).funcs;
        if !funcs.is_null() {
            if let Some(is_initialized) = (*funcs).is_dmcu_initialized {
                if is_initialized(dmcu) {
                    if let Some(recv_amd_vsdb) = (*funcs).recv_amd_vsdb {
                        return recv_amd_vsdb(dmcu, version, min_frame_rate, max_frame_rate);
                    }
                }
            }
        }
    }

    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
