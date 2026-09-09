// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Translated from xfs_dquot_item.h. The original header guard is omitted;
// dependency-provided types remain external to this translation unit.

pub struct xfs_dquot;
pub struct xfs_trans;
pub struct xfs_mount;

#[repr(C)]
pub struct xfs_dq_logitem {
    pub qli_item: xfs_log_item, // common portion
    pub qli_dquot: *mut xfs_dquot, // dquot ptr
    pub qli_flush_lsn: xfs_lsn_t, // lsn at last flush

    /*
     * We use this spinlock to coordinate access to the li_buf pointer in
     * the log item and the qli_dirty flag.
     */
    pub qli_lock: spinlock_t,
    pub qli_dirty: bool, // dirtied since last flush?
}

unsafe extern "C" {
    pub fn xfs_qm_dquot_logitem_init(dqp: *mut xfs_dquot);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
