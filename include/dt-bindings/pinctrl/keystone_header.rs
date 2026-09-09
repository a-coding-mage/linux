/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This header provides constants for Keystone pinctrl bindings.
 *
 * Copyright (C) 2016 Texas Instruments Incorporated - http://www.ti.com/
 */

pub const MUX_MODE0: i32 = 0;
pub const MUX_MODE1: i32 = 1;
pub const MUX_MODE2: i32 = 2;
pub const MUX_MODE3: i32 = 3;
pub const MUX_MODE4: i32 = 4;
pub const MUX_MODE5: i32 = 5;

pub const BUFFER_CLASS_B: i32 = 0 << 19;
pub const BUFFER_CLASS_C: i32 = 1 << 19;
pub const BUFFER_CLASS_D: i32 = 2 << 19;
pub const BUFFER_CLASS_E: i32 = 3 << 19;

pub const PULL_DISABLE: i32 = 1 << 16;
pub const PIN_PULLUP: i32 = 1 << 17;
pub const PIN_PULLDOWN: i32 = 0 << 17;

#[macro_export]
macro_rules! KEYSTONE_IOPAD_OFFSET {
    ($pa:expr, $offset:expr) => {
        (($pa & 0xffff) - $offset)
    };
}

#[macro_export]
macro_rules! K2G_CORE_IOPAD {
    ($pa:expr) => {
        $crate::KEYSTONE_IOPAD_OFFSET!($pa, 0x1000)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
