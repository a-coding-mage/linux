/* SPDX-License-Identifier: GPL-2.0 */
/*
 * File: linux/posix_acl_xattr.h
 *
 * Extended attribute system call representation of Access Control Lists.
 *
 * Copyright (C) 2000 by Andreas Gruenbacher <a.gruenbacher@computer.org>
 * Copyright (C) 2002 SGI - Silicon Graphics, Inc <linux-xfs@oss.sgi.com>
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;

/* Supplied by the corresponding UAPI and kernel headers. */
extern "C" {
    pub fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
}

/* The following types and constants are supplied by the included headers. */
#[repr(C)]
pub struct posix_acl_xattr_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct posix_acl_xattr_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct posix_acl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xattr_handler {
    _private: [u8; 0],
}

pub type gfp_t = usize;

pub const ACL_TYPE_ACCESS: c_int = 0x8000;
pub const ACL_TYPE_DEFAULT: c_int = 0x4000;

/* XATTR_NAME_POSIX_ACL_ACCESS and XATTR_NAME_POSIX_ACL_DEFAULT are header macros. */
pub const XATTR_NAME_POSIX_ACL_ACCESS: &[u8] = b"system.posix_acl_access\0";
pub const XATTR_NAME_POSIX_ACL_DEFAULT: &[u8] = b"system.posix_acl_default\0";

#[inline]
pub const fn posix_acl_xattr_size(count: c_int) -> usize {
    size_of::<posix_acl_xattr_header>()
        + (count as usize) * size_of::<posix_acl_xattr_entry>()
}

#[inline]
pub fn posix_acl_xattr_count(mut size: usize) -> c_int {
    if size < size_of::<posix_acl_xattr_header>() {
        return -1;
    }
    size -= size_of::<posix_acl_xattr_header>();
    if size % size_of::<posix_acl_xattr_entry>() != 0 {
        return -1;
    }
    (size / size_of::<posix_acl_xattr_entry>()) as c_int
}

#[cfg(CONFIG_FS_POSIX_ACL)]
extern "C" {
    pub fn posix_acl_from_xattr(
        user_ns: *mut user_namespace,
        value: *const c_void,
        size: usize,
    ) -> *mut posix_acl;
}

#[cfg(not(CONFIG_FS_POSIX_ACL))]
#[inline]
pub unsafe fn posix_acl_from_xattr(
    _user_ns: *mut user_namespace,
    _value: *const c_void,
    _size: usize,
) -> *mut posix_acl {
    /* C: return ERR_PTR(-EOPNOTSUPP); supplied by the kernel error-pointer API. */
    core::ptr::without_provenance_mut((-95isize) as usize)
}

extern "C" {
    pub fn posix_acl_to_xattr(
        user_ns: *mut user_namespace,
        acl: *const posix_acl,
        sizep: *mut usize,
        gfp: gfp_t,
    ) -> *mut c_void;
}

#[inline]
pub fn posix_acl_xattr_name(type_: c_int) -> *const c_char {
    match type_ {
        ACL_TYPE_ACCESS => XATTR_NAME_POSIX_ACL_ACCESS.as_ptr() as *const c_char,
        ACL_TYPE_DEFAULT => XATTR_NAME_POSIX_ACL_DEFAULT.as_ptr() as *const c_char,
        _ => b"\0".as_ptr() as *const c_char,
    }
}

#[inline]
pub unsafe fn posix_acl_type(name: *const c_char) -> c_int {
    if strcmp(name, XATTR_NAME_POSIX_ACL_ACCESS.as_ptr() as *const c_char) == 0 {
        ACL_TYPE_ACCESS
    } else if strcmp(name, XATTR_NAME_POSIX_ACL_DEFAULT.as_ptr() as *const c_char) == 0 {
        ACL_TYPE_DEFAULT
    } else {
        -1
    }
}

/* These are legacy handlers. Don't use them for new code. */
extern "C" {
    pub static nop_posix_acl_access: xattr_handler;
    pub static nop_posix_acl_default: xattr_handler;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
