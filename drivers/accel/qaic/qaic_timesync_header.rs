/* SPDX-License-Identifier: GPL-2.0-only */

/* Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. */

// The `mhi_device` type is supplied by the Linux MHI dependency.
pub enum mhi_device {}

extern "C" {
    pub fn qaic_timesync_init() -> ::std::os::raw::c_int;
    pub fn qaic_timesync_deinit();
    pub fn qaic_mqts_ch_stop_timer(mhi_dev: *mut mhi_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
