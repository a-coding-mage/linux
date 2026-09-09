/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// Dependency equivalent of: #include "link_dp_training.h"

/// The approximate time (us) it takes to transmit 9 USB4 DP clock sync packets.
pub const DPIA_CLK_SYNC_DELAY: u32 = 16000;

extern "C" {
    /// Train DP tunneling link for USB4 DPIA display endpoint.
    /// DPIA equivalent of dc_link_dp_perfrorm_link_training.
    /// Aborts link training upon detection of sink unplug.
    pub fn dpia_perform_link_training(
        link: *mut dc_link,
        link_res: *const link_resource,
        link_setting: *const dc_link_settings,
        skip_video_pattern: bool,
    ) -> link_training_result;

    pub fn dpia_training_abort(
        link: *mut dc_link,
        lt_settings: *mut link_training_settings,
        hop: u32,
    );

    pub fn dpia_get_eq_aux_rd_interval(
        link: *const dc_link,
        lt_settings: *const link_training_settings,
        hop: u32,
    ) -> u32;

    pub fn dpia_set_tps_notification(
        link: *mut dc_link,
        lt_settings: *const link_training_settings,
        pattern: u8,
        offset: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
