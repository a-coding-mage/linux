// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/driver translation unit:
// amdxdna_cbuf.h, amdxdna_debugfs.h, drm/drm_file.h, linux/debugfs.h,
// linux/pm_runtime.h, linux/seq_file.h, linux/string.h, linux/uaccess.h.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct amdxdna_dev {
    pub ddev: amdxdna_drm_device,
    pub dev_lock: mutex,
}

#[repr(C)]
pub struct amdxdna_drm_device {
    pub accel: *mut drm_minor,
}

#[repr(C)]
pub struct drm_minor {
    pub debugfs_root: *mut dentry,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_private: *mut c_void,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct seq_file {
    pub private: *mut amdxdna_dev,
}

pub type size_t = usize;
pub type loff_t = i64;
pub type ssize_t = isize;
pub type umode_t = u16;
pub type u64 = u64;

#[repr(C)]
pub struct file_operations {
    pub owner: *mut c_void,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn()>,
    pub llseek: Option<unsafe extern "C" fn()>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
}

unsafe extern "C" {
    static mut THIS_MODULE: c_void;
    fn seq_read();
    fn seq_lseek();
    fn single_open(file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn single_release(inode: *mut inode, file: *mut file) -> c_int;
    fn copy_from_user(to: *mut c_char, from: *const c_char, n: size_t) -> usize;
    fn strim(s: *mut c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn kstrtou64(s: *const c_char, base: u32, result: *mut u64) -> c_int;
    fn amdxdna_carveout_init(xdna: *mut amdxdna_dev, addr: u64, size: u64) -> c_int;
    fn amdxdna_get_carveout_conf(xdna: *mut amdxdna_dev, addr: *mut u64, size: *mut u64);
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn debugfs_create_file(name: *const c_char, mode: umode_t, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
}

unsafe extern "C" fn amdxdna_carveout_write(file: *mut file, buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    let xdna = file_to_xdna(file);
    let mut kbuf = [0 as c_char; 128];
    let mut size: u64 = 0;
    let mut addr: u64 = 0;
    let mut sep: *mut c_char;
    let mut ret: c_int;

    if count == 0 || count >= kbuf.len() { return -22; }
    if copy_from_user(kbuf.as_mut_ptr(), buf, count) != 0 { return -14; }
    *kbuf.as_mut_ptr().add(count) = 0;
    strim(kbuf.as_mut_ptr());
    // XDNA_DBG(xdna, "Trying to set carveout to %s", kbuf);

    sep = strchr(kbuf.as_ptr(), '@');
    if sep.is_null() { return -22; }
    *sep = 0;
    sep = sep.add(1);
    ret = kstrtou64(kbuf.as_ptr(), 0, &mut size);
    if ret != 0 { return ret as ssize_t; }
    ret = kstrtou64(sep, 0, &mut addr);
    if ret != 0 { return ret as ssize_t; }
    if size == 0 { return -22; }
    if addr % 4096 != 0 || size % 4096 != 0 { return -22; }
    ret = amdxdna_carveout_init(xdna, addr, size);
    if ret != 0 { return ret as ssize_t; }
    count as ssize_t
}

unsafe extern "C" fn amdxdna_carveout_show(m: *mut seq_file, _unused: *mut c_void) -> c_int {
    let xdna = (*m).private;
    let mut addr: u64 = 0;
    let mut size: u64 = 0;
    amdxdna_get_carveout_conf(xdna, &mut addr, &mut size);
    // seq_printf(m, "0x%llx@0x%llx\n", size, addr);
    0
}

unsafe extern "C" fn amdxdna_dbgfs_carveout_open(inode: *mut inode, file: *mut file) -> c_int {
    single_open(file, amdxdna_carveout_show, (*inode).i_private)
}

unsafe extern "C" fn amdxdna_dbgfs_carveout_release(inode: *mut inode, file: *mut file) -> c_int {
    single_release(inode, file)
}

unsafe fn file_to_xdna(file: *mut file) -> *mut amdxdna_dev {
    (*( (*file).private_data as *mut seq_file)).private
}

#[no_mangle]
pub unsafe extern "C" fn amdxdna_debugfs_init(xdna: *mut amdxdna_dev) {
    let minor = (*xdna).ddev.accel;
    debugfs_create_file(b"carveout\0".as_ptr() as *const c_char, 0o600, (*minor).debugfs_root, xdna as *mut c_void, &amdxdna_fops_carveout);
}

static amdxdna_fops_carveout: file_operations = file_operations {
    owner: core::ptr::null_mut(),
    open: Some(amdxdna_dbgfs_carveout_open),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    release: Some(amdxdna_dbgfs_carveout_release),
    write: Some(amdxdna_carveout_write),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
