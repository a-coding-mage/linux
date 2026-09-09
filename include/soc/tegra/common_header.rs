/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 NVIDIA Corporation
 */

use core::ffi::c_int;

// C dependency: struct device is declared by the Linux device headers.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/**
 * Tegra SoC core device OPP table configuration
 *
 * @init_state: pre-initialize OPP state of a device
 */
#[repr(C)]
pub struct tegra_core_opp_params {
    pub init_state: bool,
}

#[cfg(CONFIG_ARCH_TEGRA)]
unsafe extern "C" {
    pub fn soc_is_tegra() -> bool;

    pub fn devm_tegra_core_dev_init_opp_table(
        dev: *mut device,
        params: *mut tegra_core_opp_params,
    ) -> c_int;
}

#[cfg(not(CONFIG_ARCH_TEGRA))]
#[inline]
pub const fn soc_is_tegra() -> bool {
    false
}

#[cfg(not(CONFIG_ARCH_TEGRA))]
#[inline]
pub unsafe fn devm_tegra_core_dev_init_opp_table(
    _dev: *mut device,
    _params: *mut tegra_core_opp_params,
) -> c_int {
    -ENODEV
}

// Supplied by the Linux errno headers.
unsafe extern "C" {
    static ENODEV: c_int;
}

#[inline]
pub unsafe fn devm_tegra_core_dev_init_opp_table_common(dev: *mut device) -> c_int {
    let mut opp_params = tegra_core_opp_params { init_state: false };
    let err: c_int;

    opp_params.init_state = true;

    err = devm_tegra_core_dev_init_opp_table(dev, &mut opp_params);
    if err != -ENODEV {
        return err;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
