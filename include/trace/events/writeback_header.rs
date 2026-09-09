/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/writeback.h.
// The tracepoint framework declarations below intentionally remain external
// integration points supplied by the kernel tracepoint implementation.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong};

pub const TRACE_SYSTEM: &str = "writeback";

extern "C" {
    pub fn __print_flags(state: c_ulong, delimiter: *const c_char, ...) -> *const c_char;
    pub fn __print_symbolic(value: c_int, ...) -> *const c_char;
    pub fn strscpy_pad(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
}

#[repr(C)] pub struct folio { pub index: c_ulong }
#[repr(C)] pub struct address_space { pub host: *mut inode, pub writeback_index: c_ulong }
#[repr(C)] pub struct inode { pub i_ino: u64, pub i_mode: u16, pub i_mapping: *mut address_space, pub i_sb: *mut super_block, pub dirtied_when: c_ulong }
#[repr(C)] pub struct super_block { pub s_dev: dev_t }
#[repr(C)] pub struct backing_dev_info { pub id: u64 }
#[repr(C)] pub struct cgroup { }
#[repr(C)] pub struct css { pub cgroup: *mut cgroup }
#[repr(C)] pub struct mem_cgroup { pub css: css }
#[repr(C)] pub struct bdi_writeback { pub bdi: *mut backing_dev_info, pub memcg_css: *mut css, pub write_bandwidth: c_ulong, pub avg_write_bandwidth: c_ulong, pub dirty_ratelimit: c_ulong, pub balanced_dirty_ratelimit: c_ulong }
#[repr(C)] pub struct writeback_control { pub wb: *mut bdi_writeback, pub sync_mode: c_int, pub nr_to_write: c_long, pub pages_skipped: c_long, pub for_kupdate: c_int, pub for_background: c_int, pub range_cyclic: c_int, pub range_start: u64, pub range_end: u64 }
#[repr(C)] pub struct wb_writeback_work { pub nr_pages: c_long, pub sb: *mut super_block, pub sync_mode: c_int, pub for_kupdate: c_int, pub range_cyclic: c_int, pub for_background: c_int, pub reason: c_int }
#[repr(C)] pub struct dirty_throttle_control { pub thresh: c_ulong, pub bg_thresh: c_ulong, pub limit: c_ulong, pub dirty: c_ulong, pub wb_thresh: c_ulong, pub wb_dirty: c_ulong }
pub type dev_t = u64;

pub const WB_REASON_BACKGROUND: c_int = 0;
pub const WB_REASON_VMSCAN: c_int = 1;
pub const WB_REASON_SYNC: c_int = 2;
pub const WB_REASON_PERIODIC: c_int = 3;
pub const WB_REASON_FS_FREE_SPACE: c_int = 4;
pub const WB_REASON_FORKER_THREAD: c_int = 5;
pub const WB_REASON_FOREIGN_FLUSH: c_int = 6;
pub const WB_REASON_DONTCACHE: c_int = 7;

pub const WB_WORK_REASON: &[(c_int, &str)] = &[
    (WB_REASON_BACKGROUND, "background"), (WB_REASON_VMSCAN, "vmscan"),
    (WB_REASON_SYNC, "sync"), (WB_REASON_PERIODIC, "periodic"),
    (WB_REASON_FS_FREE_SPACE, "fs_free_space"), (WB_REASON_FORKER_THREAD, "forker_thread"),
    (WB_REASON_FOREIGN_FLUSH, "foreign_flush"), (WB_REASON_DONTCACHE, "dontcache"),
];

#[inline]
pub unsafe fn __trace_wb_assign_cgroup(_wb: *mut bdi_writeback) -> u64 { 1 }
#[inline]
pub unsafe fn __trace_wbc_assign_cgroup(wbc: *mut writeback_control) -> u64 {
    if !wbc.is_null() && !(*wbc).wb.is_null() { __trace_wb_assign_cgroup((*wbc).wb) } else { 1 }
}

// show_inode_state(state) is the kernel's __print_flags mapping for these bits.
#[macro_export] macro_rules! show_inode_state { ($state:expr) => { $state }; }

// Tracepoint declarations are represented as external framework metadata.
#[macro_export] macro_rules! declare_event_class { ($name:ident $(, $rest:tt)*) => { pub const $name: &str = stringify!($name); }; }
#[macro_export] macro_rules! define_event { ($template:ident, $name:ident $(, $rest:tt)*) => { pub const $name: &str = stringify!($name); }; }
#[macro_export] macro_rules! trace_event { ($name:ident $(, $rest:tt)*) => { pub const $name: &str = stringify!($name); }; }

declare_event_class!(writeback_folio_template);
define_event!(writeback_folio_template, writeback_dirty_folio);
define_event!(writeback_folio_template, folio_wait_writeback);
declare_event_class!(writeback_dirty_inode_template);
define_event!(writeback_dirty_inode_template, writeback_mark_inode_dirty);
define_event!(writeback_dirty_inode_template, writeback_dirty_inode_start);
define_event!(writeback_dirty_inode_template, writeback_dirty_inode);
declare_event_class!(writeback_write_inode_template);
define_event!(writeback_write_inode_template, writeback_write_inode_start);
define_event!(writeback_write_inode_template, writeback_write_inode);
declare_event_class!(writeback_work_class);
define_event!(writeback_work_class, writeback_queue);
define_event!(writeback_work_class, writeback_exec);
define_event!(writeback_work_class, writeback_start);
define_event!(writeback_work_class, writeback_written);
define_event!(writeback_work_class, writeback_wait);
trace_event!(writeback_pages_written);
declare_event_class!(writeback_class);
define_event!(writeback_class, writeback_wake_background);
trace_event!(writeback_bdi_register);
declare_event_class!(wbc_class);
define_event!(wbc_class, wbc_writepage);
trace_event!(writeback_queue_io);
trace_event!(global_dirty_state);
trace_event!(bdi_dirty_ratelimit);
trace_event!(balance_dirty_pages);
trace_event!(writeback_sb_inodes_requeue);
declare_event_class!(writeback_single_inode_template);
define_event!(writeback_single_inode_template, writeback_single_inode_start);
define_event!(writeback_single_inode_template, writeback_single_inode);
declare_event_class!(writeback_inode_template);
define_event!(writeback_inode_template, writeback_lazytime);
define_event!(writeback_inode_template, writeback_dirty_inode_enqueue);
define_event!(writeback_inode_template, sb_mark_inode_writeback);
define_event!(writeback_inode_template, sb_clear_inode_writeback);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
