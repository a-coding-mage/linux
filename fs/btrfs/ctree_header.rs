/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from ctree.h; included dependencies are supplied externally. */

/* C forward declarations and kernel types are intentionally referenced as external Rust types. */
#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub enum extent_buffer {}
#[allow(non_camel_case_types)] pub enum btrfs_block_rsv {}
#[allow(non_camel_case_types)] pub enum btrfs_trans_handle {}
#[allow(non_camel_case_types)] pub enum btrfs_block_group {}

/* READA values for struct btrfs_path.reada. */
#[repr(C)]
pub enum btrfs_reada { READA_NONE, READA_BACK, READA_FORWARD, READA_FORWARD_ALWAYS }

#[repr(C)]
pub struct btrfs_path {
    pub nodes: [*mut extent_buffer; BTRFS_MAX_LEVEL],
    pub slots: [i32; BTRFS_MAX_LEVEL],
    pub locks: [u8; BTRFS_MAX_LEVEL],
    pub reada: u8,
    pub lowest_level: u8,
    pub search_for_split: bool,
    pub keep_locks: bool,
    pub skip_locking: bool,
    pub search_commit_root: bool,
    pub need_commit_sem: bool,
    pub skip_release_on_error: bool,
    pub search_for_extension: bool,
    pub nowait: bool,
}

/* The state of a btrfs root. */
pub const BTRFS_ROOT_IN_TRANS_SETUP: u32 = 0;
pub const BTRFS_ROOT_SHAREABLE: u32 = 1;
pub const BTRFS_ROOT_TRACK_DIRTY: u32 = 2;
pub const BTRFS_ROOT_IN_RADIX: u32 = 3;
pub const BTRFS_ROOT_ORPHAN_ITEM_INSERTED: u32 = 4;
pub const BTRFS_ROOT_DEFRAG_RUNNING: u32 = 5;
pub const BTRFS_ROOT_FORCE_COW: u32 = 6;
pub const BTRFS_ROOT_DIRTY: u32 = 7;
pub const BTRFS_ROOT_DELETING: u32 = 8;
pub const BTRFS_ROOT_DEAD_RELOC_TREE: u32 = 9;
pub const BTRFS_ROOT_DEAD_TREE: u32 = 10;
pub const BTRFS_ROOT_HAS_LOG_TREE: u32 = 11;
pub const BTRFS_ROOT_QGROUP_FLUSHING: u32 = 12;
pub const BTRFS_ROOT_ORPHAN_CLEANUP: u32 = 13;
pub const BTRFS_ROOT_UNFINISHED_DROP: u32 = 14;
pub const BTRFS_ROOT_RESET_LOCKDEP_CLASS: u32 = 15;

#[repr(C)] pub struct btrfs_qgroup_swapped_blocks {
    pub lock: spinlock_t, pub swapped: bool,
    pub blocks: [rb_root; BTRFS_MAX_LEVEL],
}

#[repr(C)]
pub struct btrfs_root {
    pub rb_node: rb_node,
    pub node: *mut extent_buffer,
    pub commit_root: *mut extent_buffer,
    pub log_root: *mut btrfs_root,
    pub reloc_root: *mut btrfs_root,
    pub state: usize,
    pub root_item: btrfs_root_item,
    pub root_key: btrfs_key,
    pub fs_info: *mut btrfs_fs_info,
    pub dirty_log_pages: extent_io_tree,
    pub objectid_mutex: mutex,
    pub accounting_lock: spinlock_t,
    pub block_rsv: *mut btrfs_block_rsv,
    pub log_mutex: mutex,
    pub log_writer_wait: wait_queue_head_t,
    pub log_commit_wait: [wait_queue_head_t; 2],
    pub log_ctxs: [list_head; 2],
    pub log_writers: atomic_t,
    pub log_commit: [bool; 2],
    pub log_transid: i32,
    pub log_transid_committed: i32,
    pub last_log_commit: i32,
    pub last_trans: u64,
    pub free_objectid: u64,
    pub defrag_progress: btrfs_key,
    pub defrag_max: btrfs_key,
    pub dirty_list: list_head,
    pub root_list: list_head,
    pub inodes: xarray,
    pub delayed_nodes: xarray,
    pub anon_dev: dev_t,
    pub root_item_lock: spinlock_t,
    pub refs: refcount_t,
    pub delalloc_mutex: mutex,
    pub delalloc_lock: spinlock_t,
    pub delalloc_inodes: list_head,
    pub delalloc_root: list_head,
    pub nr_delalloc_inodes: u64,
    pub ordered_extent_mutex: mutex,
    pub ordered_extent_lock: spinlock_t,
    pub ordered_extents: list_head,
    pub ordered_root: list_head,
    pub nr_ordered_extents: u64,
    pub reloc_dirty_list: list_head,
    pub send_in_progress: i32,
    pub dedupe_in_progress: i32,
    pub snapshot_lock: btrfs_drew_lock,
    pub snapshot_force_cow: atomic_t,
    pub qgroup_meta_rsv_lock: spinlock_t,
    pub qgroup_meta_rsv_pertrans: u64,
    pub qgroup_meta_rsv_prealloc: u64,
    pub qgroup_flush_wait: wait_queue_head_t,
    pub nr_swapfiles: atomic_t,
    pub swapped_blocks: btrfs_qgroup_swapped_blocks,
    pub log_csum_range: extent_io_tree,
    pub relocation_src_root: u64,
}

#[inline] pub unsafe fn btrfs_root_readonly(root: *const btrfs_root) -> bool { ((*root).root_item.flags & cpu_to_le64(BTRFS_ROOT_SUBVOL_RDONLY)) != 0 }
#[inline] pub unsafe fn btrfs_root_dead(root: *const btrfs_root) -> bool { ((*root).root_item.flags & cpu_to_le64(BTRFS_ROOT_SUBVOL_DEAD)) != 0 }
#[inline] pub unsafe fn btrfs_root_id(root: *const btrfs_root) -> u64 { (*root).root_key.objectid }
#[inline] pub unsafe fn btrfs_get_root_log_transid(root: *const btrfs_root) -> i32 { READ_ONCE((*root).log_transid) }
#[inline] pub unsafe fn btrfs_set_root_log_transid(root: *mut btrfs_root, v: i32) { WRITE_ONCE((*root).log_transid, v); }
#[inline] pub unsafe fn btrfs_get_root_last_log_commit(root: *const btrfs_root) -> i32 { READ_ONCE((*root).last_log_commit) }
#[inline] pub unsafe fn btrfs_set_root_last_log_commit(root: *mut btrfs_root, v: i32) { WRITE_ONCE((*root).last_log_commit, v); }
#[inline] pub unsafe fn btrfs_get_root_last_trans(root: *const btrfs_root) -> u64 { READ_ONCE((*root).last_trans) }
#[inline] pub unsafe fn btrfs_set_root_last_trans(root: *mut btrfs_root, v: u64) { WRITE_ONCE((*root).last_trans, v); }
#[inline] pub unsafe fn btrfs_root_origin_generation(root: *const btrfs_root) -> u64 {
    if btrfs_root_id(root) == BTRFS_TREE_RELOC_OBJECTID { btrfs_root_last_snapshot(&(*root).root_item) } else { (*root).root_key.offset }
}

#[repr(C)] pub struct btrfs_replace_extent_info {
    pub disk_offset: u64, pub disk_len: u64, pub data_offset: u64, pub data_len: u64,
    pub file_offset: u64, pub extent_buf: *mut i8, pub is_new_extent: bool,
    pub update_times: bool, pub qgroup_reserved: i32, pub insertions: i32,
}
#[repr(C)] pub struct btrfs_drop_extents_args {
    pub path: *mut btrfs_path, pub start: u64, pub end: u64, pub drop_cache: bool,
    pub replace_extent: bool, pub extent_item_size: u32, pub drop_end: u64,
    pub bytes_found: u64, pub extent_inserted: bool,
}
#[repr(C)] pub struct btrfs_file_private {
    pub filldir_buf: *mut core::ffi::c_void, pub last_index: u64,
    pub llseek_cached_state: *mut extent_state, pub owner_task: *mut task_struct,
}

#[inline] pub unsafe fn BTRFS_LEAF_DATA_SIZE(info: *const btrfs_fs_info) -> u32 { (*info).nodesize - core::mem::size_of::<btrfs_header>() as u32 }
#[inline] pub unsafe fn BTRFS_MAX_ITEM_SIZE(info: *const btrfs_fs_info) -> u32 { BTRFS_LEAF_DATA_SIZE(info) - core::mem::size_of::<btrfs_item>() as u32 }
#[inline] pub unsafe fn BTRFS_NODEPTRS_PER_BLOCK(info: *const btrfs_fs_info) -> u32 { BTRFS_LEAF_DATA_SIZE(info) / core::mem::size_of::<btrfs_key_ptr>() as u32 }
#[inline] pub unsafe fn BTRFS_MAX_XATTR_SIZE(info: *const btrfs_fs_info) -> u32 { BTRFS_MAX_ITEM_SIZE(info) - core::mem::size_of::<btrfs_dir_item>() as u32 }

extern "C" {
    pub fn btrfs_ctree_init() -> i32; pub fn btrfs_ctree_exit();
    pub fn btrfs_bin_search(eb: *const extent_buffer, first_slot: i32, key: *const btrfs_key, slot: *mut i32) -> i32;
    pub fn btrfs_comp_cpu_keys(k1: *const btrfs_key, k2: *const btrfs_key) -> i32;
    pub fn btrfs_previous_item(root: *mut btrfs_root, path: *mut btrfs_path, min_objectid: u64, type_: i32) -> i32;
    pub fn btrfs_previous_extent_item(root: *mut btrfs_root, path: *mut btrfs_path, min_objectid: u64) -> i32;
    pub fn btrfs_set_item_key_safe(trans: *mut btrfs_trans_handle, path: *const btrfs_path, key: *const btrfs_key);
    pub fn btrfs_root_node(root: *mut btrfs_root) -> *mut extent_buffer;
    pub fn btrfs_find_next_key(root: *mut btrfs_root, path: *mut btrfs_path, key: *mut btrfs_key, lowest_level: i32, min_trans: u64) -> i32;
    pub fn btrfs_search_forward(root: *mut btrfs_root, key: *mut btrfs_key, path: *mut btrfs_path, min_trans: u64) -> i32;
    pub fn btrfs_read_node_slot(parent: *mut extent_buffer, slot: i32) -> *mut extent_buffer;
    pub fn btrfs_cow_block(trans:*mut btrfs_trans_handle, root:*mut btrfs_root, buf:*mut extent_buffer, parent:*mut extent_buffer, parent_slot:i32, cow_ret:*mut *mut extent_buffer, nest:i32) -> i32;
    pub fn btrfs_force_cow_block(trans:*mut btrfs_trans_handle, root:*mut btrfs_root, buf:*mut extent_buffer, parent:*mut extent_buffer, parent_slot:i32, cow_ret:*mut *mut extent_buffer, search_start:u64, empty_size:u64, nest:i32) -> i32;
    pub fn btrfs_copy_root(trans:*mut btrfs_trans_handle, root:*mut btrfs_root, buf:*mut extent_buffer, cow_ret:*mut *mut extent_buffer, new_root_objectid:u64) -> i32;
    pub fn btrfs_block_can_be_shared(trans:*const btrfs_trans_handle, root:*const btrfs_root, buf:*const extent_buffer) -> bool;
    pub fn btrfs_extend_item(trans:*mut btrfs_trans_handle, path:*const btrfs_path, data_size:u32);
    pub fn btrfs_truncate_item(trans:*mut btrfs_trans_handle, path:*const btrfs_path, new_size:u32, from_end:i32);
    pub fn btrfs_split_item(trans:*mut btrfs_trans_handle, root:*mut btrfs_root, path:*mut btrfs_path, key:*const btrfs_key, split_offset:usize) -> i32;
    pub fn btrfs_duplicate_item(trans:*mut btrfs_trans_handle, root:*mut btrfs_root, path:*mut btrfs_path, key:*const btrfs_key) -> i32;
    pub fn btrfs_find_item(fs_root:*mut btrfs_root, path:*mut btrfs_path, inum:u64, ioff:u64, key_type:u8, found_key:*mut btrfs_key) -> i32;
    pub fn btrfs_search_slot(trans:*mut btrfs_trans_handle, root:*mut btrfs_root, key:*const btrfs_key, p:*mut btrfs_path, ins_len:i32, cow:i32) -> i32;
    pub fn btrfs_search_old_slot(root:*mut btrfs_root, key:*const btrfs_key, p:*mut btrfs_path, time_seq:u64) -> i32;
    pub fn btrfs_search_slot_for_read(root:*mut btrfs_root, key:*const btrfs_key, p:*mut btrfs_path, find_higher:i32, return_any:i32) -> i32;
    pub fn btrfs_setup_item_for_insert(trans:*mut btrfs_trans_handle, root:*mut btrfs_root, path:*mut btrfs_path, key:*const btrfs_key, data_size:u32);
    pub fn btrfs_insert_item(trans:*mut btrfs_trans_handle, root:*mut btrfs_root, key:*const btrfs_key, data:*mut core::ffi::c_void, data_size:u32) -> i32;
    pub fn btrfs_next_old_item(root:*mut btrfs_root, path:*mut btrfs_path, time_seq:u64) -> i32;
    pub fn btrfs_get_next_valid_item(root:*mut btrfs_root, key:*mut btrfs_key, path:*mut btrfs_path) -> i32;
    pub fn btrfs_release_path(path: *mut btrfs_path); pub fn btrfs_alloc_path() -> *mut btrfs_path; pub fn btrfs_free_path(path: *mut btrfs_path);
    pub fn btrfs_del_items(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, path: *mut btrfs_path, slot: i32, nr: i32) -> i32;
    pub fn btrfs_insert_empty_items(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, path: *mut btrfs_path, batch: *const btrfs_item_batch) -> i32;
    pub fn btrfs_next_old_leaf(root: *mut btrfs_root, path: *mut btrfs_path, time_seq: u64) -> i32;
    pub fn btrfs_search_backwards(root:*mut btrfs_root, key:*mut btrfs_key, path:*mut btrfs_path) -> i32;
    pub fn btrfs_next_old_item(root: *mut btrfs_root, path: *mut btrfs_path, time_seq: u64) -> i32;
    pub fn btrfs_leaf_free_space(leaf: *const extent_buffer) -> i32;
}

#[inline] pub unsafe fn btrfs_comp_keys(disk_key: *const btrfs_disk_key, k2: *const btrfs_key) -> i32 {
    /* On little-endian systems disk and CPU key order are identical. */
    btrfs_comp_cpu_keys(disk_key as *const btrfs_key, k2)
}

/* C cleanup attributes are represented by explicit ownership/release at call sites. */
#[macro_export] macro_rules! BTRFS_PATH_AUTO_FREE { ($name:ident) => { let mut $name: *mut btrfs_path = core::ptr::null_mut(); }; }
#[macro_export] macro_rules! BTRFS_PATH_AUTO_RELEASE { ($name:ident) => { let mut $name: btrfs_path = unsafe { core::mem::zeroed() }; }; }

#[repr(C)] pub struct btrfs_item_batch { pub keys: *const btrfs_key, pub data_sizes: *const u32, pub total_data_size: u32, pub nr: i32 }
#[inline] pub unsafe fn btrfs_del_item(t: *mut btrfs_trans_handle, r: *mut btrfs_root, p: *mut btrfs_path) -> i32 { btrfs_del_items(t, r, p, (*p).slots[0], 1) }
#[inline] pub unsafe fn btrfs_insert_empty_item(t: *mut btrfs_trans_handle, r: *mut btrfs_root, p: *mut btrfs_path, k: *const btrfs_key, size: u32) -> i32 { let b = btrfs_item_batch { keys:k, data_sizes:&size, total_data_size:size, nr:1 }; btrfs_insert_empty_items(t,r,p,&b) }
#[inline] pub unsafe fn btrfs_next_leaf(r: *mut btrfs_root, p: *mut btrfs_path) -> i32 { btrfs_next_old_leaf(r,p,0) }
#[inline] pub unsafe fn btrfs_next_item(r: *mut btrfs_root, p: *mut btrfs_path) -> i32 { btrfs_next_old_item(r,p,0) }
#[inline] pub unsafe fn btrfs_is_fstree(rootid: u64) -> bool { if rootid == BTRFS_FS_TREE_OBJECTID { return true; } if (rootid as i64) < (BTRFS_FIRST_FREE_OBJECTID as i64) { return false; } if btrfs_qgroup_level(rootid) != 0 { return false; } true }
#[inline] pub unsafe fn btrfs_is_data_reloc_root(root: *const btrfs_root) -> bool { (*root).root_key.objectid == BTRFS_DATA_RELOC_TREE_OBJECTID }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
