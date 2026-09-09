// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV SCOM bus debugfs interface
 *
 * Copyright 2010 Benjamin Herrenschmidt, IBM Corp
 *                <benh@kernel.crashing.org>
 *     and        David Gibson, IBM Corporation.
 * Copyright 2013 IBM Corp.
 */

// Linux and architecture headers supplying the declarations used below.

use core::ffi::c_char;

extern "C" {
    fn opal_xscom_read(chip: u32, reg: u64, value: u64) -> i64;
    fn opal_xscom_write(chip: u32, reg: u64, value: u64) -> i64;
    fn __pa(addr: *const core::ffi::c_void) -> u64;
    fn be64_to_cpu(value: u64) -> u64;
    fn put_user(value: u64, ptr: *mut u64) -> i32;
    fn get_user(value: *mut u64, ptr: *const u64) -> i32;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kasprintf(flags: u32, format: *const c_char, ...) -> *mut c_char;
    fn strlen(value: *const c_char) -> usize;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_blob(name: *const c_char, mode: u32, parent: *mut dentry, blob: *mut debugfs_blob_wrapper);
    fn debugfs_create_file(name: *const c_char, mode: u32, parent: *mut dentry, data: *mut scom_debug_entry, fops: *const file_operations);
    fn simple_open(filp: *mut file, inode: *mut inode) -> i32;
    fn default_llseek(filp: *mut file, offset: i64, whence: i32) -> i64;
    fn firmware_has_feature(feature: u64) -> bool;
    fn of_get_ibm_chip_id(dn: *mut device_node) -> i32;
    fn warn_on(condition: bool);
}

const EINVAL: i32 = 22;
const EIO: i32 = 5;
const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0;
const FW_FEATURE_OPAL: u64 = 0;

#[repr(C)]
pub struct dentry { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct file { pub private_data: *mut core::ffi::c_void }

#[repr(C)]
pub struct debugfs_blob_wrapper {
    pub data: *mut core::ffi::c_void,
    pub size: usize,
}

#[repr(C)]
pub struct file_operations {
    pub read: Option<unsafe extern "C" fn(*mut file, *mut u8, usize, *mut i64) -> isize>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const u8, usize, *mut i64) -> isize>,
    pub open: Option<unsafe extern "C" fn(*mut file, *mut inode) -> i32>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, i64, i32) -> i64>,
}

static mut ARCH_DEBUGFS_DIR: *mut dentry = core::ptr::null_mut();

unsafe fn opal_scom_unmangle(mut addr: u64) -> u64 {
    let tmp = addr;
    addr &= 0xf0ffffffffffffff;
    addr |= (tmp & 0x0f00000000000000) << 4;
    addr
}

unsafe fn opal_scom_read(chip: u32, addr: u64, reg: u64, value: *mut u64) -> i32 {
    let mut v: u64 = 0;
    let reg = opal_scom_unmangle(addr.wrapping_add(reg));
    let rc = opal_xscom_read(chip, reg, __pa((&mut v as *mut u64).cast()));
    if rc != 0 {
        *value = 0xffffffffffffffff;
        return -EIO;
    }
    *value = be64_to_cpu(v);
    0
}

unsafe fn opal_scom_write(chip: u32, addr: u64, reg: u64, value: u64) -> i32 {
    let reg = opal_scom_unmangle(addr.wrapping_add(reg));
    if opal_xscom_write(chip, reg, value) != 0 { -EIO } else { 0 }
}

#[repr(C)]
pub struct scom_debug_entry {
    pub chip: u32,
    pub path: debugfs_blob_wrapper,
    pub name: [c_char; 16],
}

unsafe extern "C" fn scom_debug_read(filp: *mut file, ubuf: *mut u8, count: usize, ppos: *mut i64) -> isize {
    let ent = (*filp).private_data as *mut scom_debug_entry;
    let mut ubuf64 = ubuf as *mut u64;
    let off = *ppos;
    let mut done: isize = 0;
    if off < 0 || (off & 7) != 0 || (count & 7) != 0 { return -EINVAL as isize; }
    let reg_base = (off as u64) >> 3;
    let reg_cnt = (count as u64) >> 3;
    let mut reg = 0;
    while reg < reg_cnt {
        let mut val = 0;
        let mut rc = opal_scom_read((*ent).chip, reg_base, reg, &mut val);
        if rc == 0 { rc = put_user(val, ubuf64); }
        if rc != 0 { if done == 0 { done = rc as isize; } break; }
        ubuf64 = ubuf64.add(1); *ppos += 8; done += 8; reg += 1;
    }
    done
}

unsafe extern "C" fn scom_debug_write(filp: *mut file, ubuf: *const u8, count: usize, ppos: *mut i64) -> isize {
    let ent = (*filp).private_data as *mut scom_debug_entry;
    let mut ubuf64 = ubuf as *const u64;
    let off = *ppos;
    let mut done: isize = 0;
    if off < 0 || (off & 7) != 0 || (count & 7) != 0 { return -EINVAL as isize; }
    let reg_base = (off as u64) >> 3;
    let reg_cnt = (count as u64) >> 3;
    let mut reg = 0;
    while reg < reg_cnt {
        let mut val = 0;
        let mut rc = get_user(&mut val, ubuf64);
        if rc == 0 { rc = opal_scom_write((*ent).chip, reg_base, reg, val); }
        if rc != 0 { if done == 0 { done = rc as isize; } break; }
        ubuf64 = ubuf64.add(1); done += 8; reg += 1;
    }
    done
}

static SCOM_DEBUG_FOPS: file_operations = file_operations {
    read: Some(scom_debug_read), write: Some(scom_debug_write),
    open: Some(simple_open), llseek: Some(default_llseek),
};

unsafe fn scom_debug_init_one(root: *mut dentry, dn: *mut device_node, chip: i32) -> i32 {
    let ent = kzalloc_obj::<scom_debug_entry>();
    if ent.is_null() { return -ENOMEM; }
    (*ent).chip = chip as u32;
    // snprintf(ent->name, 16, "%08x", chip)
    let path = kasprintf(GFP_KERNEL, b"%pOF\0".as_ptr() as *const c_char, dn);
    if path.is_null() { kfree(ent.cast()); return -ENOMEM; }
    (*ent).path.data = path.cast();
    (*ent).path.size = strlen(path);
    let dir = debugfs_create_dir((*ent).name.as_ptr(), root);
    if dir.is_null() { kfree((*ent).path.data); kfree(ent.cast()); return -1; }
    debugfs_create_blob(b"devspec\0".as_ptr() as *const c_char, 0o400, dir, &mut (*ent).path);
    debugfs_create_file(b"access\0".as_ptr() as *const c_char, 0o600, dir, ent, &SCOM_DEBUG_FOPS);
    0
}

// device-tree iteration and device_initcall are supplied by the kernel build environment.
unsafe fn scom_debug_init() -> i32 {
    if !firmware_has_feature(FW_FEATURE_OPAL) { return 0; }
    let root = debugfs_create_dir(b"scom\0".as_ptr() as *const c_char, ARCH_DEBUGFS_DIR);
    if root.is_null() { return -1; }
    let mut rc = 0;
    // for_each_node_with_property(dn, "scom-controller")
    let mut dn: *mut device_node = core::ptr::null_mut();
    while !dn.is_null() {
        let chip = of_get_ibm_chip_id(dn);
        warn_on(chip == -1);
        rc |= scom_debug_init_one(root, dn, chip);
        break;
    }
    rc
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
