/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/*
 * Copyright (C) STMicroelectronics 2017 - All Rights Reserved
 * Author: Torgue Alexandre <alexandre.torgue@st.com> for STMicroelectronics.
 */

/* define PIN modes */
pub const GPIO: i32 = 0x0;
pub const AF0: i32 = 0x1;
pub const AF1: i32 = 0x2;
pub const AF2: i32 = 0x3;
pub const AF3: i32 = 0x4;
pub const AF4: i32 = 0x5;
pub const AF5: i32 = 0x6;
pub const AF6: i32 = 0x7;
pub const AF7: i32 = 0x8;
pub const AF8: i32 = 0x9;
pub const AF9: i32 = 0xa;
pub const AF10: i32 = 0xb;
pub const AF11: i32 = 0xc;
pub const AF12: i32 = 0xd;
pub const AF13: i32 = 0xe;
pub const AF14: i32 = 0xf;
pub const AF15: i32 = 0x10;
pub const ANALOG: i32 = 0x11;
pub const RSVD: i32 = 0x12;

/* define Pins number */
#[inline]
pub const fn pin_no(port: i32, line: i32) -> i32 {
    (port - 'A' as i32) * 0x10 + line
}

#[inline]
pub const fn stm32_pinmux(port: i32, line: i32, mode: i32) -> i32 {
    (pin_no(port, line) << 8) | mode
}

/* package information */
pub const STM32MP_PKG_AA: i32 = 0x1;
pub const STM32MP_PKG_AB: i32 = 0x2;
pub const STM32MP_PKG_AC: i32 = 0x4;
pub const STM32MP_PKG_AD: i32 = 0x8;
pub const STM32MP_PKG_AI: i32 = 0x100;
pub const STM32MP_PKG_AK: i32 = 0x400;
pub const STM32MP_PKG_AL: i32 = 0x800;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
