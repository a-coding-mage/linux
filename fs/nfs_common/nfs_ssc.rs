// SPDX-License-Identifier: GPL-2.0-only
/*
 * Helper for knfsd's SSC to access ops in NFS client modules
 *
 * Author: Dai Ngo <dai.ngo@oracle.com>
 *
 * Copyright (c) 2020, Oracle and/or its affiliates.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct nfs_ssc_client_ops_tbl {
    pub ssc_nfs4_ops: *const nfs4_ssc_client_ops,
    pub ssc_nfs_ops: *const nfs_ssc_client_ops,
}

pub enum nfs4_ssc_client_ops {}
pub enum nfs_ssc_client_ops {}

#[no_mangle]
pub static mut nfs_ssc_client_tbl: nfs_ssc_client_ops_tbl = nfs_ssc_client_ops_tbl {
    ssc_nfs4_ops: core::ptr::null(),
    ssc_nfs_ops: core::ptr::null(),
};

#[cfg(CONFIG_NFS_V4_2)]
/// nfs42_ssc_register - install the NFS_V4 client ops in the nfs_ssc_client_tbl
/// @ops: NFS_V4 ops to be installed
///
/// Return values:
///   None
#[no_mangle]
pub unsafe extern "C" fn nfs42_ssc_register(ops: *const nfs4_ssc_client_ops) {
    nfs_ssc_client_tbl.ssc_nfs4_ops = ops;
}

#[cfg(CONFIG_NFS_V4_2)]
/// nfs42_ssc_unregister - uninstall the NFS_V4 client ops from
///                         the nfs_ssc_client_tbl
/// @ops: ops to be uninstalled
///
/// Return values:
///   None
#[no_mangle]
pub unsafe extern "C" fn nfs42_ssc_unregister(ops: *const nfs4_ssc_client_ops) {
    if nfs_ssc_client_tbl.ssc_nfs4_ops != ops {
        return;
    }

    nfs_ssc_client_tbl.ssc_nfs4_ops = core::ptr::null();
}

#[cfg(CONFIG_NFS_V4_2)]
/// nfs_ssc_register - install the NFS_FS client ops in the nfs_ssc_client_tbl
/// @ops: NFS_FS ops to be installed
///
/// Return values:
///   None
#[no_mangle]
pub unsafe extern "C" fn nfs_ssc_register(ops: *const nfs_ssc_client_ops) {
    nfs_ssc_client_tbl.ssc_nfs_ops = ops;
}

#[cfg(CONFIG_NFS_V4_2)]
/// nfs_ssc_unregister - uninstall the NFS_FS client ops from
///                         the nfs_ssc_client_tbl
/// @ops: ops to be uninstalled
///
/// Return values:
///   None
#[no_mangle]
pub unsafe extern "C" fn nfs_ssc_unregister(ops: *const nfs_ssc_client_ops) {
    if nfs_ssc_client_tbl.ssc_nfs_ops != ops {
        return;
    }
    nfs_ssc_client_tbl.ssc_nfs_ops = core::ptr::null();
}

#[cfg(not(CONFIG_NFS_V4_2))]
#[no_mangle]
pub unsafe extern "C" fn nfs_ssc_register(_ops: *const nfs_ssc_client_ops) {}

#[cfg(not(CONFIG_NFS_V4_2))]
#[no_mangle]
pub unsafe extern "C" fn nfs_ssc_unregister(_ops: *const nfs_ssc_client_ops) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
