/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides Tegra124-specific constants for binding
 * nvidia,tegra124-car.
 */

// C header guard: _DT_BINDINGS_RESET_TEGRA124_CAR_H

pub const fn TEGRA124_RESET(x: u32) -> u32 {
    6 * 32 + x
}

pub const TEGRA124_RST_DFLL_DVCO: u32 = TEGRA124_RESET(0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
