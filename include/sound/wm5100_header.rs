/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/wm5100.h -- Platform data for WM5100
 *
 * Copyright 2011 Wolfson Microelectronics. PLC.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum wm5100_in_mode {
    WM5100_IN_SE = 0,
    WM5100_IN_DIFF = 1,
    WM5100_IN_DMIC = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum wm5100_dmic_sup {
    WM5100_DMIC_SUP_MICVDD = 0,
    WM5100_DMIC_SUP_MICBIAS1 = 1,
    WM5100_DMIC_SUP_MICBIAS2 = 2,
    WM5100_DMIC_SUP_MICBIAS3 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum wm5100_micdet_bias {
    WM5100_MICDET_MICBIAS1 = 0,
    WM5100_MICDET_MICBIAS2 = 1,
    WM5100_MICDET_MICBIAS3 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct wm5100_jack_mode {
    pub bias: wm5100_micdet_bias,
    pub hp_pol: ::core::ffi::c_int,
    pub micd_src: ::core::ffi::c_int,
}

pub const WM5100_GPIO_SET: ::core::ffi::c_uint = 0x10000;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct wm5100_pdata {
    pub irq_flags: ::core::ffi::c_int,
    pub jack_modes: [wm5100_jack_mode; 2],

    /* Input pin mode selection */
    pub in_mode: [wm5100_in_mode; 4],

    /* DMIC supply selection */
    pub dmic_sup: [wm5100_dmic_sup; 4],

    pub gpio_defaults: [::core::ffi::c_int; 6],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
