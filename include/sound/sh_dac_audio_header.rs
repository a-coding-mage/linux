/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SH_DAC specific configuration, for the dac_audio platform_device
 *
 * Copyright (C) 2009 Rafael Ignacio Zurita <rizurita@yahoo.com>
 */

// Translation of the C header guard: __INCLUDE_SH_DAC_AUDIO_H.

#[repr(C)]
pub struct dac_audio_pdata {
    pub buffer_size: ::core::ffi::c_int,
    pub channel: ::core::ffi::c_int,
    pub start: Option<unsafe extern "C" fn(pd: *mut dac_audio_pdata)>,
    pub stop: Option<unsafe extern "C" fn(pd: *mut dac_audio_pdata)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
