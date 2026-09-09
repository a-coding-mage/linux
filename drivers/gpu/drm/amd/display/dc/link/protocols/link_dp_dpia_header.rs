/* SPDX-License-Identifier: MIT */
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

// Dependency declarations supplied by link_service.h and other translation units.
pub enum dc_status {}
#[repr(C)]
pub struct dc_link {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dc_stream_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dc_tunnel_settings {
    _private: [u8; 0],
}

/* Read tunneling device capability from DPCD and update link capability
 * accordingly.
 */
extern "C" {
    pub fn dpcd_get_tunneling_device_data(link: *mut dc_link) -> dc_status;

    /* Query hot plug status of USB4 DP tunnel.
     * Returns true if HPD high.
     */
    pub fn dpia_query_hpd_status(link: *mut dc_link) -> bool;

    /* Decide the DP tunneling settings based on the DPCD capabilities
     */
    pub fn link_decide_dp_tunnel_settings(
        stream: *mut dc_stream_state,
        dp_tunnel_setting: *mut dc_tunnel_settings,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
