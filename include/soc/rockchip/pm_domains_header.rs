/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2022, The Chromium OS Authors. All rights reserved.
 */

/* CONFIG_ROCKCHIP_PM_DOMAINS */
#[cfg(feature = "CONFIG_ROCKCHIP_PM_DOMAINS")]
extern "C" {
    pub fn rockchip_pmu_block() -> ::core::ffi::c_int;
    pub fn rockchip_pmu_unblock();
}

/* !CONFIG_ROCKCHIP_PM_DOMAINS */
#[cfg(not(feature = "CONFIG_ROCKCHIP_PM_DOMAINS"))]
#[inline]
pub fn rockchip_pmu_block() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_ROCKCHIP_PM_DOMAINS"))]
#[inline]
pub fn rockchip_pmu_unblock() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
