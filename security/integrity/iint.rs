// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008 IBM Corporation
 *
 * Authors:
 * Mimi Zohar <zohar@us.ibm.com>
 *
 * File: integrity_iint.c
 *	- initialize the integrity directory in securityfs
 *	- load IMA and EVM keys
 */

use std::os::raw::{c_int, c_void};

// Opaque kernel structures
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

pub type loff_t = i64;

pub static mut integrity_dir: *mut dentry = std::ptr::null_mut();

extern "C" {
    fn __kernel_read(file: *mut file, addr: *mut c_void, count: usize, offset: *mut loff_t) -> c_int;
    fn ima_load_x509();
    fn evm_load_x509();
    fn securityfs_create_dir(name: *const u8, parent: *mut dentry) -> *mut dentry;
    fn securityfs_remove(dentry: *mut dentry);
    fn simple_empty(dentry: *mut dentry) -> c_int;
    fn pr_err(fmt: *const u8, ...);
}

// Linux kernel error code handling: negative error codes encoded in pointer values
const fn is_err(ptr: *const c_void) -> bool {
    (ptr as usize) >= usize::MAX - 4095
}

unsafe fn ptr_err(ptr: *const c_void) -> c_int {
    (ptr as isize) as c_int
}

/*
 * integrity_kernel_read - read data from the file
 *
 * This is a function for reading file content instead of kernel_read().
 * It does not perform locking checks to ensure it cannot be blocked.
 * It does not perform security checks because it is irrelevant for IMA.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn integrity_kernel_read(
    file: *mut file,
    offset: loff_t,
    addr: *mut c_void,
    count: usize,
) -> c_int {
    let mut off = offset;
    __kernel_read(file, addr, count, &mut off)
}

/*
 * integrity_load_keys - load integrity keys hook
 *
 * Hooks is called from init/main.c:kernel_init_freeable()
 * when rootfs is ready
 */
#[no_mangle]
pub unsafe extern "C" fn integrity_load_keys() {
    ima_load_x509();

    // CONFIG_IMA_LOAD_X509 is determined at kernel compile time
    if !cfg!(feature = "ima_load_x509") {
        evm_load_x509();
    }
}

#[no_mangle]
pub unsafe extern "C" fn integrity_fs_init() -> c_int {
    if !integrity_dir.is_null() {
        return 0;
    }

    integrity_dir = securityfs_create_dir(b"integrity\0".as_ptr(), std::ptr::null_mut());
    if is_err(integrity_dir as *const c_void) {
        let ret = ptr_err(integrity_dir as *const c_void);

        // -ENODEV error code
        if ret != -19 {
            pr_err(b"Unable to create integrity sysfs dir: %d\n\0".as_ptr(), ret);
        }
        integrity_dir = std::ptr::null_mut();
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn integrity_fs_fini() {
    if integrity_dir.is_null() || simple_empty(integrity_dir) == 0 {
        return;
    }

    securityfs_remove(integrity_dir);
    integrity_dir = std::ptr::null_mut();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
