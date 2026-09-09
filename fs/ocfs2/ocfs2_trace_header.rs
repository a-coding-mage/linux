/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of ocfs2_trace.h.
 *
 * The C header uses Linux tracepoint metaprogramming.  The tracepoint
 * registration and formatting machinery is supplied by the kernel tracing
 * dependency; these declarations preserve the event interfaces and names
 * without inventing that external implementation.
 */

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_void;

pub type __u64 = u64;
pub type c_char = i8;

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

/* C event classes translated as reusable Rust declaration markers. */
#[repr(C)]
pub struct ocfs2__int { pub num: i32 }
#[repr(C)]
pub struct ocfs2__uint { pub num: u32 }
#[repr(C)]
pub struct ocfs2__ull { pub blkno: u64 }
#[repr(C)]
pub struct ocfs2__pointer { pub pointer: *mut c_void }
#[repr(C)]
pub struct ocfs2__string { pub name: *const c_char }
#[repr(C)]
pub struct ocfs2__int_int { pub value1: i32, pub value2: i32 }
#[repr(C)]
pub struct ocfs2__uint_int { pub value1: u32, pub value2: i32 }
#[repr(C)]
pub struct ocfs2__uint_uint { pub value1: u32, pub value2: u32 }
#[repr(C)]
pub struct ocfs2__ull_uint { pub value1: u64, pub value2: u32 }
#[repr(C)]
pub struct ocfs2__ull_int { pub value1: u64, pub value2: i32 }
#[repr(C)]
pub struct ocfs2__ull_ull { pub value1: u64, pub value2: u64 }
#[repr(C)]
pub struct ocfs2__ull_ull_uint { pub value1: u64, pub value2: u64, pub value3: u32 }
#[repr(C)]
pub struct ocfs2__ull_uint_uint { pub value1: u64, pub value2: u32, pub value3: u32 }
#[repr(C)]
pub struct ocfs2__uint_uint_uint { pub value1: u32, pub value2: u32, pub value3: u32 }
#[repr(C)]
pub struct ocfs2__ull_ull_ull { pub value1: u64, pub value2: u64, pub value3: u64 }
#[repr(C)]
pub struct ocfs2__ull_int_int_int { pub ull: u64, pub value1: i32, pub value2: i32, pub value3: i32 }
#[repr(C)]
pub struct ocfs2__ull_uint_uint_uint { pub ull: u64, pub value1: u32, pub value2: u32, pub value3: u32 }
#[repr(C)]
pub struct ocfs2__ull_ull_uint_uint { pub value1: u64, pub value2: u64, pub value3: u32, pub value4: u32 }
#[repr(C)]
pub struct ocfs2__truncate_log_ops { pub blkno: u64, pub index: i32, pub start: u32, pub num: u32 }
#[repr(C)]
pub struct ocfs2__refcount_tree_ops { pub blkno: u64, pub index: i32, pub cpos: u64, pub clusters: u32, pub refcount: u32 }
#[repr(C)]
pub struct ocfs2__get_block { pub ino: u64, pub iblock: u64, pub bh_result: *mut c_void, pub create: i32 }
#[repr(C)]
pub struct ocfs2__file_ops { pub inode: *mut c_void, pub file: *mut c_void, pub dentry: *mut c_void, pub ino: u64, pub d_len: u32, pub d_name: *const u8, pub para: u64 }
#[repr(C)]
pub struct ocfs2__xattr_find { pub ino: u64, pub name: *const c_char, pub name_index: i32, pub hash: u32, pub location: u64, pub xe_index: i32 }
#[repr(C)]
pub struct ocfs2__dentry_ops { pub dir: *mut c_void, pub dentry: *mut c_void, pub name_len: i32, pub name: *const c_char, pub dir_blkno: u64, pub extra: u64 }

macro_rules! define_trace_event {
    ($name:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $name;
    };
}


define_trace_event!(ocfs2_adjust_rightmost_branch);
define_trace_event!(ocfs2_rotate_tree_right);
define_trace_event!(ocfs2_append_rec_to_path);
define_trace_event!(ocfs2_insert_extent_start);
define_trace_event!(ocfs2_add_clusters_in_btree);
define_trace_event!(ocfs2_num_free_extents);
define_trace_event!(ocfs2_complete_edge_insert);
define_trace_event!(ocfs2_grow_tree);
define_trace_event!(ocfs2_rotate_subtree);
define_trace_event!(ocfs2_insert_extent);
define_trace_event!(ocfs2_split_extent);
define_trace_event!(ocfs2_remove_extent);
define_trace_event!(ocfs2_commit_truncate);
define_trace_event!(ocfs2_validate_extent_block);
define_trace_event!(ocfs2_rotate_leaf);
define_trace_event!(ocfs2_add_clusters_in_btree_ret);
define_trace_event!(ocfs2_mark_extent_written);
define_trace_event!(ocfs2_truncate_log_append);
define_trace_event!(ocfs2_replay_truncate_records);
define_trace_event!(ocfs2_flush_truncate_log);
define_trace_event!(ocfs2_begin_truncate_log_recovery);
define_trace_event!(ocfs2_truncate_log_recovery_num);
define_trace_event!(ocfs2_complete_truncate_log_recovery);
define_trace_event!(ocfs2_free_cached_blocks);
define_trace_event!(ocfs2_cache_cluster_dealloc);
define_trace_event!(ocfs2_run_deallocs);
define_trace_event!(ocfs2_cache_block_dealloc);
define_trace_event!(ocfs2_trim_extent);
define_trace_event!(ocfs2_trim_group);
define_trace_event!(ocfs2_trim_mainbm);
define_trace_event!(ocfs2_trim_fs);
define_trace_event!(ocfs2_la_set_sizes);
define_trace_event!(ocfs2_alloc_should_use_local);
define_trace_event!(ocfs2_load_local_alloc);
define_trace_event!(ocfs2_begin_local_alloc_recovery);
define_trace_event!(ocfs2_reserve_local_alloc_bits);
define_trace_event!(ocfs2_local_alloc_count_bits);
define_trace_event!(ocfs2_local_alloc_find_clear_bits_search_bitmap);
define_trace_event!(ocfs2_local_alloc_find_clear_bits);
define_trace_event!(ocfs2_sync_local_to_main);
define_trace_event!(ocfs2_sync_local_to_main_free);
define_trace_event!(ocfs2_local_alloc_new_window);
define_trace_event!(ocfs2_local_alloc_new_window_result);
define_trace_event!(ocfs2_update_last_group_and_inode);
define_trace_event!(ocfs2_group_extend);
define_trace_event!(ocfs2_group_add);
define_trace_event!(ocfs2_validate_group_descriptor);
define_trace_event!(ocfs2_block_group_alloc_contig);
define_trace_event!(ocfs2_block_group_alloc_discontig);
define_trace_event!(ocfs2_block_group_alloc);
define_trace_event!(ocfs2_reserve_suballoc_bits_nospc);
define_trace_event!(ocfs2_reserve_suballoc_bits_no_new_group);
define_trace_event!(ocfs2_reserve_new_inode_new_group);
define_trace_event!(ocfs2_block_group_set_bits);
define_trace_event!(ocfs2_relink_block_group);
define_trace_event!(ocfs2_cluster_group_search_wrong_max_bits);
define_trace_event!(ocfs2_cluster_group_search_max_block);
define_trace_event!(ocfs2_block_group_search_max_block);
define_trace_event!(ocfs2_search_chain_begin);
define_trace_event!(ocfs2_search_chain_succ);
define_trace_event!(ocfs2_search_chain_end);
define_trace_event!(ocfs2_claim_suballoc_bits);
define_trace_event!(ocfs2_claim_new_inode_at_loc);
define_trace_event!(ocfs2_block_group_clear_bits);
define_trace_event!(ocfs2_free_suballoc_bits);
define_trace_event!(ocfs2_free_clusters);
define_trace_event!(ocfs2_get_suballoc_slot_bit);
define_trace_event!(ocfs2_test_suballoc_bit);
define_trace_event!(ocfs2_test_inode_bit);
define_trace_event!(ocfs2_validate_refcount_block);
define_trace_event!(ocfs2_purge_refcount_trees);
define_trace_event!(ocfs2_create_refcount_tree);
define_trace_event!(ocfs2_create_refcount_tree_blkno);
define_trace_event!(ocfs2_change_refcount_rec);
define_trace_event!(ocfs2_expand_inline_ref_root);
define_trace_event!(ocfs2_divide_leaf_refcount_block);
define_trace_event!(ocfs2_new_leaf_refcount_block);
define_trace_event!(ocfs2_insert_refcount_rec);
define_trace_event!(ocfs2_split_refcount_rec);
define_trace_event!(ocfs2_split_refcount_rec_insert);
define_trace_event!(ocfs2_increase_refcount_begin);
define_trace_event!(ocfs2_increase_refcount_change);
define_trace_event!(ocfs2_increase_refcount_insert);
define_trace_event!(ocfs2_increase_refcount_split);
define_trace_event!(ocfs2_remove_refcount_extent);
define_trace_event!(ocfs2_restore_refcount_block);
define_trace_event!(ocfs2_decrease_refcount_rec);
define_trace_event!(ocfs2_decrease_refcount);
define_trace_event!(ocfs2_mark_extent_refcounted);
define_trace_event!(ocfs2_calc_refcount_meta_credits);
define_trace_event!(ocfs2_calc_refcount_meta_credits_iterate);
define_trace_event!(ocfs2_add_refcount_flag);
define_trace_event!(ocfs2_prepare_refcount_change_for_del);
define_trace_event!(ocfs2_lock_refcount_allocators);
define_trace_event!(ocfs2_duplicate_clusters_by_page);
define_trace_event!(ocfs2_duplicate_clusters_by_jbd);
define_trace_event!(ocfs2_clear_ext_refcount);
define_trace_event!(ocfs2_replace_clusters);
define_trace_event!(ocfs2_make_clusters_writable);
define_trace_event!(ocfs2_refcount_cow_hunk);
define_trace_event!(ocfs2_symlink_get_block);
define_trace_event!(ocfs2_get_block);
define_trace_event!(ocfs2_get_block_end);
define_trace_event!(ocfs2_readpage);
define_trace_event!(ocfs2_bmap);
define_trace_event!(ocfs2_try_to_write_inline_data);
define_trace_event!(ocfs2_write_begin_nolock);
define_trace_event!(ocfs2_write_end_inline);
define_trace_event!(ocfs2_fault);
define_trace_event!(ocfs2_file_open);
define_trace_event!(ocfs2_file_release);
define_trace_event!(ocfs2_sync_file);
define_trace_event!(ocfs2_file_write_iter);
define_trace_event!(ocfs2_file_read_iter);
define_trace_event!(ocfs2_file_splice_read);
define_trace_event!(ocfs2_truncate_file);
define_trace_event!(ocfs2_truncate_file_error);
define_trace_event!(ocfs2_extend_allocation);
define_trace_event!(ocfs2_extend_allocation_end);
define_trace_event!(ocfs2_write_zero_page);
define_trace_event!(ocfs2_zero_extend_range);
define_trace_event!(ocfs2_zero_extend);
define_trace_event!(ocfs2_setattr);
define_trace_event!(ocfs2_write_remove_suid);
define_trace_event!(ocfs2_zero_partial_clusters);
define_trace_event!(ocfs2_zero_partial_clusters_range1);
define_trace_event!(ocfs2_zero_partial_clusters_range2);
define_trace_event!(ocfs2_remove_inode_range);
define_trace_event!(ocfs2_prepare_inode_for_write);
define_trace_event!(generic_file_read_iter_ret);
define_trace_event!(filemap_splice_read_ret);
define_trace_event!(ocfs2_iget_begin);
define_trace_event!(ocfs2_iget5_locked);
define_trace_event!(ocfs2_iget_end);
define_trace_event!(ocfs2_find_actor);
define_trace_event!(ocfs2_populate_inode);
define_trace_event!(ocfs2_read_locked_inode);
define_trace_event!(ocfs2_check_orphan_recovery_state);
define_trace_event!(ocfs2_validate_inode_block);
define_trace_event!(ocfs2_filecheck_validate_inode_block);
define_trace_event!(ocfs2_filecheck_repair_inode_block);
define_trace_event!(ocfs2_inode_is_valid_to_delete);
define_trace_event!(ocfs2_query_inode_wipe_begin);
define_trace_event!(ocfs2_query_inode_wipe_succ);
define_trace_event!(ocfs2_query_inode_wipe_end);
define_trace_event!(ocfs2_cleanup_delete_inode);
define_trace_event!(ocfs2_delete_inode);
define_trace_event!(ocfs2_clear_inode);
define_trace_event!(ocfs2_inode_revalidate);
define_trace_event!(ocfs2_mark_inode_dirty);
define_trace_event!(ocfs2_read_virt_blocks);
define_trace_event!(ocfs2_refresh_slot_info);
define_trace_event!(ocfs2_map_slot_buffers);
define_trace_event!(ocfs2_map_slot_buffers_block);
define_trace_event!(ocfs2_find_slot);
define_trace_event!(ocfs2_do_node_down);
define_trace_event!(ocfs2_remount);
define_trace_event!(ocfs2_fill_super);
define_trace_event!(ocfs2_parse_options);
define_trace_event!(ocfs2_put_super);
define_trace_event!(ocfs2_statfs);
define_trace_event!(ocfs2_dismount_volume);
define_trace_event!(ocfs2_initialize_super);
define_trace_event!(ocfs2_validate_xattr_block);
define_trace_event!(ocfs2_xattr_extend_allocation);
define_trace_event!(ocfs2_init_xattr_set_ctxt);
define_trace_event!(ocfs2_xattr_bucket_find);
define_trace_event!(ocfs2_xattr_index_block_find);
define_trace_event!(ocfs2_xattr_index_block_find_rec);
define_trace_event!(ocfs2_iterate_xattr_buckets);
define_trace_event!(ocfs2_iterate_xattr_bucket);
define_trace_event!(ocfs2_cp_xattr_block_to_bucket_begin);
define_trace_event!(ocfs2_cp_xattr_block_to_bucket_end);
define_trace_event!(ocfs2_xattr_create_index_block_begin);
define_trace_event!(ocfs2_xattr_create_index_block);
define_trace_event!(ocfs2_defrag_xattr_bucket);
define_trace_event!(ocfs2_mv_xattr_bucket_cross_cluster);
define_trace_event!(ocfs2_divide_xattr_bucket_begin);
define_trace_event!(ocfs2_divide_xattr_bucket_move);
define_trace_event!(ocfs2_cp_xattr_bucket);
define_trace_event!(ocfs2_mv_xattr_buckets);
define_trace_event!(ocfs2_adjust_xattr_cross_cluster);
define_trace_event!(ocfs2_add_new_xattr_cluster_begin);
define_trace_event!(ocfs2_add_new_xattr_cluster);
define_trace_event!(ocfs2_add_new_xattr_cluster_insert);
define_trace_event!(ocfs2_extend_xattr_bucket);
define_trace_event!(ocfs2_add_new_xattr_bucket);
define_trace_event!(ocfs2_xattr_bucket_value_truncate);
define_trace_event!(ocfs2_rm_xattr_cluster);
define_trace_event!(ocfs2_reflink_xattr_header);
define_trace_event!(ocfs2_create_empty_xattr_block);
define_trace_event!(ocfs2_xattr_set_entry_bucket);
define_trace_event!(ocfs2_xattr_set_entry_index_block);
define_trace_event!(ocfs2_xattr_bucket_value_refcount);
define_trace_event!(ocfs2_reflink_xattr_buckets);
define_trace_event!(ocfs2_reflink_xattr_rec);
define_trace_event!(ocfs2_resv_insert);
define_trace_event!(ocfs2_resmap_find_free_bits_begin);
define_trace_event!(ocfs2_resmap_find_free_bits_end);
define_trace_event!(ocfs2_resv_find_window_begin);
define_trace_event!(ocfs2_resv_find_window_prev);
define_trace_event!(ocfs2_resv_find_window_next);
define_trace_event!(ocfs2_cannibalize_resv_begin);
define_trace_event!(ocfs2_cannibalize_resv_end);
define_trace_event!(ocfs2_resmap_resv_bits);
define_trace_event!(ocfs2_resmap_claimed_bits_begin);
define_trace_event!(ocfs2_resmap_claimed_bits_end);
define_trace_event!(ocfs2_recover_local_quota_file);
define_trace_event!(ocfs2_finish_quota_recovery);
define_trace_event!(olq_set_dquot);
define_trace_event!(ocfs2_validate_quota_block);
define_trace_event!(ocfs2_sync_dquot);
define_trace_event!(ocfs2_sync_dquot_helper);
define_trace_event!(ocfs2_write_dquot);
define_trace_event!(ocfs2_release_dquot);
define_trace_event!(ocfs2_acquire_dquot);
define_trace_event!(ocfs2_get_next_id);
define_trace_event!(ocfs2_mark_dquot_dirty);
define_trace_event!(ocfs2_search_dirblock);
define_trace_event!(ocfs2_validate_dir_block);
define_trace_event!(ocfs2_find_entry_el);
define_trace_event!(ocfs2_dx_dir_search);
define_trace_event!(ocfs2_dx_dir_search_leaf_info);
define_trace_event!(ocfs2_delete_entry_dx);
define_trace_event!(ocfs2_readdir);
define_trace_event!(ocfs2_find_files_on_disk);
define_trace_event!(ocfs2_check_dir_for_entry);
define_trace_event!(ocfs2_dx_dir_attach_index);
define_trace_event!(ocfs2_dx_dir_format_cluster);
define_trace_event!(ocfs2_dx_dir_index_root_block);
define_trace_event!(ocfs2_extend_dir);
define_trace_event!(ocfs2_dx_dir_rebalance);
define_trace_event!(ocfs2_dx_dir_rebalance_split);
define_trace_event!(ocfs2_prepare_dir_for_insert);
define_trace_event!(ocfs2_lookup);
define_trace_event!(ocfs2_mkdir);
define_trace_event!(ocfs2_create);
define_trace_event!(ocfs2_unlink);
define_trace_event!(ocfs2_symlink_create);
define_trace_event!(ocfs2_mv_orphaned_inode_to_new);
define_trace_event!(ocfs2_lookup_ret);
define_trace_event!(ocfs2_mknod);
define_trace_event!(ocfs2_link);
define_trace_event!(ocfs2_unlink_noent);
define_trace_event!(ocfs2_double_lock);
define_trace_event!(ocfs2_double_lock_end);
define_trace_event!(ocfs2_rename);
define_trace_event!(ocfs2_rename_not_permitted);
define_trace_event!(ocfs2_rename_target_exists);
define_trace_event!(ocfs2_rename_disagree);
define_trace_event!(ocfs2_rename_over_existing);
define_trace_event!(ocfs2_create_symlink_data);
define_trace_event!(ocfs2_symlink_begin);
define_trace_event!(ocfs2_blkno_stringify);
define_trace_event!(ocfs2_orphan_add_begin);
define_trace_event!(ocfs2_orphan_add_end);
define_trace_event!(ocfs2_orphan_del);
define_trace_event!(ocfs2_dentry_revalidate);
define_trace_event!(ocfs2_dentry_revalidate_negative);
define_trace_event!(ocfs2_dentry_revalidate_delete);
define_trace_event!(ocfs2_dentry_revalidate_orphaned);
define_trace_event!(ocfs2_dentry_revalidate_nofsdata);
define_trace_event!(ocfs2_dentry_revalidate_ret);
define_trace_event!(ocfs2_find_local_alias);
define_trace_event!(ocfs2_dentry_attach_lock);
define_trace_event!(ocfs2_dentry_attach_lock_found);
define_trace_event!(ocfs2_get_dentry_begin);
define_trace_event!(ocfs2_get_dentry_test_bit);
define_trace_event!(ocfs2_get_dentry_stale);
define_trace_event!(ocfs2_get_dentry_generation);
define_trace_event!(ocfs2_get_dentry_end);
define_trace_event!(ocfs2_get_parent);
define_trace_event!(ocfs2_get_parent_end);
define_trace_event!(ocfs2_encode_fh_begin);
define_trace_event!(ocfs2_encode_fh_self);
define_trace_event!(ocfs2_encode_fh_parent);
define_trace_event!(ocfs2_encode_fh_type);
define_trace_event!(ocfs2_commit_cache_begin);
define_trace_event!(ocfs2_commit_cache_end);
define_trace_event!(ocfs2_extend_trans);
define_trace_event!(ocfs2_assure_trans_credits);
define_trace_event!(ocfs2_extend_trans_restart);
define_trace_event!(ocfs2_allocate_extend_trans);
define_trace_event!(ocfs2_journal_access);
define_trace_event!(ocfs2_journal_dirty);
define_trace_event!(ocfs2_journal_init);
define_trace_event!(ocfs2_journal_init_maxlen);
define_trace_event!(ocfs2_journal_shutdown);
define_trace_event!(ocfs2_journal_shutdown_wait);
define_trace_event!(ocfs2_complete_recovery);
define_trace_event!(ocfs2_complete_recovery_end);
define_trace_event!(ocfs2_complete_recovery_slot);
define_trace_event!(ocfs2_recovery_thread_node);
define_trace_event!(ocfs2_recovery_thread_end);
define_trace_event!(ocfs2_recovery_thread);
define_trace_event!(ocfs2_replay_journal_recovered);
define_trace_event!(ocfs2_replay_journal_lock_err);
define_trace_event!(ocfs2_replay_journal_skip);
define_trace_event!(ocfs2_recover_node);
define_trace_event!(ocfs2_recover_node_skip);
define_trace_event!(ocfs2_mark_dead_nodes);
define_trace_event!(ocfs2_queue_orphan_scan_begin);
define_trace_event!(ocfs2_queue_orphan_scan_end);
define_trace_event!(ocfs2_orphan_filldir);
define_trace_event!(ocfs2_recover_orphans);
define_trace_event!(ocfs2_recover_orphans_iput);
define_trace_event!(ocfs2_wait_on_mount);
define_trace_event!(ocfs2_read_blocks_sync);
define_trace_event!(ocfs2_read_blocks_sync_jbd);
define_trace_event!(ocfs2_read_blocks_from_disk);
define_trace_event!(ocfs2_read_blocks_bh);
define_trace_event!(ocfs2_read_blocks_end);
define_trace_event!(ocfs2_write_block);
define_trace_event!(ocfs2_read_blocks_begin);
define_trace_event!(ocfs2_purge_copied_metadata_tree);
define_trace_event!(ocfs2_metadata_cache_purge);
define_trace_event!(ocfs2_buffer_cached_begin);
define_trace_event!(ocfs2_buffer_cached_end);
define_trace_event!(ocfs2_append_cache_array);
define_trace_event!(ocfs2_insert_cache_tree);
define_trace_event!(ocfs2_expand_cache);
define_trace_event!(ocfs2_set_buffer_uptodate);
define_trace_event!(ocfs2_set_buffer_uptodate_begin);
define_trace_event!(ocfs2_remove_metadata_array);
define_trace_event!(ocfs2_remove_metadata_tree);
define_trace_event!(ocfs2_remove_block_from_cache);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
