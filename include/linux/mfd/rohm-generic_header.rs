/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2018 ROHM Semiconductors */

// Translated from linux/mfd/rohm-generic.h.
// The declarations below depend on types supplied by the Linux Rust bindings.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rohm_chip_type {
    ROHM_CHIP_TYPE_BD9571,
    ROHM_CHIP_TYPE_BD9573,
    ROHM_CHIP_TYPE_BD9574,
    ROHM_CHIP_TYPE_BD9576,
    ROHM_CHIP_TYPE_BD71815,
    ROHM_CHIP_TYPE_BD71828,
    ROHM_CHIP_TYPE_BD71837,
    ROHM_CHIP_TYPE_BD71847,
    ROHM_CHIP_TYPE_BD72720,
    ROHM_CHIP_TYPE_BD96801,
    ROHM_CHIP_TYPE_BD96802,
    ROHM_CHIP_TYPE_BD96805,
    ROHM_CHIP_TYPE_BD96806,
    ROHM_CHIP_TYPE_AMOUNT,
}

#[repr(C)]
pub struct rohm_regmap_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

pub const ROHM_DVS_LEVEL_RUN: u64 = 1 << 0;
pub const ROHM_DVS_LEVEL_IDLE: u64 = 1 << 1;
pub const ROHM_DVS_LEVEL_SUSPEND: u64 = 1 << 2;
pub const ROHM_DVS_LEVEL_LPSR: u64 = 1 << 3;
pub const ROHM_DVS_LEVEL_SNVS: u64 = 1 << 4;
pub const ROHM_DVS_LEVEL_VALID_AMOUNT: u32 = 5;
pub const ROHM_DVS_LEVEL_UNKNOWN: u64 = 0;

/**
 * Dynamic voltage scaling register descriptions.
 *
 * Description of ROHM PMICs voltage configuration registers for different
 * system states. This is used to correctly configure the PMIC at startup
 * based on values read from DT.
 */
#[repr(C)]
pub struct rohm_dvs_config {
    pub level_map: u64,
    pub run_reg: u32,
    pub run_mask: u32,
    pub run_on_mask: u32,
    pub idle_reg: u32,
    pub idle_mask: u32,
    pub idle_on_mask: u32,
    pub suspend_reg: u32,
    pub suspend_mask: u32,
    pub suspend_on_mask: u32,
    pub lpsr_reg: u32,
    pub lpsr_mask: u32,
    pub lpsr_on_mask: u32,
    pub snvs_reg: u32,
    pub snvs_mask: u32,
    pub snvs_on_mask: u32,
}

// Preserves the source condition: IS_ENABLED(CONFIG_REGULATOR_ROHM).
#[cfg(feature = "CONFIG_REGULATOR_ROHM")]
extern "C" {
    pub fn rohm_regulator_set_dvs_levels(
        dvs: *const rohm_dvs_config,
        np: *mut device_node,
        desc: *const regulator_desc,
        regmap: *mut regmap,
    ) -> i32;

    pub fn rohm_regulator_set_voltage_sel_restricted(
        rdev: *mut regulator_dev,
        sel: u32,
    ) -> i32;
}

// External Linux types supplied by the including translation unit.
extern "C" {
    pub type device;
    pub type regmap;
    pub type device_node;
    pub type regulator_desc;
    pub type regulator_dev;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
