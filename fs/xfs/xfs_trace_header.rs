// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of xfs_trace.h.  Linux tracepoint declarations are a
// build-time DSL; the declarations below retain their externally visible
// names while leaving field types and callback bodies to the corresponding
// XFS dependency definitions.

#![allow(non_camel_case_types, dead_code)]

// C forward declarations (the concrete layouts are supplied by XFS).
macro_rules! opaque_types { ($($name:ident),* $(,)?) => { $(pub enum $name {})* }; }
opaque_types!(
    xfs_agf, xfs_ail, xfs_alloc_arg, xfs_attr_list_context, xfs_buf_log_item,
    xfs_da_args, xfs_da_node_entry, xfs_dquot, xfs_log_item, xlog,
    xlog_ticket, xlog_recover, xlog_recover_item, xlog_rec_header,
    xlog_in_core, xfs_buf_log_format, xfs_inode_log_format, xfs_bmbt_irec,
    xfs_btree_cur, xfs_defer_op_type, xfs_refcount_irec, xfs_fsmap,
    xfs_fsmap_irec, xfs_group, xfs_rmap_irec, xfs_icreate_log,
    xfs_iunlink_item, xfs_owner_info, xfs_trans_res, xfs_inobt_rec_incore,
    xfs_dqtrx, xfs_icwalk, xfs_perag, xfbtree, xfs_btree_ops,
    xfs_bmap_intent, xfs_exchmaps_intent, xfs_exchmaps_req, xfs_exchrange,
    xfs_getparents, xfs_parent_irec, xfs_attrlist_cursor_kern,
    xfs_extent_free_item, xfs_rmap_intent, xfs_refcount_intent,
    xfs_metadir_update, xfs_rtgroup, xfs_open_zone, xfs_healthmon_event,
    xfs_healthmon, fserror_event
);

// `union xfs_btree_ptr;` is an incomplete C union and is intentionally opaque.
#[repr(C)]
pub union xfs_btree_ptr { _opaque: [u8; 0] }

// TRACE_DEFINE_ENUM and TRACE_EVENT/DECLARE_EVENT_CLASS are kernel tracepoint
// registration directives.  Their Rust-level names are retained here so
// users of this header can refer to the same event set.
macro_rules! xfs_trace_events { ($($name:ident),* $(,)?) => {
    $(pub const $name: &str = stringify!($name);)*
}; }

xfs_trace_events!(
    xfs_attr_list_class, xfs_calc_atomic_write_unit_max,
    xfs_calc_max_atomic_write_fsblocks, xfs_calc_max_atomic_write_log_geometry,
    xlog_intent_recovery_failed, xfs_perag_class, xfs_group_class,
    xfs_zone_class, xfs_zone_free_blocks, xfs_zone_alloc_class,
    xfs_zone_gc_select_victim, xfs_zones_mount, xfs_inodegc_worker,
    xfs_fs_class, xfs_inodegc_shrinker_scan, xfs_ag_class,
    xfs_attr_list_node_descend, xfs_bmap_class, xfs_buf_class,
    xfs_buf_flags_class, xfs_buf_ioerror, xfs_buf_item_class,
    xfs_filestream_class, xfs_filestream_pick, xfs_lock_class,
    xfs_inode_class, xfs_fault_class, xfs_iref_class, xfs_iomap_prealloc_size,
    xfs_irec_merge_pre, xfs_irec_merge_post, xfs_namespace_class, xfs_rename,
    xfs_dquot_class, xfs_trans_mod_dquot, xfs_dqtrx_class, xfs_loggrant_class,
    xfs_log_item_class, xfs_log_force, xfs_ail_push_class, xfs_ail_class,
    xfs_log_assign_tail_lsn, xfs_file_class, xfs_iomap_atomic_write_cow,
    xfs_imap_class, xfs_simple_io_class, xfs_itrunc_class, xfs_bunmap,
    xfs_extent_busy_class, xfs_extent_busy_trim, xfs_rtalloc_extent_busy,
    xfs_rtalloc_extent_busy_trim, xfs_agf_class, xfs_free_extent,
    xfs_alloc_class, xfs_alloc_cur_check, xfs_da_class, xfs_attr_class,
    xfs_dir2_space_class, xfs_dir2_leafn_moveents, xfs_swap_extent_class,
    xfs_log_recover, xfs_log_recover_record, xfs_log_recover_item_class,
    xfs_log_recover_buf_item_class, xfs_log_recover_ino_item_class,
    xfs_log_recover_icreate_item_class, xfs_discard_class, xfs_rtdiscard_class,
    xfs_btree_cur_class, xfs_btree_alloc_block, xfs_btree_free_block,
    xfs_defer_class, xfs_defer_error_class, xfs_defer_pending_class,
    xfs_free_extent_deferred_class, xfs_defer_pending_item_class,
    xfs_rmap_class, xfs_btree_error_class, xfs_rmap_convert_state,
    xfs_rmapbt_class, xfs_rmap_deferred_class, xfs_bmap_deferred_class,
    xfs_ag_resv_class, xfs_ag_resv_init_error, xfs_refcount_class,
    xfs_refcount_lookup, xfs_refcount_extent_class, xfs_refcount_extent_at_class,
    xfs_refcount_double_extent_class, xfs_refcount_double_extent_at_class,
    xfs_refcount_triple_extent_class, xfs_refcount_deferred_class,
    xfs_inode_error_class, xfs_double_io_class, xfs_inode_irec_class,
    xfs_wb_invalid_class, xfs_iomap_invalid_class, xfs_reflink_remap_blocks,
    xfs_fsmap_mapping, xfs_fsmap_group_key_class, xfs_fsmap_linear_key_class,
    xfs_getfsmap_class, xfs_trans_resv_class, xfs_log_get_max_trans_res,
    xfs_trans_class, xfs_iunlink_update_bucket, xfs_iunlink_update_dinode,
    xfs_iunlink_reload_next, xfs_inode_reload_unlinked_bucket,
    xfs_ag_inode_class, xfs_fs_corrupt_class, xfs_group_corrupt_class,
    xfs_inode_corrupt_class, xfs_iwalk_ag_rec, xfs_pwork_init,
    xfs_check_new_dalign, xfs_btree_commit_afakeroot, xfs_btree_commit_ifakeroot,
    xfs_btree_bload_level_geometry, xfs_btree_bload_block,
    xfs_timestamp_range_class, xfs_icwalk_class, xlog_iclog_class,
    xfs_das_state_class, xfs_force_shutdown, xfs_group_intents_class,
    xmbuf_create, xmbuf_free, xfbtree_init, xfbtree_buf_class,
    xfbtree_freesp_class, xfs_exchrange_inode_class, xfs_exchrange_class,
    xfs_exchrange_freshness, xfs_exchmaps_overhead, xfs_exchmaps_estimate_class,
    xfs_exchmaps_intent_class, xfs_exchmaps_delta_nextents_step,
    xfs_exchmaps_delta_nextents, xfs_getparents_rec_class, xfs_getparents_class,
    xfs_metadir_update_class, xfs_metadir_update_error_class, xfs_metadir_class,
    xfs_metafile_resv_class, xfs_growfs_check_rtgeom, xfs_freeblocks_resv_class,
    xfs_healthmon_lost_event, xfs_healthmon_create, xfs_healthmon_copybuf,
    xfs_healthmon_class, xfs_healthmon_event_class, xfs_healthmon_report_fs,
    xfs_healthmon_report_group, xfs_healthmon_report_inode,
    xfs_healthmon_report_shutdown, xfs_healthmon_report_media,
    xfs_healthmon_report_file_ioerror, xfs_verify_media, xfs_verify_media_end,
    xfs_verify_media_error, xfs_bmap_replace_cow_mapping
);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
