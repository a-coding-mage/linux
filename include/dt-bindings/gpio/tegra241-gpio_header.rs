/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2021, NVIDIA CORPORATION. All rights reserved. */

/*
 * This header provides constants for the nvidia,tegra241-gpio DT binding.
 *
 * The first cell in Tegra's GPIO specifier is the GPIO ID. The macros below
 * provide names for this.
 *
 * The second cell contains standard flag values specified in gpio.h.
 */

/* Dependency intent: <dt-bindings/gpio/gpio.h> */

/* GPIOs implemented by main GPIO controller */
pub const TEGRA241_MAIN_GPIO_PORT_A: i32 = 0;
pub const TEGRA241_MAIN_GPIO_PORT_B: i32 = 1;
pub const TEGRA241_MAIN_GPIO_PORT_C: i32 = 2;
pub const TEGRA241_MAIN_GPIO_PORT_D: i32 = 3;
pub const TEGRA241_MAIN_GPIO_PORT_E: i32 = 4;
pub const TEGRA241_MAIN_GPIO_PORT_F: i32 = 5;
pub const TEGRA241_MAIN_GPIO_PORT_G: i32 = 6;
pub const TEGRA241_MAIN_GPIO_PORT_H: i32 = 7;
pub const TEGRA241_MAIN_GPIO_PORT_I: i32 = 8;
pub const TEGRA241_MAIN_GPIO_PORT_J: i32 = 9;
pub const TEGRA241_MAIN_GPIO_PORT_K: i32 = 10;
pub const TEGRA241_MAIN_GPIO_PORT_L: i32 = 11;

#[macro_export]
macro_rules! TEGRA241_MAIN_GPIO {
    (A, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_A * 8) + ($offset) };
    (B, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_B * 8) + ($offset) };
    (C, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_C * 8) + ($offset) };
    (D, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_D * 8) + ($offset) };
    (E, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_E * 8) + ($offset) };
    (F, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_F * 8) + ($offset) };
    (G, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_G * 8) + ($offset) };
    (H, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_H * 8) + ($offset) };
    (I, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_I * 8) + ($offset) };
    (J, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_J * 8) + ($offset) };
    (K, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_K * 8) + ($offset) };
    (L, $offset:expr) => { (TEGRA241_MAIN_GPIO_PORT_L * 8) + ($offset) };
}

/* GPIOs implemented by AON GPIO controller */
pub const TEGRA241_AON_GPIO_PORT_AA: i32 = 0;
pub const TEGRA241_AON_GPIO_PORT_BB: i32 = 1;

#[macro_export]
macro_rules! TEGRA241_AON_GPIO {
    (AA, $offset:expr) => { (TEGRA241_AON_GPIO_PORT_AA * 8) + ($offset) };
    (BB, $offset:expr) => { (TEGRA241_AON_GPIO_PORT_BB * 8) + ($offset) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
