/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Texas Instruments
 *
 * Simple driver for Texas Instruments LM3642 LED driver chip
 *
 * Author: G.Shark Jeong <gshark.jeong@gmail.com>
 *         Daniel Jeong <daniel.jeong@ti.com>
 */

// Header guard: __LINUX_LM3642_H

pub const LM3642_NAME: &str = "leds-lm3642";

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum lm3642_torch_pin_enable {
    LM3642_TORCH_PIN_DISABLE = 0x00,
    LM3642_TORCH_PIN_ENABLE = 0x10,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum lm3642_strobe_pin_enable {
    LM3642_STROBE_PIN_DISABLE = 0x00,
    LM3642_STROBE_PIN_ENABLE = 0x20,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum lm3642_tx_pin_enable {
    LM3642_TX_PIN_DISABLE = 0x00,
    LM3642_TX_PIN_ENABLE = 0x40,
}

#[repr(C)]
pub struct lm3642_platform_data {
    pub torch_pin: lm3642_torch_pin_enable,
    pub strobe_pin: lm3642_strobe_pin_enable,
    pub tx_pin: lm3642_tx_pin_enable,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
