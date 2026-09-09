/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency supplied by the translated link_service header.
use crate::{
    dc_connection_type, dc_detect_reason, dc_link, dc_link_status, dc_sink,
    dc_sink_init_data, signal_type,
};

unsafe extern "C" {
    pub fn link_detect(link: *mut dc_link, reason: dc_detect_reason) -> bool;

    pub fn link_detect_connection_type(
        link: *mut dc_link,
        type_: *mut dc_connection_type,
    ) -> bool;

    pub fn link_add_remote_sink(
        link: *mut dc_link,
        edid: *const u8,
        len: core::ffi::c_uint,
        init_data: *mut dc_sink_init_data,
    ) -> *mut dc_sink;

    pub fn link_remove_remote_sink(link: *mut dc_link, sink: *mut dc_sink);

    pub fn link_reset_cur_dp_mst_topology(link: *mut dc_link) -> bool;

    pub fn link_get_status(link: *const dc_link) -> *const dc_link_status;

    pub fn link_is_hdcp14(link: *mut dc_link, signal: signal_type) -> bool;

    pub fn link_is_hdcp22(link: *mut dc_link, signal: signal_type) -> bool;

    pub fn link_clear_dprx_states(link: *mut dc_link);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
