/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This header provides constants for most at91 pinctrl bindings.
 *
 * Copyright (C) 2013 Jean-Christophe PLAGNIOL-VILLARD <plagnioj@jcrosoft.com>
 */

pub const AT91_PINCTRL_NONE: i32 = 0 << 0;
pub const AT91_PINCTRL_PULL_UP: i32 = 1 << 0;
pub const AT91_PINCTRL_MULTI_DRIVE: i32 = 1 << 1;
pub const AT91_PINCTRL_DEGLITCH: i32 = 1 << 2;
pub const AT91_PINCTRL_PULL_DOWN: i32 = 1 << 3;
pub const AT91_PINCTRL_DIS_SCHMIT: i32 = 1 << 4;
pub const AT91_PINCTRL_OUTPUT: i32 = 1 << 7;
#[macro_export]
macro_rules! AT91_PINCTRL_OUTPUT_VAL {
    ($x:expr) => {{ ($x & 0x1) << 8 }};
}
pub const AT91_PINCTRL_SLEWRATE: i32 = 1 << 9;
pub const AT91_PINCTRL_DEBOUNCE: i32 = 1 << 16;
#[macro_export]
macro_rules! AT91_PINCTRL_DEBOUNCE_VAL {
    ($x:expr) => {{ $x << 17 }};
}

pub const AT91_PINCTRL_PULL_UP_DEGLITCH: i32 =
    AT91_PINCTRL_PULL_UP | AT91_PINCTRL_DEGLITCH;

pub const AT91_PINCTRL_DRIVE_STRENGTH_DEFAULT: i32 = 0x0 << 5;
pub const AT91_PINCTRL_DRIVE_STRENGTH_LOW: i32 = 0x1 << 5;
pub const AT91_PINCTRL_DRIVE_STRENGTH_MED: i32 = 0x2 << 5;
pub const AT91_PINCTRL_DRIVE_STRENGTH_HI: i32 = 0x3 << 5;

pub const AT91_PINCTRL_SLEWRATE_ENA: i32 = 0x0 << 9;
pub const AT91_PINCTRL_SLEWRATE_DIS: i32 = 0x1 << 9;

pub const AT91_PIOA: i32 = 0;
pub const AT91_PIOB: i32 = 1;
pub const AT91_PIOC: i32 = 2;
pub const AT91_PIOD: i32 = 3;
pub const AT91_PIOE: i32 = 4;

pub const AT91_PERIPH_GPIO: i32 = 0;
pub const AT91_PERIPH_A: i32 = 1;
pub const AT91_PERIPH_B: i32 = 2;
pub const AT91_PERIPH_C: i32 = 3;
pub const AT91_PERIPH_D: i32 = 4;

pub const ATMEL_PIO_DRVSTR_LO: i32 = 1;
pub const ATMEL_PIO_DRVSTR_ME: i32 = 2;
pub const ATMEL_PIO_DRVSTR_HI: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
