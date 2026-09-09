/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007 Oracle.  All rights reserved.
 */

// Dependencies supplied by other translated files: linux/types.h, linux/list.h,
// linux/refcount.h, linux/completion.h, linux/rbtree.h, linux/wait.h, and async-thread.h.

#[repr(C)]
pub struct inode;
#[repr(C)]
pub struct page;
#[repr(C)]
pub struct extent_state;
#[repr(C)]
pub struct btrfs_block_group;
#[repr(C)]
pub struct btrfs_inode;
#[repr(C)]
pub struct btrfs_root;
#[repr(C)]
pub struct btrfs_fs_info;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct refcount_t;
#[repr(C)]
pub struct wait_queue_head_t;
#[repr(C)]
pub struct rb_node;
#[repr(C)]
pub struct completion;
#[repr(C)]
pub struct btrfs_work;

#[repr(C)]
pub struct btrfs_ordered_sum {
    /*
     * Logical start address and length for of the blocks covered by
     * the sums array.
     */
    pub logical: u64,
    pub len: u32,

    pub list: list_head,
    /* last field is a variable length array of csums */
    pub sums: [u8; 0],
}

/* Bits for btrfs_ordered_extent::flags. */
pub const BTRFS_ORDERED_IO_DONE: u32 = 0;
pub const BTRFS_ORDERED_COMPLETE: u32 = 1;
pub const BTRFS_ORDERED_IOERR: u32 = 2;
pub const BTRFS_ORDERED_TRUNCATED: u32 = 3;
pub const BTRFS_ORDERED_LOGGED: u32 = 4;
pub const BTRFS_ORDERED_LOGGED_CSUM: u32 = 5;
pub const BTRFS_ORDERED_PENDING: u32 = 6;
pub const BTRFS_ORDERED_REGULAR: u32 = 7;
pub const BTRFS_ORDERED_NOCOW: u32 = 8;
pub const BTRFS_ORDERED_PREALLOC: u32 = 9;
pub const BTRFS_ORDERED_COMPRESSED: u32 = 10;
pub const BTRFS_ORDERED_ENCODED: u32 = 11;
pub const BTRFS_ORDERED_DIRECT: u32 = 12;
pub const BTRFS_ORDERED_NR_FLAGS: u32 = 13;

// static_assert(BTRFS_ORDERED_NR_FLAGS <= BITS_PER_LONG);

/* One and only one flag can be set. */
pub const BTRFS_ORDERED_EXCLUSIVE_FLAGS: usize =
    (1usize << BTRFS_ORDERED_REGULAR)
    | (1usize << BTRFS_ORDERED_NOCOW)
    | (1usize << BTRFS_ORDERED_PREALLOC)
    | (1usize << BTRFS_ORDERED_COMPRESSED);

/* BTRFS_ORDERED_* flags that specify the type of the extent. */
pub const BTRFS_ORDERED_TYPE_FLAGS: usize = BTRFS_ORDERED_EXCLUSIVE_FLAGS
    | (1usize << BTRFS_ORDERED_DIRECT)
    | (1usize << BTRFS_ORDERED_ENCODED);

#[repr(C)]
pub struct btrfs_ordered_extent {
    /* logical offset in the file */
    pub file_offset: u64,
    pub num_bytes: u64,
    pub ram_bytes: u64,
    pub disk_bytenr: u64,
    pub disk_num_bytes: u64,
    pub offset: u64,
    pub bytes_left: u64,
    pub truncated_len: u64,
    pub flags: usize,
    pub compress_type: i32,
    pub qgroup_rsv: i32,
    pub refs: refcount_t,
    pub inode: *mut btrfs_inode,
    pub csum_list: list_head,
    pub log_list: list_head,
    pub wait: wait_queue_head_t,
    pub rb_node: rb_node,
    pub root_extent_list: list_head,
    pub work: btrfs_work,
    pub completion: completion,
    pub flush_work: btrfs_work,
    pub work_list: list_head,
    pub bioc_list: list_head,
}

extern "C" {
    pub fn btrfs_finish_one_ordered(ordered_extent: *mut btrfs_ordered_extent) -> i32;
    pub fn btrfs_finish_ordered_io(ordered_extent: *mut btrfs_ordered_extent) -> i32;
    pub fn btrfs_put_ordered_extent(entry: *mut btrfs_ordered_extent);
    pub fn btrfs_remove_ordered_extent(entry: *mut btrfs_ordered_extent);
    pub fn btrfs_finish_ordered_extent(
        ordered: *mut btrfs_ordered_extent,
        file_offset: u64,
        len: u64,
        uptodate: bool,
    );
    pub fn btrfs_mark_ordered_io_finished(
        inode: *mut btrfs_inode,
        file_offset: u64,
        num_bytes: u64,
        uptodate: bool,
    );
    pub fn btrfs_dec_test_ordered_pending(
        inode: *mut btrfs_inode,
        cached: *mut *mut btrfs_ordered_extent,
        file_offset: u64,
        io_size: u64,
    ) -> bool;
}

#[repr(C)]
pub struct btrfs_file_extent {
    pub disk_bytenr: u64,
    pub disk_num_bytes: u64,
    pub num_bytes: u64,
    pub ram_bytes: u64,
    pub offset: u64,
    pub compression: u8,
}

extern "C" {
    pub fn btrfs_alloc_ordered_extent(
        inode: *mut btrfs_inode,
        file_offset: u64,
        file_extent: *const btrfs_file_extent,
        flags: usize,
    ) -> *mut btrfs_ordered_extent;
    pub fn btrfs_add_ordered_sum(entry: *mut btrfs_ordered_extent, sum: *mut btrfs_ordered_sum);
    pub fn btrfs_lookup_ordered_extent(
        inode: *mut btrfs_inode,
        file_offset: u64,
    ) -> *mut btrfs_ordered_extent;
    pub fn btrfs_start_ordered_extent_nowriteback(
        entry: *mut btrfs_ordered_extent,
        nowriteback_start: u64,
        nowriteback_len: u32,
    );
    pub fn btrfs_wait_ordered_range(inode: *mut btrfs_inode, start: u64, len: u64) -> i32;
    pub fn btrfs_lookup_first_ordered_extent(
        inode: *mut btrfs_inode,
        file_offset: u64,
    ) -> *mut btrfs_ordered_extent;
    pub fn btrfs_lookup_first_ordered_range(
        inode: *mut btrfs_inode,
        file_offset: u64,
        len: u64,
    ) -> *mut btrfs_ordered_extent;
    pub fn btrfs_lookup_ordered_range(
        inode: *mut btrfs_inode,
        file_offset: u64,
        len: u64,
    ) -> *mut btrfs_ordered_extent;
    pub fn btrfs_get_ordered_extents_for_logging(inode: *mut btrfs_inode, list: *mut list_head);
    pub fn btrfs_wait_ordered_extents(
        root: *mut btrfs_root,
        nr: u64,
        bg: *const btrfs_block_group,
    ) -> u64;
    pub fn btrfs_wait_ordered_roots(
        fs_info: *mut btrfs_fs_info,
        nr: u64,
        bg: *const btrfs_block_group,
    );
    pub fn btrfs_lock_and_flush_ordered_range(
        inode: *mut btrfs_inode,
        start: u64,
        end: u64,
        cached_state: *mut *mut extent_state,
    );
    pub fn btrfs_try_lock_ordered_range(
        inode: *mut btrfs_inode,
        start: u64,
        end: u64,
        cached_state: *mut *mut extent_state,
    ) -> bool;
    pub fn btrfs_split_ordered_extent(
        ordered: *mut btrfs_ordered_extent,
        len: u64,
    ) -> *mut btrfs_ordered_extent;
    pub fn btrfs_mark_ordered_extent_error(ordered: *mut btrfs_ordered_extent);
    pub fn btrfs_mark_ordered_extent_truncated(ordered: *mut btrfs_ordered_extent, truncate_len: u64);
    pub fn ordered_data_init() -> i32;
    pub fn ordered_data_exit();
}

#[inline]
pub unsafe fn btrfs_start_ordered_extent(entry: *mut btrfs_ordered_extent) {
    btrfs_start_ordered_extent_nowriteback(entry, 0, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
