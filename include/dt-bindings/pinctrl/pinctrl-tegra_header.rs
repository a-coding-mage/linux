/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This header provides constants for Tegra pinctrl bindings.
 *
 * Copyright (c) 2013, NVIDIA CORPORATION.  All rights reserved.
 *
 * Author: Laxman Dewangan <ldewangan@nvidia.com>
 */

/*
 * Enable/disable for diffeent dt properties. This is applicable for
 * properties nvidia,enable-input, nvidia,tristate, nvidia,open-drain,
 * nvidia,lock, nvidia,rcv-sel, nvidia,high-speed-mode, nvidia,schmitt.
 */
pub const TEGRA_PIN_DISABLE: i32 = 0;
pub const TEGRA_PIN_ENABLE: i32 = 1;

pub const TEGRA_PIN_PULL_NONE: i32 = 0;
pub const TEGRA_PIN_PULL_DOWN: i32 = 1;
pub const TEGRA_PIN_PULL_UP: i32 = 2;

/* Low power mode driver */
pub const TEGRA_PIN_LP_DRIVE_DIV_8: i32 = 0;
pub const TEGRA_PIN_LP_DRIVE_DIV_4: i32 = 1;
pub const TEGRA_PIN_LP_DRIVE_DIV_2: i32 = 2;
pub const TEGRA_PIN_LP_DRIVE_DIV_1: i32 = 3;

/* Rising/Falling slew rate */
pub const TEGRA_PIN_SLEW_RATE_FASTEST: i32 = 0;
pub const TEGRA_PIN_SLEW_RATE_FAST: i32 = 1;
pub const TEGRA_PIN_SLEW_RATE_SLOW: i32 = 2;
pub const TEGRA_PIN_SLEW_RATE_SLOWEST: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
