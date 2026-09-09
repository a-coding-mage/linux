// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rcbag {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_buftarg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_trans {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_rmap_irec {
    _private: [u8; 0],
}

extern "C" {
    pub fn rcbag_init(
        mp: *mut xfs_mount,
        btp: *mut xfs_buftarg,
        bagp: *mut *mut rcbag,
    ) -> i32;
    pub fn rcbag_free(bagp: *mut *mut rcbag);
    pub fn rcbag_add(
        bag: *mut rcbag,
        tp: *mut xfs_trans,
        rmap: *const xfs_rmap_irec,
    ) -> i32;
    pub fn rcbag_count(bag: *const rcbag) -> u64;

    pub fn rcbag_next_edge(
        bag: *mut rcbag,
        tp: *mut xfs_trans,
        next_rmap: *const xfs_rmap_irec,
        next_valid: bool,
        next_bnop: *mut u32,
    ) -> i32;
    pub fn rcbag_remove_ending_at(
        bag: *mut rcbag,
        tp: *mut xfs_trans,
        next_bno: u32,
    ) -> i32;

    pub fn rcbag_dump(bag: *mut rcbag, tp: *mut xfs_trans);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
