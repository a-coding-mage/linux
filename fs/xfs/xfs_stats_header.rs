// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependency: <linux/percpu.h> supplies the per-CPU pointer operations used by
// the original declarations and macros.

#[repr(usize)]
pub enum Xbts {
    Lookup = 0,
    Compare = 1,
    Insrec = 2,
    Delrec = 3,
    Newroot = 4,
    Killroot = 5,
    Increment = 6,
    Decrement = 7,
    Lshift = 8,
    Rshift = 9,
    Split = 10,
    Join = 11,
    Alloc = 12,
    Free = 13,
    Moves = 14,
    Max = 15,
}

pub const __XBTS_lookup: usize = 0;
pub const __XBTS_compare: usize = 1;
pub const __XBTS_insrec: usize = 2;
pub const __XBTS_delrec: usize = 3;
pub const __XBTS_newroot: usize = 4;
pub const __XBTS_killroot: usize = 5;
pub const __XBTS_increment: usize = 6;
pub const __XBTS_decrement: usize = 7;
pub const __XBTS_lshift: usize = 8;
pub const __XBTS_rshift: usize = 9;
pub const __XBTS_split: usize = 10;
pub const __XBTS_join: usize = 11;
pub const __XBTS_alloc: usize = 12;
pub const __XBTS_free: usize = 13;
pub const __XBTS_moves: usize = 14;
pub const __XBTS_MAX: usize = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __xfsstats {
    pub xs_allocx: u32, pub xs_allocb: u32, pub xs_freex: u32, pub xs_freeb: u32,
    pub xs_abt_lookup: u32, pub xs_abt_compare: u32, pub xs_abt_insrec: u32, pub xs_abt_delrec: u32,
    pub xs_blk_mapr: u32, pub xs_blk_mapw: u32, pub xs_blk_unmap: u32,
    pub xs_add_exlist: u32, pub xs_del_exlist: u32, pub xs_look_exlist: u32, pub xs_cmp_exlist: u32,
    pub xs_bmbt_lookup: u32, pub xs_bmbt_compare: u32, pub xs_bmbt_insrec: u32, pub xs_bmbt_delrec: u32,
    pub xs_dir_lookup: u32, pub xs_dir_create: u32, pub xs_dir_remove: u32, pub xs_dir_getdents: u32,
    pub xs_trans_sync: u32, pub xs_trans_async: u32, pub xs_trans_empty: u32,
    pub xs_ig_attempts: u32, pub xs_ig_found: u32, pub xs_ig_frecycle: u32, pub xs_ig_missed: u32,
    pub xs_ig_dup: u32, pub xs_ig_reclaims: u32, pub xs_ig_attrchg: u32,
    pub xs_log_writes: u32, pub xs_log_blocks: u32, pub xs_log_noiclogs: u32, pub xs_log_force: u32,
    pub xs_log_force_sleep: u32, pub xs_try_logspace: u32, pub xs_sleep_logspace: u32,
    pub xs_push_ail: u32, pub xs_push_ail_success: u32, pub xs_push_ail_pushbuf: u32,
    pub xs_push_ail_pinned: u32, pub xs_push_ail_locked: u32, pub xs_push_ail_flushing: u32,
    pub xs_push_ail_restarts: u32, pub xs_push_ail_flush: u32,
    pub xs_xstrat_quick: u32, pub xs_xstrat_split: u32, pub xs_write_calls: u32, pub xs_read_calls: u32,
    pub xs_attr_get: u32, pub xs_attr_set: u32, pub xs_attr_remove: u32, pub xs_attr_list: u32,
    pub xs_iflush_count: u32, pub xs_icluster_flushcnt: u32, pub xs_icluster_flushinode: u32,
    pub xs_inodes_active: u32, pub __unused_vn_alloc: u32, pub __unused_vn_get: u32, pub __unused_vn_hold: u32,
    pub xs_inode_destroy: u32, pub xs_inode_destroy2: u32, pub xs_inode_mark_reclaimable: u32, pub __unused_vn_free: u32,
    pub xb_get: u32, pub xb_create: u32, pub xb_get_locked: u32, pub xb_get_locked_waited: u32,
    pub xb_busy_locked: u32, pub xb_miss_locked: u32, pub xb_page_retries: u32, pub xb_page_found: u32, pub xb_get_read: u32,
    pub xs_abtb_2: [u32; __XBTS_MAX], pub xs_abtc_2: [u32; __XBTS_MAX], pub xs_bmbt_2: [u32; __XBTS_MAX],
    pub xs_ibt_2: [u32; __XBTS_MAX], pub xs_fibt_2: [u32; __XBTS_MAX], pub xs_rmap_2: [u32; __XBTS_MAX],
    pub xs_refcbt_2: [u32; __XBTS_MAX], pub xs_rmap_mem_2: [u32; __XBTS_MAX], pub xs_rcbag_2: [u32; __XBTS_MAX],
    pub xs_rtrmap_2: [u32; __XBTS_MAX], pub xs_rtrmap_mem_2: [u32; __XBTS_MAX], pub xs_rtrefcbt_2: [u32; __XBTS_MAX],
    pub xs_qm_dqreclaims: u32, pub xs_qm_dqreclaim_misses: u32, pub xs_qm_dquot_dups: u32,
    pub xs_qm_dqcachemisses: u32, pub xs_qm_dqcachehits: u32, pub xs_qm_dqwants: u32,
    pub xs_qm_dquot: u32, pub xs_qm_dquot_unused: u32,
    pub xs_gc_read_calls: u32, pub xs_gc_write_calls: u32, pub xs_gc_zone_reset_calls: u32,
    pub xs_inodes_meta: u32,
    pub xs_xstrat_bytes: u64, pub xs_write_bytes: u64, pub xs_read_bytes: u64, pub xs_defer_relog: u64, pub xs_gc_bytes: u64,
}

#[repr(C)]
pub union xfsstats { pub s: __xfsstats, pub a: [u32; xfsstats_offset!(xs_qm_dquot)], }

#[macro_export]
macro_rules! xfsstats_offset { ($f:ident) => { core::mem::offset_of!($crate::__xfsstats, $f) / core::mem::size_of::<u32>() }; }
#[macro_export]
macro_rules! XFS_STATS_CALC_INDEX { ($member:ident) => { core::mem::offset_of!($crate::__xfsstats, $member) / core::mem::size_of::<u32>() }; }

extern "C" {
    pub fn xfs_stats_format(stats: *mut xfsstats, buf: *mut i8) -> i32;
    pub fn xfs_stats_clearall(stats: *mut xfsstats);
    pub static mut xfsstats: xstats;
    pub fn xfs_init_procfs() -> i32;
    pub fn xfs_cleanup_procfs();
}

// `struct xstats`, `per_cpu_ptr`, `current_cpu`, and the mount structure are
// supplied by other translation units.
extern "C" { pub type xstats; }

#[macro_export]
macro_rules! XFS_STATS_INC { ($mp:expr, $v:ident) => {{ unsafe { (*per_cpu_ptr((*$crate::xfsstats).xs_stats, current_cpu())).s.$v += 1; (*per_cpu_ptr((*$mp).m_stats.xs_stats, current_cpu())).s.$v += 1; } }}; }
#[macro_export]
macro_rules! XFS_STATS_DEC { ($mp:expr, $v:ident) => {{ unsafe { (*per_cpu_ptr((*$crate::xfsstats).xs_stats, current_cpu())).s.$v -= 1; (*per_cpu_ptr((*$mp).m_stats.xs_stats, current_cpu())).s.$v -= 1; } }}; }
#[macro_export]
macro_rules! XFS_STATS_ADD { ($mp:expr, $v:ident, $inc:expr) => {{ unsafe { (*per_cpu_ptr((*$crate::xfsstats).xs_stats, current_cpu())).s.$v += $inc; (*per_cpu_ptr((*$mp).m_stats.xs_stats, current_cpu())).s.$v += $inc; } }}; }
#[macro_export]
macro_rules! XFS_STATS_INC_OFF { ($mp:expr, $off:expr) => {{ unsafe { (*per_cpu_ptr((*$crate::xfsstats).xs_stats, current_cpu())).a[$off] += 1; (*per_cpu_ptr((*$mp).m_stats.xs_stats, current_cpu())).a[$off] += 1; } }}; }
#[macro_export]
macro_rules! XFS_STATS_DEC_OFF { ($mp:expr, $off:expr) => {{ unsafe { let _ = (*per_cpu_ptr((*$crate::xfsstats).xs_stats, current_cpu())).a[$off]; let _ = (*per_cpu_ptr((*$mp).m_stats.xs_stats, current_cpu())).a[$off]; } }}; }
#[macro_export]
macro_rules! XFS_STATS_ADD_OFF { ($mp:expr, $off:expr, $inc:expr) => {{ unsafe { (*per_cpu_ptr((*$crate::xfsstats).xs_stats, current_cpu())).a[$off] += $inc; (*per_cpu_ptr((*$mp).m_stats.xs_stats, current_cpu())).a[$off] += $inc; } }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
