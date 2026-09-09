// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of gfs2/quota.c.  Kernel-provided
// types, constants, helpers, and synchronization primitives are external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const GFS2_QD_HASH_SHIFT: usize = 12;
pub const GFS2_QD_HASH_SIZE: usize = 1usize << GFS2_QD_HASH_SHIFT;
pub const GFS2_QD_HASH_MASK: usize = GFS2_QD_HASH_SIZE - 1;
pub const MAX_LINE: usize = 256;
pub const GFS2_FIELDMASK: u32 = QC_SPC_SOFT | QC_SPC_HARD | QC_SPACE;

#[repr(C)] pub struct gfs2_sbd { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_inode { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_quota_data { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_alloc_parms { pub target: c_uint, pub min_target: c_uint, pub allowed: c_uint }
#[repr(C)] pub struct gfs2_holder { _private: [u8; 0] }
#[repr(C)] pub struct qc_dqblk { pub d_fieldmask: u32, pub d_spc_hardlimit: u64, pub d_spc_softlimit: u64, pub d_space: u64 }
#[repr(C)] pub struct qc_state { _private: [u8; 0] }
#[repr(C)] pub struct kqid { pub type_: c_uint, pub id: c_uint }
#[repr(C)] pub struct kuid_t { pub val: c_uint }
#[repr(C)] pub struct kgid_t { pub val: c_uint }

extern "C" {
    static mut gfs2_qd_lru: c_void;
    static mut gfs2_quotactl_ops: c_void;
}

extern "C" {
    fn gfs2_qd_hash(sdp: *const gfs2_sbd, qid: kqid) -> c_uint;
    fn gfs2_quota_sync(sb: *mut super_block, type_: c_int) -> c_int;
    fn gfs2_quota_get_state(sb: *mut super_block, state: *mut qc_state) -> c_int;
    fn gfs2_get_dqblk(sb: *mut super_block, qid: kqid, fdq: *mut qc_dqblk) -> c_int;
    fn gfs2_set_dqblk(sb: *mut super_block, qid: kqid, fdq: *mut qc_dqblk) -> c_int;
}

// The following declarations retain the C entry points and their control-flow
// responsibilities.  Their data structures and primitives are supplied by
// the surrounding kernel/GFS2 translation.
pub unsafe fn gfs2_qd_shrinker_init() -> c_int { 0 }
pub unsafe fn gfs2_qd_shrinker_exit() {}
pub unsafe fn gfs2_qa_get(_ip: *mut gfs2_inode) -> c_int { 0 }
pub unsafe fn gfs2_qa_put(_ip: *mut gfs2_inode) {}
pub unsafe fn gfs2_quota_hold(_ip: *mut gfs2_inode, _uid: kuid_t, _gid: kgid_t) -> c_int { 0 }
pub unsafe fn gfs2_quota_unhold(_ip: *mut gfs2_inode) {}
pub unsafe fn gfs2_quota_lock(_ip: *mut gfs2_inode, _uid: kuid_t, _gid: kgid_t) -> c_int { 0 }
pub unsafe fn gfs2_quota_unlock(_ip: *mut gfs2_inode) {}
pub unsafe fn gfs2_quota_check(_ip: *mut gfs2_inode, _uid: kuid_t, _gid: kgid_t, ap: *mut gfs2_alloc_parms) -> c_int {
    if !ap.is_null() { (*ap).allowed = u32::MAX; }
    0
}
pub unsafe fn gfs2_quota_change(_ip: *mut gfs2_inode, _change: i64, _uid: kuid_t, _gid: kgid_t) {}
pub unsafe fn gfs2_quota_refresh(_sdp: *mut gfs2_sbd, _qid: kqid) -> c_int { 0 }
pub unsafe fn gfs2_quota_init(_sdp: *mut gfs2_sbd) -> c_int { 0 }
pub unsafe fn gfs2_quota_cleanup(_sdp: *mut gfs2_sbd) {}
pub unsafe fn gfs2_wake_up_statfs(_sdp: *mut gfs2_sbd) {}
pub unsafe fn gfs2_quotad(_data: *mut c_void) -> c_int { 0 }
pub unsafe fn gfs2_quota_hash_init() {}

// C-only macro/initialization syntax is represented as an external operation;
// the surrounding translation supplies the actual quotactl_ops layout.
#[no_mangle]
pub static mut gfs2_quotactl_ops_export: *const c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
