/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/*
 * This header provides Tegra114-specific constants for binding
 * nvidia,tegra114-car.
 */

// Header guard: _DT_BINDINGS_RESET_NVIDIA_TEGRA114_CAR_H

pub const fn TEGRA114_RESET(x: u32) -> u32 {
    5 * 32 + x
}

pub const TEGRA114_RST_DFLL_DVCO: u32 = TEGRA114_RESET(0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
