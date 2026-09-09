/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/* Copyright (c) 2026, NVIDIA CORPORATION. All rights reserved. */

/*
 * This header provides constants for binding nvidia,tegra238-gpio*.
 *
 * The first cell in Tegra's GPIO specifier is the GPIO ID. The macros below
 * provide names for this.
 *
 * The second cell contains standard flag values specified in gpio.h.
 */

// Dependency intent preserved from: <dt-bindings/gpio/gpio.h>

/* GPIOs implemented by main GPIO controller */
pub const TEGRA238_MAIN_GPIO_PORT_A: i32 = 0;
pub const TEGRA238_MAIN_GPIO_PORT_B: i32 = 1;
pub const TEGRA238_MAIN_GPIO_PORT_C: i32 = 2;
pub const TEGRA238_MAIN_GPIO_PORT_D: i32 = 3;
pub const TEGRA238_MAIN_GPIO_PORT_E: i32 = 4;
pub const TEGRA238_MAIN_GPIO_PORT_F: i32 = 5;
pub const TEGRA238_MAIN_GPIO_PORT_G: i32 = 6;
pub const TEGRA238_MAIN_GPIO_PORT_H: i32 = 7;
pub const TEGRA238_MAIN_GPIO_PORT_J: i32 = 8;
pub const TEGRA238_MAIN_GPIO_PORT_K: i32 = 9;
pub const TEGRA238_MAIN_GPIO_PORT_L: i32 = 10;
pub const TEGRA238_MAIN_GPIO_PORT_M: i32 = 11;
pub const TEGRA238_MAIN_GPIO_PORT_N: i32 = 12;
pub const TEGRA238_MAIN_GPIO_PORT_P: i32 = 13;
pub const TEGRA238_MAIN_GPIO_PORT_Q: i32 = 14;
pub const TEGRA238_MAIN_GPIO_PORT_R: i32 = 15;
pub const TEGRA238_MAIN_GPIO_PORT_S: i32 = 16;
pub const TEGRA238_MAIN_GPIO_PORT_T: i32 = 17;
pub const TEGRA238_MAIN_GPIO_PORT_U: i32 = 18;
pub const TEGRA238_MAIN_GPIO_PORT_V: i32 = 19;
pub const TEGRA238_MAIN_GPIO_PORT_W: i32 = 20;
pub const TEGRA238_MAIN_GPIO_PORT_X: i32 = 21;

#[macro_export]
macro_rules! TEGRA238_MAIN_GPIO {
    (A, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_A * 8) + ($offset) };
    (B, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_B * 8) + ($offset) };
    (C, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_C * 8) + ($offset) };
    (D, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_D * 8) + ($offset) };
    (E, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_E * 8) + ($offset) };
    (F, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_F * 8) + ($offset) };
    (G, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_G * 8) + ($offset) };
    (H, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_H * 8) + ($offset) };
    (J, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_J * 8) + ($offset) };
    (K, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_K * 8) + ($offset) };
    (L, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_L * 8) + ($offset) };
    (M, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_M * 8) + ($offset) };
    (N, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_N * 8) + ($offset) };
    (P, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_P * 8) + ($offset) };
    (Q, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_Q * 8) + ($offset) };
    (R, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_R * 8) + ($offset) };
    (S, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_S * 8) + ($offset) };
    (T, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_T * 8) + ($offset) };
    (U, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_U * 8) + ($offset) };
    (V, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_V * 8) + ($offset) };
    (W, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_W * 8) + ($offset) };
    (X, $offset:expr) => { ($crate::TEGRA238_MAIN_GPIO_PORT_X * 8) + ($offset) };
}

/* GPIOs implemented by AON GPIO controller */
pub const TEGRA238_AON_GPIO_PORT_AA: i32 = 0;
pub const TEGRA238_AON_GPIO_PORT_BB: i32 = 1;
pub const TEGRA238_AON_GPIO_PORT_CC: i32 = 2;
pub const TEGRA238_AON_GPIO_PORT_DD: i32 = 3;
pub const TEGRA238_AON_GPIO_PORT_EE: i32 = 4;
pub const TEGRA238_AON_GPIO_PORT_FF: i32 = 5;
pub const TEGRA238_AON_GPIO_PORT_GG: i32 = 6;
pub const TEGRA238_AON_GPIO_PORT_HH: i32 = 7;

#[macro_export]
macro_rules! TEGRA238_AON_GPIO {
    (AA, $offset:expr) => { ($crate::TEGRA238_AON_GPIO_PORT_AA * 8) + ($offset) };
    (BB, $offset:expr) => { ($crate::TEGRA238_AON_GPIO_PORT_BB * 8) + ($offset) };
    (CC, $offset:expr) => { ($crate::TEGRA238_AON_GPIO_PORT_CC * 8) + ($offset) };
    (DD, $offset:expr) => { ($crate::TEGRA238_AON_GPIO_PORT_DD * 8) + ($offset) };
    (EE, $offset:expr) => { ($crate::TEGRA238_AON_GPIO_PORT_EE * 8) + ($offset) };
    (FF, $offset:expr) => { ($crate::TEGRA238_AON_GPIO_PORT_FF * 8) + ($offset) };
    (GG, $offset:expr) => { ($crate::TEGRA238_AON_GPIO_PORT_GG * 8) + ($offset) };
    (HH, $offset:expr) => { ($crate::TEGRA238_AON_GPIO_PORT_HH * 8) + ($offset) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
