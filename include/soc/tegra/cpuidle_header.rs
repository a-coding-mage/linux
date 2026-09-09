/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2013, NVIDIA CORPORATION.  All rights reserved.
 */

#[cfg(CONFIG_ARM_TEGRA_CPUIDLE)]
unsafe extern "C" {
    pub fn tegra_cpuidle_pcie_irqs_in_use();
}

#[cfg(not(CONFIG_ARM_TEGRA_CPUIDLE))]
#[inline]
pub unsafe fn tegra_cpuidle_pcie_irqs_in_use() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
