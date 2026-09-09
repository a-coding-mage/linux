/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2012, NVIDIA Corporation. All rights reserved.
 */

// Equivalent of the C condition: CONFIG_ARM && CONFIG_ARCH_TEGRA.
#[cfg(all(target_arch = "arm", feature = "CONFIG_ARCH_TEGRA"))]
extern "C" {
    pub fn tegra_pending_sgi() -> bool;
}

// Fallback when CONFIG_ARM and CONFIG_ARCH_TEGRA are not both enabled.
#[cfg(not(all(target_arch = "arm", feature = "CONFIG_ARCH_TEGRA")))]
#[inline]
pub fn tegra_pending_sgi() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
