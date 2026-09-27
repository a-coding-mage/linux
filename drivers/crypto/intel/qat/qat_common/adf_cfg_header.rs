/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependencies supplied by the surrounding translation unit:
// linux/list.h, linux/rwsem.h, linux/debugfs.h, adf_accel_devices.h,
// adf_cfg_common.h, and adf_cfg_strings.h.

#[repr(C)]
pub struct adf_cfg_key_val {
    pub key: [::kernel::ffi::c_char; ADF_CFG_MAX_KEY_LEN_IN_BYTES],
    pub val: [::kernel::ffi::c_char; ADF_CFG_MAX_VAL_LEN_IN_BYTES],
    pub type_: adf_cfg_val_type,
    pub list: list_head,
}

#[repr(C)]
pub struct adf_cfg_section {
    pub name: [::kernel::ffi::c_char; ADF_CFG_MAX_SECTION_LEN_IN_BYTES],
    pub list: list_head,
    pub param_head: list_head,
}

#[repr(C)]
pub struct adf_cfg_device_data {
    pub sec_list: list_head,
    pub debug: *mut dentry,
    pub lock: rw_semaphore,
}

extern "C" {
    pub fn adf_cfg_dev_add(accel_dev: *mut adf_accel_dev) -> ::kernel::ffi::c_int;
    pub fn adf_cfg_dev_remove(accel_dev: *mut adf_accel_dev);
    pub fn adf_cfg_dev_dbgfs_add(accel_dev: *mut adf_accel_dev);
    pub fn adf_cfg_dev_dbgfs_rm(accel_dev: *mut adf_accel_dev);
    pub fn adf_cfg_section_add(
        accel_dev: *mut adf_accel_dev,
        name: *const ::kernel::ffi::c_char,
    ) -> ::kernel::ffi::c_int;
    pub fn adf_cfg_del_all_except(
        accel_dev: *mut adf_accel_dev,
        section_name: *const ::kernel::ffi::c_char,
    );
    pub fn adf_cfg_add_key_value_param(
        accel_dev: *mut adf_accel_dev,
        section_name: *const ::kernel::ffi::c_char,
        key: *const ::kernel::ffi::c_char,
        val: *const ::kernel::ffi::c_void,
        type_: adf_cfg_val_type,
    ) -> ::kernel::ffi::c_int;
    pub fn adf_cfg_get_param_value(
        accel_dev: *mut adf_accel_dev,
        section: *const ::kernel::ffi::c_char,
        name: *const ::kernel::ffi::c_char,
        value: *mut ::kernel::ffi::c_char,
    ) -> ::kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
