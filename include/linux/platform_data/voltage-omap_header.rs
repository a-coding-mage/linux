/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP Voltage Management Routines
 *
 * Copyright (C) 2011, Texas Instruments, Inc.
 */

/**
 * struct omap_volt_data - Omap voltage specific data.
 * @volt_nominal:\tThe possible voltage value in uV
 * @sr_efuse_offs:\tThe offset of the efuse register(from system
 *\t\t\tcontrol module base address) from where to read
 *\t\t\tthe n-target value for the smartreflex module.
 * @sr_errminlimit:\tError min limit value for smartreflex. This value
 *\t\t\tdiffers at differnet opp and thus is linked
 *\t\t\twith voltage.
 * @vp_errgain:\t\tError gain value for the voltage processor. This
 *\t\t\tfield also differs according to the voltage/opp.
 */
#[repr(C)]
pub struct omap_volt_data {
    pub volt_nominal: u32,
    pub sr_efuse_offs: u32,
    pub sr_errminlimit: u8,
    pub vp_errgain: u8,
}

#[repr(C)]
pub struct voltagedomain {
    _private: [u8; 0],
}

extern "C" {
    pub fn voltdm_lookup(name: *const core::ffi::c_char) -> *mut voltagedomain;
    pub fn voltdm_get_voltage(voltdm: *mut voltagedomain) -> core::ffi::c_ulong;
    pub fn omap_voltage_get_voltdata(
        voltdm: *mut voltagedomain,
        volt: core::ffi::c_ulong,
    ) -> *mut omap_volt_data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
