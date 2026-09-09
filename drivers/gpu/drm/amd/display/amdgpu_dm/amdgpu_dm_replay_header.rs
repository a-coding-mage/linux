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
 *
 */

// Dependencies supplied by the surrounding translation unit:
// amdgpu.h, dc.h, and modules/inc/mod_power.h

extern "C" {
    pub fn amdgpu_dm_link_supports_replay(
        link: *mut dc_link,
        aconnector: *mut amdgpu_dm_connector,
    ) -> bool;

    pub fn amdgpu_dm_set_replay_caps(
        link: *mut dc_link,
        aconnector: *mut amdgpu_dm_connector,
    ) -> bool;

    pub fn amdgpu_dm_link_setup_replay(
        stream: *mut dc_stream_state,
        vrr_params: *mut mod_vrr_params,
    ) -> bool;

    pub fn amdgpu_dm_replay_set_event(
        dm: *mut amdgpu_display_manager,
        stream: *mut dc_stream_state,
        set_event: bool,
        event: replay_event,
        wait_for_disable: bool,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
