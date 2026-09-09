//! Rust source-level representation of `xfs/scrub/trace.h`.
//!
//! The Linux tracepoint DSL is declarative C preprocessor input rather than
//! executable C.  The declarations below preserve its externally visible
//! event names, conditional families, and argument interfaces for consumers
//! that provide the tracepoint backend.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/// A tracepoint declaration: the backend supplies registration and emission.
#[macro_export]
macro_rules! trace_event { ($name:ident ( $( $arg:ident : $ty:ty ),* $(,)? )) => {
    #[allow(unused_variables)]
    pub unsafe fn $name($( $arg: $ty ),*) { }
}; }

// Dependency types are intentionally opaque; their definitions belong to the
// corresponding XFS translation units.
pub enum xfs_inode {}
pub enum xfs_mount {}
pub enum xfs_scrub {}
pub enum xfs_scrub_metadata {}
pub enum xfs_scrub_vec_head {}
pub enum xfs_scrub_vec {}
pub enum xfs_perag {}
pub enum xfs_group {}
pub enum xfs_btree_cur {}
pub enum xfs_parent_rec {}
pub enum xchk_iscan {}
pub enum xchk_nlink {}
pub enum xchk_dirtree {}
pub enum xchk_dirtree_outcomes {}
pub enum xfile {}
pub enum xfarray_sortinfo {}
pub enum xfs_rmap_update_params {}

// Type aliases retain the integer intent of the tracepoint interfaces.
pub type dev_t = u64;
pub type xfs_ino_t = u64;
pub type xfs_agnumber_t = u32;
pub type xfs_agblock_t = u32;
pub type xfs_agino_t = u32;
pub type xfs_fileoff_t = u64;
pub type xfs_filblks_t = u64;
pub type xfs_fsblock_t = u64;
pub type xfs_extlen_t = u32;
pub type xfs_nlink_t = u32;

// The following declarations correspond one-for-one to TRACE_EVENT and
// DEFINE_EVENT instances in the header.  The trace backend may replace the
// bodies with registration/emission logic.
macro_rules! scrub_events { ($($n:ident),* $(,)?) => { $(
    pub unsafe fn $n() {}
)* }; }

scrub_events!(
    xchk_start, xchk_done, xchk_deadlock_retry, xchk_dirtree_start,
    xchk_dirtree_done, xrep_attempt, xrep_done, xchk_fsgates_enable,
    xchk_fsgates_disable, xchk_scrubv_start, xchk_scrubv_barrier_fail,
    xchk_scrubv_item, xchk_scrubv_outcome, xchk_fs_error, xchk_block_error,
    xchk_block_preen, xchk_ino_error, xchk_ino_preen, xchk_ino_warning,
    xchk_fblock_error, xchk_fblock_warning, xchk_fblock_preen,
    xchk_incomplete, xchk_btree_op_error, xchk_ifork_btree_op_error,
    xchk_btree_error, xchk_ifork_btree_error, xchk_btree_rec, xchk_btree_key,
    xchk_xref_error, xchk_iallocbt_check_cluster, xchk_inode_is_allocated,
    xchk_fscounters_calc, xchk_fscounters_within_range, xchk_fsfreeze,
    xchk_fsthaw, xchk_refcount_incorrect, xfile_create, xfile_destroy,
    xfile_load, xfile_store, xfile_seek_data, xfile_get_folio,
    xfile_put_folio, xfile_discard, xfarray_create, xfarray_isort,
    xfarray_foliosort, xfarray_qsort, xfarray_sort, xfarray_sort_scan,
    xfarray_sort_stats, xchk_iscan_move_cursor, xchk_iscan_visit,
    xchk_iscan_skip, xchk_iscan_advance_ag, xchk_iscan_want_live_update,
    xchk_iscan_start, xchk_iscan_iget, xchk_iscan_iget_batch,
    xchk_iscan_iget_retry_wait, xchk_iscan_agi_retry_wait,
    xchk_nlinks_collect_dirent, xchk_nlinks_collect_pptr,
    xchk_nlinks_collect_metafile, xchk_nlinks_live_update,
    xchk_nlinks_check_zero, xchk_nlinks_update_incore,
    xchk_nlinks_compare_inode, xchk_dir_defer, xchk_dir_slowpath,
    xchk_dir_ultraslowpath, xchk_parent_defer, xchk_parent_slowpath,
    xchk_parent_ultraslowpath, xchk_dirtree_create_path,
    xchk_dirpath_walk_upwards, xchk_dirpath_disappeared, xchk_dirpath_badgen,
    xchk_dirpath_nondir_parent, xchk_dirpath_unlinked_parent,
    xchk_dirpath_found_next_step, xchk_dirpath_crosses_tree,
    xchk_dirpath_set_outcome, xchk_dirpath_evaluate_path,
    xchk_dirtree_evaluate, xchk_dirpath_changed, xchk_dirtree_live_update,
    xchk_metapath_lookup
);

// Online-repair tracepoints from the remainder of the header.
#[cfg(feature = "xfs_online_repair")]
scrub_events!(
    xrep_dirtree_delete_path, xrep_dirtree_create_adoption,
    xrep_dirtree_decided_fate, xrep_metapath_lookup, xrep_metapath_try_unlink,
    xrep_metapath_unlink, xrep_metapath_link, xreap_dispose_unmap_extent,
    xreap_dispose_free_extent, xreap_agextent_binval, xreap_bmapi_binval,
    xrep_agfl_insert, xreap_agextent_limits, xreap_agcow_limits,
    xreap_rgcow_limits, xreap_bmapi_limits, xreap_agextent_select,
    xreap_bmapi_select, xrep_ibt_walk_rmap, xrep_abt_found, xrep_ibt_found,
    xrep_refc_found, xrep_bmap_found, xrep_rmap_found, xrep_findroot_block,
    xrep_calc_ag_resblks, xrep_calc_ag_resblks_btsize,
    xrep_reset_counters, xrep_newbt_alloc_ag_blocks, xrep_newbt_alloc_file_blocks,
    xrep_newbt_free_blocks, xrep_newbt_claim_block, xrep_dinode_header,
    xrep_dinode_mode, xrep_dinode_flags, xrep_dinode_size,
    xrep_dinode_extsize_hints, xrep_dinode_zap_symlink, xrep_dinode_zap_dir,
    xrep_dinode_fixed, xrep_dinode_zap_forks, xrep_dinode_zap_dfork,
    xrep_dinode_zap_afork, xrep_dinode_ensure_forkoff, xrep_inode_blockcounts,
    xrep_inode_ids, xrep_inode_flags, xrep_inode_blockdir_size,
    xrep_inode_sfdir_size, xrep_inode_dir_size, xrep_inode_fixed,
    xrep_dinode_count_rmaps, xrep_dinode_findmode_dirent,
    xrep_dinode_findmode_dirent_inval, xrep_cow_mark_file_range,
    xrep_cow_free_staging, xrep_nlinks_update_inode,
    xrep_nlinks_unfixable_inode, xrep_rmap_live_update, xrep_tempfile_create,
    xrep_tempfile_prealloc, xrep_tempfile_copyin, xreap_ifork_extent,
    xreap_bmapi_binval_scan, xrep_xattr_recover_leafblock,
    xrep_xattr_salvage_rec, xrep_xattr_insert_rec, xrep_parent_stash_xattr,
    xrep_parent_insert_xattr, xrep_xattr_salvage_pptr, xrep_xattr_insert_pptr,
    xrep_xattr_rebuild_tree, xrep_xattr_reset_fork, xrep_xattr_full_reset,
    xrep_xattr_stash_parentadd, xrep_xattr_stash_parentremove,
    xrep_dir_recover_dirblock, xrep_dir_rebuild_tree, xrep_dir_reset_fork,
    xrep_parent_reset_dotdot, xrep_dir_salvage_entry, xrep_dir_stash_createname,
    xrep_dir_replay_createname, xrep_adoption_reparent,
    xrep_dir_stash_removename, xrep_dir_replay_removename,
    xrep_adoption_trans_roll, xrep_dir_salvaged_parent,
    xrep_findparent_dirent, xrep_findparent_from_dcache,
    xrep_xattr_replay_parentadd, xrep_xattr_replay_parentremove,
    xrep_parent_replay_parentadd, xrep_parent_replay_parentremove,
    xrep_parent_stash_parentadd, xrep_parent_stash_parentremove,
    xrep_nlinks_set_record, xrep_adoption_check_child,
    xrep_adoption_invalidate_child, xrep_dirtree_delete_child,
    xrep_symlink_salvage_target, xrep_symlink_rebuild, xrep_symlink_reset_fork,
    xrep_iunlink_visit, xrep_iunlink_reload_next, xrep_iunlink_reload_ondisk,
    xrep_iunlink_walk_ondisk_bucket, xrep_iunlink_resolve_infinite_loop,
    xrep_iunlink_resolve_uncached, xrep_iunlink_resolve_wronglist,
    xrep_iunlink_resolve_nolist, xrep_iunlink_resolve_ok,
    xrep_iunlink_resolve_allocated, xrep_iunlink_relink_next,
    xrep_iunlink_relink_prev, xrep_iunlink_add_to_bucket,
    xrep_iunlink_commit_bucket
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
