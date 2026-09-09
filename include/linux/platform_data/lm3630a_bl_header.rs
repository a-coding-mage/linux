/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Simple driver for Texas Instruments LM3630A LED Flash driver chip
 * Copyright (C) 2012 Texas Instruments
 */

pub const LM3630A_NAME: &str = "lm3630a_bl";

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lm3630a_pwm_ctrl {
    LM3630A_PWM_DISABLE = 0x00,
    LM3630A_PWM_BANK_A,
    LM3630A_PWM_BANK_B,
    LM3630A_PWM_BANK_ALL,
    LM3630A_PWM_BANK_A_ACT_LOW = 0x05,
    LM3630A_PWM_BANK_B_ACT_LOW,
    LM3630A_PWM_BANK_ALL_ACT_LOW,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lm3630a_leda_ctrl {
    LM3630A_LEDA_DISABLE = 0x00,
    LM3630A_LEDA_ENABLE = 0x04,
    LM3630A_LEDA_ENABLE_LINEAR = 0x14,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lm3630a_ledb_ctrl {
    LM3630A_LEDB_DISABLE = 0x00,
    LM3630A_LEDB_ON_A = 0x01,
    LM3630A_LEDB_ENABLE = 0x02,
    LM3630A_LEDB_ENABLE_LINEAR = 0x0A,
}

pub const LM3630A_MAX_BRIGHTNESS: i32 = 255;

/*
 * @leda_label    : optional led a label.
 * @leda_init_brt : led a init brightness. 4~255
 * @leda_max_brt  : led a max brightness.  4~255
 * @leda_ctrl     : led a disable, enable linear, enable exponential
 * @ledb_label    : optional led b label.
 * @ledb_init_brt : led b init brightness. 4~255
 * @ledb_max_brt  : led b max brightness.  4~255
 * @ledb_ctrl     : led b disable, enable linear, enable exponential
 * @pwm_period    : pwm period
 * @pwm_ctrl      : pwm disable, bank a or b, active high or low
 */
#[repr(C)]
pub struct lm3630a_platform_data {
    /* led a config. */
    pub leda_label: *const core::ffi::c_char,
    pub leda_init_brt: i32,
    pub leda_max_brt: i32,
    pub leda_ctrl: lm3630a_leda_ctrl,
    /* led b config. */
    pub ledb_label: *const core::ffi::c_char,
    pub ledb_init_brt: i32,
    pub ledb_max_brt: i32,
    pub ledb_ctrl: lm3630a_ledb_ctrl,
    /* pwm config. */
    pub pwm_period: u32,
    pub pwm_ctrl: lm3630a_pwm_ctrl,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
