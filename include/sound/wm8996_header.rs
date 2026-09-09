/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/wm8996.h -- Platform data for WM8996
 *
 * Copyright 2011 Wolfson Microelectronics. PLC.
 */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum wm8996_inmode {
    WM8996_DIFFERRENTIAL_1 = 0,   /* IN1xP - IN1xN */
    WM8996_INVERTING = 1,         /* IN1xN */
    WM8996_NON_INVERTING = 2,     /* IN1xP */
    WM8996_DIFFERENTIAL_2 = 3,    /* IN2xP - IN2xP */
}

/**
 * ReTune Mobile configurations are specified with a label, sample
 * rate and set of values to write (the enable bits will be ignored).
 *
 * Configurations are expected to be generated using the ReTune Mobile
 * control panel in WISCE - see http://www.wolfsonmicro.com/wisce/
 */
#[repr(C)]
pub struct wm8996_retune_mobile_config {
    pub name: *const ::core::ffi::c_char,
    pub rate: ::core::ffi::c_int,
    pub regs: [u16; 20],
}

pub const WM8996_SET_DEFAULT: u32 = 0x10000;

#[repr(C)]
pub struct wm8996_pdata {
    pub irq_flags: ::core::ffi::c_int,  /** Set IRQ trigger flags; default active low */

    pub micdet_def: ::core::ffi::c_int,  /** Default MICDET_SRC/HP1FB_SRC/MICD_BIAS */

    pub inl_mode: wm8996_inmode,
    pub inr_mode: wm8996_inmode,

    pub spkmute_seq: u32,  /** Value for register 0x802 */

    pub gpio_default: [u32; 5],

    pub num_retune_mobile_cfgs: ::core::ffi::c_int,
    pub retune_mobile_cfgs: *mut wm8996_retune_mobile_config,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
