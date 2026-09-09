// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2013 Trond Myklebust <Trond.Myklebust@netapp.com>
 */

// C dependencies:
// #include <uapi/linux/pr.h>
// #include <linux/blkdev.h>
// #include <linux/nfs_fs.h>
// #include "nfs4_fs.h"
// #include "internal.h"
// #include "nfs4session.h"
// #include "callback.h"
// #include "pnfs.h"
//
// CREATE_TRACE_POINTS
// #include "nfs4trace.h"

// The following symbols are exported Linux tracepoints. Their definitions are
// supplied by the tracepoint infrastructure and external translation units.
unsafe extern "C" {
    pub static nfs4_pnfs_read: core::ffi::c_void;
    pub static nfs4_pnfs_write: core::ffi::c_void;
    pub static nfs4_pnfs_commit_ds: core::ffi::c_void;

    pub static pnfs_mds_fallback_pg_init_read: core::ffi::c_void;
    pub static pnfs_mds_fallback_pg_init_write: core::ffi::c_void;
    pub static pnfs_mds_fallback_pg_get_mirror_count: core::ffi::c_void;
    pub static pnfs_mds_fallback_read_done: core::ffi::c_void;
    pub static pnfs_mds_fallback_write_done: core::ffi::c_void;
    pub static pnfs_mds_fallback_read_pagelist: core::ffi::c_void;
    pub static pnfs_mds_fallback_write_pagelist: core::ffi::c_void;
    pub static pnfs_ds_connect: core::ffi::c_void;

    pub static ff_layout_read_error: core::ffi::c_void;
    pub static ff_layout_write_error: core::ffi::c_void;
    pub static ff_layout_commit_error: core::ffi::c_void;

    pub static bl_ext_tree_prepare_commit: core::ffi::c_void;
    pub static bl_pr_key_reg: core::ffi::c_void;
    pub static bl_pr_key_reg_err: core::ffi::c_void;
    pub static bl_pr_key_unreg: core::ffi::c_void;
    pub static bl_pr_key_unreg_err: core::ffi::c_void;

    pub static fl_getdevinfo: core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
