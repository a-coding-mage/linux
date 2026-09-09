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
 *
 */

// C dependency: #include "link_service.h"

extern "C" {
    pub fn dp_enable_link_phy(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
        clock_source: clock_source_id,
        link_settings: *const dc_link_settings,
    );

    pub fn dp_disable_link_phy(
        link: *mut dc_link,
        link_res: *const link_resource,
        signal: signal_type,
    );

    pub fn dp_set_hw_lane_settings(
        link: *mut dc_link,
        link_res: *const link_resource,
        link_settings: *const link_training_settings,
        offset: u32,
    );

    pub fn dp_set_drive_settings(
        link: *mut dc_link,
        link_res: *const link_resource,
        lt_settings: *mut link_training_settings,
    );

    pub fn dp_set_fec_ready(
        link: *mut dc_link,
        link_res: *const link_resource,
        ready: bool,
    ) -> dc_status;

    pub fn dp_set_fec_enable(
        link: *mut dc_link,
        link_res: *const link_resource,
        enable: bool,
    );

    pub fn dpcd_write_rx_power_ctrl(link: *mut dc_link, on: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
