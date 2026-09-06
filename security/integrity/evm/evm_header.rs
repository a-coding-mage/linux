// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2005-2010 IBM Corporation
//
// Authors:
// Mimi Zohar <zohar@us.ibm.com>
// Kylene Hall <kjhall@us.ibm.com>
//
// File: evm.h
//
// Depends on: linux/xattr.h, linux/security.h, ../integrity.h

use std::os::raw::{c_char, c_ulong};

const EVM_INIT_HMAC: u32 = 0x0001;
const EVM_INIT_X509: u32 = 0x0002;
const EVM_ALLOW_METADATA_WRITES: u32 = 0x0004;
const EVM_SIGV3_REQUIRED: u32 = 0x0008;
const EVM_SETUP_COMPLETE: u32 = 0x80000000; // userland has signaled key load

const EVM_KEY_MASK: u32 = EVM_INIT_HMAC | EVM_INIT_X509;
const EVM_INIT_MASK: u32 = EVM_INIT_HMAC | EVM_INIT_X509 | EVM_SETUP_COMPLETE |
                            EVM_ALLOW_METADATA_WRITES | EVM_SIGV3_REQUIRED;

#[repr(C)]
pub struct xattr_list {
    pub list: list_head,
    pub name: *mut c_char,
    pub enabled: bool,
}

const EVM_NEW_FILE: u32 = 0x00000001;
const EVM_IMMUTABLE_DIGSIG: u32 = 0x00000002;

// EVM integrity metadata associated with an inode
#[repr(C)]
pub struct evm_iint_cache {
    pub flags: c_ulong,
    pub evm_status: u8, // enum integrity_status:4 bitfield
    pub metadata_inode: integrity_inode_attributes,
}

extern "C" {
    pub static evm_blob_sizes: lsm_blob_sizes;
}

#[inline]
pub unsafe fn evm_iint_inode(inode: *const inode) -> *mut evm_iint_cache {
    if (*inode).i_security.is_null() {
        return std::ptr::null_mut();
    }

    ((*inode).i_security as *mut u8).add(evm_blob_sizes.lbs_inode) as *mut evm_iint_cache
}

extern "C" {
    pub static evm_initialized: i32;
}

const EVM_ATTR_FSUUID: u32 = 0x0001;

extern "C" {
    pub static evm_hmac_attrs: i32;

    // List of EVM protected security xattrs
    pub static evm_config_xattrnames: list_head;
}

#[repr(C)]
pub struct evm_digest {
    pub hdr: ima_digest_data_hdr,
    pub digest: [c_char; 64], // IMA_MAX_DIGEST_SIZE from external header
}

extern "C" {
    pub fn evm_protected_xattr(req_xattr_name: *const c_char) -> i32;

    pub fn evm_init_key() -> i32;
    pub fn evm_update_evmxattr(
        dentry: *mut dentry,
        req_xattr_name: *const c_char,
        req_xattr_value: *const c_char,
        req_xattr_value_len: usize,
    ) -> i32;
    pub fn evm_calc_hmac(
        dentry: *mut dentry,
        req_xattr_name: *const c_char,
        req_xattr_value: *const c_char,
        req_xattr_value_len: usize,
        data: *mut evm_digest,
        iint: *mut evm_iint_cache,
    ) -> i32;
    pub fn evm_calc_hash(
        dentry: *mut dentry,
        req_xattr_name: *const c_char,
        req_xattr_value: *const c_char,
        req_xattr_value_len: usize,
        type_: i8,
        data: *mut evm_digest,
        iint: *mut evm_iint_cache,
    ) -> i32;
    pub fn evm_init_hmac(
        inode: *mut inode,
        xattrs: *const xattr,
        hmac_val: *mut c_char,
    ) -> i32;
    pub fn evm_init_secfs() -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
