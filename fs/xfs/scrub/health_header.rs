// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2019-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Opaque types supplied by the surrounding XFS scrub implementation.
#[repr(C)]
pub struct xfs_scrub {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_btree_cur {
    _private: [u8; 0],
}

extern "C" {
    pub fn xchk_health_mask_for_scrub_type(scrub_type: u32) -> u32;
    pub fn xchk_update_health(sc: *mut xfs_scrub);
    pub fn xchk_ag_btree_del_cursor_if_sick(
        sc: *mut xfs_scrub,
        curp: *mut *mut xfs_btree_cur,
        sm_type: u32,
    );
    pub fn xchk_mark_healthy_if_clean(sc: *mut xfs_scrub, mask: u32);
    pub fn xchk_file_looks_zapped(sc: *mut xfs_scrub, mask: u32) -> bool;
    pub fn xchk_health_record(sc: *mut xfs_scrub) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
