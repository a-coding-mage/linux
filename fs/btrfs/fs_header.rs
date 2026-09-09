/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translated headers are intentionally left external.

pub const BTRFS_MIN_BLOCKSIZE: usize = SZ_4K;
pub const BTRFS_MAX_BLOCKSIZE: usize = SZ_64K;
pub const BTRFS_MAX_FOLIO_SIZE: usize = SZ_2M;
// static_assert(BTRFS_MAX_FOLIO_SIZE > PAGE_SIZE);
// CONFIG_BTRFS_EXPERIMENTAL selects 512; otherwise BITS_PER_LONG.
#[cfg(feature = "btrfs_experimental")]
pub const BTRFS_MAX_BLOCKS_PER_FOLIO: usize = 512;
#[cfg(not(feature = "btrfs_experimental"))]
pub const BTRFS_MAX_BLOCKS_PER_FOLIO: usize = BITS_PER_LONG;
pub const BTRFS_MAX_EXTENT_SIZE: usize = SZ_128M;
pub const BTRFS_MAX_TRIM_LENGTH: usize = SZ_2G;
pub const BTRFS_OLDEST_GENERATION: u64 = 0;
pub const BTRFS_EMPTY_DIR_SIZE: usize = 0;
pub const BTRFS_DIRTY_METADATA_THRESH: usize = SZ_32M;
pub const BTRFS_SUPER_INFO_OFFSET: usize = SZ_64K;
pub const BTRFS_SUPER_INFO_SIZE: usize = 4096;
pub const BTRFS_CSUM_FMT: &str = "0x%*phN";
pub const BTRFS_UNLINK_METADATA_UNITS: usize = 6;
pub const BTRFS_DEVICE_RANGE_RESERVED: usize = SZ_1M;

pub const BTRFS_FS_STATE_REMOUNTING: usize = 0;
pub const BTRFS_FS_STATE_RO: usize = 1;
pub const BTRFS_FS_STATE_TRANS_ABORTED: usize = 2;
pub const BTRFS_FS_STATE_LOG_REPLAY_ABORTED: usize = 3;
pub const BTRFS_FS_STATE_DEV_REPLACING: usize = 4;
pub const BTRFS_FS_STATE_DUMMY_FS_INFO: usize = 5;
pub const BTRFS_FS_STATE_NO_DATA_CSUMS: usize = 6;
pub const BTRFS_FS_STATE_SKIP_META_CSUMS: usize = 7;
pub const BTRFS_FS_STATE_LOG_CLEANUP_ERROR: usize = 8;
pub const BTRFS_FS_STATE_NO_DELAYED_IPUT: usize = 9;
pub const BTRFS_FS_STATE_EMERGENCY_SHUTDOWN: usize = 10;
pub const BTRFS_FS_STATE_COUNT: usize = 11;

pub const BTRFS_FS_CLOSING_START: usize = 0;
pub const BTRFS_FS_CLOSING_DONE: usize = 1;
pub const BTRFS_FS_LOG_RECOVERING: usize = 2;
pub const BTRFS_FS_OPEN: usize = 3;
pub const BTRFS_FS_QUOTA_ENABLED: usize = 4;
pub const BTRFS_FS_SQUOTA_ENABLING: usize = 5;
pub const BTRFS_FS_UPDATE_UUID_TREE_GEN: usize = 6;
pub const BTRFS_FS_CREATING_FREE_SPACE_TREE: usize = 7;
pub const BTRFS_FS_BTREE_ERR: usize = 8;
pub const BTRFS_FS_LOG1_ERR: usize = 9;
pub const BTRFS_FS_LOG2_ERR: usize = 10;
pub const BTRFS_FS_QUOTA_OVERRIDE: usize = 11;
pub const BTRFS_FS_FROZEN: usize = 12;
pub const BTRFS_FS_BALANCE_RUNNING: usize = 13;
pub const BTRFS_FS_RELOC_RUNNING: usize = 14;
pub const BTRFS_FS_CLEANER_RUNNING: usize = 15;
pub const BTRFS_FS_CSUM_IMPL_FAST: usize = 16;
pub const BTRFS_FS_DISCARD_RUNNING: usize = 17;
pub const BTRFS_FS_CLEANUP_SPACE_CACHE_V1: usize = 18;
pub const BTRFS_FS_FREE_SPACE_TREE_UNTRUSTED: usize = 19;
pub const BTRFS_FS_TREE_MOD_LOG_USERS: usize = 20;
pub const BTRFS_FS_COMMIT_TRANS: usize = 21;
pub const BTRFS_FS_UNFINISHED_DROPS: usize = 22;
pub const BTRFS_FS_NEED_ZONE_FINISH: usize = 23;
pub const BTRFS_FS_NEED_TRANS_COMMIT: usize = 24;
pub const BTRFS_FS_ACTIVE_ZONE_TRACKING: usize = 25;
pub const BTRFS_FS_FEATURE_CHANGED: usize = 26;
pub const BTRFS_FS_UNALIGNED_TREE_BLOCK: usize = 27;

pub const BTRFS_MOUNT_NODATASUM: u64 = 1 << 0;
pub const BTRFS_MOUNT_NODATACOW: u64 = 1 << 1;
pub const BTRFS_MOUNT_NOBARRIER: u64 = 1 << 2;
pub const BTRFS_MOUNT_SSD: u64 = 1 << 3;
pub const BTRFS_MOUNT_DEGRADED: u64 = 1 << 4;
pub const BTRFS_MOUNT_COMPRESS: u64 = 1 << 5;
pub const BTRFS_MOUNT_NOTREELOG: u64 = 1 << 6;
pub const BTRFS_MOUNT_FLUSHONCOMMIT: u64 = 1 << 7;
pub const BTRFS_MOUNT_SSD_SPREAD: u64 = 1 << 8;
pub const BTRFS_MOUNT_NOSSD: u64 = 1 << 9;
pub const BTRFS_MOUNT_DISCARD_SYNC: u64 = 1 << 10;
pub const BTRFS_MOUNT_FORCE_COMPRESS: u64 = 1 << 11;
pub const BTRFS_MOUNT_SPACE_CACHE: u64 = 1 << 12;
pub const BTRFS_MOUNT_CLEAR_CACHE: u64 = 1 << 13;
pub const BTRFS_MOUNT_USER_SUBVOL_RM_ALLOWED: u64 = 1 << 14;
pub const BTRFS_MOUNT_ENOSPC_DEBUG: u64 = 1 << 15;
pub const BTRFS_MOUNT_AUTO_DEFRAG: u64 = 1 << 16;
pub const BTRFS_MOUNT_USEBACKUPROOT: u64 = 1 << 17;
pub const BTRFS_MOUNT_SKIP_BALANCE: u64 = 1 << 18;
pub const BTRFS_MOUNT_PANIC_ON_FATAL_ERROR: u64 = 1 << 19;
pub const BTRFS_MOUNT_RESCAN_UUID_TREE: u64 = 1 << 20;
pub const BTRFS_MOUNT_FRAGMENT_DATA: u64 = 1 << 21;
pub const BTRFS_MOUNT_FRAGMENT_METADATA: u64 = 1 << 22;
pub const BTRFS_MOUNT_FREE_SPACE_TREE: u64 = 1 << 23;
pub const BTRFS_MOUNT_NOLOGREPLAY: u64 = 1 << 24;
pub const BTRFS_MOUNT_REF_VERIFY: u64 = 1 << 25;
pub const BTRFS_MOUNT_DISCARD_ASYNC: u64 = 1 << 26;
pub const BTRFS_MOUNT_IGNOREBADROOTS: u64 = 1 << 27;
pub const BTRFS_MOUNT_IGNOREDATACSUMS: u64 = 1 << 28;
pub const BTRFS_MOUNT_NODISCARD: u64 = 1 << 29;
pub const BTRFS_MOUNT_NOSPACECACHE: u64 = 1 << 30;
pub const BTRFS_MOUNT_IGNOREMETACSUMS: u64 = 1 << 31;
pub const BTRFS_MOUNT_IGNORESUPERFLAGS: u64 = 1 << 32;
pub const BTRFS_MOUNT_REF_TRACKER: u64 = 1 << 33;
pub const BTRFS_MOUNT_FULL_RO_MASK: u64 = BTRFS_MOUNT_NOLOGREPLAY | BTRFS_MOUNT_IGNOREBADROOTS | BTRFS_MOUNT_IGNOREDATACSUMS | BTRFS_MOUNT_IGNOREMETACSUMS | BTRFS_MOUNT_IGNORESUPERFLAGS | BTRFS_MOUNT_USEBACKUPROOT;

pub const BTRFS_DEFAULT_COMMIT_INTERVAL: u32 = 30;
pub const BTRFS_WARNING_COMMIT_INTERVAL: u32 = 300;
pub const BTRFS_DEFAULT_MAX_INLINE: u32 = 2048;
pub const BTRFS_NR_DISCARD_LISTS: usize = 3;
pub const BTRFS_DISCARD_INDEX_UNUSED: usize = 0;
pub const BTRFS_DISCARD_INDEX_START: usize = 1;

#[repr(C)]
pub enum btrfs_compression_type { BTRFS_COMPRESS_NONE = 0, BTRFS_COMPRESS_ZLIB = 1, BTRFS_COMPRESS_LZO = 2, BTRFS_COMPRESS_ZSTD = 3, BTRFS_NR_COMPRESS_TYPES = 4, BTRFS_DEFRAG_DONT_COMPRESS }
#[repr(C)]
pub enum btrfs_exclusive_operation { BTRFS_EXCLOP_NONE, BTRFS_EXCLOP_BALANCE_PAUSED, BTRFS_EXCLOP_BALANCE, BTRFS_EXCLOP_DEV_ADD, BTRFS_EXCLOP_DEV_REMOVE, BTRFS_EXCLOP_DEV_REPLACE, BTRFS_EXCLOP_RESIZE, BTRFS_EXCLOP_SWAP_ACTIVATE }

#[repr(C)] pub struct btrfs_dev_replace {
    pub replace_state: u64, pub time_started: time64_t, pub time_stopped: time64_t,
    pub num_write_errors: atomic64_t, pub num_uncorrectable_read_errors: atomic64_t,
    pub cursor_left: u64, pub committed_cursor_left: u64, pub cursor_left_last_write_of_item: u64, pub cursor_right: u64,
    pub cont_reading_from_srcdev_mode: u64, pub is_valid: i32, pub item_needs_writeback: i32,
    pub srcdev: *mut btrfs_device, pub tgtdev: *mut btrfs_device, pub lock_finishing_cancel_unmount: mutex,
    pub rwsem: rw_semaphore, pub scrub_progress: btrfs_scrub_progress, pub bio_counter: percpu_counter,
    pub replace_wait: wait_queue_head_t, pub replace_task: *mut task_struct,
}
#[repr(C)] pub struct btrfs_free_cluster { pub lock: spinlock_t, pub refill_lock: spinlock_t, pub root: rb_root, pub max_size: u64, pub window_start: u64, pub fragmented: bool, pub block_group: *mut btrfs_block_group, pub block_group_list: list_head }
#[repr(C)] pub struct btrfs_discard_ctl { pub discard_workers: *mut workqueue_struct, pub work: delayed_work, pub lock: spinlock_t, pub block_group: *mut btrfs_block_group, pub discard_list: [list_head; BTRFS_NR_DISCARD_LISTS], pub prev_discard: u64, pub prev_discard_time: u64, pub discardable_extents: atomic_t, pub discardable_bytes: atomic64_t, pub max_discard_size: u64, pub delay_ms: u64, pub iops_limit: u32, pub kbps_limit: u32, pub discard_extent_bytes: u64, pub discard_bitmap_bytes: u64, pub discard_bytes_saved: atomic64_t }
#[repr(C)] pub struct btrfs_commit_stats { pub commit_count: u64, pub max_commit_dur: u64, pub last_commit_dur: u64, pub total_commit_dur: u64, pub critical_section_start_time: u64 }
#[repr(C)] pub struct btrfs_delayed_root { pub lock: spinlock_t, pub nodes: i32, pub node_list: list_head, pub prepare_list: list_head, pub items: atomic_t, pub items_seq: atomic_t, pub wait: wait_queue_head_t }

// The complete btrfs_fs_info layout is retained exactly; external kernel types are unresolved dependencies.
#[repr(C)] pub struct btrfs_fs_info {
    pub chunk_tree_uuid: [u8; BTRFS_UUID_SIZE], pub flags: usize,
    pub tree_root: *mut btrfs_root, pub chunk_root: *mut btrfs_root, pub dev_root: *mut btrfs_root, pub fs_root: *mut btrfs_root, pub quota_root: *mut btrfs_root, pub uuid_root: *mut btrfs_root, pub data_reloc_root: *mut btrfs_root, pub block_group_root: *mut btrfs_root, pub stripe_root: *mut btrfs_root, pub remap_root: *mut btrfs_root, pub log_root_tree: *mut btrfs_root,
    pub global_root_lock: rwlock_t, pub global_root_tree: rb_root, pub fs_roots_radix_lock: spinlock_t, pub fs_roots_radix: radix_tree_root,
    pub block_group_cache_lock: rwlock_t, pub block_group_cache_tree: rb_root_cached, pub free_chunk_space: atomic64_t, pub excluded_extents: extent_io_tree, pub mapping_tree: rb_root_cached, pub mapping_tree_lock: rwlock_t,
    pub global_block_rsv: btrfs_block_rsv, pub trans_block_rsv: btrfs_block_rsv, pub chunk_block_rsv: btrfs_block_rsv, pub remap_block_rsv: btrfs_block_rsv, pub delayed_block_rsv: btrfs_block_rsv, pub delayed_refs_rsv: btrfs_block_rsv, pub treelog_rsv: btrfs_block_rsv, pub empty_block_rsv: btrfs_block_rsv,
    pub generation: u64, pub last_trans_committed: u64, pub last_reloc_trans: u64, pub last_trans_log_full_commit: u64, pub mount_opt: u64, pub compr_wsm: [*mut core::ffi::c_void; BTRFS_NR_COMPRESS_TYPES], pub compress_type: i32, pub compress_level: i32, pub commit_interval: u32, pub max_inline: u64,
    pub running_transaction: *mut btrfs_transaction, pub transaction_throttle: wait_queue_head_t, pub transaction_wait: wait_queue_head_t, pub transaction_blocked_wait: wait_queue_head_t, pub async_submit_wait: wait_queue_head_t,
    pub super_lock: spinlock_t, pub super_copy: *mut btrfs_super_block, pub super_for_commit: *mut btrfs_super_block, pub sb: *mut super_block, pub btree_inode: *mut inode, pub tree_log_mutex: mutex, pub transaction_kthread_mutex: mutex, pub cleaner_mutex: mutex, pub chunk_mutex: mutex, pub remap_mutex: mutex, pub ro_block_group_mutex: mutex, pub stripe_hash_table: *mut btrfs_stripe_hash_table, pub ordered_operations_mutex: mutex, pub commit_root_sem: rw_semaphore, pub cleanup_work_sem: rw_semaphore, pub subvol_sem: rw_semaphore, pub trans_lock: spinlock_t, pub reloc_mutex: mutex, pub reloc_ctl_lock: spinlock_t,
    pub trans_list: list_head, pub dead_roots: list_head, pub caching_block_groups: list_head, pub delayed_iput_lock: spinlock_t, pub delayed_iputs: list_head, pub nr_delayed_iputs: atomic_t, pub delayed_iputs_wait: wait_queue_head_t, pub tree_mod_seq: atomic64_t, pub tree_mod_log_lock: rwlock_t, pub tree_mod_log: rb_root, pub tree_mod_seq_list: list_head, pub async_delalloc_pages: atomic_t, pub ordered_root_lock: spinlock_t, pub ordered_roots: list_head, pub delalloc_root_mutex: mutex, pub delalloc_root_lock: spinlock_t, pub delalloc_roots: list_head,
    pub workers: *mut btrfs_workqueue, pub delalloc_workers: *mut btrfs_workqueue, pub flush_workers: *mut btrfs_workqueue, pub endio_workers: *mut workqueue_struct, pub endio_meta_workers: *mut workqueue_struct, pub rmw_workers: *mut workqueue_struct, pub endio_write_workers: *mut btrfs_workqueue, pub endio_freespace_worker: *mut btrfs_workqueue, pub caching_workers: *mut btrfs_workqueue, pub fixup_workers: *mut workqueue_struct, pub delayed_workers: *mut btrfs_workqueue,
    pub transaction_kthread: *mut task_struct, pub cleaner_kthread: *mut task_struct, pub thread_pool_size: u32, pub space_info_kobj: *mut kobject, pub qgroups_kobj: *mut kobject, pub discard_kobj: *mut kobject, pub stats_read_blocks: percpu_counter, pub dirty_metadata_bytes: percpu_counter, pub delalloc_bytes: percpu_counter, pub ordered_bytes: percpu_counter, pub dirty_metadata_batch: i32, pub delalloc_batch: i32, pub evictable_extent_maps: percpu_counter, pub em_shrinker_last_root: u64, pub em_shrinker_last_ino: u64, pub em_shrinker_nr_to_scan: atomic64_t, pub em_shrinker_work: work_struct, pub dirty_cowonly_roots: list_head, pub fs_devices: *mut btrfs_fs_devices, pub space_info: list_head, pub data_sinfo: *mut btrfs_space_info, pub reloc_ctl: *mut reloc_control, pub data_alloc_cluster: btrfs_free_cluster, pub meta_alloc_cluster: btrfs_free_cluster,
    pub defrag_inodes_lock: spinlock_t, pub defrag_inodes: rb_root, pub defrag_running: atomic_t, pub profiles_lock: seqlock_t, pub avail_data_alloc_bits: u64, pub avail_metadata_alloc_bits: u64, pub avail_system_alloc_bits: u64, pub balance_lock: spinlock_t, pub balance_mutex: mutex, pub balance_pause_req: atomic_t, pub balance_cancel_req: atomic_t, pub balance_ctl: *mut btrfs_balance_control, pub balance_wait_q: wait_queue_head_t, pub reloc_cancel_req: atomic_t, pub data_chunk_allocations: u32, pub metadata_ratio: u32,
    pub scrub_lock: mutex, pub scrubs_running: atomic_t, pub scrub_pause_req: atomic_t, pub scrubs_paused: atomic_t, pub scrub_cancel_req: atomic_t, pub scrub_pause_wait: wait_queue_head_t, pub scrub_workers_refcnt: refcount_t, pub scrub_workers: *mut workqueue_struct, pub discard_ctl: btrfs_discard_ctl, pub qgroup_flags: u64, pub qgroup_tree: rb_root, pub qgroup_lock: spinlock_t, pub qgroup_ioctl_lock: mutex, pub dirty_qgroups: list_head, pub qgroup_seq: u64, pub qgroup_rescan_lock: mutex, pub qgroup_rescan_progress: btrfs_key, pub qgroup_rescan_workers: *mut btrfs_workqueue, pub qgroup_rescan_completion: completion, pub qgroup_rescan_work: btrfs_work, pub qgroup_rescan_running: bool, pub qgroup_drop_subtree_thres: u8, pub qgroup_enable_gen: u64,
    pub fs_error: i32, pub fs_state: usize, pub delayed_root: btrfs_delayed_root, pub buffer_tree: xarray, pub backup_root_index: i32, pub dev_replace: btrfs_dev_replace, pub uuid_tree_rescan_sem: semaphore, pub async_reclaim_work: work_struct, pub async_data_reclaim_work: work_struct, pub preempt_reclaim_work: work_struct, pub reclaim_bgs_work: work_struct, pub reclaim_bgs: list_head, pub bg_reclaim_threshold: i32, pub unused_bgs_lock: spinlock_t, pub unused_bgs: list_head, pub fully_remapped_bgs: list_head, pub unused_bg_unpin_mutex: mutex, pub reclaim_bgs_lock: mutex,
    pub nodesize: u32, pub nodesize_bits: u32, pub sectorsize: u32, pub sectorsize_bits: u32, pub block_min_order: u32, pub block_max_order: u32, pub writeback_bio_size: u32, pub csum_size: u32, pub csums_per_leaf: u32, pub csum_type: u32, pub max_extent_size: u64, pub swapfile_pins_lock: spinlock_t, pub swapfile_pins: rb_root, pub exclusive_operation: btrfs_exclusive_operation, pub zone_size: u64, pub limits: queue_limits, pub max_zone_append_size: u64, pub zoned_meta_io_lock: mutex, pub treelog_bg_lock: spinlock_t, pub treelog_bg: u64, pub relocation_bg_lock: spinlock_t, pub data_reloc_bg: u64, pub zoned_data_reloc_io_lock: mutex, pub active_meta_bg: *mut btrfs_block_group, pub active_system_bg: *mut btrfs_block_group, pub nr_global_roots: u64, pub zone_active_bgs_lock: spinlock_t, pub zone_active_bgs: list_head, pub commit_stats: btrfs_commit_stats, pub last_root_drop_gen: u64, pub btrfs_trans_num_writers_map: lockdep_map, pub btrfs_trans_num_extwriters_map: lockdep_map, pub btrfs_state_change_map: [lockdep_map; 4], pub btrfs_trans_pending_ordered_map: lockdep_map, pub btrfs_ordered_extent_map: lockdep_map,
    #[cfg(feature = "btrfs_debug")] pub ref_verify_lock: spinlock_t, #[cfg(feature = "btrfs_debug")] pub block_tree: rb_root, #[cfg(feature = "btrfs_debug")] pub debug_kobj: *mut kobject, #[cfg(feature = "btrfs_debug")] pub allocated_roots: list_head, #[cfg(feature = "btrfs_debug")] pub eb_leak_lock: spinlock_t, #[cfg(feature = "btrfs_debug")] pub allocated_ebs: list_head,
    pub use_bitmap: Option<unsafe extern "C" fn(*mut btrfs_free_space_ctl, *mut btrfs_free_space) -> bool>,
}

pub const BTRFS_NR_COMPRESS_TYPES: usize = 4;
pub const BTRFS_MAX_EXTENT_ITEM_SIZE: usize = 0; // ((BTRFS_LEAF_DATA_SIZE(r->fs_info) >> 4) - sizeof(struct btrfs_item))

pub unsafe fn btrfs_alloc_write_mask(mapping: *const address_space) -> gfp_t { mapping_gfp_constraint(mapping, !__GFP_FS) }
pub unsafe fn btrfs_min_folio_size(fs_info: *const btrfs_fs_info) -> u32 { 1u32 << (PAGE_SHIFT + (*fs_info).block_min_order) }
pub unsafe fn btrfs_get_fs_generation(fs_info: *const btrfs_fs_info) -> u64 { READ_ONCE((*fs_info).generation) }
pub unsafe fn btrfs_set_fs_generation(fs_info: *mut btrfs_fs_info, gen: u64) { WRITE_ONCE((*fs_info).generation, gen) }
pub unsafe fn btrfs_get_last_trans_committed(fs_info: *const btrfs_fs_info) -> u64 { READ_ONCE((*fs_info).last_trans_committed) }
pub unsafe fn btrfs_set_last_trans_committed(fs_info: *mut btrfs_fs_info, gen: u64) { WRITE_ONCE((*fs_info).last_trans_committed, gen) }
pub unsafe fn btrfs_set_last_root_drop_gen(fs_info: *mut btrfs_fs_info, gen: u64) { WRITE_ONCE((*fs_info).last_root_drop_gen, gen) }
pub unsafe fn btrfs_get_last_root_drop_gen(fs_info: *const btrfs_fs_info) -> u64 { READ_ONCE((*fs_info).last_root_drop_gen) }
pub unsafe fn btrfs_csum_bytes_to_leaves(fs_info: *const btrfs_fs_info, csum_bytes: u64) -> u64 { DIV_ROUND_UP_ULL(csum_bytes >> (*fs_info).sectorsize_bits, (*fs_info).csums_per_leaf as u64) }
pub unsafe fn btrfs_calc_insert_metadata_size(fs_info: *const btrfs_fs_info, num_items: u32) -> u64 { (*fs_info).nodesize as u64 * BTRFS_MAX_LEVEL as u64 * 2 * num_items as u64 }
pub unsafe fn btrfs_calc_metadata_size(fs_info: *const btrfs_fs_info, num_items: u32) -> u64 { (*fs_info).nodesize as u64 * BTRFS_MAX_LEVEL as u64 * num_items as u64 }
pub unsafe fn btrfs_is_zoned(fs_info: *const btrfs_fs_info) -> bool { IS_ENABLED(CONFIG_BLK_DEV_ZONED) && (*fs_info).zone_size > 0 }
pub unsafe fn count_max_extents(fs_info: *const btrfs_fs_info, size: u64) -> u32 { div_u64(size + if fs_info.is_null() { BTRFS_MAX_EXTENT_SIZE as u64 } else { (*fs_info).max_extent_size } - 1, if fs_info.is_null() { BTRFS_MAX_EXTENT_SIZE as u64 } else { (*fs_info).max_extent_size }) as u32 }
pub unsafe fn btrfs_blocks_per_folio(fs_info: *const btrfs_fs_info, folio: *const folio) -> u32 { (folio_size(folio) >> (*fs_info).sectorsize_bits) as u32 }

extern "C" {
    pub fn btrfs_supported_blocksize(blocksize: u32) -> bool;
    pub fn btrfs_exclop_start(fs_info: *mut btrfs_fs_info, ty: btrfs_exclusive_operation) -> bool;
    pub fn btrfs_exclop_start_try_lock(fs_info: *mut btrfs_fs_info, ty: btrfs_exclusive_operation) -> bool;
    pub fn btrfs_exclop_start_unlock(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_exclop_finish(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_exclop_balance(fs_info: *mut btrfs_fs_info, op: btrfs_exclusive_operation);
    pub fn btrfs_check_ioctl_vol_args_path(vol_args: *const btrfs_ioctl_vol_args) -> i32;
    pub fn btrfs_csum_type_size(ty: u16) -> u16;
    pub fn btrfs_super_csum_size(s: *const btrfs_super_block) -> i32;
    pub fn btrfs_super_csum_name(csum_type: u16) -> *const core::ffi::c_char;
    pub fn btrfs_get_num_csums() -> usize;
    pub fn btrfs_csum(csum_type: u16, data: *const u8, len: usize, out: *mut u8);
    pub fn btrfs_csum_init(ctx: *mut btrfs_csum_ctx, csum_type: u16);
    pub fn btrfs_csum_update(ctx: *mut btrfs_csum_ctx, data: *const u8, len: usize);
    pub fn btrfs_csum_final(ctx: *mut btrfs_csum_ctx, out: *mut u8);
    pub fn __btrfs_set_fs_incompat(fs_info: *mut btrfs_fs_info, flag: u64, name: *const core::ffi::c_char);
    pub fn __btrfs_clear_fs_incompat(fs_info: *mut btrfs_fs_info, flag: u64, name: *const core::ffi::c_char);
    pub fn __btrfs_set_fs_compat_ro(fs_info: *mut btrfs_fs_info, flag: u64, name: *const core::ffi::c_char);
    pub fn __btrfs_clear_fs_compat_ro(fs_info: *mut btrfs_fs_info, flag: u64, name: *const core::ffi::c_char);
}

#[repr(C)] pub union btrfs_csum_state { pub crc32: u32, pub xxh64: xxh64_state, pub sha256: sha256_ctx, pub blake2b: blake2b_ctx }
#[repr(C)] pub struct btrfs_csum_ctx { pub csum_type: u16, pub state: btrfs_csum_state }
pub unsafe fn btrfs_is_empty_uuid(uuid: *const u8) -> bool { uuid_is_null(uuid as *const uuid_t) }
pub unsafe fn btrfs_is_full_ro(fs_info: *const btrfs_fs_info) -> bool { !sb_rdonly((*fs_info).sb) && false || (sb_rdonly((*fs_info).sb) && ((*fs_info).mount_opt & BTRFS_MOUNT_FULL_RO_MASK) != 0) }
pub unsafe fn btrfs_fs_closing(fs_info: *const btrfs_fs_info) -> bool { test_bit(BTRFS_FS_CLOSING_START, &(*fs_info).flags) }
pub unsafe fn btrfs_fs_closing_done(fs_info: *const btrfs_fs_info) -> bool { btrfs_fs_closing(fs_info) && test_bit(BTRFS_FS_CLOSING_DONE, &(*fs_info).flags) }
pub unsafe fn btrfs_need_cleaner_sleep(fs_info: *const btrfs_fs_info) -> i32 { (test_bit(BTRFS_FS_STATE_RO, &(*fs_info).fs_state) || btrfs_fs_closing(fs_info)) as i32 }
pub unsafe fn btrfs_wake_unfinished_drop(fs_info: *mut btrfs_fs_info) { clear_and_wake_up_bit(BTRFS_FS_UNFINISHED_DROPS, &mut (*fs_info).flags) }
pub unsafe fn btrfs_is_shutdown(fs_info: *const btrfs_fs_info) -> bool { test_bit(BTRFS_FS_STATE_EMERGENCY_SHUTDOWN, &(*fs_info).fs_state) }
pub unsafe fn btrfs_force_shutdown(fs_info: *mut btrfs_fs_info) { WRITE_ONCE((*fs_info).fs_error, -EIO); if !test_and_set_bit(BTRFS_FS_STATE_EMERGENCY_SHUTDOWN, &mut (*fs_info).fs_state) { btrfs_crit(fs_info, "emergency shutdown"); fserror_report_shutdown((*fs_info).sb, GFP_KERNEL); } }

// CONFIG_BTRFS_FS_RUN_SANITY_TESTS controls test-only visibility and the dummy-fs check.
pub unsafe fn btrfs_is_testing(fs_info: *const btrfs_fs_info) -> bool { test_bit(BTRFS_FS_STATE_DUMMY_FS_INFO, &(*fs_info).fs_state) }

// C preprocessor helpers retained as Rust equivalents where a direct item is possible.
#[inline] pub unsafe fn btrfs_clear_opt(o: &mut u64, opt: u64) { *o &= !opt; }
#[inline] pub unsafe fn btrfs_set_opt(o: &mut u64, opt: u64) { *o |= opt; }
#[inline] pub fn btrfs_raw_test_opt(o: u64, opt: u64) -> u64 { o & opt }
#[inline] pub unsafe fn btrfs_test_opt(fs_info: *const btrfs_fs_info, opt: u64) -> bool { ((*fs_info).mount_opt & opt) != 0 }
#[inline] pub unsafe fn folio_test_fixup_pending(folio: *const folio) -> bool { folio_test_owner_2(folio) }
#[inline] pub unsafe fn folio_set_fixup_pending(folio: *mut folio) { folio_set_owner_2(folio); }
#[inline] pub unsafe fn folio_clear_fixup_pending(folio: *mut folio) { folio_clear_owner_2(folio); }
#[inline] pub unsafe fn btrfs_fs_error(fs_info: *const btrfs_fs_info) -> i32 { READ_ONCE((*fs_info).fs_error) }
#[inline] pub unsafe fn btrfs_fs_log_cleanup_error(fs_info: *const btrfs_fs_info) -> bool { test_bit(BTRFS_FS_STATE_LOG_CLEANUP_ERROR, &(*fs_info).fs_state) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
