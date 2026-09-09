/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from btrfs/extent-io-tree.h. */

use core::ffi::c_void;

/* Types supplied by the surrounding kernel translation. */
pub struct extent_changeset;
pub struct btrfs_fs_info;
pub struct btrfs_inode;
pub struct rb_root;
pub struct rb_node;
pub struct spinlock_t;
pub struct wait_queue_head_t;
pub struct refcount_t;
pub struct list_head;

/* Bits for the extent state. */
pub const EXTENT_DIRTY: u32 = 1 << 0;
pub const EXTENT_LOCKED: u32 = 1 << 1;
pub const EXTENT_DIO_LOCKED: u32 = 1 << 2;
pub const EXTENT_DIRTY_LOG1: u32 = 1 << 3;
pub const EXTENT_DIRTY_LOG2: u32 = 1 << 4;
pub const EXTENT_DELALLOC: u32 = 1 << 5;
pub const EXTENT_DEFRAG: u32 = 1 << 6;
pub const EXTENT_BOUNDARY: u32 = 1 << 7;
pub const EXTENT_NODATASUM: u32 = 1 << 8;
pub const EXTENT_CLEAR_META_RESV: u32 = 1 << 9;
pub const EXTENT_NEED_WAIT: u32 = 1 << 10;
pub const EXTENT_NORESERVE: u32 = 1 << 11;
pub const EXTENT_QGROUP_RESERVED: u32 = 1 << 12;
pub const EXTENT_CLEAR_DATA_RESV: u32 = 1 << 13;
/* Must be cleared only during ordered extent completion or on error paths. */
pub const EXTENT_DELALLOC_NEW: u32 = 1 << 14;
/* Mark that a range is being locked for finishing an ordered extent. */
pub const EXTENT_FINISHING_ORDERED: u32 = 1 << 15;
/* Indicates inode byte accounting when clearing a new delalloc range. */
pub const EXTENT_ADD_INODE_BYTES: u32 = 1 << 16;
/* Set during truncate when clearing an entire range. */
pub const EXTENT_CLEAR_ALL_BITS: u32 = 1 << 17;
/* This must be last; a request for NOWAIT semantics. */
pub const EXTENT_NOWAIT: u32 = 1 << 18;

pub const EXTENT_DO_ACCOUNTING: u32 = EXTENT_CLEAR_META_RESV | EXTENT_CLEAR_DATA_RESV;
pub const EXTENT_CTLBITS: u32 = EXTENT_DO_ACCOUNTING | EXTENT_ADD_INODE_BYTES | EXTENT_CLEAR_ALL_BITS;
pub const EXTENT_LOCK_BITS: u32 = EXTENT_LOCKED | EXTENT_DIO_LOCKED;

pub const CHUNK_ALLOCATED: u32 = EXTENT_DIRTY;
pub const CHUNK_TRIMMED: u32 = EXTENT_DEFRAG;
pub const CHUNK_STATE_MASK: u32 = CHUNK_ALLOCATED | CHUNK_TRIMMED;

pub const IO_TREE_FS_PINNED_EXTENTS: u32 = 0;
pub const IO_TREE_FS_EXCLUDED_EXTENTS: u32 = 1;
pub const IO_TREE_BTREE_INODE_IO: u32 = 2;
pub const IO_TREE_INODE_IO: u32 = 3;
pub const IO_TREE_RELOC_BLOCKS: u32 = 4;
pub const IO_TREE_TRANS_DIRTY_PAGES: u32 = 5;
pub const IO_TREE_ROOT_DIRTY_LOG_PAGES: u32 = 6;
pub const IO_TREE_INODE_FILE_EXTENT: u32 = 7;
pub const IO_TREE_LOG_CSUM_RANGE: u32 = 8;
pub const IO_TREE_SELFTEST: u32 = 9;
pub const IO_TREE_DEVICE_ALLOC_STATE: u32 = 10;

#[repr(C)]
pub union extent_io_tree_owner {
    pub fs_info: *mut btrfs_fs_info,
    pub inode: *mut btrfs_inode,
}

#[repr(C)]
pub struct extent_io_tree {
    pub state: rb_root,
    pub owner_data: extent_io_tree_owner,
    pub owner: u8,
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct extent_state {
    pub start: u64,
    pub end: u64,
    pub rb_node: rb_node,
    pub wq: wait_queue_head_t,
    pub refs: refcount_t,
    pub state: u32,
    #[cfg(CONFIG_BTRFS_DEBUG)]
    pub leak_list: list_head,
}

extern "C" {
    pub fn btrfs_extent_io_tree_to_inode(tree: *const extent_io_tree) -> *const btrfs_inode;
    pub fn btrfs_extent_io_tree_to_fs_info(tree: *const extent_io_tree) -> *const btrfs_fs_info;
    pub fn btrfs_extent_io_tree_init(fs_info: *mut btrfs_fs_info, tree: *mut extent_io_tree, owner: u32);
    pub fn btrfs_extent_io_tree_release(tree: *mut extent_io_tree);
    pub fn btrfs_lock_extent_bits(tree: *mut extent_io_tree, start: u64, end: u64, bits: u32, cached: *mut *mut extent_state) -> i32;
    pub fn btrfs_try_lock_extent_bits(tree: *mut extent_io_tree, start: u64, end: u64, bits: u32, cached: *mut *mut extent_state) -> bool;
    pub fn btrfs_extent_state_init_cachep() -> i32;
    pub fn btrfs_extent_state_free_cachep();
    pub fn btrfs_count_range_bits(tree: *mut extent_io_tree, start: *mut u64, search_end: u64, max_bytes: u64, bits: u32, contig: bool, cached_state: *mut *mut extent_state) -> u64;
    pub fn btrfs_free_extent_state(state: *mut extent_state);
    pub fn btrfs_test_range_bit(tree: *mut extent_io_tree, start: u64, end: u64, bit: u32, cached_state: *mut extent_state) -> bool;
    pub fn btrfs_test_range_bit_exists(tree: *mut extent_io_tree, start: u64, end: u64, bit: u32) -> bool;
    pub fn btrfs_get_range_bits(tree: *mut extent_io_tree, start: u64, end: u64, bits: *mut u32, cached_state: *mut *mut extent_state);
    pub fn btrfs_clear_record_extent_bits(tree: *mut extent_io_tree, start: u64, end: u64, bits: u32, changeset: *mut extent_changeset) -> i32;
    pub fn btrfs_clear_extent_bit_changeset(tree: *mut extent_io_tree, start: u64, end: u64, bits: u32, cached: *mut *mut extent_state, changeset: *mut extent_changeset) -> i32;
    pub fn btrfs_set_record_extent_bits(tree: *mut extent_io_tree, start: u64, end: u64, bits: u32, changeset: *mut extent_changeset) -> i32;
    pub fn btrfs_set_extent_bit(tree: *mut extent_io_tree, start: u64, end: u64, bits: u32, cached_state: *mut *mut extent_state) -> i32;
    pub fn btrfs_convert_extent_bit(tree: *mut extent_io_tree, start: u64, end: u64, bits: u32, clear_bits: u32, cached_state: *mut *mut extent_state) -> i32;
    pub fn btrfs_find_first_extent_bit(tree: *mut extent_io_tree, start: u64, start_ret: *mut u64, end_ret: *mut u64, bits: u32, cached_state: *mut *mut extent_state) -> bool;
    pub fn btrfs_find_first_clear_extent_bit(tree: *mut extent_io_tree, start: u64, start_ret: *mut u64, end_ret: *mut u64, bits: u32);
    pub fn btrfs_find_contiguous_extent_bit(tree: *mut extent_io_tree, start: u64, start_ret: *mut u64, end_ret: *mut u64, bits: u32) -> bool;
    pub fn btrfs_find_delalloc_range(tree: *mut extent_io_tree, start: *mut u64, end: *mut u64, max_bytes: u64, cached_state: *mut *mut extent_state) -> bool;
    pub fn btrfs_next_extent_state(tree: *mut extent_io_tree, state: *mut extent_state) -> *mut extent_state;
}

#[inline]
pub unsafe fn btrfs_lock_extent(tree: *mut extent_io_tree, start: u64, end: u64, cached: *mut *mut extent_state) -> i32 {
    btrfs_lock_extent_bits(tree, start, end, EXTENT_LOCKED, cached)
}

#[inline]
pub unsafe fn btrfs_try_lock_extent(tree: *mut extent_io_tree, start: u64, end: u64, cached: *mut *mut extent_state) -> bool {
    btrfs_try_lock_extent_bits(tree, start, end, EXTENT_LOCKED, cached)
}

#[inline]
pub unsafe fn btrfs_clear_extent_bit(tree: *mut extent_io_tree, start: u64, end: u64, bits: u32, cached: *mut *mut extent_state) -> i32 {
    btrfs_clear_extent_bit_changeset(tree, start, end, bits, cached, core::ptr::null_mut())
}

#[inline]
pub unsafe fn btrfs_unlock_extent(tree: *mut extent_io_tree, start: u64, end: u64, cached: *mut *mut extent_state) -> i32 {
    btrfs_clear_extent_bit_changeset(tree, start, end, EXTENT_LOCKED, cached, core::ptr::null_mut())
}

#[inline]
pub unsafe fn btrfs_clear_extent_dirty(tree: *mut extent_io_tree, start: u64, end: u64, cached: *mut *mut extent_state) -> i32 {
    btrfs_clear_extent_bit(tree, start, end, EXTENT_DIRTY | EXTENT_DELALLOC | EXTENT_DO_ACCOUNTING, cached)
}

#[inline]
pub unsafe fn btrfs_lock_dio_extent(tree: *mut extent_io_tree, start: u64, end: u64, cached: *mut *mut extent_state) -> i32 {
    btrfs_lock_extent_bits(tree, start, end, EXTENT_DIO_LOCKED, cached)
}

#[inline]
pub unsafe fn btrfs_try_lock_dio_extent(tree: *mut extent_io_tree, start: u64, end: u64, cached: *mut *mut extent_state) -> bool {
    btrfs_try_lock_extent_bits(tree, start, end, EXTENT_DIO_LOCKED, cached)
}

#[inline]
pub unsafe fn btrfs_unlock_dio_extent(tree: *mut extent_io_tree, start: u64, end: u64, cached: *mut *mut extent_state) -> i32 {
    btrfs_clear_extent_bit_changeset(tree, start, end, EXTENT_DIO_LOCKED, cached, core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
