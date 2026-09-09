// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 James.Bottomley@HansenPartnership.com
 */

// Dependencies supplied by the surrounding kernel translation unit.

use core::ffi::c_int;

#[repr(C)]
pub struct inode {
    pub i_cdev: *mut cdev,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct cdev;
#[repr(C)]
pub struct tpm_chip;

#[repr(C)]
pub struct file_priv {
    pub chip: *mut tpm_chip,
}

#[repr(C)]
pub struct tpm_space;

#[repr(C)]
pub struct tpmrm_priv {
    pub priv_: file_priv,
    pub space: tpm_space,
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut core::ffi::c_void,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn() -> c_int>,
    pub write: Option<unsafe extern "C" fn() -> c_int>,
    pub poll: Option<unsafe extern "C" fn() -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
}

extern "C" {
    static THIS_MODULE: *mut core::ffi::c_void;
    static TPM2_SPACE_BUFFER_SIZE: usize;

    fn kzalloc_obj<T>() -> *mut T;
    fn kfree<T>(ptr: *mut T);
    fn tpm2_init_space(space: *mut tpm_space, size: usize) -> c_int;
    fn tpm_common_open(
        file: *mut file,
        chip: *mut tpm_chip,
        priv_: *mut file_priv,
        space: *mut tpm_space,
    );
    fn nonseekable_open(inode: *mut inode, file: *mut file) -> c_int;
    fn tpm_common_release(file: *mut file, priv_: *mut file_priv);
    fn tpm2_del_space(chip: *mut tpm_chip, space: *mut tpm_space);
    fn tpm_common_read() -> c_int;
    fn tpm_common_write() -> c_int;
    fn tpm_common_poll() -> c_int;
}

const ENOMEM: c_int = 12;

unsafe extern "C" fn tpmrm_open(inode: *mut inode, file: *mut file) -> c_int {
    let chip = (*inode).i_cdev as *mut tpm_chip;
    let priv_ = kzalloc_obj::<tpmrm_priv>();
    if priv_.is_null() {
        return -ENOMEM;
    }

    let rc = tpm2_init_space(&mut (*priv_).space, TPM2_SPACE_BUFFER_SIZE);
    if rc != 0 {
        kfree(priv_);
        return -ENOMEM;
    }

    tpm_common_open(file, chip, &mut (*priv_).priv_, &mut (*priv_).space);

    nonseekable_open(inode, file)
}

unsafe extern "C" fn tpmrm_release(inode: *mut inode, file: *mut file) -> c_int {
    let fpriv = (*file).private_data as *mut file_priv;
    let priv_ = fpriv as *mut tpmrm_priv;

    tpm_common_release(file, fpriv);
    tpm2_del_space((*fpriv).chip, &mut (*priv_).space);
    kfree(priv_);

    0
}

#[no_mangle]
pub static tpmrm_fops: file_operations = file_operations {
    owner: unsafe { THIS_MODULE },
    open: Some(tpmrm_open),
    read: Some(tpm_common_read),
    write: Some(tpm_common_write),
    poll: Some(tpm_common_poll),
    release: Some(tpmrm_release),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
