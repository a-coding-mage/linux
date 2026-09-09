/* SPDX-License-Identifier: GPL-2.0+ */

// Dependency: linux/regmap.h

use core::ffi::c_void;

// External C type supplied by the surrounding kernel code.
pub struct device;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bcm2835_soc {
    BCM2835_PM_SOC_BCM2835,
    BCM2835_PM_SOC_BCM2711,
    BCM2835_PM_SOC_BCM2712,
}

#[repr(C)]
pub struct bcm2835_pm {
    pub dev: *mut device,
    pub base: *mut c_void,
    pub asb: *mut c_void,
    pub rpivid_asb: *mut c_void,
    pub soc: bcm2835_soc,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
