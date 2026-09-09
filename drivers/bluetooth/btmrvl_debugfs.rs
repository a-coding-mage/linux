// SPDX-License-Identifier: GPL-2.0-only
/*
 * Marvell Bluetooth driver: debugfs related functions
 *
 * Copyright (C) 2009, Marvell International Ltd.
 */

// Dependencies supplied by the Linux kernel and the btmrvl driver.

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct btmrvl_debugfs_data {
    pub config_dir: *mut dentry,
    pub status_dir: *mut dentry,
}

// External kernel/driver types and functions are supplied by other translation units.
pub enum dentry {}
pub enum file {}
pub enum hci_dev {}
pub enum btmrvl_private {}
pub enum file_operations {}

extern "C" {
    fn hci_get_drvdata(hdev: *mut hci_dev) -> *mut btmrvl_private;
    fn kstrtol_from_user(ubuf: *const c_char, count: usize, base: c_uint,
                         result: *mut c_long) -> isize;
    fn btmrvl_prepare_command(priv_: *mut btmrvl_private);
    fn wake_up_interruptible(wait_q: *mut c_void);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn simple_read_from_buffer(userbuf: *mut c_char, count: usize, ppos: *mut i64,
                               buf: *const c_char, len: c_int) -> isize;
    fn simple_open(file: *mut file, inode: *mut c_void) -> c_int;
    fn default_llseek(file: *mut file, offset: i64, whence: c_int) -> i64;
    fn kzalloc_obj<T>() -> *mut T;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_u8(name: *const c_char, mode: u16, parent: *mut dentry, value: *mut u8) -> *mut dentry;
    fn debugfs_create_x16(name: *const c_char, mode: u16, parent: *mut dentry, value: *mut u16) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: u16, parent: *mut dentry,
                           data: *mut btmrvl_private, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove_recursive(dent: *mut dentry);
    fn kfree(ptr: *mut btmrvl_debugfs_data);
}

type c_uint = u32;
type ssize_t = isize;

// The following field-bearing types are provided by btmrvl_drv.h in the complete translation.
#[allow(dead_code)]
unsafe fn btmrvl_hscfgcmd_write(file_: *mut file, ubuf: *const c_char, count: usize,
                                 _ppos: *mut i64) -> ssize_t {
    let priv_ = (*file_).private_data;
    let mut result: c_long = 0;
    let ret = kstrtol_from_user(ubuf, count, 10, &mut result);
    if ret != 0 { return ret; }
    (*priv_).btmrvl_dev.hscfgcmd = result;
    if (*priv_).btmrvl_dev.hscfgcmd != 0 {
        btmrvl_prepare_command(priv_);
        wake_up_interruptible(&mut (*priv_).main_thread.wait_q as *mut _ as *mut c_void);
    }
    count as ssize_t
}

unsafe fn btmrvl_hscfgcmd_read(file_: *mut file, userbuf: *mut c_char, count: usize,
                                ppos: *mut i64) -> ssize_t {
    let priv_ = (*file_).private_data;
    let mut buf = [0i8; 16];
    let ret = snprintf(buf.as_mut_ptr(), 15, "%d\n\0".as_ptr() as *const c_char,
                       (*priv_).btmrvl_dev.hscfgcmd);
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), ret)
}

static btmrvl_hscfgcmd_fops: file_operations = file_operations {};

unsafe fn btmrvl_pscmd_write(file_: *mut file, ubuf: *const c_char, count: usize,
                              _ppos: *mut i64) -> ssize_t {
    let priv_ = (*file_).private_data;
    let mut result: c_long = 0;
    let ret = kstrtol_from_user(ubuf, count, 10, &mut result);
    if ret != 0 { return ret; }
    (*priv_).btmrvl_dev.pscmd = result;
    if (*priv_).btmrvl_dev.pscmd != 0 {
        btmrvl_prepare_command(priv_);
        wake_up_interruptible(&mut (*priv_).main_thread.wait_q as *mut _ as *mut c_void);
    }
    count as ssize_t
}

unsafe fn btmrvl_pscmd_read(file_: *mut file, userbuf: *mut c_char, count: usize,
                             ppos: *mut i64) -> ssize_t {
    let priv_ = (*file_).private_data;
    let mut buf = [0i8; 16];
    let ret = snprintf(buf.as_mut_ptr(), 15, "%d\n\0".as_ptr() as *const c_char,
                       (*priv_).btmrvl_dev.pscmd);
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), ret)
}

static btmrvl_pscmd_fops: file_operations = file_operations {};

unsafe fn btmrvl_hscmd_write(file_: *mut file, ubuf: *const c_char, count: usize,
                              _ppos: *mut i64) -> ssize_t {
    let priv_ = (*file_).private_data;
    let mut result: c_long = 0;
    let ret = kstrtol_from_user(ubuf, count, 10, &mut result);
    if ret != 0 { return ret; }
    (*priv_).btmrvl_dev.hscmd = result;
    if (*priv_).btmrvl_dev.hscmd != 0 {
        btmrvl_prepare_command(priv_);
        wake_up_interruptible(&mut (*priv_).main_thread.wait_q as *mut _ as *mut c_void);
    }
    count as ssize_t
}

unsafe fn btmrvl_hscmd_read(file_: *mut file, userbuf: *mut c_char, count: usize,
                             ppos: *mut i64) -> ssize_t {
    let priv_ = (*file_).private_data;
    let mut buf = [0i8; 16];
    let ret = snprintf(buf.as_mut_ptr(), 15, "%d\n\0".as_ptr() as *const c_char,
                       (*priv_).btmrvl_dev.hscmd);
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), ret)
}

static btmrvl_hscmd_fops: file_operations = file_operations {};

pub unsafe fn btmrvl_debugfs_init(hdev: *mut hci_dev) {
    let priv_ = hci_get_drvdata(hdev);
    if (*hdev).debugfs.is_null() { return; }
    let dbg = kzalloc_obj::<btmrvl_debugfs_data>();
    (*priv_).debugfs_data = dbg;
    if dbg.is_null() { return; }
    (*dbg).config_dir = debugfs_create_dir("config\0".as_ptr() as *const c_char, (*hdev).debugfs);
    debugfs_create_u8("psmode\0".as_ptr() as *const c_char, 0o644, (*dbg).config_dir, &mut (*priv_).btmrvl_dev.psmode);
    debugfs_create_file("pscmd\0".as_ptr() as *const c_char, 0o644, (*dbg).config_dir, priv_, &btmrvl_pscmd_fops);
    debugfs_create_x16("gpiogap\0".as_ptr() as *const c_char, 0o644, (*dbg).config_dir, &mut (*priv_).btmrvl_dev.gpio_gap);
    debugfs_create_u8("hsmode\0".as_ptr() as *const c_char, 0o644, (*dbg).config_dir, &mut (*priv_).btmrvl_dev.hsmode);
    debugfs_create_file("hscmd\0".as_ptr() as *const c_char, 0o644, (*dbg).config_dir, priv_, &btmrvl_hscmd_fops);
    debugfs_create_file("hscfgcmd\0".as_ptr() as *const c_char, 0o644, (*dbg).config_dir, priv_, &btmrvl_hscfgcmd_fops);
    (*dbg).status_dir = debugfs_create_dir("status\0".as_ptr() as *const c_char, (*hdev).debugfs);
    debugfs_create_u8("curpsmode\0".as_ptr() as *const c_char, 0o444, (*dbg).status_dir, &mut (*priv_).adapter.psmode);
    debugfs_create_u8("psstate\0".as_ptr() as *const c_char, 0o444, (*dbg).status_dir, &mut (*priv_).adapter.ps_state);
    debugfs_create_u8("hsstate\0".as_ptr() as *const c_char, 0o444, (*dbg).status_dir, &mut (*priv_).adapter.hs_state);
    debugfs_create_u8("txdnldready\0".as_ptr() as *const c_char, 0o444, (*dbg).status_dir, &mut (*priv_).btmrvl_dev.tx_dnld_rdy);
}

pub unsafe fn btmrvl_debugfs_remove(hdev: *mut hci_dev) {
    let priv_ = hci_get_drvdata(hdev);
    let dbg = (*priv_).debugfs_data;
    if dbg.is_null() { return; }
    debugfs_remove_recursive((*dbg).config_dir);
    debugfs_remove_recursive((*dbg).status_dir);
    kfree(dbg);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
