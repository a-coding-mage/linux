/* SPDX-License-Identifier: (GPL-2.0-only OR MIT) */
/*
 * Copyright (c) 2024 Amlogic, Inc. All rights reserved.
 * Author: Xianwei Zhao <xianwei.zhao@amlogic.com>
 */

/* Normal PIN bank */
pub const AMLOGIC_GPIO_A: i32 = 0;
pub const AMLOGIC_GPIO_B: i32 = 1;
pub const AMLOGIC_GPIO_C: i32 = 2;
pub const AMLOGIC_GPIO_D: i32 = 3;
pub const AMLOGIC_GPIO_E: i32 = 4;
pub const AMLOGIC_GPIO_F: i32 = 5;
pub const AMLOGIC_GPIO_G: i32 = 6;
pub const AMLOGIC_GPIO_H: i32 = 7;
pub const AMLOGIC_GPIO_I: i32 = 8;
pub const AMLOGIC_GPIO_J: i32 = 9;
pub const AMLOGIC_GPIO_K: i32 = 10;
pub const AMLOGIC_GPIO_L: i32 = 11;
pub const AMLOGIC_GPIO_M: i32 = 12;
pub const AMLOGIC_GPIO_N: i32 = 13;
pub const AMLOGIC_GPIO_O: i32 = 14;
pub const AMLOGIC_GPIO_P: i32 = 15;
pub const AMLOGIC_GPIO_Q: i32 = 16;
pub const AMLOGIC_GPIO_R: i32 = 17;
pub const AMLOGIC_GPIO_S: i32 = 18;
pub const AMLOGIC_GPIO_T: i32 = 19;
pub const AMLOGIC_GPIO_U: i32 = 20;
pub const AMLOGIC_GPIO_V: i32 = 21;
pub const AMLOGIC_GPIO_W: i32 = 22;
pub const AMLOGIC_GPIO_X: i32 = 23;
pub const AMLOGIC_GPIO_Y: i32 = 24;
pub const AMLOGIC_GPIO_Z: i32 = 25;

/* Special PIN bank */
pub const AMLOGIC_GPIO_DV: i32 = 26;
pub const AMLOGIC_GPIO_AO: i32 = 27;
pub const AMLOGIC_GPIO_CC: i32 = 28;
pub const AMLOGIC_GPIO_TEST_N: i32 = 29;
pub const AMLOGIC_GPIO_ANALOG: i32 = 30;

#[macro_export]
macro_rules! AML_PINMUX {
    ($bank:expr, $offset:expr, $mode:expr) => {
        (((($bank) << 8) + ($offset)) << 8) | ($mode)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
