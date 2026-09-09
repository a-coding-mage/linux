// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// C dependencies supplied by the surrounding kernel build are intentionally
// referenced here as external Rust items.

use core::ffi::{c_char, c_int, c_void};

pub const ADF_DEVICE_NAME_LENGTH: usize = 64;
pub const ADF_DEVICE_NAME_PREFIX: &[u8] = b"qat_";

#[repr(C)]
pub struct AdfDevClass {
    pub name: *const c_char,
}

#[repr(C)]
pub struct AdfHwDevice {
    pub dev_class: *mut AdfDevClass,
}

#[repr(C)]
pub struct PciDev;

#[repr(C)]
pub struct AccelPciDev {
    pub pci_dev: *mut PciDev,
}

#[repr(C)]
pub struct AdfAccelDev {
    pub hw_device: *mut AdfHwDevice,
    pub accel_pci_dev: AccelPciDev,
    pub debugfs_dir: *mut Dentry,
    pub is_vf: bool,
}

#[repr(C)]
pub struct Dentry;

unsafe extern "C" {
    pub fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    pub fn pci_name(dev: *mut PciDev) -> *const c_char;
    pub fn debugfs_create_dir(name: *const c_char, parent: *mut Dentry) -> *mut Dentry;
    pub fn debugfs_remove(dentry: *mut Dentry);

    pub fn adf_cfg_dev_dbgfs_add(accel_dev: *mut AdfAccelDev);
    pub fn adf_cfg_dev_dbgfs_rm(accel_dev: *mut AdfAccelDev);
    pub fn adf_fw_counters_dbgfs_add(accel_dev: *mut AdfAccelDev);
    pub fn adf_fw_counters_dbgfs_rm(accel_dev: *mut AdfAccelDev);
    pub fn adf_heartbeat_dbgfs_add(accel_dev: *mut AdfAccelDev);
    pub fn adf_heartbeat_dbgfs_rm(accel_dev: *mut AdfAccelDev);
    pub fn adf_pm_dbgfs_add(accel_dev: *mut AdfAccelDev);
    pub fn adf_pm_dbgfs_rm(accel_dev: *mut AdfAccelDev);
    pub fn adf_cnv_dbgfs_add(accel_dev: *mut AdfAccelDev);
    pub fn adf_cnv_dbgfs_rm(accel_dev: *mut AdfAccelDev);
    pub fn adf_tl_dbgfs_add(accel_dev: *mut AdfAccelDev);
    pub fn adf_tl_dbgfs_rm(accel_dev: *mut AdfAccelDev);
}

/// adf_dbgfs_init() - add persistent debugfs entries
/// @accel_dev: Pointer to acceleration device.
///
/// This function creates debugfs entries that are persistent through a device
/// state change (from up to down or vice versa).
#[no_mangle]
pub unsafe extern "C" fn adf_dbgfs_init(accel_dev: *mut AdfAccelDev) {
    let mut name = [0 as c_char; ADF_DEVICE_NAME_LENGTH];

    // Create dev top level debugfs entry
    let hw_device = (*accel_dev).hw_device;
    let dev_class = (*hw_device).dev_class;
    let pci_dev = (*accel_dev).accel_pci_dev.pci_dev;
    snprintf(
        name.as_mut_ptr(),
        name.len(),
        b"%s%s_%s\0".as_ptr() as *const c_char,
        ADF_DEVICE_NAME_PREFIX.as_ptr(),
        (*dev_class).name,
        pci_name(pci_dev),
    );

    (*accel_dev).debugfs_dir = debugfs_create_dir(name.as_ptr(), core::ptr::null_mut());

    adf_cfg_dev_dbgfs_add(accel_dev);
}

/// adf_dbgfs_exit() - remove persistent debugfs entries
/// @accel_dev: Pointer to acceleration device.
#[no_mangle]
pub unsafe extern "C" fn adf_dbgfs_exit(accel_dev: *mut AdfAccelDev) {
    adf_cfg_dev_dbgfs_rm(accel_dev);
    debugfs_remove((*accel_dev).debugfs_dir);
}

/// adf_dbgfs_add() - add non-persistent debugfs entries
/// @accel_dev: Pointer to acceleration device.
///
/// This function creates debugfs entries that are not persistent through
/// a device state change (from up to down or vice versa).
#[no_mangle]
pub unsafe extern "C" fn adf_dbgfs_add(accel_dev: *mut AdfAccelDev) {
    if !(*accel_dev).is_vf {
        adf_fw_counters_dbgfs_add(accel_dev);
        adf_heartbeat_dbgfs_add(accel_dev);
        adf_pm_dbgfs_add(accel_dev);
        adf_cnv_dbgfs_add(accel_dev);
        adf_tl_dbgfs_add(accel_dev);
    }
}

/// adf_dbgfs_rm() - remove non-persistent debugfs entries
/// @accel_dev: Pointer to acceleration device.
#[no_mangle]
pub unsafe extern "C" fn adf_dbgfs_rm(accel_dev: *mut AdfAccelDev) {
    if !(*accel_dev).is_vf {
        adf_tl_dbgfs_rm(accel_dev);
        adf_cnv_dbgfs_rm(accel_dev);
        adf_pm_dbgfs_rm(accel_dev);
        adf_heartbeat_dbgfs_rm(accel_dev);
        adf_fw_counters_dbgfs_rm(accel_dev);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
