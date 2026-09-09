/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of ocfs2/journal.h. */

// Dependencies supplied by other translated units:
// linux/fs.h, linux/jbd2.h

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_journal_state {
    OCFS2_JOURNAL_FREE = 0,
    OCFS2_JOURNAL_LOADED,
    OCFS2_JOURNAL_IN_SHUTDOWN,
}

pub struct ocfs2_super;
pub struct ocfs2_dinode;
pub struct ocfs2_caching_info;
pub struct ocfs2_extent_list;
pub struct ocfs2_inode_info;
pub struct super_block;
pub struct inode;
pub struct buffer_head;
pub struct journal_t;
pub struct atomic_t;
pub struct spinlock_t;
pub struct rw_semaphore;
pub struct wait_queue_head_t;
pub struct list_head;
pub struct work_struct;
pub struct handle_t;

#[repr(C)]
pub struct ocfs2_recovery_map {
    pub rm_used: ::core::ffi::c_uint,
    pub rm_entries: [::core::ffi::c_uint; 0],
}

#[repr(C)]
pub struct ocfs2_journal {
    pub j_state: ocfs2_journal_state,
    pub j_journal: *mut journal_t,
    pub j_inode: *mut inode,
    pub j_osb: *mut ocfs2_super,
    pub j_bh: *mut buffer_head,
    pub j_num_trans: atomic_t,
    pub j_lock: spinlock_t,
    pub j_trans_id: ::core::ffi::c_ulong,
    pub j_trans_barrier: rw_semaphore,
    pub j_checkpointed: wait_queue_head_t,
    pub j_la_cleanups: list_head,
    pub j_recovery_work: work_struct,
}

extern "C" {
    pub static mut trans_inc_lock: spinlock_t;

    pub fn ocfs2_metadata_cache_get_super(ci: *mut ocfs2_caching_info) -> *mut super_block;
    pub fn OCFS2_SB(sb: *mut super_block) -> *mut ocfs2_super;
    pub fn OCFS2_I(inode: *mut inode) -> *mut ocfs2_inode_info;
    pub fn INODE_CACHE(inode: *mut inode) -> *mut ocfs2_caching_info;
    pub fn ocfs2_mount_local(osb: *mut ocfs2_super) -> ::core::ffi::c_int;
    pub fn wake_up(queue: *mut wait_queue_head_t);
    pub fn wait_event(queue: *mut wait_queue_head_t, condition: bool);
    pub fn spin_lock(lock: *mut spinlock_t);
    pub fn spin_unlock(lock: *mut spinlock_t);
    pub fn time_after(a: ::core::ffi::c_ulong, b: ::core::ffi::c_ulong) -> bool;
    pub fn is_handle_aborted(handle: *mut handle_t) -> bool;
    pub fn le16_to_cpu(value: u16) -> u16;
    pub fn ocfs2_extend_meta_needed(root_el: *mut ocfs2_extent_list) -> ::core::ffi::c_int;
    pub fn ocfs2_clusters_to_blocks(sb: *mut super_block, clusters: u32) -> ::core::ffi::c_int;
    pub fn ocfs2_extent_recs_per_gd(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn jbd2_journal_inode_ranged_write(handle: *mut handle_t, jinode: *mut ::core::ffi::c_void, start: i64, len: i64) -> ::core::ffi::c_int;
    pub fn jbd2_journal_begin_ordered_truncate(journal: *mut journal_t, jinode: *mut ::core::ffi::c_void, new_size: i64) -> ::core::ffi::c_int;

    pub fn ocfs2_orphan_scan_init(osb: *mut ocfs2_super);
    pub fn ocfs2_orphan_scan_start(osb: *mut ocfs2_super);
    pub fn ocfs2_orphan_scan_stop(osb: *mut ocfs2_super);
    pub fn ocfs2_complete_recovery(work: *mut work_struct);
    pub fn ocfs2_wait_for_recovery(osb: *mut ocfs2_super);
    pub fn ocfs2_recovery_init(osb: *mut ocfs2_super) -> ::core::ffi::c_int;
    pub fn ocfs2_recovery_exit(osb: *mut ocfs2_super);
    pub fn ocfs2_recovery_disable_quota(osb: *mut ocfs2_super);
    pub fn ocfs2_compute_replay_slots(osb: *mut ocfs2_super) -> ::core::ffi::c_int;
    pub fn ocfs2_free_replay_slots(osb: *mut ocfs2_super);
    pub fn ocfs2_set_journal_params(osb: *mut ocfs2_super);
    pub fn ocfs2_journal_alloc(osb: *mut ocfs2_super) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_init(osb: *mut ocfs2_super, dirty: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_shutdown(osb: *mut ocfs2_super);
    pub fn ocfs2_journal_wipe(journal: *mut ocfs2_journal, full: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_load(journal: *mut ocfs2_journal, local: ::core::ffi::c_int, replayed: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_check_journals_nolocks(osb: *mut ocfs2_super) -> ::core::ffi::c_int;
    pub fn ocfs2_recovery_thread(osb: *mut ocfs2_super, node_num: ::core::ffi::c_int);
    pub fn ocfs2_mark_dead_nodes(osb: *mut ocfs2_super) -> ::core::ffi::c_int;
    pub fn ocfs2_complete_mount_recovery(osb: *mut ocfs2_super);
    pub fn ocfs2_complete_quota_recovery(osb: *mut ocfs2_super);
    pub fn ocfs2_start_trans(osb: *mut ocfs2_super, max_buffs: ::core::ffi::c_int) -> *mut handle_t;
    pub fn ocfs2_commit_trans(osb: *mut ocfs2_super, handle: *mut handle_t) -> ::core::ffi::c_int;
    pub fn ocfs2_extend_trans(handle: *mut handle_t, nblocks: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_assure_trans_credits(handle: *mut handle_t, nblocks: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_allocate_extend_trans(handle: *mut handle_t, thresh: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_dirty(handle: *mut handle_t, bh: *mut buffer_head);
}

pub const OCFS2_MAX_TRANS_DATA: u32 = 64;
pub const OCFS2_JOURNAL_ACCESS_CREATE: ::core::ffi::c_int = 0;
pub const OCFS2_JOURNAL_ACCESS_WRITE: ::core::ffi::c_int = 1;
pub const OCFS2_JOURNAL_ACCESS_UNDO: ::core::ffi::c_int = 2;
pub const OCFS2_INODE_UPDATE_CREDITS: ::core::ffi::c_int = 1;
pub const OCFS2_XATTR_BLOCK_UPDATE_CREDITS: ::core::ffi::c_int = 1;
pub const OCFS2_QUOTA_BLOCK_UPDATE_CREDITS: ::core::ffi::c_int = 1;
pub const OCFS2_QINFO_WRITE_CREDITS: ::core::ffi::c_int = OCFS2_INODE_UPDATE_CREDITS + OCFS2_QUOTA_BLOCK_UPDATE_CREDITS;
pub const OCFS2_LOCAL_QINFO_WRITE_CREDITS: ::core::ffi::c_int = OCFS2_QUOTA_BLOCK_UPDATE_CREDITS;
pub const OCFS2_QWRITE_CREDITS: ::core::ffi::c_int = OCFS2_QINFO_WRITE_CREDITS + OCFS2_QUOTA_BLOCK_UPDATE_CREDITS;
pub const OCFS2_QSYNC_CREDITS: ::core::ffi::c_int = OCFS2_QINFO_WRITE_CREDITS + 2 * OCFS2_QUOTA_BLOCK_UPDATE_CREDITS;
pub const OCFS2_SUBALLOC_ALLOC: ::core::ffi::c_int = 3;
pub const OCFS2_SUBALLOC_FREE: ::core::ffi::c_int = 2;
pub const OCFS2_TRUNCATE_LOG_UPDATE: ::core::ffi::c_int = OCFS2_INODE_UPDATE_CREDITS;
pub const OCFS2_TRUNCATE_LOG_FLUSH_ONE_REC: ::core::ffi::c_int = OCFS2_SUBALLOC_FREE + OCFS2_TRUNCATE_LOG_UPDATE;
pub const OCFS2_DIR_LINK_ADDITIONAL_CREDITS: ::core::ffi::c_int = 1 + OCFS2_SUBALLOC_ALLOC + 1;
pub const OCFS2_WINDOW_MOVE_CREDITS: ::core::ffi::c_int = OCFS2_INODE_UPDATE_CREDITS + OCFS2_SUBALLOC_ALLOC + OCFS2_SUBALLOC_FREE;
pub const OCFS2_SIMPLE_DIR_EXTEND_CREDITS: ::core::ffi::c_int = 2;
pub const OCFS2_DELETE_INODE_CREDITS: ::core::ffi::c_int = 3 * OCFS2_INODE_UPDATE_CREDITS + 4;
pub const OCFS2_INODE_ADD_TO_ORPHAN_CREDITS: ::core::ffi::c_int = 2 * OCFS2_INODE_UPDATE_CREDITS + 4;
pub const OCFS2_INODE_DEL_FROM_ORPHAN_CREDITS: ::core::ffi::c_int = OCFS2_INODE_ADD_TO_ORPHAN_CREDITS;
pub const OCFS2_XATTR_BLOCK_CREATE_CREDITS: ::core::ffi::c_int = OCFS2_SUBALLOC_ALLOC * 2 + OCFS2_INODE_UPDATE_CREDITS + OCFS2_XATTR_BLOCK_UPDATE_CREDITS;
pub const OCFS2_DX_ROOT_REMOVE_CREDITS: ::core::ffi::c_int = OCFS2_INODE_UPDATE_CREDITS + OCFS2_SUBALLOC_FREE;
pub const OCFS2_REFCOUNT_TREE_CREATE_CREDITS: ::core::ffi::c_int = OCFS2_INODE_UPDATE_CREDITS + 1 + OCFS2_SUBALLOC_ALLOC;
pub const OCFS2_REFCOUNT_TREE_SET_CREDITS: ::core::ffi::c_int = OCFS2_INODE_UPDATE_CREDITS + 1;
pub const OCFS2_REFCOUNT_TREE_REMOVE_CREDITS: ::core::ffi::c_int = OCFS2_INODE_UPDATE_CREDITS + 1;
pub const OCFS2_EXPAND_REFCOUNT_TREE_CREDITS: ::core::ffi::c_int = OCFS2_SUBALLOC_ALLOC * 2 + 3;

extern "C" {
    pub fn ocfs2_journal_access_di(h: *mut handle_t, ci: *mut ocfs2_caching_info, bh: *mut buffer_head, ty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_access_eb(h: *mut handle_t, ci: *mut ocfs2_caching_info, bh: *mut buffer_head, ty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_access_rb(h: *mut handle_t, ci: *mut ocfs2_caching_info, bh: *mut buffer_head, ty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_access_gd(h: *mut handle_t, ci: *mut ocfs2_caching_info, bh: *mut buffer_head, ty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_access_xb(h: *mut handle_t, ci: *mut ocfs2_caching_info, bh: *mut buffer_head, ty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_access_dq(h: *mut handle_t, ci: *mut ocfs2_caching_info, bh: *mut buffer_head, ty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_access_db(h: *mut handle_t, ci: *mut ocfs2_caching_info, bh: *mut buffer_head, ty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_access_dr(h: *mut handle_t, ci: *mut ocfs2_caching_info, bh: *mut buffer_head, ty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_access_dl(h: *mut handle_t, ci: *mut ocfs2_caching_info, bh: *mut buffer_head, ty: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_journal_access(h: *mut handle_t, ci: *mut ocfs2_caching_info, bh: *mut buffer_head, ty: ::core::ffi::c_int) -> ::core::ffi::c_int;

    pub fn ocfs2_inc_trans_id(j: *mut ocfs2_journal) -> ::core::ffi::c_ulong;
    pub fn ocfs2_set_ci_lock_trans(journal: *mut ocfs2_journal, ci: *mut ocfs2_caching_info);
    pub fn ocfs2_ci_fully_checkpointed(ci: *mut ocfs2_caching_info) -> ::core::ffi::c_int;
    pub fn ocfs2_ci_is_new(ci: *mut ocfs2_caching_info) -> ::core::ffi::c_int;
    pub fn ocfs2_inode_is_new(inode: *mut inode) -> ::core::ffi::c_int;
    pub fn ocfs2_ci_set_new(osb: *mut ocfs2_super, ci: *mut ocfs2_caching_info);
    pub fn ocfs2_start_checkpoint(osb: *mut ocfs2_super);
    pub fn ocfs2_checkpoint_inode(inode: *mut inode);
    pub fn ocfs2_quota_trans_credits(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn ocfs2_inline_to_extents_credits(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn ocfs2_add_dir_index_credits(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn ocfs2_mknod_credits(sb: *mut super_block, is_dir: ::core::ffi::c_int, xattr_credits: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_link_credits(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn ocfs2_unlink_credits(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn ocfs2_rename_credits(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn ocfs2_calc_dxi_expand_credits(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn ocfs2_calc_extend_credits(sb: *mut super_block, root_el: *mut ocfs2_extent_list) -> ::core::ffi::c_int;
    pub fn ocfs2_calc_symlink_credits(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn ocfs2_calc_group_alloc_credits(sb: *mut super_block, cpg: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ocfs2_calc_bg_discontig_credits(sb: *mut super_block) -> ::core::ffi::c_int;
    pub fn ocfs2_jbd2_inode_add_write(handle: *mut handle_t, inode: *mut inode, start_byte: i64, length: i64) -> ::core::ffi::c_int;
    pub fn ocfs2_begin_ordered_truncate(inode: *mut inode, new_size: i64) -> ::core::ffi::c_int;
    pub fn ocfs2_update_inode_fsync_trans(handle: *mut handle_t, inode: *mut inode, datasync: ::core::ffi::c_int);
}

pub const OCFS2_GROUP_EXTEND_CREDITS: ::core::ffi::c_int = OCFS2_INODE_UPDATE_CREDITS + 1;
pub const OCFS2_GROUP_ADD_CREDITS: ::core::ffi::c_int = OCFS2_INODE_UPDATE_CREDITS + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
