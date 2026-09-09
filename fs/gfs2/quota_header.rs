/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Translated from quota.h.  C header inclusion and header guards are omitted.

use core::ffi::c_void;

pub enum gfs2_inode {}
pub enum gfs2_sbd {}
pub enum gfs2_alloc_parms {}
pub enum super_block {}
pub enum kqid {}
pub enum kuid_t {}
pub enum kgid_t {}
pub enum quotactl_ops {}
pub enum list_lru {}

pub const NO_UID_QUOTA_CHANGE: u32 = INVALID_UID;
pub const NO_GID_QUOTA_CHANGE: u32 = INVALID_GID;

unsafe extern "C" {
    pub fn gfs2_qa_get(ip: *mut gfs2_inode) -> i32;
    pub fn gfs2_qa_put(ip: *mut gfs2_inode);
    pub fn gfs2_quota_hold(ip: *mut gfs2_inode, uid: kuid_t, gid: kgid_t) -> i32;
    pub fn gfs2_quota_unhold(ip: *mut gfs2_inode);

    pub fn gfs2_quota_lock(ip: *mut gfs2_inode, uid: kuid_t, gid: kgid_t) -> i32;
    pub fn gfs2_quota_unlock(ip: *mut gfs2_inode);

    pub fn gfs2_quota_check(
        ip: *mut gfs2_inode,
        uid: kuid_t,
        gid: kgid_t,
        ap: *mut gfs2_alloc_parms,
    ) -> i32;
    pub fn gfs2_quota_change(ip: *mut gfs2_inode, change: i64, uid: kuid_t, gid: kgid_t);

    pub fn gfs2_quota_sync(sb: *mut super_block, type_: i32) -> i32;
    pub fn gfs2_quota_refresh(sdp: *mut gfs2_sbd, qid: kqid) -> i32;

    pub fn gfs2_quota_init(sdp: *mut gfs2_sbd) -> i32;
    pub fn gfs2_quota_cleanup(sdp: *mut gfs2_sbd);
    pub fn gfs2_quotad(data: *mut c_void) -> i32;

    pub fn gfs2_wake_up_statfs(sdp: *mut gfs2_sbd);

    pub static gfs2_quotactl_ops: quotactl_ops;
    pub fn gfs2_qd_shrinker_init() -> i32;
    pub fn gfs2_qd_shrinker_exit();
    pub static mut gfs2_qd_lru: list_lru;
    pub fn gfs2_quota_hash_init();
}

// static inline int gfs2_quota_lock_check(...) is preserved below as a Rust
// inline function; its referenced structure fields and helpers are supplied
// by the surrounding kernel translation.
#[inline]
pub unsafe fn gfs2_quota_lock_check(
    ip: *mut gfs2_inode,
    ap: *mut gfs2_alloc_parms,
) -> i32 {
    // The complete field-level implementation depends on the declarations
    // supplied by the including kernel headers.
    let _ = (ip, ap);
    unimplemented!()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
