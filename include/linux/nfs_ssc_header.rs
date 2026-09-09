/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/linux/nfs_ssc.h
 *
 * Author: Dai Ngo <dai.ngo@oracle.com>
 *
 * Copyright (c) 2020, Oracle and/or its affiliates.
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    pub static mut nfs_ssc_client_tbl: nfs_ssc_client_ops_tbl;
}

/*
 * NFS_V4
 */
#[repr(C)]
pub struct nfs4_ssc_client_ops {
    pub sco_open: Option<unsafe extern "C" fn(
        ss_mnt: *mut vfsmount,
        src_fh: *mut nfs_fh,
        stateid: *mut nfs4_stateid,
    ) -> *mut file>,
    pub sco_close: Option<unsafe extern "C" fn(filep: *mut file)>,
}

/*
 * NFS_FS
 */
#[repr(C)]
pub struct nfs_ssc_client_ops {
    pub sco_sb_deactive: Option<unsafe extern "C" fn(sb: *mut super_block)>,
}

#[repr(C)]
pub struct nfs_ssc_client_ops_tbl {
    pub ssc_nfs4_ops: *const nfs4_ssc_client_ops,
    pub ssc_nfs_ops: *const nfs_ssc_client_ops,
}

extern "C" {
    pub fn nfs42_ssc_register_ops();
    pub fn nfs42_ssc_unregister_ops();

    pub fn nfs42_ssc_register(ops: *const nfs4_ssc_client_ops);
    pub fn nfs42_ssc_unregister(ops: *const nfs4_ssc_client_ops);
}

// Corresponds to CONFIG_NFSD_V4_2_INTER_SSC.
#[cfg(feature = "CONFIG_NFSD_V4_2_INTER_SSC")]
pub unsafe fn nfs42_ssc_open(
    ss_mnt: *mut vfsmount,
    src_fh: *mut nfs_fh,
    stateid: *mut nfs4_stateid,
) -> *mut file {
    if !nfs_ssc_client_tbl.ssc_nfs4_ops.is_null() {
        let ops = &*nfs_ssc_client_tbl.ssc_nfs4_ops;
        return (ops.sco_open.unwrap_unchecked())(ss_mnt, src_fh, stateid);
    }
    ERR_PTR(-EIO)
}

// Corresponds to CONFIG_NFSD_V4_2_INTER_SSC.
#[cfg(feature = "CONFIG_NFSD_V4_2_INTER_SSC")]
pub unsafe fn nfs42_ssc_close(filep: *mut file) {
    if !nfs_ssc_client_tbl.ssc_nfs4_ops.is_null() {
        let ops = &*nfs_ssc_client_tbl.ssc_nfs4_ops;
        (ops.sco_close.unwrap_unchecked())(filep);
    }
}

#[repr(C)]
pub struct nfsd4_ssc_umount_item {
    pub nsui_list: list_head,
    pub nsui_busy: bool,
    /*
     * nsui_refcnt inited to 2, 1 on list and 1 for consumer. Entry
     * is removed when refcnt drops to 1 and nsui_expire expires.
     */
    pub nsui_refcnt: refcount_t,
    pub nsui_expire: c_ulong,
    pub nsui_vfsmount: *mut vfsmount,
    pub nsui_ipaddr: [c_char; RPC_MAX_ADDRBUFLEN + 1],
}

/*
 * NFS_FS
 */
extern "C" {
    pub fn nfs_ssc_register(ops: *const nfs_ssc_client_ops);
    pub fn nfs_ssc_unregister(ops: *const nfs_ssc_client_ops);
}

pub unsafe fn nfs_do_sb_deactive(sb: *mut super_block) {
    if !nfs_ssc_client_tbl.ssc_nfs_ops.is_null() {
        let ops = &*nfs_ssc_client_tbl.ssc_nfs_ops;
        (ops.sco_sb_deactive.unwrap_unchecked())(sb);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
