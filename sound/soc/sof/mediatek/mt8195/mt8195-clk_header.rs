/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (c) 2021 MediaTek Corporation. All rights reserved.
 *
 *  Header file for the mt8195 DSP clock  definition
 */

// Forward declaration from C: struct snd_sof_dev;
#[repr(C)]
pub struct snd_sof_dev {
    _unused: [u8; 0],
}

/*DSP clock*/
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum adsp_clk_id {
    CLK_TOP_ADSP = 0,
    CLK_TOP_CLK26M = 1,
    CLK_TOP_AUDIO_LOCAL_BUS = 2,
    CLK_TOP_MAINPLL_D7_D2 = 3,
    CLK_SCP_ADSP_AUDIODSP = 4,
    CLK_TOP_AUDIO_H = 5,
    ADSP_CLK_MAX = 6,
}

unsafe extern "C" {
    pub fn mt8195_adsp_init_clock(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int;
    pub fn adsp_clock_on(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int;
    pub fn adsp_clock_off(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
