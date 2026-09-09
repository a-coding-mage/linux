/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Simple driver for Texas Instruments LM3630 LED Flash driver chip
 * Copyright (C) 2012 Texas Instruments
 */

// Original header guard: __LINUX_LM3639_H

pub const LM3639_NAME: &str = "lm3639_bl";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_pwm {
    LM3639_PWM_DISABLE = 0x00,
    LM3639_PWM_EN_ACTLOW = 0x48,
    LM3639_PWM_EN_ACTHIGH = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_strobe {
    LM3639_STROBE_DISABLE = 0x00,
    LM3639_STROBE_EN_ACTLOW = 0x10,
    LM3639_STROBE_EN_ACTHIGH = 0x30,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_txpin {
    LM3639_TXPIN_DISABLE = 0x00,
    LM3639_TXPIN_EN_ACTLOW = 0x04,
    LM3639_TXPIN_EN_ACTHIGH = 0x0C,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_fleds {
    LM3639_FLED_DIASBLE_ALL = 0x00,
    LM3639_FLED_EN_1 = 0x40,
    LM3639_FLED_EN_2 = 0x20,
    LM3639_FLED_EN_ALL = 0x60,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_bleds {
    LM3639_BLED_DIASBLE_ALL = 0x00,
    LM3639_BLED_EN_1 = 0x10,
    LM3639_BLED_EN_2 = 0x08,
    LM3639_BLED_EN_ALL = 0x18,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_bled_mode {
    LM3639_BLED_MODE_EXPONETIAL = 0x00,
    LM3639_BLED_MODE_LINEAR = 0x10,
}

#[repr(C)]
pub struct lm3639_platform_data {
    pub max_brt_led: u32,
    pub init_brt_led: u32,

    /* input pins */
    pub pin_pwm: lm3639_pwm,
    pub pin_strobe: lm3639_strobe,
    pub pin_tx: lm3639_txpin,

    /* output pins */
    pub fled_pins: lm3639_fleds,
    pub bled_pins: lm3639_bleds,
    pub bled_mode: lm3639_bled_mode,

    pub pwm_set_intensity: Option<unsafe extern "C" fn(brightness: i32, max_brightness: i32)>,
    pub pwm_get_intensity: Option<unsafe extern "C" fn() -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
