/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2012 Netapp, Inc. All rights reserved.
 *
 * Function and structures exported by the NFS module
 * for use by NFS version-specific modules.
 */

// Dependencies supplied by the corresponding Linux/Rust bindings:
// linux/fs.h, linux/sunrpc/sched.h, and linux/nfs_xdr.h.

#[repr(C)]
pub struct nfs_subversion {
    pub owner: *mut module, // THIS_MODULE pointer
    pub nfs_fs: *mut file_system_type, // NFS filesystem type
    pub rpc_vers: *const rpc_version, // NFS version information
    pub rpc_ops: *const nfs_rpc_ops, // NFS operations
    pub sops: *const super_operations, // NFS Super operations
    pub xattr: *const *const xattr_handler, // NFS xattr handlers
}

extern "C" {
    pub fn find_nfs_version(version: u32) -> *mut nfs_subversion;
    pub fn get_nfs_version(subversion: *mut nfs_subversion) -> i32;
    pub fn put_nfs_version(subversion: *mut nfs_subversion);
    pub fn register_nfs_version(subversion: *mut nfs_subversion);
    pub fn unregister_nfs_version(subversion: *mut nfs_subversion);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
