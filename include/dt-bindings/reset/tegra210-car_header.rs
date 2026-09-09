/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides Tegra210-specific constants for binding
 * nvidia,tegra210-car.
 */

// C header guard: _DT_BINDINGS_RESET_TEGRA210_CAR_H

pub const fn TEGRA210_RESET(x: i32) -> i32 {
    7 * 32 + x
}

pub const TEGRA210_RST_DFLL_DVCO: i32 = TEGRA210_RESET(0);
pub const TEGRA210_RST_ADSP: i32 = TEGRA210_RESET(1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
