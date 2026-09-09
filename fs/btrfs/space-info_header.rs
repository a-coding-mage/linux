/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translation units: trace/events/btrfs.h,
// linux/spinlock.h, linux/list.h, linux/kobject.h, linux/lockdep.h,
// linux/wait.h, linux/rwsem.h, and volumes.h.

pub enum btrfs_fs_info {}
pub enum btrfs_block_group {}

/* Different levels for flushing space when doing space reservations. */
#[repr(C)]
pub enum btrfs_reserve_flush_enum {
    BTRFS_RESERVE_NO_FLUSH,
    BTRFS_RESERVE_FLUSH_LIMIT,
    BTRFS_RESERVE_FLUSH_EVICT,
    BTRFS_RESERVE_FLUSH_DATA,
    BTRFS_RESERVE_FLUSH_FREE_SPACE_INODE,
    BTRFS_RESERVE_FLUSH_ALL,
    BTRFS_RESERVE_FLUSH_ALL_STEAL,
    BTRFS_RESERVE_FLUSH_ZONED_RELOCATION,
    BTRFS_RESERVE_FLUSH_EMERGENCY,
}

#[repr(C)]
pub enum btrfs_flush_state {
    FLUSH_DELAYED_ITEMS_NR = 1,
    FLUSH_DELAYED_ITEMS = 2,
    FLUSH_DELAYED_REFS_NR = 3,
    FLUSH_DELAYED_REFS = 4,
    FLUSH_DELALLOC = 5,
    FLUSH_DELALLOC_WAIT = 6,
    FLUSH_DELALLOC_FULL = 7,
    ALLOC_CHUNK = 8,
    ALLOC_CHUNK_FORCE = 9,
    RUN_DELAYED_IPUTS = 10,
    COMMIT_TRANS = 11,
    RESET_ZONES = 12,
    RECLAIM_ZONES = 13,
}

#[repr(C)]
pub enum btrfs_space_info_sub_group {
    BTRFS_SUB_GROUP_PRIMARY,
    BTRFS_SUB_GROUP_DATA_RELOC,
    BTRFS_SUB_GROUP_TREELOG,
}

pub const BTRFS_SPACE_INFO_SUB_GROUP_MAX: usize = 1;

#[repr(C)]
pub struct btrfs_space_info {
    pub fs_info: *mut btrfs_fs_info,
    pub parent: *mut btrfs_space_info,
    pub sub_group: [*mut btrfs_space_info; BTRFS_SPACE_INFO_SUB_GROUP_MAX],
    pub subgroup_id: i32,
    pub lock: spinlock_t,
    pub total_bytes: u64,
    pub bytes_used: u64,
    pub bytes_pinned: u64,
    pub bytes_reserved: u64,
    pub bytes_may_use: u64,
    pub bytes_readonly: u64,
    pub bytes_zone_unusable: u64,
    pub max_extent_size: u64,
    pub chunk_size: u64,
    pub bg_reclaim_threshold: i32,
    pub clamp: i32,
    pub full: bool,
    pub chunk_alloc: bool,
    pub flush: bool,
    pub force_alloc: u32,
    pub disk_used: u64,
    pub disk_total: u64,
    pub flags: u64,
    pub list: list_head,
    pub ro_bgs: list_head,
    pub priority_tickets: list_head,
    pub tickets: list_head,
    pub reclaim_size: u64,
    pub tickets_id: u64,
    pub groups_sem: rw_semaphore,
    pub block_groups: [list_head; BTRFS_NR_RAID_TYPES],
    pub kobj: kobject,
    pub block_group_kobjs: [*mut kobject; BTRFS_NR_RAID_TYPES],
    pub reclaim_count: u64,
    pub reclaim_bytes: u64,
    pub reclaim_errors: u64,
    pub dynamic_reclaim: bool,
    pub periodic_reclaim: bool,
    pub periodic_reclaim_ready: bool,
    pub reclaimable_bytes: i64,
}

pub const fn btrfs_mixed_space_info(space_info: *const btrfs_space_info) -> bool {
    unsafe { ((*space_info).flags & BTRFS_BLOCK_GROUP_METADATA) != 0 && ((*space_info).flags & BTRFS_BLOCK_GROUP_DATA) != 0 }
}

macro_rules! DECLARE_SPACE_INFO_UPDATE {
    ($name:ident, $trace_name:literal) => {
        pub unsafe fn btrfs_space_info_update_$name(sinfo: *mut btrfs_space_info, bytes: i64) {
            let fs_info = (*sinfo).fs_info;
            let abs_bytes: u64 = if bytes < 0 { (-bytes) as u64 } else { bytes as u64 };
            lockdep_assert_held(&(*sinfo).lock);
            trace_update_$name(fs_info, sinfo, (*sinfo).$name, bytes);
            trace_btrfs_space_reservation(fs_info, $trace_name, (*sinfo).flags, abs_bytes, bytes > 0);
            if bytes < 0 && (*sinfo).$name < (-bytes) as u64 {
                WARN_ON(1);
                (*sinfo).$name = 0;
                return;
            }
            (*sinfo).$name = ((*sinfo).$name as i64 + bytes) as u64;
        }
    };
}

DECLARE_SPACE_INFO_UPDATE!(bytes_may_use, "space_info");
DECLARE_SPACE_INFO_UPDATE!(bytes_pinned, "pinned");
DECLARE_SPACE_INFO_UPDATE!(bytes_zone_unusable, "zone_unusable");

pub unsafe fn btrfs_space_info_used(s_info: *const btrfs_space_info, may_use_included: bool) -> u64 {
    lockdep_assert_held(&(*s_info).lock);
    (*s_info).bytes_used + (*s_info).bytes_reserved + (*s_info).bytes_pinned +
        (*s_info).bytes_readonly + (*s_info).bytes_zone_unusable +
        if may_use_included { (*s_info).bytes_may_use } else { 0 }
}

extern "C" {
    pub fn btrfs_init_space_info(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_add_bg_to_space_info(info: *mut btrfs_fs_info, block_group: *mut btrfs_block_group);
    pub fn btrfs_update_space_info_chunk_size(space_info: *mut btrfs_space_info, chunk_size: u64);
    pub fn btrfs_find_space_info(info: *const btrfs_fs_info, flags: u64) -> *mut btrfs_space_info;
    pub fn btrfs_clear_space_info_full(info: *mut btrfs_fs_info);
    pub fn btrfs_dump_space_info(info: *mut btrfs_space_info, bytes: u64, dump_block_groups: bool);
    pub fn btrfs_reserve_metadata_bytes(space_info: *mut btrfs_space_info, orig_bytes: u64, flush: btrfs_reserve_flush_enum) -> i32;
    pub fn btrfs_try_granting_tickets(space_info: *mut btrfs_space_info);
    pub fn btrfs_can_overcommit(space_info: *const btrfs_space_info, bytes: u64, flush: btrfs_reserve_flush_enum) -> bool;
    pub fn btrfs_reserve_data_bytes(space_info: *mut btrfs_space_info, bytes: u64, flush: btrfs_reserve_flush_enum) -> i32;
    pub fn btrfs_dump_space_info_for_trans_abort(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_init_async_reclaim_work(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_account_ro_block_groups_free_space(sinfo: *mut btrfs_space_info) -> u64;
    pub fn btrfs_space_info_update_reclaimable(space_info: *mut btrfs_space_info, bytes: i64);
    pub fn btrfs_set_periodic_reclaim_ready(space_info: *mut btrfs_space_info, ready: bool);
    pub fn btrfs_calc_reclaim_threshold(space_info: *const btrfs_space_info) -> i32;
    pub fn btrfs_reclaim_sweep(fs_info: *const btrfs_fs_info);
    pub fn btrfs_return_free_space(space_info: *mut btrfs_space_info, len: u64);
}

pub unsafe fn btrfs_space_info_free_bytes_may_use(space_info: *mut btrfs_space_info, num_bytes: u64) {
    spin_lock(&mut (*space_info).lock);
    btrfs_space_info_update_bytes_may_use(space_info, -(num_bytes as i64));
    btrfs_try_granting_tickets(space_info);
    spin_unlock(&mut (*space_info).lock);
}

pub unsafe fn btrfs_space_info_type_str(space_info: *const btrfs_space_info) -> *const u8 {
    match (*space_info).flags {
        BTRFS_BLOCK_GROUP_SYSTEM => b"SYSTEM\0".as_ptr(),
        x if x == (BTRFS_BLOCK_GROUP_METADATA | BTRFS_BLOCK_GROUP_DATA) => b"DATA+METADATA\0".as_ptr(),
        BTRFS_BLOCK_GROUP_DATA => b"DATA\0".as_ptr(),
        BTRFS_BLOCK_GROUP_METADATA => b"METADATA\0".as_ptr(),
        _ => b"UNKNOWN\0".as_ptr(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
