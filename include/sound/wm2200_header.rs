/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/wm2200.h -- Platform data for WM2200
 *
 * Copyright 2012 Wolfson Microelectronics. PLC.
 */

// C header guard: __LINUX_SND_WM2200_H

pub const WM2200_GPIO_SET: u32 = 0x10000;
pub const WM2200_MAX_MICBIAS: usize = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum wm2200_in_mode {
    WM2200_IN_SE = 0,
    WM2200_IN_DIFF = 1,
    WM2200_IN_DMIC = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum wm2200_dmic_sup {
    WM2200_DMIC_SUP_MICVDD = 0,
    WM2200_DMIC_SUP_MICBIAS1 = 1,
    WM2200_DMIC_SUP_MICBIAS2 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum wm2200_mbias_lvl {
    WM2200_MBIAS_LVL_1V5 = 1,
    WM2200_MBIAS_LVL_1V8 = 2,
    WM2200_MBIAS_LVL_1V9 = 3,
    WM2200_MBIAS_LVL_2V0 = 4,
    WM2200_MBIAS_LVL_2V2 = 5,
    WM2200_MBIAS_LVL_2V4 = 6,
    WM2200_MBIAS_LVL_2V5 = 7,
    WM2200_MBIAS_LVL_2V6 = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm2200_micbias {
    pub mb_lvl: wm2200_mbias_lvl, // Regulated voltage
    pub discharge: u32,            // Actively discharge; C bit-field width: 1
    pub fast_start: u32,           // Enable aggressive startup ramp rate; C bit-field width: 1
    pub bypass: u32,               // Use bypass mode; C bit-field width: 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm2200_pdata {
    pub irq_flags: i32,
    pub gpio_defaults: [i32; 4],
    pub in_mode: [wm2200_in_mode; 3],
    pub dmic_sup: [wm2200_dmic_sup; 3],
    // MICBIAS configurations
    pub micbias: [wm2200_micbias; WM2200_MAX_MICBIAS],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
