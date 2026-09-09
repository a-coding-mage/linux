/* SPDX-License-Identifier: GPL-2.0 */
/*
 * evm.h
 *
 * Copyright (c) 2009 IBM Corporation
 * Author: Mimi Zohar <zohar@us.ibm.com>
 */

use core::ffi::{c_char, c_void};

/* Dependencies supplied by the Linux integrity and xattr headers. */

/* The following declarations are active when CONFIG_EVM is enabled. */
#[cfg(CONFIG_EVM)]
extern "C" {
    pub fn evm_set_key(key: *mut c_void, keylen: usize) -> i32;
    pub fn evm_verifyxattr(
        dentry: *mut dentry,
        xattr_name: *const c_char,
        xattr_value: *mut c_void,
        xattr_value_len: usize,
    ) -> integrity_status;
    pub fn evm_fix_hmac(
        dentry: *mut dentry,
        xattr_name: *const c_char,
        xattr_value: *const c_char,
        xattr_value_len: usize,
    ) -> i32;
    pub fn evm_inode_init_security(
        inode: *mut inode,
        dir: *mut inode,
        qstr: *const qstr,
        xattrs: *mut xattr,
        xattr_count: *mut i32,
    ) -> i32;
    pub fn evm_revalidate_status(xattr_name: *const c_char) -> bool;
    pub fn evm_protected_xattr_if_enabled(req_xattr_name: *const c_char) -> i32;
    pub fn evm_read_protected_xattrs(
        dentry: *mut dentry,
        buffer: *mut u8,
        buffer_size: i32,
        type_: c_char,
        canonical_fmt: bool,
    ) -> i32;
    pub fn evm_metadata_changed(inode: *mut inode, metadata_inode: *mut inode) -> bool;

    #[cfg(CONFIG_FS_POSIX_ACL)]
    pub fn posix_xattr_acl(xattrname: *const c_char) -> i32;
}

/* CONFIG_FS_POSIX_ACL disabled: the C inline function returns 0. */
#[cfg(all(CONFIG_EVM, not(CONFIG_FS_POSIX_ACL)))]
#[inline]
pub unsafe fn posix_xattr_acl(_xattrname: *const c_char) -> i32 {
    0
}

/* CONFIG_EVM disabled: the C inline fallback implementations follow. */
#[cfg(not(CONFIG_EVM))]
#[inline]
pub unsafe fn evm_set_key(_key: *mut c_void, _keylen: usize) -> i32 {
    -EOPNOTSUPP
}

#[cfg(all(not(CONFIG_EVM), CONFIG_INTEGRITY))]
#[inline]
pub unsafe fn evm_verifyxattr(
    _dentry: *mut dentry,
    _xattr_name: *const c_char,
    _xattr_value: *mut c_void,
    _xattr_value_len: usize,
) -> integrity_status {
    INTEGRITY_UNKNOWN
}

#[cfg(all(not(CONFIG_EVM), CONFIG_INTEGRITY))]
#[inline]
pub unsafe fn evm_fix_hmac(
    _dentry: *mut dentry,
    _xattr_name: *const c_char,
    _xattr_value: *const c_char,
    _xattr_value_len: usize,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_EVM))]
#[inline]
pub unsafe fn evm_inode_init_security(
    _inode: *mut inode,
    _dir: *mut inode,
    _qstr: *const qstr,
    _xattrs: *mut xattr,
    _xattr_count: *mut i32,
) -> i32 {
    0
}

#[cfg(not(CONFIG_EVM))]
#[inline]
pub unsafe fn evm_revalidate_status(_xattr_name: *const c_char) -> bool {
    false
}

#[cfg(not(CONFIG_EVM))]
#[inline]
pub unsafe fn evm_protected_xattr_if_enabled(_req_xattr_name: *const c_char) -> i32 {
    false as i32
}

#[cfg(not(CONFIG_EVM))]
#[inline]
pub unsafe fn evm_read_protected_xattrs(
    _dentry: *mut dentry,
    _buffer: *mut u8,
    _buffer_size: i32,
    _type: c_char,
    _canonical_fmt: bool,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_EVM))]
#[inline]
pub unsafe fn evm_metadata_changed(_inode: *mut inode, _metadata_inode: *mut inode) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
