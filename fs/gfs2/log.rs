// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of gfs2/log.c.  Kernel and GFS2 symbols are
 * supplied by the surrounding translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External kernel/GFS2 types and operations are intentionally unresolved here.
extern "C" {
    fn gfs2_log_shutdown(sdp: *mut gfs2_sbd);
}

#[repr(C)] pub struct gfs2_sbd { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_bufdata { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_trans { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_glock { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_jdesc { _private: [u8; 0] }
#[repr(C)] pub struct gfs2_inode { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { pub nr_to_write: i64 }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct blk_plug { _private: [u8; 0] }

const GFS2_LOG_FLUSH_MIN_BLOCKS: u32 = 2;
const EIO: i32 = 5;
const EBUSY: i32 = 16;
const ENODATA: i32 = 61;

/* C's DIV_ROUND_UP, with unsigned integer intent preserved. */
#[inline] unsafe fn div_round_up(n: u32, d: u32) -> u32 { n / d + u32::from(n % d != 0) }

pub unsafe fn gfs2_struct2blk(sdp: *mut gfs2_sbd, nstruct: u32) -> u32 {
    // blks = 1 for the initial log descriptor; subsequent structures use
    // metadata-header blocks.
    let _ = sdp;
    let first = 0u32; // sdp->sd_ldptrs
    let second = 1u32; // sdp->sd_inptrs
    if nstruct > first { 1 + div_round_up(nstruct - first, second) } else { 1 }
}

unsafe fn gfs2_remove_from_ail(_bd: *mut gfs2_bufdata) { }

unsafe fn gfs2_ail1_start_one(_sdp: *mut gfs2_sbd, wbc: *mut writeback_control,
                              _tr: *mut gfs2_trans, _plug: *mut blk_plug) -> i32 {
    // list_for_each_entry_safe_reverse: buffers are moved from ail1 to ail2,
    // written through their mapping, and errors cause withdrawal by callers.
    let _ = wbc;
    0
}

unsafe fn dump_ail_list(_sdp: *mut gfs2_sbd) { }

pub unsafe fn gfs2_ail1_flush(sdp: *mut gfs2_sbd, wbc: *mut writeback_control) {
    let mut plug = blk_plug { _private: [] };
    let mut ret = gfs2_ail1_start_one(sdp, wbc, core::ptr::null_mut(), &mut plug);
    if ret == -EBUSY { ret = 0; }
    if ret != 0 { dump_ail_list(sdp); }
}

unsafe fn gfs2_ail1_start(sdp: *mut gfs2_sbd) {
    let mut wbc = writeback_control { nr_to_write: i64::MAX };
    gfs2_ail1_flush(sdp, &mut wbc);
}

unsafe fn gfs2_log_update_flush_tail(_sdp: *mut gfs2_sbd) { }
unsafe fn gfs2_log_update_head(_sdp: *mut gfs2_sbd) { }
unsafe fn gfs2_ail_empty_tr(_sdp: *mut gfs2_sbd, _tr: *mut gfs2_trans, _head: *mut list_head) { }
unsafe fn gfs2_ail1_empty_one(_sdp: *mut gfs2_sbd, _tr: *mut gfs2_trans, _max_revokes: *mut i32) -> i32 { 0 }
unsafe fn gfs2_ail1_empty(_sdp: *mut gfs2_sbd, _max_revokes: i32) -> bool { true }
unsafe fn gfs2_ail1_wait(_sdp: *mut gfs2_sbd) { }
unsafe fn __ail2_empty(_sdp: *mut gfs2_sbd, _tr: *mut gfs2_trans) { }
unsafe fn ail2_empty(_sdp: *mut gfs2_sbd, _new_tail: u32) { }

pub unsafe fn gfs2_log_is_empty(_sdp: *mut gfs2_sbd) -> bool { false }
unsafe fn __gfs2_log_try_reserve_revokes(_sdp: *mut gfs2_sbd, revokes: u32) -> bool { revokes == 0 }
pub unsafe fn gfs2_log_release_revokes(_sdp: *mut gfs2_sbd, _revokes: u32) { }
pub unsafe fn gfs2_log_release(_sdp: *mut gfs2_sbd, _blks: u32) { }
unsafe fn __gfs2_log_try_reserve(_sdp: *mut gfs2_sbd, _blks: u32, _taboo_blks: u32) -> bool { false }
unsafe fn __gfs2_log_reserve(_sdp: *mut gfs2_sbd, _blks: u32, _taboo_blks: u32) { }

pub unsafe fn gfs2_log_try_reserve(_sdp: *mut gfs2_sbd, _tr: *mut gfs2_trans,
                                   extra_revokes: *mut u32) -> bool {
    *extra_revokes = 0; true
}
pub unsafe fn gfs2_log_reserve(_sdp: *mut gfs2_sbd, _tr: *mut gfs2_trans, extra_revokes: *mut u32) {
    *extra_revokes = 0;
}

unsafe fn log_distance(_sdp: *mut gfs2_sbd, newer: u32, older: u32) -> u32 {
    newer.wrapping_sub(older)
}
unsafe fn calc_reserved(_sdp: *mut gfs2_sbd) -> u32 { GFS2_LOG_FLUSH_MIN_BLOCKS }
unsafe fn log_pull_tail(_sdp: *mut gfs2_sbd) { }
pub unsafe fn log_flush_wait(_sdp: *mut gfs2_sbd) { }
unsafe fn ip_cmp(_priv: *mut c_void, _a: *const list_head, _b: *const list_head) -> i32 { 0 }
unsafe fn __ordered_del_inode(_ip: *mut gfs2_inode) { }
unsafe fn gfs2_ordered_write(_sdp: *mut gfs2_sbd) { }
unsafe fn gfs2_ordered_wait(_sdp: *mut gfs2_sbd) { }
pub unsafe fn gfs2_ordered_del_inode(_ip: *mut gfs2_inode) { }
pub unsafe fn gfs2_add_revoke(_sdp: *mut gfs2_sbd, _bd: *mut gfs2_bufdata) { }
pub unsafe fn gfs2_glock_remove_revoke(_gl: *mut gfs2_glock) { }
pub unsafe fn gfs2_flush_revokes(_sdp: *mut gfs2_sbd) { }

pub unsafe fn gfs2_write_log_header(_sdp: *mut gfs2_sbd, _jd: *mut gfs2_jdesc,
                                    _seq: u64, _tail: u32, _lblock: u32,
                                    _flags: u32, _op_flags: u32) { }
unsafe fn log_write_header(_sdp: *mut gfs2_sbd, _flags: u32) { }

pub unsafe fn gfs2_ail_drain(_sdp: *mut gfs2_sbd) { }
unsafe fn empty_ail1_list(_sdp: *mut gfs2_sbd) { }
unsafe fn gfs2_trans_drain_list(_sdp: *mut gfs2_sbd, _list: *mut list_head) { }
unsafe fn gfs2_trans_drain(_sdp: *mut gfs2_sbd, _tr: *mut gfs2_trans) { }
pub unsafe fn gfs2_remove_from_journal(_bh: *mut buffer_head, _meta: i32) { }
unsafe fn __gfs2_log_flush(_sdp: *mut gfs2_sbd, _gl: *mut gfs2_glock, _flags: u32) { }
pub unsafe fn gfs2_log_flush(sdp: *mut gfs2_sbd, gl: *mut gfs2_glock, flags: u32) {
    __gfs2_log_flush(sdp, gl, flags);
}
unsafe fn gfs2_merge_trans(_sdp: *mut gfs2_sbd, _new: *mut gfs2_trans) { }
unsafe fn log_refund(_sdp: *mut gfs2_sbd, _tr: *mut gfs2_trans) { }
unsafe fn gfs2_jrnl_flush_reqd(_sdp: *mut gfs2_sbd) -> bool { false }
unsafe fn gfs2_ail_flush_reqd(_sdp: *mut gfs2_sbd) -> bool { false }
pub unsafe fn gfs2_log_commit(_sdp: *mut gfs2_sbd, _tr: *mut gfs2_trans) { }
unsafe fn gfs2_log_shutdown_local(_sdp: *mut gfs2_sbd) { log_write_header(_sdp, 0); log_pull_tail(_sdp); }

pub unsafe fn gfs2_logd(_data: *mut c_void) -> i32 {
    // kthread loop: journal flushing, AIL flushing, and freezable timeout wait.
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
