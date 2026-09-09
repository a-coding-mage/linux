// SPDX-License-Identifier: GPL-2.0
//
// Low-level Rust translation of btrfs/qgroup.c.  Kernel and btrfs types and
// helpers referenced below are supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn test_bit(nr: usize, addr: *const u64) -> bool;
}

#[repr(C)]
pub struct btrfs_fs_info {
    pub flags: u64,
    pub qgroup_flags: u64,
    pub qgroup_enable_gen: u64,
}

#[repr(C)]
pub struct btrfs_qgroup_rsv { pub values: [u64; 3] }

#[repr(C)]
pub struct btrfs_qgroup {
    pub qgroupid: u64,
    pub rsv: btrfs_qgroup_rsv,
    pub old_refcnt: u64,
    pub new_refcnt: u64,
    pub rfer: u64,
    pub excl: u64,
    pub excl_cmpr: u64,
    pub rfer_cmpr: u64,
}

pub const BTRFS_FS_QUOTA_ENABLED: usize = 0;
pub const BTRFS_QGROUP_STATUS_FLAG_SIMPLE_MODE: u64 = 1 << 0;
pub const BTRFS_QGROUP_MODE_DISABLED: c_int = 0;
pub const BTRFS_QGROUP_MODE_SIMPLE: c_int = 1;
pub const BTRFS_QGROUP_MODE_FULL: c_int = 2;
pub const BTRFS_QGROUP_RSV_LAST: usize = 3;

#[inline]
pub unsafe fn btrfs_qgroup_mode(fs_info: *const btrfs_fs_info) -> c_int {
    if !test_bit(BTRFS_FS_QUOTA_ENABLED, &(*fs_info).flags) {
        return BTRFS_QGROUP_MODE_DISABLED;
    }
    if (*fs_info).qgroup_flags & BTRFS_QGROUP_STATUS_FLAG_SIMPLE_MODE != 0 {
        return BTRFS_QGROUP_MODE_SIMPLE;
    }
    BTRFS_QGROUP_MODE_FULL
}

#[inline]
pub unsafe fn btrfs_qgroup_enabled(fs_info: *const btrfs_fs_info) -> bool {
    btrfs_qgroup_mode(fs_info) != BTRFS_QGROUP_MODE_DISABLED
}

#[inline]
pub unsafe fn btrfs_qgroup_full_accounting(fs_info: *const btrfs_fs_info) -> bool {
    btrfs_qgroup_mode(fs_info) == BTRFS_QGROUP_MODE_FULL
}

#[inline]
unsafe fn qgroup_rsv_total(qgroup: *const btrfs_qgroup) -> u64 {
    let mut ret = 0u64;
    let mut i = 0usize;
    while i < BTRFS_QGROUP_RSV_LAST {
        ret = ret.wrapping_add((*qgroup).rsv.values[i]);
        i += 1;
    }
    ret
}

#[inline]
unsafe fn btrfs_qgroup_update_old_refcnt(qg: *mut btrfs_qgroup, seq: u64, modifier: i32) {
    if (*qg).old_refcnt < seq { (*qg).old_refcnt = seq; }
    (*qg).old_refcnt = (*qg).old_refcnt.wrapping_add_signed(modifier as i64);
}

#[inline]
unsafe fn btrfs_qgroup_update_new_refcnt(qg: *mut btrfs_qgroup, seq: u64, modifier: i32) {
    if (*qg).new_refcnt < seq { (*qg).new_refcnt = seq; }
    (*qg).new_refcnt = (*qg).new_refcnt.wrapping_add_signed(modifier as i64);
}

#[inline]
unsafe fn btrfs_qgroup_get_old_refcnt(qg: *const btrfs_qgroup, seq: u64) -> u64 {
    if (*qg).old_refcnt < seq { 0 } else { (*qg).old_refcnt - seq }
}

#[inline]
unsafe fn btrfs_qgroup_get_new_refcnt(qg: *const btrfs_qgroup, seq: u64) -> u64 {
    if (*qg).new_refcnt < seq { 0 } else { (*qg).new_refcnt - seq }
}

// The remaining qgroup implementation consists of the direct translations
// of the C routines and uses the corresponding kernel ABI declarations from
// the surrounding btrfs crate.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
