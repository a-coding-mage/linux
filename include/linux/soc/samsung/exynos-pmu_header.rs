/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Header for Exynos PMU Driver support
 */

// Forward declarations supplied by other translated dependencies.
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sys_powerdown {
    SYS_AFTR,
    SYS_LPA,
    SYS_SLEEP,
    NUM_SYS_POWERDOWN,
}

extern "C" {
    pub fn exynos_sys_powerdown_conf(mode: sys_powerdown);
}

// CONFIG_EXYNOS_PMU controls whether these functions are provided externally
// by the PMU implementation or as the original ENODEV stubs.
#[cfg(CONFIG_EXYNOS_PMU)]
extern "C" {
    pub fn exynos_get_pmu_regmap() -> *mut regmap;
    pub fn exynos_get_pmu_regmap_by_phandle(
        np: *mut device_node,
        propname: *const core::ffi::c_char,
    ) -> *mut regmap;
}

#[cfg(not(CONFIG_EXYNOS_PMU))]
pub unsafe fn exynos_get_pmu_regmap() -> *mut regmap {
    ERR_PTR(-ENODEV)
}

#[cfg(not(CONFIG_EXYNOS_PMU))]
pub unsafe fn exynos_get_pmu_regmap_by_phandle(
    _np: *mut device_node,
    _propname: *const core::ffi::c_char,
) -> *mut regmap {
    ERR_PTR(-ENODEV)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
