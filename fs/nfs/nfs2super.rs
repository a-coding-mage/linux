// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012 Netapp, Inc. All rights reserved.
 */

// Dependencies supplied by the Linux kernel and NFS implementation.

extern "C" {
    static THIS_MODULE: *mut module;
    static nfs_fs_type: nfs_fs_type;
    static nfs_version2: rpc_version;
    static nfs_v2_clientops: nfs_rpc_ops;
    static nfs_sops: super_operations;

    fn register_nfs_version(nfs: *mut nfs_subversion);
    fn unregister_nfs_version(nfs: *mut nfs_subversion);
}

#[repr(C)]
struct module;
#[repr(C)]
struct nfs_fs_type;
#[repr(C)]
struct rpc_version;
#[repr(C)]
struct nfs_rpc_ops;
#[repr(C)]
struct super_operations;

#[repr(C)]
struct nfs_subversion {
    owner: *mut module,
    nfs_fs: *const nfs_fs_type,
    rpc_vers: *const rpc_version,
    rpc_ops: *const nfs_rpc_ops,
    sops: *const super_operations,
}

static mut nfs_v2: nfs_subversion = nfs_subversion {
    owner: unsafe { &raw mut THIS_MODULE },
    nfs_fs: unsafe { &nfs_fs_type },
    rpc_vers: unsafe { &nfs_version2 },
    rpc_ops: unsafe { &nfs_v2_clientops },
    sops: unsafe { &nfs_sops },
};

unsafe extern "C" fn init_nfs_v2() -> i32 {
    unsafe {
        register_nfs_version(&raw mut nfs_v2);
    }
    0
}

unsafe extern "C" fn exit_nfs_v2() {
    unsafe {
        unregister_nfs_version(&raw mut nfs_v2);
    }
}

// MODULE_DESCRIPTION("NFSv2 client support");
// MODULE_LICENSE("GPL");

// module_init(init_nfs_v2);
// module_exit(exit_nfs_v2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
