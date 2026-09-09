// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2005, 2012 IBM Corporation
 *
 * Authors:
 *	Kent Yoder <key@linux.vnet.ibm.com>
 *	Seiji Munetoh <munetoh@jp.ibm.com>
 *	Stefan Berger <stefanb@us.ibm.com>
 *	Reiner Sailer <sailer@watson.ibm.com>
 *	Kylene Hall <kjhall@us.ibm.com>
 *	Nayna Jain <nayna@linux.vnet.ibm.com>
 *
 * Access to the event log created by a system's firmware / BIOS
 */

// Dependencies supplied by the surrounding kernel TPM implementation.

unsafe extern "C" {
    fn tpm_read_log_acpi(chip: *mut tpm_chip) -> i32;
    fn tpm_read_log_efi(chip: *mut tpm_chip) -> i32;
    fn tpm_read_log_of(chip: *mut tpm_chip) -> i32;
    fn inode_lock(inode: *mut inode);
    fn inode_unlock(inode: *mut inode);
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn seq_open(file: *mut file, ops: *const seq_operations) -> i32;
    fn seq_release(inode: *mut inode, file: *mut file) -> i32;
    fn seq_read(file: *mut file, p: *mut u8, count: usize, pos: *mut i64) -> isize;
    fn seq_lseek(file: *mut file, offset: i64, whence: i32) -> i64;
    fn dev_name(dev: *const device) -> *const core::ffi::c_char;
    fn securityfs_create_dir(name: *const core::ffi::c_char, parent: *mut dentry) -> *mut dentry;
    fn securityfs_create_file(
        name: *const core::ffi::c_char,
        mode: u32,
        parent: *mut dentry,
        data: *mut core::ffi::c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn securityfs_remove(dentry: *mut dentry);
}

#[repr(C)]
pub struct inode {
    pub i_nlink: u32,
    pub i_private: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct dentry;

#[repr(C)]
pub struct seq_file {
    pub private: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct seq_operations;

#[repr(C)]
pub struct file_operations {
    pub owner: *mut core::ffi::c_void,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut u8, usize, *mut i64) -> isize>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, i64, i32) -> i64>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
}

#[repr(C)]
pub struct tpm_chip_seqops {
    pub seqops: *const seq_operations,
    pub chip: *mut tpm_chip,
}

#[repr(C)]
pub struct tpm_chip {
    pub dev: device,
    pub log: tpm_chip_log,
    pub flags: u32,
    pub bios_dir: *mut dentry,
    pub bin_log_seqops: tpm_chip_seqops,
    pub ascii_log_seqops: tpm_chip_seqops,
}

#[repr(C)]
pub struct tpm_chip_log {
    pub bios_event_log: *mut core::ffi::c_void,
}

unsafe extern "C" {
    static mut tpm2_binary_b_measurements_seqops: seq_operations;
    static mut tpm1_binary_b_measurements_seqops: seq_operations;
    static mut tpm1_ascii_b_measurements_seqops: seq_operations;
    static mut THIS_MODULE: core::ffi::c_void;
}

const ENODEV: i32 = 19;
const EFAULT: i32 = 14;
const TPM_CHIP_FLAG_VIRTUAL: u32 = 1 << 0;
const TPM_CHIP_FLAG_TPM2: u32 = 1 << 1;
const EFI_TCG2_EVENT_LOG_FORMAT_TCG_2: i32 = 2;

unsafe extern "C" fn tpm_bios_measurements_open(inode: *mut inode, file: *mut file) -> i32 {
    inode_lock(inode);
    if (*inode).i_nlink == 0 {
        inode_unlock(inode);
        return -ENODEV;
    }
    let chip_seqops = (*inode).i_private as *mut tpm_chip_seqops;
    let seqops = (*chip_seqops).seqops;
    let chip = (*chip_seqops).chip;
    get_device(&mut (*chip).dev);
    inode_unlock(inode);

    let err = seq_open(file, seqops);
    if err == 0 {
        let seq = (*file).private_data as *mut seq_file;
        (*seq).private = chip as *mut core::ffi::c_void;
    } else {
        put_device(&mut (*chip).dev);
    }
    err
}

unsafe extern "C" fn tpm_bios_measurements_release(inode: *mut inode, file: *mut file) -> i32 {
    let seq = (*file).private_data as *mut seq_file;
    let chip = (*seq).private as *mut tpm_chip;
    put_device(&mut (*chip).dev);
    seq_release(inode, file)
}

static mut TPM_BIOS_MEASUREMENTS_OPS: file_operations = file_operations {
    owner: unsafe { &mut THIS_MODULE },
    open: Some(tpm_bios_measurements_open),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    release: Some(tpm_bios_measurements_release),
};

unsafe fn tpm_read_log(chip: *mut tpm_chip) -> i32 {
    if !(*chip).log.bios_event_log.is_null() {
        return -EFAULT;
    }
    let mut rc = tpm_read_log_acpi(chip);
    if rc != -ENODEV { return rc; }
    rc = tpm_read_log_efi(chip);
    if rc != -ENODEV { return rc; }
    tpm_read_log_of(chip)
}

pub unsafe fn tpm_bios_log_setup(chip: *mut tpm_chip) {
    if (*chip).flags & TPM_CHIP_FLAG_VIRTUAL != 0 { return; }
    let log_version = tpm_read_log(chip);
    if log_version < 0 { return; }
    (*chip).bios_dir = securityfs_create_dir(dev_name(&(*chip).dev), core::ptr::null_mut());
    if (*chip).bios_dir.is_null() { return; }
    (*chip).bin_log_seqops.chip = chip;
    (*chip).bin_log_seqops.seqops = if log_version == EFI_TCG2_EVENT_LOG_FORMAT_TCG_2 {
        &tpm2_binary_b_measurements_seqops
    } else {
        &tpm1_binary_b_measurements_seqops
    };
    let dentry = securityfs_create_file(
        b"binary_bios_measurements\0".as_ptr() as *const _,
        0o440,
        (*chip).bios_dir,
        &mut (*chip).bin_log_seqops as *mut _ as *mut core::ffi::c_void,
        &TPM_BIOS_MEASUREMENTS_OPS,
    );
    if dentry.is_null() { tpm_bios_log_teardown(chip); return; }
    if (*chip).flags & TPM_CHIP_FLAG_TPM2 == 0 {
        (*chip).ascii_log_seqops.chip = chip;
        (*chip).ascii_log_seqops.seqops = &tpm1_ascii_b_measurements_seqops;
        let dentry = securityfs_create_file(
            b"ascii_bios_measurements\0".as_ptr() as *const _,
            0o440,
            (*chip).bios_dir,
            &mut (*chip).ascii_log_seqops as *mut _ as *mut core::ffi::c_void,
            &TPM_BIOS_MEASUREMENTS_OPS,
        );
        if dentry.is_null() { tpm_bios_log_teardown(chip); }
    }
}

pub unsafe fn tpm_bios_log_teardown(chip: *mut tpm_chip) {
    securityfs_remove((*chip).bios_dir);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
