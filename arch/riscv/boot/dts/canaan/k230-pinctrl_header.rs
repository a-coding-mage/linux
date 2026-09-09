/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/*
 * Copyright (C) 2024 Canaan Bright Sight Co. Ltd
 * Copyright (C) 2024 Ze Huang <18771902331@163.com>
 */

// Translated from the C header guard _K230_PINCTRL_H.

pub const K230_MSC_3V3: i32 = 0;
pub const K230_MSC_1V8: i32 = 1;

pub const BANK_VOLTAGE_DEFAULT: i32 = K230_MSC_1V8;
pub const BANK_VOLTAGE_IO50_IO61: i32 = K230_MSC_3V3;

#[inline]
pub const fn k230_pinmux(pin: i32, mode: i32) -> i32 {
    (pin << 8) | mode
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
