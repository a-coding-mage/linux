/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2022 Linaro. All rights reserved.
 * Author: Casey Connolly <casey.connolly@linaro.org>
 */

// Dependency intent: corresponds to <linux/device.h>.

pub const COMMON_SUBTYPE: u32 = 0x00;
pub const PM8941_SUBTYPE: u32 = 0x01;
pub const PM8841_SUBTYPE: u32 = 0x02;
pub const PM8019_SUBTYPE: u32 = 0x03;
pub const PM8226_SUBTYPE: u32 = 0x04;
pub const PM8110_SUBTYPE: u32 = 0x05;
pub const PMA8084_SUBTYPE: u32 = 0x06;
pub const PMI8962_SUBTYPE: u32 = 0x07;
pub const PMD9635_SUBTYPE: u32 = 0x08;
pub const PM8994_SUBTYPE: u32 = 0x09;
pub const PMI8994_SUBTYPE: u32 = 0x0a;
pub const PM8916_SUBTYPE: u32 = 0x0b;
pub const PM8004_SUBTYPE: u32 = 0x0c;
pub const PM8909_SUBTYPE: u32 = 0x0d;
pub const PM8028_SUBTYPE: u32 = 0x0e;
pub const PM8901_SUBTYPE: u32 = 0x0f;
pub const PM8950_SUBTYPE: u32 = 0x10;
pub const PMI8950_SUBTYPE: u32 = 0x11;
pub const PMK8001_SUBTYPE: u32 = 0x12;
pub const PMI8996_SUBTYPE: u32 = 0x13;
pub const PM8998_SUBTYPE: u32 = 0x14;
pub const PMI8998_SUBTYPE: u32 = 0x15;
pub const PM8005_SUBTYPE: u32 = 0x18;
pub const PM8937_SUBTYPE: u32 = 0x19;
pub const PM660L_SUBTYPE: u32 = 0x1a;
pub const PM660_SUBTYPE: u32 = 0x1b;
pub const PM8150_SUBTYPE: u32 = 0x1e;
pub const PM8150L_SUBTYPE: u32 = 0x1f;
pub const PM8150B_SUBTYPE: u32 = 0x20;
pub const PMK8002_SUBTYPE: u32 = 0x21;
pub const PM8009_SUBTYPE: u32 = 0x24;
pub const PMI632_SUBTYPE: u32 = 0x25;
pub const PM8150C_SUBTYPE: u32 = 0x26;
pub const PM6150_SUBTYPE: u32 = 0x28;
pub const SMB2351_SUBTYPE: u32 = 0x29;
pub const PM8008_SUBTYPE: u32 = 0x2c;
pub const PM6125_SUBTYPE: u32 = 0x2d;
pub const PM7250B_SUBTYPE: u32 = 0x2e;
pub const PMK8350_SUBTYPE: u32 = 0x2f;
pub const PMR735B_SUBTYPE: u32 = 0x34;
pub const PM6350_SUBTYPE: u32 = 0x36;
pub const PM4125_SUBTYPE: u32 = 0x37;
pub const PM8010_SUBTYPE: u32 = 0x41;
pub const PM8550VS_SUBTYPE: u32 = 0x45;
pub const PM8550VE_SUBTYPE: u32 = 0x46;
pub const PMR735D_SUBTYPE: u32 = 0x48;
pub const PM8550_SUBTYPE: u32 = 0x49;
pub const PMK8550_SUBTYPE: u32 = 0x4a;
pub const PMM8650AU_SUBTYPE: u32 = 0x4e;
pub const PMM8650AU_PSAIL_SUBTYPE: u32 = 0x4f;
pub const PM8750B_SUBTYPE: u32 = 0x56;
pub const PMD8028_SUBTYPE: u32 = 0x57;
pub const PMK8850_SUBTYPE: u32 = 0x5c;
pub const PMH0101_SUBTYPE: u32 = 0x5d;
pub const SMB2370_SUBTYPE: u32 = 0x5f;
pub const PMH0104_SUBTYPE: u32 = 0x60;
pub const PMH0110_SUBTYPE: u32 = 0x61;
pub const PMCX0102_SUBTYPE: u32 = 0x62;
pub const PMI8998_FAB_ID_SMIC: u32 = 0x11;
pub const PMI8998_FAB_ID_GF: u32 = 0x30;

pub const PM660_FAB_ID_GF: u32 = 0x0;
pub const PM660_FAB_ID_TSMC: u32 = 0x2;
pub const PM660_FAB_ID_MX: u32 = 0x3;

#[repr(C)]
pub struct qcom_spmi_pmic {
    pub type_: core::ffi::c_uint,
    pub subtype: core::ffi::c_uint,
    pub major: core::ffi::c_uint,
    pub minor: core::ffi::c_uint,
    pub rev2: core::ffi::c_uint,
    pub fab_id: core::ffi::c_uint,
    pub name: *const core::ffi::c_char,
}

// External dependency declaration corresponding to struct device.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn qcom_pmic_get(dev: *mut device) -> *const qcom_spmi_pmic;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
