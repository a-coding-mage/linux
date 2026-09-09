/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for binding nvidia,tegra*-gpio.
 *
 * The first cell in Tegra's GPIO specifier is the GPIO ID. The macros below
 * provide names for this.
 *
 * The second cell contains standard flag values specified in gpio.h.
 */

// Dependency: <dt-bindings/gpio/gpio.h>

pub const TEGRA_GPIO_PORT_A: i32 = 0;
pub const TEGRA_GPIO_PORT_B: i32 = 1;
pub const TEGRA_GPIO_PORT_C: i32 = 2;
pub const TEGRA_GPIO_PORT_D: i32 = 3;
pub const TEGRA_GPIO_PORT_E: i32 = 4;
pub const TEGRA_GPIO_PORT_F: i32 = 5;
pub const TEGRA_GPIO_PORT_G: i32 = 6;
pub const TEGRA_GPIO_PORT_H: i32 = 7;
pub const TEGRA_GPIO_PORT_I: i32 = 8;
pub const TEGRA_GPIO_PORT_J: i32 = 9;
pub const TEGRA_GPIO_PORT_K: i32 = 10;
pub const TEGRA_GPIO_PORT_L: i32 = 11;
pub const TEGRA_GPIO_PORT_M: i32 = 12;
pub const TEGRA_GPIO_PORT_N: i32 = 13;
pub const TEGRA_GPIO_PORT_O: i32 = 14;
pub const TEGRA_GPIO_PORT_P: i32 = 15;
pub const TEGRA_GPIO_PORT_Q: i32 = 16;
pub const TEGRA_GPIO_PORT_R: i32 = 17;
pub const TEGRA_GPIO_PORT_S: i32 = 18;
pub const TEGRA_GPIO_PORT_T: i32 = 19;
pub const TEGRA_GPIO_PORT_U: i32 = 20;
pub const TEGRA_GPIO_PORT_V: i32 = 21;
pub const TEGRA_GPIO_PORT_W: i32 = 22;
pub const TEGRA_GPIO_PORT_X: i32 = 23;
pub const TEGRA_GPIO_PORT_Y: i32 = 24;
pub const TEGRA_GPIO_PORT_Z: i32 = 25;
pub const TEGRA_GPIO_PORT_AA: i32 = 26;
pub const TEGRA_GPIO_PORT_BB: i32 = 27;
pub const TEGRA_GPIO_PORT_CC: i32 = 28;
pub const TEGRA_GPIO_PORT_DD: i32 = 29;
pub const TEGRA_GPIO_PORT_EE: i32 = 30;
pub const TEGRA_GPIO_PORT_FF: i32 = 31;

macro_rules! TEGRA_GPIO {
    ($port:ident, $offset:expr) => {
        (paste_port!($port) * 8) + $offset
    };
}

macro_rules! paste_port {
    (A) => { TEGRA_GPIO_PORT_A };
    (B) => { TEGRA_GPIO_PORT_B };
    (C) => { TEGRA_GPIO_PORT_C };
    (D) => { TEGRA_GPIO_PORT_D };
    (E) => { TEGRA_GPIO_PORT_E };
    (F) => { TEGRA_GPIO_PORT_F };
    (G) => { TEGRA_GPIO_PORT_G };
    (H) => { TEGRA_GPIO_PORT_H };
    (I) => { TEGRA_GPIO_PORT_I };
    (J) => { TEGRA_GPIO_PORT_J };
    (K) => { TEGRA_GPIO_PORT_K };
    (L) => { TEGRA_GPIO_PORT_L };
    (M) => { TEGRA_GPIO_PORT_M };
    (N) => { TEGRA_GPIO_PORT_N };
    (O) => { TEGRA_GPIO_PORT_O };
    (P) => { TEGRA_GPIO_PORT_P };
    (Q) => { TEGRA_GPIO_PORT_Q };
    (R) => { TEGRA_GPIO_PORT_R };
    (S) => { TEGRA_GPIO_PORT_S };
    (T) => { TEGRA_GPIO_PORT_T };
    (U) => { TEGRA_GPIO_PORT_U };
    (V) => { TEGRA_GPIO_PORT_V };
    (W) => { TEGRA_GPIO_PORT_W };
    (X) => { TEGRA_GPIO_PORT_X };
    (Y) => { TEGRA_GPIO_PORT_Y };
    (Z) => { TEGRA_GPIO_PORT_Z };
    (AA) => { TEGRA_GPIO_PORT_AA };
    (BB) => { TEGRA_GPIO_PORT_BB };
    (CC) => { TEGRA_GPIO_PORT_CC };
    (DD) => { TEGRA_GPIO_PORT_DD };
    (EE) => { TEGRA_GPIO_PORT_EE };
    (FF) => { TEGRA_GPIO_PORT_FF };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
