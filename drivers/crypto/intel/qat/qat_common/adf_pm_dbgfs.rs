// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

// Declarations supplied by the Linux kernel and accelerator-device headers.
pub type ssize_t = isize;
pub type size_t = usize;
pub type loff_t = i64;

#[repr(C)]
pub struct inode {
    pub i_private: *mut c_void,
}

#[repr(C)]
pub struct file {
    pub f_inode: *mut inode,
}

#[repr(C)]
pub struct dentry;

#[repr(C)]
pub struct module;

#[repr(C)]
pub struct adf_accel_dev {
    pub power_management: adf_pm,
    pub debugfs_dir: *mut dentry,
}

#[repr(C)]
pub struct adf_pm {
    pub present: bool,
    pub print_pm_status:
        Option<unsafe extern "C" fn(*mut adf_accel_dev, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub debugfs_pm_status: *mut dentry,
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut module,
    pub read:
        Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
}

unsafe extern "C" {
    static THIS_MODULE: *mut module;

    fn file_inode(f: *mut file) -> *mut inode;
    fn debugfs_create_file(
        name: *const c_char,
        mode: u32,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_remove(dentry: *mut dentry);
}

unsafe extern "C" fn pm_status_read(
    f: *mut file,
    buf: *mut c_char,
    count: size_t,
    pos: *mut loff_t,
) -> ssize_t {
    let accel_dev = (*file_inode(f)).i_private as *mut adf_accel_dev;
    let pm = (*accel_dev).power_management;

    if let Some(print_pm_status) = pm.print_pm_status {
        return print_pm_status(accel_dev, buf, count, pos);
    }

    count as ssize_t
}

static pm_status_fops: file_operations = file_operations {
    owner: unsafe { THIS_MODULE },
    read: Some(pm_status_read),
};

#[no_mangle]
pub unsafe extern "C" fn adf_pm_dbgfs_add(accel_dev: *mut adf_accel_dev) {
    let pm = &mut (*accel_dev).power_management;

    if !pm.present || pm.print_pm_status.is_none() {
        return;
    }

    static PM_STATUS: &[u8] = b"pm_status\0";
    pm.debugfs_pm_status = debugfs_create_file(
        PM_STATUS.as_ptr() as *const c_char,
        0o400,
        (*accel_dev).debugfs_dir,
        accel_dev as *mut c_void,
        &pm_status_fops,
    );
}

#[no_mangle]
pub unsafe extern "C" fn adf_pm_dbgfs_rm(accel_dev: *mut adf_accel_dev) {
    let pm = &mut (*accel_dev).power_management;

    if !pm.present {
        return;
    }

    debugfs_remove(pm.debugfs_pm_status);
    pm.debugfs_pm_status = ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
