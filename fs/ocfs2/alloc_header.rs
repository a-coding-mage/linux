/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * alloc.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

/* For xattr tree leaf, we limit the leaf byte size to be 64K. */
pub const OCFS2_MAX_XATTR_TREE_LEAF_SIZE: u32 = 65536;

#[repr(C)]
pub struct ocfs2_extent_tree {
    pub et_ops: *const ocfs2_extent_tree_operations,
    pub et_root_bh: *mut buffer_head,
    pub et_root_el: *mut ocfs2_extent_list,
    pub et_ci: *mut ocfs2_caching_info,
    pub et_root_journal_access: ocfs2_journal_access_func,
    pub et_object: *mut core::ffi::c_void,
    pub et_max_leaf_clusters: u32,
    pub et_dealloc: *mut ocfs2_cached_dealloc_ctxt,
}

#[repr(C)]
pub struct ocfs2_extent_tree_operations {
    _private: [u8; 0],
}

pub type ocfs2_journal_access_func = unsafe extern "C" fn();

#[repr(C)]
pub struct ocfs2_cached_dealloc_ctxt {
    pub c_first_suballocator: *mut ocfs2_per_slot_free_list,
    pub c_global_allocator: *mut ocfs2_cached_block_free,
}

#[repr(C)]
pub struct ocfs2_truncate_context {
    pub tc_dealloc: ocfs2_cached_dealloc_ctxt,
    pub tc_ext_alloc_locked: i32, /* is it cluster locked? */
    /* these get destroyed once it's passed to ocfs2_commit_truncate. */
    pub tc_last_eb_bh: *mut buffer_head,
}

#[repr(C)]
pub struct ocfs2_path_item {
    pub bh: *mut buffer_head,
    pub el: *mut ocfs2_extent_list,
}

pub const OCFS2_MAX_PATH_DEPTH: usize = 5;

#[repr(C)]
pub struct ocfs2_path {
    pub p_tree_depth: i32,
    pub p_root_access: ocfs2_journal_access_func,
    pub p_node: [ocfs2_path_item; OCFS2_MAX_PATH_DEPTH],
}

#[repr(C)]
pub struct ocfs2_extent_rec {
    pub e_int_clusters: u32,
    pub e_leaf_clusters: u16,
}

#[repr(C)]
pub struct ocfs2_extent_list {
    pub l_tree_depth: u16,
}

pub const RESTART_NONE: ocfs2_alloc_restarted = ocfs2_alloc_restarted::RESTART_NONE;
pub const RESTART_TRANS: ocfs2_alloc_restarted = ocfs2_alloc_restarted::RESTART_TRANS;
pub const RESTART_META: ocfs2_alloc_restarted = ocfs2_alloc_restarted::RESTART_META;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ocfs2_alloc_restarted {
    RESTART_NONE = 0,
    RESTART_TRANS,
    RESTART_META,
}

#[inline]
pub unsafe fn ocfs2_extend_meta_needed(root_el: *mut ocfs2_extent_list) -> i32 {
    le16_to_cpu((*root_el).l_tree_depth) as i32 + 2
}

#[inline]
pub unsafe fn ocfs2_init_dealloc_ctxt(c: *mut ocfs2_cached_dealloc_ctxt) {
    (*c).c_first_suballocator = core::ptr::null_mut();
    (*c).c_global_allocator = core::ptr::null_mut();
}

#[inline]
pub unsafe fn ocfs2_dealloc_has_cluster(c: *mut ocfs2_cached_dealloc_ctxt) -> i32 {
    ((*c).c_global_allocator != core::ptr::null_mut()) as i32
}

#[inline]
pub unsafe fn ocfs2_rec_clusters(
    el: *mut ocfs2_extent_list,
    rec: *mut ocfs2_extent_rec,
) -> u32 {
    if (*el).l_tree_depth != 0 {
        le32_to_cpu((*rec).e_int_clusters)
    } else {
        le16_to_cpu((*rec).e_leaf_clusters) as u32
    }
}

#[inline]
pub unsafe fn ocfs2_is_empty_extent(rec: *mut ocfs2_extent_rec) -> i32 {
    ((*rec).e_leaf_clusters == 0) as i32
}

#[inline]
pub unsafe fn path_root_bh(path: *mut ocfs2_path) -> *mut buffer_head {
    (*path).p_node[0].bh
}
#[inline]
pub unsafe fn path_root_el(path: *mut ocfs2_path) -> *mut ocfs2_extent_list {
    (*path).p_node[0].el
}
#[inline]
pub unsafe fn path_root_access(path: *mut ocfs2_path) -> ocfs2_journal_access_func {
    (*path).p_root_access
}
#[inline]
pub unsafe fn path_leaf_bh(path: *mut ocfs2_path) -> *mut buffer_head {
    (*path).p_node[(*path).p_tree_depth as usize].bh
}
#[inline]
pub unsafe fn path_leaf_el(path: *mut ocfs2_path) -> *mut ocfs2_extent_list {
    (*path).p_node[(*path).p_tree_depth as usize].el
}
#[inline]
pub unsafe fn path_num_items(path: *mut ocfs2_path) -> i32 {
    (*path).p_tree_depth + 1
}

/* External types and functions are supplied by the surrounding translation. */
extern "C" {
    pub fn ocfs2_init_dinode_extent_tree(et: *mut ocfs2_extent_tree, ci: *mut ocfs2_caching_info, bh: *mut buffer_head);
    pub fn ocfs2_init_xattr_tree_extent_tree(et: *mut ocfs2_extent_tree, ci: *mut ocfs2_caching_info, bh: *mut buffer_head);
    pub fn ocfs2_init_xattr_value_extent_tree(et: *mut ocfs2_extent_tree, ci: *mut ocfs2_caching_info, vb: *mut ocfs2_xattr_value_buf);
    pub fn ocfs2_init_dx_root_extent_tree(et: *mut ocfs2_extent_tree, ci: *mut ocfs2_caching_info, bh: *mut buffer_head);
    pub fn ocfs2_init_refcount_extent_tree(et: *mut ocfs2_extent_tree, ci: *mut ocfs2_caching_info, bh: *mut buffer_head);
    pub fn ocfs2_read_extent_block(ci: *mut ocfs2_caching_info, eb_blkno: u64, bh: *mut *mut buffer_head) -> i32;
    pub fn ocfs2_insert_extent(handle: *mut handle_t, et: *mut ocfs2_extent_tree, cpos: u32, start_blk: u64, new_clusters: u32, flags: u8, meta_ac: *mut ocfs2_alloc_context) -> i32;
    pub fn ocfs2_add_clusters_in_btree(handle: *mut handle_t, et: *mut ocfs2_extent_tree, logical_offset: *mut u32, clusters_to_add: u32, mark_unwritten: i32, data_ac: *mut ocfs2_alloc_context, meta_ac: *mut ocfs2_alloc_context, reason_ret: *mut ocfs2_alloc_restarted) -> i32;
    pub fn ocfs2_split_extent(handle: *mut handle_t, et: *mut ocfs2_extent_tree, path: *mut ocfs2_path, split_index: i32, split_rec: *mut ocfs2_extent_rec, meta_ac: *mut ocfs2_alloc_context, dealloc: *mut ocfs2_cached_dealloc_ctxt) -> i32;
    pub fn ocfs2_mark_extent_written(inode: *mut inode, et: *mut ocfs2_extent_tree, handle: *mut handle_t, cpos: u32, len: u32, phys: u32, meta_ac: *mut ocfs2_alloc_context, dealloc: *mut ocfs2_cached_dealloc_ctxt) -> i32;
    pub fn ocfs2_change_extent_flag(handle: *mut handle_t, et: *mut ocfs2_extent_tree, cpos: u32, len: u32, phys: u32, meta_ac: *mut ocfs2_alloc_context, dealloc: *mut ocfs2_cached_dealloc_ctxt, new_flags: i32, clear_flags: i32) -> i32;
    pub fn ocfs2_remove_extent(handle: *mut handle_t, et: *mut ocfs2_extent_tree, cpos: u32, len: u32, meta_ac: *mut ocfs2_alloc_context, dealloc: *mut ocfs2_cached_dealloc_ctxt) -> i32;
    pub fn ocfs2_remove_btree_range(inode: *mut inode, et: *mut ocfs2_extent_tree, cpos: u32, phys_cpos: u32, len: u32, flags: i32, dealloc: *mut ocfs2_cached_dealloc_ctxt, refcount_loc: u64, refcount_tree_locked: bool) -> i32;
    pub fn ocfs2_num_free_extents(et: *mut ocfs2_extent_tree) -> i32;
    pub fn ocfs2_dinode_new_extent_list(inode: *mut inode, di: *mut ocfs2_dinode);
    pub fn ocfs2_set_inode_data_inline(inode: *mut inode, di: *mut ocfs2_dinode);
    pub fn ocfs2_convert_inline_data_to_extents(inode: *mut inode, di_bh: *mut buffer_head) -> i32;
    pub fn ocfs2_truncate_log_init(osb: *mut ocfs2_super) -> i32;
    pub fn ocfs2_truncate_log_shutdown(osb: *mut ocfs2_super);
    pub fn ocfs2_schedule_truncate_log_flush(osb: *mut ocfs2_super, cancel: i32);
    pub fn ocfs2_flush_truncate_log(osb: *mut ocfs2_super) -> i32;
    pub fn ocfs2_begin_truncate_log_recovery(osb: *mut ocfs2_super, slot_num: i32, tl_copy: *mut *mut ocfs2_dinode) -> i32;
    pub fn ocfs2_complete_truncate_log_recovery(osb: *mut ocfs2_super, tl_copy: *mut ocfs2_dinode) -> i32;
    pub fn ocfs2_truncate_log_needs_flush(osb: *mut ocfs2_super) -> i32;
    pub fn ocfs2_truncate_log_append(osb: *mut ocfs2_super, handle: *mut handle_t, start_blk: u64, num_clusters: u32) -> i32;
    pub fn __ocfs2_flush_truncate_log(osb: *mut ocfs2_super) -> i32;
    pub fn ocfs2_try_to_free_truncate_log(osb: *mut ocfs2_super, needed: u32) -> i32;
    pub fn ocfs2_cache_cluster_dealloc(ctxt: *mut ocfs2_cached_dealloc_ctxt, blkno: u64, bit: u32) -> i32;
    pub fn ocfs2_cache_block_dealloc(ctxt: *mut ocfs2_cached_dealloc_ctxt, type_: i32, slot: i32, suballoc: u64, blkno: u64, bit: u32) -> i32;
    pub fn ocfs2_run_deallocs(osb: *mut ocfs2_super, ctxt: *mut ocfs2_cached_dealloc_ctxt) -> i32;
    pub fn ocfs2_zero_range_for_truncate(inode: *mut inode, handle: *mut handle_t, range_start: u64, range_end: u64) -> i32;
    pub fn ocfs2_commit_truncate(osb: *mut ocfs2_super, inode: *mut inode, di_bh: *mut buffer_head) -> i32;
    pub fn ocfs2_truncate_inline(inode: *mut inode, di_bh: *mut buffer_head, start: u32, end: u32, trunc: i32) -> i32;
    pub fn ocfs2_find_leaf(ci: *mut ocfs2_caching_info, root_el: *mut ocfs2_extent_list, cpos: u32, leaf_bh: *mut *mut buffer_head) -> i32;
    pub fn ocfs2_search_extent_list(el: *mut ocfs2_extent_list, v_cluster: u32) -> i32;
    pub fn ocfs2_trim_fs(sb: *mut super_block, range: *mut fstrim_range) -> i32;
    pub fn ocfs2_map_and_dirty_folio(inode: *mut inode, handle: *mut handle_t, from: usize, to: usize, folio: *mut folio, zero: i32, phys: *mut u64);
    pub fn ocfs2_reinit_path(path: *mut ocfs2_path, keep_root: i32);
    pub fn ocfs2_free_path(path: *mut ocfs2_path);
    pub fn ocfs2_find_path(ci: *mut ocfs2_caching_info, path: *mut ocfs2_path, cpos: u32) -> i32;
    pub fn ocfs2_new_path_from_path(path: *mut ocfs2_path) -> *mut ocfs2_path;
    pub fn ocfs2_new_path_from_et(et: *mut ocfs2_extent_tree) -> *mut ocfs2_path;
    pub fn ocfs2_path_bh_journal_access(handle: *mut handle_t, ci: *mut ocfs2_caching_info, path: *mut ocfs2_path, idx: i32) -> i32;
    pub fn ocfs2_journal_access_path(ci: *mut ocfs2_caching_info, handle: *mut handle_t, path: *mut ocfs2_path) -> i32;
    pub fn ocfs2_find_cpos_for_right_leaf(sb: *mut super_block, path: *mut ocfs2_path, cpos: *mut u32) -> i32;
    pub fn ocfs2_find_cpos_for_left_leaf(sb: *mut super_block, path: *mut ocfs2_path, cpos: *mut u32) -> i32;
    pub fn ocfs2_find_subtree_root(et: *mut ocfs2_extent_tree, left: *mut ocfs2_path, right: *mut ocfs2_path) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
