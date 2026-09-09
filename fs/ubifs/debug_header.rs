/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2006-2008 Nokia Corporation.
 *
 * Authors: Artem Bityutskiy (Битюцкий Артём)
 *          Adrian Hunter
 */

// Declarations supplied by the surrounding UBIFS/kernel translation.

pub type DbgLeafCallback = unsafe extern "C" fn(*mut ubifs_info, *mut ubifs_zbranch, *mut core::ffi::c_void) -> i32;
pub type DbgZnodeCallback = unsafe extern "C" fn(*mut ubifs_info, *mut ubifs_znode, *mut core::ffi::c_void) -> i32;

pub const UBIFS_DFS_DIR_NAME: &str = "ubi%d_%d";
pub const UBIFS_DFS_DIR_LEN: usize = 3 + 1 + 2 + 3 + 1;

#[repr(C)]
pub struct ubifs_debug_info {
    pub old_zroot: ubifs_zbranch,
    pub old_zroot_level: i32,
    pub old_zroot_sqnum: u64,
    pub pc_happened: i32,
    pub pc_delay: i32,
    pub pc_timeout: c_ulong,
    pub pc_cnt: u32,
    pub pc_cnt_max: u32,
    pub chk_lpt_sz: i64,
    pub chk_lpt_sz2: i64,
    pub chk_lpt_wastage: i64,
    pub chk_lpt_lebs: i32,
    pub new_nhead_offs: i32,
    pub new_ihead_lnum: i32,
    pub new_ihead_offs: i32,
    pub saved_lst: ubifs_lp_stats,
    pub saved_bi: ubifs_budg_info,
    pub saved_free: i64,
    pub saved_idx_gc_cnt: i32,
    // C bit-fields, each one bit wide.
    pub chk_gen: u32,
    pub chk_index: u32,
    pub chk_orph: u32,
    pub chk_lprops: u32,
    pub chk_fs: u32,
    pub tst_rcvry: u32,
    pub dfs_dir_name: [c_char; UBIFS_DFS_DIR_LEN],
    pub dfs_dir: *mut dentry,
    pub dfs_dump_lprops: *mut dentry,
    pub dfs_dump_budg: *mut dentry,
    pub dfs_dump_tnc: *mut dentry,
    pub dfs_chk_gen: *mut dentry,
    pub dfs_chk_index: *mut dentry,
    pub dfs_chk_orph: *mut dentry,
    pub dfs_chk_lprops: *mut dentry,
    pub dfs_chk_fs: *mut dentry,
    pub dfs_tst_rcvry: *mut dentry,
    pub dfs_ro_error: *mut dentry,
}

#[repr(C)]
pub struct ubifs_global_debug_info {
    pub chk_gen: u32,
    pub chk_index: u32,
    pub chk_orph: u32,
    pub chk_lprops: u32,
    pub chk_fs: u32,
    pub tst_rcvry: u32,
}

unsafe extern "C" {
    pub fn ubifs_assert_failed(c: *mut ubifs_info, expr: *const c_char, file: *const c_char, line: i32);
    pub static mut ubifs_dbg: ubifs_global_debug_info;
}

// The following C preprocessor debugging macros are represented as Rust macros
// so callers retain their source-level interfaces and variadic formatting behavior.
#[macro_export]
macro_rules! ubifs_assert { ($c:expr, $expr:expr) => { if !$expr { unsafe { ubifs_assert_failed($c as *mut ubifs_info, stringify!($expr).as_ptr() as *const c_char, file!().as_ptr() as *const c_char, line!() as i32); } } }; }
#[macro_export]
macro_rules! ubifs_dbg_msg { ($type:expr, $($arg:tt)*) => { pr_debug!(concat!("UBIFS DBG ", $type, " (pid %d): ", $($arg)*, "\n"), current().pid); }; }
pub const DBG_KEY_BUF_LEN: usize = 48;
pub const UBIFS_DBG_GEN: &str = "gen";
pub const UBIFS_DBG_JNL: &str = "jnl";
pub const UBIFS_DBG_TNC: &str = "tnc";
pub const UBIFS_DBG_LP: &str = "lp";
pub const UBIFS_DBG_FIND: &str = "find";
pub const UBIFS_DBG_MNT: &str = "mnt";
pub const UBIFS_DBG_IO: &str = "io";
pub const UBIFS_DBG_CMT: &str = "cmt";
pub const UBIFS_DBG_BUDG: &str = "budg";
pub const UBIFS_DBG_LOG: &str = "log";
pub const UBIFS_DBG_GC: &str = "gc";
pub const UBIFS_DBG_SCAN: &str = "scan";
pub const UBIFS_DBG_RCVRY: &str = "rcvry";

macro_rules! dbg_check { ($name:ident, $field:ident) => {
    pub unsafe fn $name(c: *const ubifs_info) -> i32 { (ubifs_dbg.$field != 0 || (*c).dbg.$field != 0) as i32 }
}; }
dbg_check!(dbg_is_chk_gen, chk_gen);
dbg_check!(dbg_is_chk_index, chk_index);
dbg_check!(dbg_is_chk_orph, chk_orph);
dbg_check!(dbg_is_chk_lprops, chk_lprops);
dbg_check!(dbg_is_chk_fs, chk_fs);
dbg_check!(dbg_is_tst_rcvry, tst_rcvry);
pub unsafe fn dbg_is_power_cut(c: *const ubifs_info) -> i32 { ((*c).dbg.pc_happened != 0) as i32 }

unsafe extern "C" {
    pub fn ubifs_debugging_init(c: *mut ubifs_info) -> i32;
    pub fn ubifs_debugging_exit(c: *mut ubifs_info);
    pub fn dbg_ntype(type_: i32) -> *const c_char;
    pub fn dbg_cstate(cmt_state: i32) -> *const c_char;
    pub fn dbg_jhead(jhead: i32) -> *const c_char;
    pub fn dbg_get_key_dump(c: *const ubifs_info, key: *const ubifs_key) -> *const c_char;
    pub fn dbg_snprintf_key(c: *const ubifs_info, key: *const ubifs_key, buffer: *mut c_char, len: i32) -> *const c_char;
    pub fn ubifs_dump_inode(c: *mut ubifs_info, inode: *const inode);
    pub fn ubifs_dump_node(c: *const ubifs_info, node: *const core::ffi::c_void, node_len: i32);
    pub fn ubifs_dump_budget_req(req: *const ubifs_budget_req);
    pub fn ubifs_dump_lstats(lst: *const ubifs_lp_stats);
    pub fn ubifs_dump_budg(c: *mut ubifs_info, bi: *const ubifs_budg_info);
    pub fn ubifs_dump_lprop(c: *const ubifs_info, lp: *const ubifs_lprops);
    pub fn ubifs_dump_lprops(c: *mut ubifs_info);
    pub fn ubifs_dump_lpt_info(c: *mut ubifs_info);
    pub fn ubifs_dump_leb(c: *const ubifs_info, lnum: i32);
    pub fn ubifs_dump_znode(c: *const ubifs_info, znode: *const ubifs_znode);
    pub fn ubifs_dump_heap(c: *mut ubifs_info, heap: *mut ubifs_lpt_heap, cat: i32);
    pub fn ubifs_dump_pnode(c: *mut ubifs_info, pnode: *mut ubifs_pnode, parent: *mut ubifs_nnode, iip: i32);
    pub fn ubifs_dump_tnc(c: *mut ubifs_info);
    pub fn ubifs_dump_index(c: *mut ubifs_info);
    pub fn ubifs_dump_lpt_lebs(c: *const ubifs_info);
    pub fn dbg_walk_index(c: *mut ubifs_info, leaf_cb: DbgLeafCallback, znode_cb: DbgZnodeCallback, priv_: *mut core::ffi::c_void) -> i32;
    pub fn dbg_save_space_info(c: *mut ubifs_info);
    pub fn dbg_check_space_info(c: *mut ubifs_info) -> i32;
    pub fn dbg_check_lprops(c: *mut ubifs_info) -> i32;
    pub fn dbg_old_index_check_init(c: *mut ubifs_info, zroot: *mut ubifs_zbranch) -> i32;
    pub fn dbg_check_old_index(c: *mut ubifs_info, zroot: *mut ubifs_zbranch) -> i32;
    pub fn dbg_check_cats(c: *mut ubifs_info) -> i32;
    pub fn dbg_check_ltab(c: *mut ubifs_info) -> i32;
    pub fn dbg_chk_lpt_free_spc(c: *mut ubifs_info) -> i32;
    pub fn dbg_chk_lpt_sz(c: *mut ubifs_info, action: i32, len: i32) -> i32;
    pub fn dbg_check_synced_i_size(c: *const ubifs_info, inode: *mut inode) -> i32;
    pub fn dbg_check_dir(c: *mut ubifs_info, dir: *const inode) -> i32;
    pub fn dbg_check_tnc(c: *mut ubifs_info, extra: i32) -> i32;
    pub fn dbg_check_idx_size(c: *mut ubifs_info, idx_size: i64) -> i32;
    pub fn dbg_check_filesystem(c: *mut ubifs_info) -> i32;
    pub fn dbg_check_heap(c: *mut ubifs_info, heap: *mut ubifs_lpt_heap, cat: i32, add_pos: i32);
    pub fn dbg_check_lpt_nodes(c: *mut ubifs_info, cnode: *mut ubifs_cnode, row: i32, col: i32) -> i32;
    pub fn dbg_check_inode_size(c: *mut ubifs_info, inode: *const inode, size: loff_t) -> i32;
    pub fn dbg_check_data_nodes_order(c: *mut ubifs_info, head: *mut list_head) -> i32;
    pub fn dbg_check_nondata_nodes_order(c: *mut ubifs_info, head: *mut list_head) -> i32;
    pub fn dbg_leb_write(c: *mut ubifs_info, lnum: i32, buf: *const core::ffi::c_void, offs: i32, len: i32) -> i32;
    pub fn dbg_leb_change(c: *mut ubifs_info, lnum: i32, buf: *const core::ffi::c_void, len: i32) -> i32;
    pub fn dbg_leb_unmap(c: *mut ubifs_info, lnum: i32) -> i32;
    pub fn dbg_leb_map(c: *mut ubifs_info, lnum: i32) -> i32;
    pub fn dbg_debugfs_init();
    pub fn dbg_debugfs_exit();
    pub fn dbg_debugfs_init_fs(c: *mut ubifs_info);
    pub fn dbg_debugfs_exit_fs(c: *mut ubifs_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
