/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of ocfs2.h. Kernel and sibling-header dependencies are external. */

pub const OCFS2_CACHE_INFO_MAX_ARRAY: usize = 2;
#[repr(C)] pub enum ocfs2_caching_info_flags { OCFS2_CACHE_FL_INLINE = 1 << 1 }
#[repr(C)] pub union ocfs2_cache_union { pub ci_array: [sector_t; OCFS2_CACHE_INFO_MAX_ARRAY], pub ci_tree: rb_root }
#[repr(C)] pub struct ocfs2_caching_info { pub ci_ops: *const ocfs2_caching_operations, pub ci_created_trans: c_ulong, pub ci_last_trans: c_ulong, pub ci_flags: c_uint, pub ci_num_cached: c_uint, pub ci_cache: ocfs2_cache_union }
pub struct ocfs2_caching_operations;
pub unsafe extern "C" fn ocfs2_metadata_cache_get_super(ci: *mut ocfs2_caching_info) -> *mut super_block { extern_fn!(); }

pub const OCFS2_NODE_MAP_MAX_NODES: usize = 256;
#[repr(C)] pub struct ocfs2_node_map { pub num_nodes: u16, pub map: [c_ulong; BITS_TO_LONGS(OCFS2_NODE_MAP_MAX_NODES)] }
#[repr(C)] pub enum ocfs2_ast_action { OCFS2_AST_INVALID=0, OCFS2_AST_ATTACH, OCFS2_AST_CONVERT, OCFS2_AST_DOWNCONVERT }
#[repr(C)] pub enum ocfs2_unlock_action { OCFS2_UNLOCK_INVALID=0, OCFS2_UNLOCK_CANCEL_CONVERT, OCFS2_UNLOCK_DROP_LOCK }
pub const OCFS2_LOCK_ATTACHED:u64=0x1; pub const OCFS2_LOCK_BUSY:u64=0x2; pub const OCFS2_LOCK_BLOCKED:u64=0x4; pub const OCFS2_LOCK_LOCAL:u64=0x8; pub const OCFS2_LOCK_NEEDS_REFRESH:u64=0x10; pub const OCFS2_LOCK_REFRESHING:u64=0x20; pub const OCFS2_LOCK_INITIALIZED:u64=0x40; pub const OCFS2_LOCK_FREEING:u64=0x80; pub const OCFS2_LOCK_QUEUED:u64=0x100; pub const OCFS2_LOCK_NOCACHE:u64=0x200; pub const OCFS2_LOCK_PENDING:u64=0x400; pub const OCFS2_LOCK_UPCONVERT_FINISHING:u64=0x800; pub const OCFS2_LOCK_NONBLOCK_FINISHED:u64=0x1000;
pub struct ocfs2_lock_res_ops;
pub type ocfs2_lock_callback = unsafe extern "C" fn(c_int, c_ulong);
#[repr(C)] pub struct ocfs2_lock_res { pub l_priv:*mut c_void, pub l_ops:*const ocfs2_lock_res_ops, pub l_blocked_list:list_head, pub l_mask_waiters:list_head, pub l_holders:list_head, pub l_flags:c_ulong, pub l_name:[c_char;OCFS2_LOCK_ID_MAX_LEN], pub l_ro_holders:c_uint, pub l_ex_holders:c_uint, pub l_level:i8, pub l_requested:i8, pub l_blocking:i8, pub l_type:u8, pub l_action:u8, pub l_unlock_action:u8, pub l_pending_gen:c_uint, pub l_lock:spinlock_t, pub l_lksb:ocfs2_dlm_lksb, pub l_event:wait_queue_head_t, pub l_debug_list:list_head }
#[repr(C)] pub enum ocfs2_orphan_reco_type { ORPHAN_NO_NEED_TRUNCATE=0, ORPHAN_NEED_TRUNCATE }
#[repr(C)] pub enum ocfs2_orphan_scan_state { ORPHAN_SCAN_ACTIVE, ORPHAN_SCAN_INACTIVE }
#[repr(C)] pub struct ocfs2_orphan_scan { pub os_lock:mutex, pub os_osb:*mut ocfs2_super, pub os_lockres:ocfs2_lock_res, pub os_orphan_scan_work:delayed_work, pub os_scantime:time64_t, pub os_count:u32, pub os_seqno:u32, pub os_state:atomic_t }
#[repr(C)] pub struct ocfs2_dlm_debug { pub d_refcnt:kref, pub d_filter_secs:u32, pub d_lockres_tracking:list_head }
#[repr(C)] pub enum ocfs2_vol_state { VOLUME_INIT=0, VOLUME_MOUNTED, VOLUME_MOUNTED_QUOTAS, VOLUME_DISMOUNTED, VOLUME_DISABLED }
#[repr(C)] pub struct ocfs2_alloc_stats { pub moves:atomic_t, pub local_data:atomic_t, pub bitmap_data:atomic_t, pub bg_allocs:atomic_t, pub bg_extends:atomic_t }
#[repr(C)] pub enum ocfs2_local_alloc_state { OCFS2_LA_UNUSED=0, OCFS2_LA_ENABLED, OCFS2_LA_THROTTLED, OCFS2_LA_DISABLED }
#[repr(C)] pub enum ocfs2_mount_options { OCFS2_MOUNT_HB_LOCAL=1, OCFS2_MOUNT_BARRIER=2, OCFS2_MOUNT_NOINTR=4, OCFS2_MOUNT_ERRORS_PANIC=8, OCFS2_MOUNT_DATA_WRITEBACK=16, OCFS2_MOUNT_LOCALFLOCKS=32, OCFS2_MOUNT_NOUSERXATTR=64, OCFS2_MOUNT_INODE64=128, OCFS2_MOUNT_POSIX_ACL=256, OCFS2_MOUNT_NO_POSIX_ACL=512, OCFS2_MOUNT_USRQUOTA=1024, OCFS2_MOUNT_GRPQUOTA=2048, OCFS2_MOUNT_COHERENCY_BUFFERED=4096, OCFS2_MOUNT_HB_NONE=8192, OCFS2_MOUNT_HB_GLOBAL=16384, OCFS2_MOUNT_JOURNAL_ASYNC_COMMIT=32768, OCFS2_MOUNT_ERRORS_CONT=65536, OCFS2_MOUNT_ERRORS_ROFS=131072 }
pub const OCFS2_OSB_SOFT_RO:u64=1; pub const OCFS2_OSB_HARD_RO:u64=2; pub const OCFS2_OSB_ERROR_FS:u64=4; pub const OCFS2_DEFAULT_ATIME_QUANTUM:u32=60;
#[repr(C)] pub struct ocfs2_triggers { pub ot_triggers:jbd2_buffer_trigger_type, pub ot_offset:c_int, pub sb:*mut super_block }
#[repr(C)] pub enum ocfs2_journal_trigger_type { OCFS2_JTR_DI,OCFS2_JTR_EB,OCFS2_JTR_RB,OCFS2_JTR_GD,OCFS2_JTR_DB,OCFS2_JTR_XB,OCFS2_JTR_DQ,OCFS2_JTR_DR,OCFS2_JTR_DL,OCFS2_JTR_NONE }
pub const OCFS2_JOURNAL_TRIGGER_COUNT:usize=OCFS2_JTR_NONE as usize;
pub unsafe extern "C" fn ocfs2_initialize_journal_triggers(sb:*mut super_block,triggers:*mut ocfs2_triggers);
#[repr(C)] pub enum ocfs2_recovery_state { OCFS2_REC_ENABLED=0, OCFS2_REC_QUOTA_WANT_DISABLE, OCFS2_REC_QUOTA_DISABLED, OCFS2_REC_WANT_DISABLE, OCFS2_REC_DISABLED }
pub struct ocfs2_journal; pub struct ocfs2_slot_info; pub struct ocfs2_recovery_map; pub struct ocfs2_replay_map; pub struct ocfs2_quota_recovery;

#[repr(C)] pub struct ocfs2_super {
 pub commit_task:*mut task_struct,pub sb:*mut super_block,pub root_inode:*mut inode,pub sys_root_inode:*mut inode,pub global_system_inodes:[*mut inode;NUM_GLOBAL_SYSTEM_INODES],pub local_system_inodes:*mut *mut inode,pub slot_info:*mut ocfs2_slot_info,pub slot_recovery_generations:*mut u32,pub node_map_lock:spinlock_t,pub root_blkno:u64,pub system_dir_blkno:u64,pub bitmap_blkno:u64,pub bitmap_cpg:u32,pub uuid_str:*mut c_char,pub uuid_hash:u32,pub vol_label:*mut u8,pub first_cluster_group_blkno:u64,pub fs_generation:u32,pub s_feature_compat:u32,pub s_feature_incompat:u32,pub s_feature_ro_compat:u32,pub osb_lock:spinlock_t,pub s_next_generation:u32,pub osb_flags:c_ulong,pub s_inode_steal_slot:u16,pub s_meta_steal_slot:u16,pub s_num_inodes_stolen:atomic_t,pub s_num_meta_stolen:atomic_t,pub s_mount_opt:c_ulong,pub s_atime_quantum:c_uint,pub max_slots:c_uint,pub node_num:c_uint,pub slot_num:c_int,pub preferred_slot:c_int,pub s_sectsize_bits:c_int,pub s_clustersize:c_int,pub s_clustersize_bits:c_int,pub s_xattr_inline_size:c_uint,pub vol_state:atomic_t,pub recovery_lock:mutex,pub recovery_map:*mut ocfs2_recovery_map,pub replay_map:*mut ocfs2_replay_map,pub recovery_thread_task:*mut task_struct,pub recovery_state:ocfs2_recovery_state,pub checkpoint_event:wait_queue_head_t,pub journal:*mut ocfs2_journal,pub osb_commit_interval:c_ulong,pub s_journal_triggers:[ocfs2_triggers;OCFS2_JOURNAL_TRIGGER_COUNT],pub la_enable_wq:delayed_work,pub local_alloc_bits:c_uint,pub local_alloc_default_bits:c_uint,pub osb_clusters_at_boot:c_uint,pub local_alloc_state:ocfs2_local_alloc_state,pub local_alloc_bh:*mut buffer_head,pub la_last_gd:u64,pub osb_la_resmap:ocfs2_reservation_map,pub osb_resv_level:c_uint,pub osb_dir_resv_level:c_uint,pub local_alloc_copy:*mut ocfs2_dinode,pub quota_rec:*mut ocfs2_quota_recovery,pub osb_ecc_stats:ocfs2_blockcheck_stats,pub alloc_stats:ocfs2_alloc_stats,pub dev_str:[c_char;20],pub osb_stackflags:u8,pub osb_cluster_stack:[c_char;OCFS2_STACK_LABEL_LEN+1],pub osb_cluster_name:[c_char;OCFS2_CLUSTER_NAME_LEN+1],pub cconn:*mut ocfs2_cluster_connection,pub osb_super_lockres:ocfs2_lock_res,pub osb_rename_lockres:ocfs2_lock_res,pub osb_nfs_sync_lockres:ocfs2_lock_res,pub nfs_sync_rwlock:rw_semaphore,pub osb_trim_fs_lockres:ocfs2_lock_res,pub obs_trim_fs_mutex:mutex,pub osb_dlm_debug:*mut ocfs2_dlm_debug,pub osb_debug_root:*mut dentry,pub recovery_event:wait_queue_head_t,pub dc_task_lock:spinlock_t,pub dc_task:*mut task_struct,pub dc_event:wait_queue_head_t,pub dc_wake_sequence:c_ulong,pub dc_work_sequence:c_ulong,pub blocked_lock_list:list_head,pub blocked_lock_count:c_ulong,pub dquot_drop_list:llist_head,pub dquot_drop_work:work_struct,pub osb_mount_event:wait_queue_head_t,pub osb_tl_inode:*mut inode,pub osb_tl_bh:*mut buffer_head,pub osb_truncate_log_wq:delayed_work,pub osb_tl_disable:atomic_t,pub truncated_clusters:c_uint,pub osb_recovering_orphan_dirs:ocfs2_node_map,pub osb_orphan_wipes:*mut c_uint,pub osb_wipe_event:wait_queue_head_t,pub osb_orphan_scan:ocfs2_orphan_scan,pub osb_xattr_lock:spinlock_t,pub osb_dx_mask:c_uint,pub osb_dx_seed:[u32;4],pub osb_inode_alloc_group:u64,pub osb_rf_lock_tree:rb_root,pub osb_ref_tree_lru:*mut ocfs2_refcount_tree,pub ocfs2_wq:*mut workqueue_struct,pub osb_dev_kset:*mut kset,pub osb_fc_ent:ocfs2_filecheck_sysfs_entry }
pub struct ocfs2_refcount_tree;
pub type ocfs2_journal_access_func=unsafe extern "C" fn(*mut handle_t,*mut ocfs2_caching_info,*mut buffer_head,c_int)->c_int;

#[inline] pub unsafe fn ocfs2_should_order_data(i:*mut inode)->c_int { if !S_ISREG((*i).i_mode) {return 0} ; if OCFS2_SB((*i).i_sb).as_ref().unwrap().s_mount_opt & OCFS2_MOUNT_DATA_WRITEBACK as c_ulong !=0 {0} else {1} }
#[inline] pub unsafe fn ocfs2_sparse_alloc(o:*mut ocfs2_super)->c_int { ((*o).s_feature_incompat & OCFS2_FEATURE_INCOMPAT_SPARSE_ALLOC !=0) as c_int }
#[inline] pub unsafe fn ocfs2_writes_unwritten_extents(o:*mut ocfs2_super)->c_int { if ocfs2_sparse_alloc(o)==0 {0} else {((*o).s_feature_ro_compat & OCFS2_FEATURE_RO_COMPAT_UNWRITTEN !=0) as c_int} }
#[inline] pub unsafe fn ocfs2_supports_append_dio(o:*mut ocfs2_super)->c_int { ((*o).s_feature_incompat&OCFS2_FEATURE_INCOMPAT_APPEND_DIO!=0) as c_int }
#[inline] pub unsafe fn ocfs2_supports_inline_data(o:*mut ocfs2_super)->c_int { ((*o).s_feature_incompat&OCFS2_FEATURE_INCOMPAT_INLINE_DATA!=0) as c_int }
#[inline] pub unsafe fn ocfs2_supports_xattr(o:*mut ocfs2_super)->c_int { ((*o).s_feature_incompat&OCFS2_FEATURE_INCOMPAT_XATTR!=0) as c_int }
#[inline] pub unsafe fn ocfs2_meta_ecc(o:*mut ocfs2_super)->c_int { ((*o).s_feature_incompat&OCFS2_FEATURE_INCOMPAT_META_ECC!=0) as c_int }
#[inline] pub unsafe fn ocfs2_supports_indexed_dirs(o:*mut ocfs2_super)->c_int { ((*o).s_feature_incompat&OCFS2_FEATURE_INCOMPAT_INDEXED_DIRS!=0) as c_int }
#[inline] pub unsafe fn ocfs2_supports_discontig_bg(o:*mut ocfs2_super)->c_int { ((*o).s_feature_incompat&OCFS2_FEATURE_INCOMPAT_DISCONTIG_BG!=0) as c_int }
#[inline] pub unsafe fn ocfs2_refcount_tree(o:*mut ocfs2_super)->c_int { ((*o).s_feature_incompat&OCFS2_FEATURE_INCOMPAT_REFCOUNT_TREE!=0) as c_int }
#[inline] pub unsafe fn ocfs2_mount_local(o:*mut ocfs2_super)->c_int { ((*o).s_feature_incompat&OCFS2_FEATURE_INCOMPAT_LOCAL_MOUNT!=0) as c_int }
#[inline] pub unsafe fn ocfs2_uses_extended_slot_map(o:*mut ocfs2_super)->c_int { ((*o).s_feature_incompat&OCFS2_FEATURE_INCOMPAT_EXTENDED_SLOT_MAP!=0) as c_int }
#[inline] pub unsafe fn ocfs2_clusters_to_blocks(sb:*mut super_block,c:u32)->u64 { (c as u64)<<((*OCFS2_SB(sb)).s_clustersize_bits-sb.as_ref().unwrap().s_blocksize_bits) }
#[inline] pub unsafe fn ocfs2_blocks_to_clusters(sb:*mut super_block,b:u64)->u32 { (b>>((*OCFS2_SB(sb)).s_clustersize_bits-sb.as_ref().unwrap().s_blocksize_bits)) as u32 }
#[inline] pub unsafe fn ocfs2_blocks_for_bytes(sb:*mut super_block,b:u64)->u64 { (b+sb.as_ref().unwrap().s_blocksize as u64-1)>>sb.as_ref().unwrap().s_blocksize_bits }
#[inline] pub unsafe fn ocfs2_clusters_for_bytes(sb:*mut super_block,b:u64)->u32 { ((b+(*OCFS2_SB(sb)).s_clustersize as u64-1)>>(*OCFS2_SB(sb)).s_clustersize_bits) as u32 }
#[inline] pub unsafe fn ocfs2_bytes_to_clusters(sb:*mut super_block,b:u64)->u32 { (b>>(*OCFS2_SB(sb)).s_clustersize_bits) as u32 }
#[inline] pub unsafe fn ocfs2_align_bytes_to_sectors(b:u64)->c_ulong {(b+511)>>9}

/* External C kernel types, constants, helpers, and macros are supplied by the translated dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
