/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for binding nvidia,tegra186-gpio*.
 *
 * The first cell in Tegra's GPIO specifier is the GPIO ID. The macros below
 * provide names for this.
 *
 * The second cell contains standard flag values specified in gpio.h.
 */

// Dependency supplied by the corresponding GPIO bindings header.

/* GPIOs implemented by main GPIO controller */
pub const TEGRA186_MAIN_GPIO_PORT_A: i32 = 0;
pub const TEGRA186_MAIN_GPIO_PORT_B: i32 = 1;
pub const TEGRA186_MAIN_GPIO_PORT_C: i32 = 2;
pub const TEGRA186_MAIN_GPIO_PORT_D: i32 = 3;
pub const TEGRA186_MAIN_GPIO_PORT_E: i32 = 4;
pub const TEGRA186_MAIN_GPIO_PORT_F: i32 = 5;
pub const TEGRA186_MAIN_GPIO_PORT_G: i32 = 6;
pub const TEGRA186_MAIN_GPIO_PORT_H: i32 = 7;
pub const TEGRA186_MAIN_GPIO_PORT_I: i32 = 8;
pub const TEGRA186_MAIN_GPIO_PORT_J: i32 = 9;
pub const TEGRA186_MAIN_GPIO_PORT_K: i32 = 10;
pub const TEGRA186_MAIN_GPIO_PORT_L: i32 = 11;
pub const TEGRA186_MAIN_GPIO_PORT_M: i32 = 12;
pub const TEGRA186_MAIN_GPIO_PORT_N: i32 = 13;
pub const TEGRA186_MAIN_GPIO_PORT_O: i32 = 14;
pub const TEGRA186_MAIN_GPIO_PORT_P: i32 = 15;
pub const TEGRA186_MAIN_GPIO_PORT_Q: i32 = 16;
pub const TEGRA186_MAIN_GPIO_PORT_R: i32 = 17;
pub const TEGRA186_MAIN_GPIO_PORT_T: i32 = 18;
pub const TEGRA186_MAIN_GPIO_PORT_X: i32 = 19;
pub const TEGRA186_MAIN_GPIO_PORT_Y: i32 = 20;
pub const TEGRA186_MAIN_GPIO_PORT_BB: i32 = 21;
pub const TEGRA186_MAIN_GPIO_PORT_CC: i32 = 22;

macro_rules! TEGRA186_MAIN_GPIO {
    ($port:ident, $offset:expr) => {
        (concat_idents!(TEGRA186_MAIN_GPIO_PORT_, $port) * 8) + $offset
    };
}

/* GPIOs implemented by AON GPIO controller */
pub const TEGRA186_AON_GPIO_PORT_S: i32 = 0;
pub const TEGRA186_AON_GPIO_PORT_U: i32 = 1;
pub const TEGRA186_AON_GPIO_PORT_V: i32 = 2;
pub const TEGRA186_AON_GPIO_PORT_W: i32 = 3;
pub const TEGRA186_AON_GPIO_PORT_Z: i32 = 4;
pub const TEGRA186_AON_GPIO_PORT_AA: i32 = 5;
pub const TEGRA186_AON_GPIO_PORT_EE: i32 = 6;
pub const TEGRA186_AON_GPIO_PORT_FF: i32 = 7;

macro_rules! TEGRA186_AON_GPIO {
    ($port:ident, $offset:expr) => {
        (concat_idents!(TEGRA186_AON_GPIO_PORT_, $port) * 8) + $offset
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
