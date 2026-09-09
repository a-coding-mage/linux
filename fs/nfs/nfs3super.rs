// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012 Netapp, Inc. All rights reserved.
 */

// Linux kernel dependencies supplied by the surrounding repository.

#[repr(C)]
pub struct nfs_subversion {
    pub owner: *mut core::ffi::c_void,
    pub nfs_fs: *mut nfs_fs_type,
    pub rpc_vers: *mut nfs_version,
    pub rpc_ops: *mut nfs_rpc_ops,
    pub sops: *mut nfs_server_ops,
}

#[repr(C)]
pub struct nfs_fs_type;
#[repr(C)]
pub struct nfs_version;
#[repr(C)]
pub struct nfs_rpc_ops;
#[repr(C)]
pub struct nfs_server_ops;

unsafe extern "C" {
    pub static mut THIS_MODULE: *mut core::ffi::c_void;
    pub static mut nfs_fs_type: nfs_fs_type;
    pub static mut nfs_version3: nfs_version;
    pub static mut nfs_v3_clientops: nfs_rpc_ops;
    pub static mut nfs_sops: nfs_server_ops;

    pub fn register_nfs_version(version: *mut nfs_subversion);
    pub fn unregister_nfs_version(version: *mut nfs_subversion);
}

#[no_mangle]
pub static mut nfs_v3: nfs_subversion = nfs_subversion {
    owner: unsafe { THIS_MODULE },
    nfs_fs: unsafe { &raw mut nfs_fs_type },
    rpc_vers: unsafe { &raw mut nfs_version3 },
    rpc_ops: unsafe { &raw mut nfs_v3_clientops },
    sops: unsafe { &raw mut nfs_sops },
};

// __init
#[no_mangle]
pub unsafe extern "C" fn init_nfs_v3() -> i32 {
    unsafe {
        register_nfs_version(&raw mut nfs_v3);
    }
    0
}

// __exit
#[no_mangle]
pub unsafe extern "C" fn exit_nfs_v3() {
    unsafe {
        unregister_nfs_version(&raw mut nfs_v3);
    }
}

// MODULE_DESCRIPTION("NFSv3 client support");
// MODULE_LICENSE("GPL");
// module_init(init_nfs_v3);
// module_exit(exit_nfs_v3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
