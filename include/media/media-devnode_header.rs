/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Media device node
 *
 * Copyright (C) 2010 Nokia Corporation
 *
 * Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 *           Sakari Ailus <sakari.ailus@iki.fi>
 *
 * Common functions for media-related drivers to register and unregister media
 * device nodes.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_uint, c_void};

/* Dependencies supplied by the kernel headers. */
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct poll_table_struct { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct cdev { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct media_device { _private: [u8; 0] }

pub type ssize_t = isize;
pub type size_t = usize;
pub type loff_t = i64;
pub type __poll_t = u32;

/* debugfs top-level media directory */
extern "C" {
    pub static mut media_debugfs_root: *mut dentry;
}

/* Flag to mark the media_devnode struct as registered. */
pub const MEDIA_FLAG_REGISTERED: usize = 0;

#[repr(C)]
pub struct media_file_operations {
    pub owner: *mut module,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table_struct) -> __poll_t>,
    pub ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub open: Option<unsafe extern "C" fn(*mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut file) -> c_int>,
}

#[repr(C)]
pub struct media_devnode {
    pub media_dev: *mut media_device,
    pub fops: *const media_file_operations,
    pub dev: device,
    pub cdev: cdev,
    pub parent: *mut device,
    pub minor: c_int,
    pub flags: c_ulong,
    pub release: Option<unsafe extern "C" fn(*mut media_devnode)>,
}

extern "C" {
    pub fn media_devnode_register(mdev: *mut media_device, devnode: *mut media_devnode,
                                  owner: *mut module) -> c_int;
    pub fn media_devnode_unregister_prepare(devnode: *mut media_devnode);
    pub fn media_devnode_unregister(devnode: *mut media_devnode);
}

#[inline]
pub unsafe fn media_devnode_data(filp: *mut file) -> *mut media_devnode {
    (*filp).private_data as *mut media_devnode
}

extern "C" {
    fn test_bit(nr: usize, addr: *const c_ulong) -> bool;
}

#[inline]
pub unsafe fn media_devnode_is_registered(devnode: *mut media_devnode) -> bool {
    if devnode.is_null() {
        return false;
    }
    test_bit(MEDIA_FLAG_REGISTERED, &(*devnode).flags)
}

/* The C container_of(cd, struct media_devnode, dev) operation. */
#[macro_export]
macro_rules! to_media_devnode {
    ($cd:expr) => {
        unsafe {
            ($cd as *mut u8).sub(core::mem::offset_of!($crate::media_devnode, dev))
                as *mut $crate::media_devnode
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
