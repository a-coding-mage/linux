/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Persistent object (dat entry/disk inode) allocator/deallocator
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Originally written by Koji Sato.
 * Two allocators were unified by Ryusuke Konishi and Amagai Yoshiji.
 */

// Dependencies supplied by the surrounding kernel translation.

/**
 * nilfs_palloc_entries_per_group - get the number of entries per group
 * @inode: inode of metadata file using this allocator
 *
 * The number of entries per group is defined by the number of bits
 * that a bitmap block can maintain.
 *
 * Return: Number of entries per group.
 */
#[inline]
pub unsafe fn nilfs_palloc_entries_per_group(inode: *const inode) -> c_ulong {
	1 as c_ulong << ((*inode).i_blkbits + 3 /* log2(8 = CHAR_BITS) */)
}

extern "C" {
	pub fn nilfs_palloc_init_blockgroup(inode: *mut inode, entry_size: c_uint) -> c_int;
	pub fn nilfs_palloc_get_entry_block(
		inode: *mut inode,
		nr: __u64,
		create: c_int,
		bh: *mut *mut buffer_head,
	) -> c_int;
	pub fn nilfs_palloc_entry_offset(
		inode: *const inode,
		nr: __u64,
		bh: *const buffer_head,
	) -> size_t;

	pub fn nilfs_palloc_count_max_entries(inode: *mut inode, n: u64, max_entries: *mut u64) -> c_int;
}

/**
 * struct nilfs_palloc_req - persistent allocator request and reply
 * @pr_entry_nr: entry number (vblocknr or inode number)
 * @pr_desc_bh: buffer head of the buffer containing block group descriptors
 * @pr_bitmap_bh: buffer head of the buffer containing a block group bitmap
 * @pr_entry_bh: buffer head of the buffer containing translation entries
 */
#[repr(C)]
pub struct nilfs_palloc_req {
	pub pr_entry_nr: __u64,
	pub pr_desc_bh: *mut buffer_head,
	pub pr_bitmap_bh: *mut buffer_head,
	pub pr_entry_bh: *mut buffer_head,
}

extern "C" {
	pub fn nilfs_palloc_prepare_alloc_entry(
		inode: *mut inode,
		req: *mut nilfs_palloc_req,
		wrap: bool,
	) -> c_int;
	pub fn nilfs_palloc_commit_alloc_entry(inode: *mut inode, req: *mut nilfs_palloc_req);
	pub fn nilfs_palloc_abort_alloc_entry(inode: *mut inode, req: *mut nilfs_palloc_req);
	pub fn nilfs_palloc_commit_free_entry(inode: *mut inode, req: *mut nilfs_palloc_req);
	pub fn nilfs_palloc_prepare_free_entry(inode: *mut inode, req: *mut nilfs_palloc_req) -> c_int;
	pub fn nilfs_palloc_abort_free_entry(inode: *mut inode, req: *mut nilfs_palloc_req);
	pub fn nilfs_palloc_freev(inode: *mut inode, entry_nr: *mut __u64, count: size_t) -> c_int;
}

macro_rules! nilfs_set_bit_atomic {
	($($args:tt)*) => { ext2_set_bit_atomic!($($args)*) };
}
macro_rules! nilfs_clear_bit_atomic {
	($($args:tt)*) => { ext2_clear_bit_atomic!($($args)*) };
}
macro_rules! nilfs_find_next_zero_bit {
	($($args:tt)*) => { find_next_zero_bit_le!($($args)*) };
}
macro_rules! nilfs_find_next_bit {
	($($args:tt)*) => { find_next_bit_le!($($args)*) };
}

/**
 * struct nilfs_bh_assoc - block offset and buffer head association
 * @blkoff: block offset
 * @bh: buffer head
 */
#[repr(C)]
pub struct nilfs_bh_assoc {
	pub blkoff: c_ulong,
	pub bh: *mut buffer_head,
}

/**
 * struct nilfs_palloc_cache - persistent object allocator cache
 * @lock: cache protecting lock
 * @prev_desc: blockgroup descriptors cache
 * @prev_bitmap: blockgroup bitmap cache
 * @prev_entry: translation entries cache
 */
#[repr(C)]
pub struct nilfs_palloc_cache {
	pub lock: spinlock_t,
	pub prev_desc: nilfs_bh_assoc,
	pub prev_bitmap: nilfs_bh_assoc,
	pub prev_entry: nilfs_bh_assoc,
}

extern "C" {
	pub fn nilfs_palloc_setup_cache(inode: *mut inode, cache: *mut nilfs_palloc_cache);
	pub fn nilfs_palloc_clear_cache(inode: *mut inode);
	pub fn nilfs_palloc_destroy_cache(inode: *mut inode);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
