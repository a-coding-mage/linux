// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/fs/ceph/acl.c
 *
 * Copyright (C) 2013 Guangliang Zhao, <lucienchao@gmail.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }
#[repr(C)]
pub struct posix_acl { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_client { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_inode_info { i_ceph_lock: c_void }
#[repr(C)]
pub struct ceph_acl_sec_ctx {
    pub acl: *mut posix_acl,
    pub default_acl: *mut posix_acl,
    pub pagelist: *mut ceph_pagelist,
}
#[repr(C)]
pub struct ceph_pagelist { _private: [u8; 0] }
#[repr(C)]
pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)]
pub struct iattr {
    pub ia_ctime: timespec64,
    pub ia_mode: umode_t,
    pub ia_valid: c_uint,
}
#[repr(C)]
pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i32 }

pub type umode_t = u16;
pub type size_t = usize;

extern "C" {
    static init_user_ns: c_void;
    fn ceph_inode(inode: *mut inode) -> *mut ceph_inode_info;
    fn ceph_inode_to_client(inode: *mut inode) -> *mut ceph_client;
    fn __ceph_caps_issued_mask_metric(ci: *mut ceph_inode_info, mask: u64, unused: c_int) -> c_int;
    fn set_cached_acl(inode: *mut inode, ty: c_int, acl: *mut posix_acl);
    fn forget_cached_acl(inode: *mut inode, ty: c_int);
    fn __ceph_getxattr(inode: *mut inode, name: *const c_char, value: *mut c_char, size: c_int) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn posix_acl_from_xattr(ns: *const c_void, value: *mut c_char, size: c_int) -> *mut posix_acl;
    fn posix_acl_update_mode(idmap: *mut mnt_idmap, inode: *mut inode, mode: *mut umode_t, acl: *mut *mut posix_acl) -> c_int;
    fn posix_acl_to_xattr(ns: *const c_void, acl: *mut posix_acl, size: *mut usize, flags: c_uint) -> *mut c_char;
    fn d_inode(dentry: *mut dentry) -> *mut inode;
    fn inode_get_ctime(inode: *mut inode) -> timespec64;
    fn ceph_snap(inode: *mut inode) -> u64;
    fn current_time(inode: *mut inode) -> timespec64;
    fn __ceph_setattr(idmap: *mut mnt_idmap, inode: *mut inode, attrs: *mut iattr, unused: *mut c_void) -> c_int;
    fn __ceph_setxattr(inode: *mut inode, name: *const c_char, value: *mut c_char, size: usize, flags: c_int) -> c_int;
    fn posix_acl_create(dir: *mut inode, mode: *mut umode_t, default_acl: *mut *mut posix_acl, acl: *mut *mut posix_acl) -> c_int;
    fn posix_acl_equiv_mode(acl: *mut posix_acl, mode: *mut umode_t) -> c_int;
    fn posix_acl_release(acl: *mut posix_acl);
    fn ceph_pagelist_alloc(flags: c_uint) -> *mut ceph_pagelist;
    fn ceph_pagelist_reserve(pagelist: *mut ceph_pagelist, size: usize) -> c_int;
    fn ceph_pagelist_encode_32(pagelist: *mut ceph_pagelist, value: u32);
    fn ceph_pagelist_encode_string(pagelist: *mut ceph_pagelist, value: *const c_char, len: usize);
    fn ceph_pagelist_append(pagelist: *mut ceph_pagelist, value: *mut c_void, len: usize);
    fn ceph_pagelist_release(pagelist: *mut ceph_pagelist);
}

const ACL_TYPE_ACCESS: c_int = 0;
const ACL_TYPE_DEFAULT: c_int = 1;
const ECHILD: c_int = 10;
const ENOMEM: c_int = 12;
const EROFS: c_int = 30;
const EINVAL: c_int = 22;
const ERANGE: c_int = 34;
const ENODATA: c_int = 61;
const EIO: c_int = 5;
const CEPH_CAP_XATTR_SHARED: u64 = 1;
const CEPH_NOSNAP: u64 = u64::MAX;
const GFP_NOFS: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const ATTR_MODE: c_uint = 1;
const ATTR_CTIME: c_uint = 2;
const PAGE_SIZE: usize = 4096;

static XATTR_NAME_POSIX_ACL_ACCESS: &[u8] = b"system.posix_acl_access\0";
static XATTR_NAME_POSIX_ACL_DEFAULT: &[u8] = b"system.posix_acl_default\0";

#[inline]
unsafe fn ceph_set_cached_acl(inode: *mut inode, ty: c_int, acl: *mut posix_acl) {
    let ci = ceph_inode(inode);
    spin_lock(&mut (*ci).i_ceph_lock);
    if __ceph_caps_issued_mask_metric(ci, CEPH_CAP_XATTR_SHARED, 0) != 0 {
        set_cached_acl(inode, ty, acl);
    } else {
        forget_cached_acl(inode, ty);
    }
    spin_unlock(&mut (*ci).i_ceph_lock);
}

unsafe fn spin_lock(_lock: *mut c_void) {}
unsafe fn spin_unlock(_lock: *mut c_void) {}

pub unsafe fn ceph_get_acl(inode: *mut inode, ty: c_int, rcu: bool) -> *mut posix_acl {
    let cl = ceph_inode_to_client(inode);
    if rcu { return (-ECHILD as isize) as *mut posix_acl; }
    let name = match ty {
        ACL_TYPE_ACCESS => XATTR_NAME_POSIX_ACL_ACCESS.as_ptr() as *const c_char,
        ACL_TYPE_DEFAULT => XATTR_NAME_POSIX_ACL_DEFAULT.as_ptr() as *const c_char,
        _ => core::hint::unreachable_unchecked(),
    };
    let mut retry_cnt = 0;
    let mut value: *mut c_char = core::ptr::null_mut();
    let mut size = __ceph_getxattr(inode, name, core::ptr::null_mut(), 0);
    if size > 0 {
        value = kzalloc(size as usize, GFP_NOFS);
        if value.is_null() { return (-ENOMEM as isize) as *mut posix_acl; }
        size = __ceph_getxattr(inode, name, value, size);
    }
    while size == -ERANGE && retry_cnt < 10 {
        retry_cnt += 1; kfree(value as *mut c_void); value = core::ptr::null_mut();
        size = __ceph_getxattr(inode, name, core::ptr::null_mut(), 0);
        if size > 0 { value = kzalloc(size as usize, GFP_NOFS); if value.is_null() { return (-ENOMEM as isize) as *mut posix_acl; } size = __ceph_getxattr(inode, name, value, size); }
    }
    let acl = if size > 0 { posix_acl_from_xattr(&init_user_ns, value, size) } else if size == -ENODATA || size == 0 { core::ptr::null_mut() } else { let _ = cl; (-EIO as isize) as *mut posix_acl };
    kfree(value as *mut c_void);
    if (acl as isize) >= 0 { ceph_set_cached_acl(inode, ty, acl); }
    acl
}

pub unsafe fn ceph_set_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, ty: c_int) -> c_int {
    let inode = d_inode(dentry); let mut ret = 0; let mut size = 0usize; let mut name = core::ptr::null(); let mut value = core::ptr::null_mut();
    let old_ctime = inode_get_ctime(inode); let old_mode = (*inode_as_real(inode)).mode(); let mut new_mode = old_mode;
    if ceph_snap(inode) != CEPH_NOSNAP { return -EROFS; }
    match ty { ACL_TYPE_ACCESS => { name = XATTR_NAME_POSIX_ACL_ACCESS.as_ptr() as *const c_char; if !acl.is_null() { ret = posix_acl_update_mode(idmap, inode, &mut new_mode, &mut (acl as *mut posix_acl)); if ret != 0 { return ret; } } }, ACL_TYPE_DEFAULT => { if !S_ISDIR(old_mode) { return if !acl.is_null() { -EINVAL } else { 0 }; } name = XATTR_NAME_POSIX_ACL_DEFAULT.as_ptr() as *const c_char; }, _ => return -EINVAL }
    if !acl.is_null() { value = posix_acl_to_xattr(&init_user_ns, acl, &mut size, GFP_NOFS); if value.is_null() { return -ENOMEM; } }
    if new_mode != old_mode { let mut attrs = iattr { ia_ctime: current_time(inode), ia_mode: new_mode, ia_valid: ATTR_MODE | ATTR_CTIME }; ret = __ceph_setattr(idmap, inode, &mut attrs, core::ptr::null_mut()); if ret != 0 { kfree(value as *mut c_void); return ret; } }
    ret = __ceph_setxattr(inode, name, value, size, 0); if ret != 0 { kfree(value as *mut c_void); return ret; }
    ceph_set_cached_acl(inode, ty, acl); kfree(value as *mut c_void); ret
}

pub unsafe fn ceph_pre_init_acls(dir: *mut inode, mode: *mut umode_t, as_ctx: *mut ceph_acl_sec_ctx) -> c_int {
    let mut acl = core::ptr::null_mut(); let mut default_acl = core::ptr::null_mut();
    let mut err = posix_acl_create(dir, mode, &mut default_acl, &mut acl); if err != 0 { return err; }
    if !acl.is_null() { err = posix_acl_equiv_mode(acl, mode); if err < 0 { posix_acl_release(acl); posix_acl_release(default_acl); return err; } if err == 0 { posix_acl_release(acl); acl = core::ptr::null_mut(); } }
    if acl.is_null() && default_acl.is_null() { return 0; }
    let pagelist = ceph_pagelist_alloc(GFP_KERNEL); if pagelist.is_null() { posix_acl_release(acl); posix_acl_release(default_acl); return -ENOMEM; }
    err = ceph_pagelist_reserve(pagelist, PAGE_SIZE); if err != 0 { posix_acl_release(acl); posix_acl_release(default_acl); ceph_pagelist_release(pagelist); return err; }
    ceph_pagelist_encode_32(pagelist, if !acl.is_null() && !default_acl.is_null() { 2 } else { 1 });
    if !acl.is_null() { let mut n = 0; let p = posix_acl_to_xattr(&init_user_ns, acl, &mut n, GFP_KERNEL); if p.is_null() { return -ENOMEM; } let s = XATTR_NAME_POSIX_ACL_ACCESS.len()-1; err = ceph_pagelist_reserve(pagelist, s+n+8); if err == 0 { ceph_pagelist_encode_string(pagelist, XATTR_NAME_POSIX_ACL_ACCESS.as_ptr() as *const c_char, s); ceph_pagelist_encode_32(pagelist,n as u32); ceph_pagelist_append(pagelist,p as *mut c_void,n); } kfree(p as *mut c_void); if err != 0 { return err; } }
    if !default_acl.is_null() { let mut n = 0; let p = posix_acl_to_xattr(&init_user_ns, default_acl, &mut n, GFP_KERNEL); if p.is_null() { return -ENOMEM; } let s = XATTR_NAME_POSIX_ACL_DEFAULT.len()-1; err = ceph_pagelist_reserve(pagelist, s+n+8); if err == 0 { ceph_pagelist_encode_string(pagelist, XATTR_NAME_POSIX_ACL_DEFAULT.as_ptr() as *const c_char, s); ceph_pagelist_encode_32(pagelist,n as u32); ceph_pagelist_append(pagelist,p as *mut c_void,n); } kfree(p as *mut c_void); if err != 0 { return err; } }
    (*as_ctx).acl = acl; (*as_ctx).default_acl = default_acl; (*as_ctx).pagelist = pagelist; 0
}

pub unsafe fn ceph_init_inode_acls(inode: *mut inode, as_ctx: *mut ceph_acl_sec_ctx) { if inode.is_null() { return; } ceph_set_cached_acl(inode, ACL_TYPE_ACCESS, (*as_ctx).acl); ceph_set_cached_acl(inode, ACL_TYPE_DEFAULT, (*as_ctx).default_acl); }

#[repr(C)] struct inode_real { mode: umode_t }
unsafe fn inode_as_real(i: *mut inode) -> *mut inode_real { i as *mut inode_real }
impl inode_real { unsafe fn mode(&self) -> umode_t { self.mode } }
unsafe fn S_ISDIR(mode: umode_t) -> bool { mode & 0o170000 == 0o040000 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
