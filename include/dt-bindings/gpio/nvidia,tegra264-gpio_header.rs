/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/* Copyright (c) 2026, NVIDIA CORPORATION. All rights reserved. */

/*
 * This header provides constants for binding nvidia,tegra264-gpio*.
 *
 * The first cell in Tegra's GPIO specifier is the GPIO ID. The macros below
 * provide names for this.
 *
 * The second cell contains standard flag values specified in gpio.h.
 */

// Dependency intent: <dt-bindings/gpio/gpio.h>

/* GPIOs implemented by main GPIO controller */
pub const TEGRA264_MAIN_GPIO_PORT_T: i32 = 0;
pub const TEGRA264_MAIN_GPIO_PORT_U: i32 = 1;
pub const TEGRA264_MAIN_GPIO_PORT_V: i32 = 2;
pub const TEGRA264_MAIN_GPIO_PORT_W: i32 = 3;
pub const TEGRA264_MAIN_GPIO_PORT_AL: i32 = 4;
pub const TEGRA264_MAIN_GPIO_PORT_Y: i32 = 5;
pub const TEGRA264_MAIN_GPIO_PORT_Z: i32 = 6;
pub const TEGRA264_MAIN_GPIO_PORT_X: i32 = 7;
pub const TEGRA264_MAIN_GPIO_PORT_H: i32 = 8;
pub const TEGRA264_MAIN_GPIO_PORT_J: i32 = 9;
pub const TEGRA264_MAIN_GPIO_PORT_K: i32 = 10;
pub const TEGRA264_MAIN_GPIO_PORT_L: i32 = 11;
pub const TEGRA264_MAIN_GPIO_PORT_M: i32 = 12;
pub const TEGRA264_MAIN_GPIO_PORT_P: i32 = 13;
pub const TEGRA264_MAIN_GPIO_PORT_Q: i32 = 14;
pub const TEGRA264_MAIN_GPIO_PORT_R: i32 = 15;
pub const TEGRA264_MAIN_GPIO_PORT_S: i32 = 16;
pub const TEGRA264_MAIN_GPIO_PORT_F: i32 = 17;
pub const TEGRA264_MAIN_GPIO_PORT_G: i32 = 18;

macro_rules! TEGRA264_MAIN_GPIO {
    (T, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_T * 8) + ($offset)) };
    (U, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_U * 8) + ($offset)) };
    (V, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_V * 8) + ($offset)) };
    (W, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_W * 8) + ($offset)) };
    (AL, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_AL * 8) + ($offset)) };
    (Y, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_Y * 8) + ($offset)) };
    (Z, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_Z * 8) + ($offset)) };
    (X, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_X * 8) + ($offset)) };
    (H, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_H * 8) + ($offset)) };
    (J, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_J * 8) + ($offset)) };
    (K, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_K * 8) + ($offset)) };
    (L, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_L * 8) + ($offset)) };
    (M, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_M * 8) + ($offset)) };
    (P, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_P * 8) + ($offset)) };
    (Q, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_Q * 8) + ($offset)) };
    (R, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_R * 8) + ($offset)) };
    (S, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_S * 8) + ($offset)) };
    (F, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_F * 8) + ($offset)) };
    (G, $offset:expr) => { ((TEGRA264_MAIN_GPIO_PORT_G * 8) + ($offset)) };
}

/* GPIOs implemented by AON GPIO controller */
pub const TEGRA264_AON_GPIO_PORT_AA: i32 = 0;
pub const TEGRA264_AON_GPIO_PORT_BB: i32 = 1;
pub const TEGRA264_AON_GPIO_PORT_CC: i32 = 2;
pub const TEGRA264_AON_GPIO_PORT_DD: i32 = 3;
pub const TEGRA264_AON_GPIO_PORT_EE: i32 = 4;

macro_rules! TEGRA264_AON_GPIO {
    (AA, $offset:expr) => { ((TEGRA264_AON_GPIO_PORT_AA * 8) + ($offset)) };
    (BB, $offset:expr) => { ((TEGRA264_AON_GPIO_PORT_BB * 8) + ($offset)) };
    (CC, $offset:expr) => { ((TEGRA264_AON_GPIO_PORT_CC * 8) + ($offset)) };
    (DD, $offset:expr) => { ((TEGRA264_AON_GPIO_PORT_DD * 8) + ($offset)) };
    (EE, $offset:expr) => { ((TEGRA264_AON_GPIO_PORT_EE * 8) + ($offset)) };
}

pub const TEGRA264_UPHY_GPIO_PORT_A: i32 = 0;
pub const TEGRA264_UPHY_GPIO_PORT_B: i32 = 1;
pub const TEGRA264_UPHY_GPIO_PORT_C: i32 = 2;
pub const TEGRA264_UPHY_GPIO_PORT_D: i32 = 3;
pub const TEGRA264_UPHY_GPIO_PORT_E: i32 = 4;

macro_rules! TEGRA264_UPHY_GPIO {
    (A, $offset:expr) => { ((TEGRA264_UPHY_GPIO_PORT_A * 8) + ($offset)) };
    (B, $offset:expr) => { ((TEGRA264_UPHY_GPIO_PORT_B * 8) + ($offset)) };
    (C, $offset:expr) => { ((TEGRA264_UPHY_GPIO_PORT_C * 8) + ($offset)) };
    (D, $offset:expr) => { ((TEGRA264_UPHY_GPIO_PORT_D * 8) + ($offset)) };
    (E, $offset:expr) => { ((TEGRA264_UPHY_GPIO_PORT_E * 8) + ($offset)) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
