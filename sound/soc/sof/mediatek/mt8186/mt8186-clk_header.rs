/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */

/*
 * Copyright (c) 2022 MediaTek Corporation. All rights reserved.
 *
 *  Header file for the mt8186 DSP clock definition
 */

// Forward declaration from C: struct snd_sof_dev;
#[repr(C)]
pub struct snd_sof_dev {
    _unused: [u8; 0],
}

/* DSP clock */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adsp_clk_id {
    CLK_TOP_AUDIODSP,
    CLK_TOP_ADSP_BUS,
    ADSP_CLK_MAX,
}

unsafe extern "C" {
    pub fn mt8186_adsp_init_clock(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int;
    pub fn mt8186_adsp_clock_on(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int;
    pub fn mt8186_adsp_clock_off(sdev: *mut snd_sof_dev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
