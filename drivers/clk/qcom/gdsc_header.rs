/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015, 2017-2018, 2022, The Linux Foundation. All rights reserved.
 */

/* Dependencies supplied by the surrounding translation unit. */
use core::ffi::c_char;

#[repr(C)]
pub struct icc_path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_controller_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct generic_pm_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_domain_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gdsc {
    pub pd: generic_pm_domain,
    pub parent: *mut generic_pm_domain,
    pub regmap: *mut regmap,
    pub gdscr: core::ffi::c_uint,
    pub collapse_ctrl: core::ffi::c_uint,
    pub collapse_mask: core::ffi::c_uint,
    pub gds_hw_ctrl: core::ffi::c_uint,
    pub clamp_io_ctrl: core::ffi::c_uint,
    pub cxcs: *mut core::ffi::c_uint,
    pub cxc_count: core::ffi::c_uint,
    pub en_rest_wait_val: core::ffi::c_uint,
    pub en_few_wait_val: core::ffi::c_uint,
    pub clk_dis_wait_val: core::ffi::c_uint,
    pub pwrsts: u8,
    pub flags: u16,
    pub rcdev: *mut reset_controller_dev,
    pub resets: *mut core::ffi::c_uint,
    pub reset_count: core::ffi::c_uint,
    pub supply: *const c_char,
    pub rsupply: *mut regulator,
    pub needs_icc: bool,
    pub icc_path_index: core::ffi::c_uint,
    pub icc_path: *mut icc_path,
}

pub const PWRSTS_OFF: u8 = 1 << 0;
pub const PWRSTS_RET: u8 = 1 << 1;
pub const PWRSTS_ON: u8 = 1 << 2;
pub const PWRSTS_OFF_ON: u8 = PWRSTS_OFF | PWRSTS_ON;
pub const PWRSTS_RET_ON: u8 = PWRSTS_RET | PWRSTS_ON;

pub const VOTABLE: u16 = 1 << 0;
pub const CLAMP_IO: u16 = 1 << 1;
pub const HW_CTRL: u16 = 1 << 2;
pub const SW_RESET: u16 = 1 << 3;
pub const AON_RESET: u16 = 1 << 4;
pub const POLL_CFG_GDSCR: u16 = 1 << 5;
pub const ALWAYS_ON: u16 = 1 << 6;
pub const RETAIN_FF_ENABLE: u16 = 1 << 7;
pub const NO_RET_PERIPH: u16 = 1 << 8;
pub const HW_CTRL_TRIGGER: u16 = 1 << 9;

#[repr(C)]
pub struct gdsc_desc {
    pub dev: *mut device,
    pub scs: *mut *mut gdsc,
    pub num: usize,
    pub pd_list: *mut dev_pm_domain_list,
}

/* CONFIG_QCOM_GDSC conditional declarations. */
#[cfg(feature = "CONFIG_QCOM_GDSC")]
extern "C" {
    pub fn gdsc_register(
        desc: *mut gdsc_desc,
        rcdev: *mut reset_controller_dev,
        r: *mut regmap,
    ) -> core::ffi::c_int;
    pub fn gdsc_unregister(desc: *mut gdsc_desc);
    pub fn gdsc_gx_do_nothing_enable(domain: *mut generic_pm_domain) -> core::ffi::c_int;
    pub fn gdsc_gx_disable(domain: *mut generic_pm_domain) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_QCOM_GDSC"))]
pub unsafe fn gdsc_register(
    _desc: *mut gdsc_desc,
    _rcdev: *mut reset_controller_dev,
    _r: *mut regmap,
) -> core::ffi::c_int {
    -38
}

#[cfg(not(feature = "CONFIG_QCOM_GDSC"))]
pub unsafe fn gdsc_unregister(_desc: *mut gdsc_desc) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
