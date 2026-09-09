/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved. */

/*
 * This header provides constants for the nvidia,tegra256-gpio DT binding.
 *
 * The first cell in Tegra's GPIO specifier is the GPIO ID.
 * The macros below provide names for this.
 *
 * The second cell contains standard flag values specified in gpio.h.
 */

// Dependency intent from C: <dt-bindings/gpio/gpio.h>

/* GPIOs implemented by main GPIO controller */
pub const TEGRA256_MAIN_GPIO_PORT_A: u32 = 0;
pub const TEGRA256_MAIN_GPIO_PORT_B: u32 = 1;
pub const TEGRA256_MAIN_GPIO_PORT_C: u32 = 2;
pub const TEGRA256_MAIN_GPIO_PORT_D: u32 = 3;

macro_rules! TEGRA256_MAIN_GPIO {
	(A, $offset:expr) => {(TEGRA256_MAIN_GPIO_PORT_A * 8) + ($offset)};
	(B, $offset:expr) => {(TEGRA256_MAIN_GPIO_PORT_B * 8) + ($offset)};
	(C, $offset:expr) => {(TEGRA256_MAIN_GPIO_PORT_C * 8) + ($offset)};
	(D, $offset:expr) => {(TEGRA256_MAIN_GPIO_PORT_D * 8) + ($offset)};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
