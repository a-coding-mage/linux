// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004 IBM Corporation
 * Authors:
 * Leendert van Doorn <leendert@watson.ibm.com>
 * Dave Safford <safford@watson.ibm.com>
 * Reiner Sailer <sailer@watson.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 *
 * Copyright (C) 2013 Obsidian Research Corp
 * Jason Gunthorpe <jgunthorpe@obsidianresearch.com>
 *
 * Device file system interface to the TPM
 */

// Dependency declarations and macros are supplied by the surrounding kernel
// translation unit.

extern "C" {
    fn test_and_set_bit(nr: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_ulong) -> bool;
    fn clear_bit(nr: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_ulong);
    fn dev_dbg(dev: *const device, fmt: *const ::core::ffi::c_char, ...);
    fn tpm_common_open(
        file: *mut file,
        chip: *mut tpm_chip,
        priv_data: *mut file_priv,
        arg: *mut ::core::ffi::c_void,
    );
    fn nonseekable_open(inode: *mut inode, file: *mut file) -> ::core::ffi::c_int;
    fn tpm_common_release(file: *mut file, priv_data: *mut file_priv);
    fn kfree(ptr: *mut ::core::ffi::c_void);
    fn tpm_common_read(
        file: *mut file,
        buf: *mut ::core::ffi::c_char,
        count: usize,
        offset: *mut loff_t,
    ) -> isize;
    fn tpm_common_write(
        file: *mut file,
        buf: *const ::core::ffi::c_char,
        count: usize,
        offset: *mut loff_t,
    ) -> isize;
    fn tpm_common_poll(file: *mut file, wait: *mut poll_table_struct) -> __poll_t;
}

unsafe fn tpm_open(inode: *mut inode, file: *mut file) -> ::core::ffi::c_int {
    let chip: *mut tpm_chip = container_of!((*inode).i_cdev, tpm_chip, cdev);
    let mut priv_data: *mut file_priv;

    /* It's assured that the chip will be opened just once,
     * by the check of is_open variable, which is protected
     * by driver_lock. */
    if test_and_set_bit(0, &mut (*chip).is_open) {
        dev_dbg(&(*chip).dev, "Another process owns this TPM\0".as_ptr() as *const _);
        return -EBUSY;
    }

    priv_data = kzalloc_obj!(file_priv);
    if priv_data.is_null() {
        clear_bit(0, &mut (*chip).is_open);
        return -ENOMEM;
    }

    tpm_common_open(file, chip, priv_data, core::ptr::null_mut());

    nonseekable_open(inode, file)
}

/*
 * Called on file close
 */
unsafe fn tpm_release(_inode: *mut inode, file: *mut file) -> ::core::ffi::c_int {
    let priv_data: *mut file_priv = (*file).private_data as *mut file_priv;

    tpm_common_release(file, priv_data);
    clear_bit(0, &mut (*(*priv_data).chip).is_open);
    kfree(priv_data as *mut ::core::ffi::c_void);

    0
}

pub static tpm_fops: file_operations = file_operations {
    .owner = THIS_MODULE,
    .open = Some(tpm_open),
    .read = Some(tpm_common_read),
    .write = Some(tpm_common_write),
    .poll = Some(tpm_common_poll),
    .release = Some(tpm_release),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
