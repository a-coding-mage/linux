/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008 Oracle.  All rights reserved.
 */

// Translated from compression.h. C header dependencies are supplied elsewhere.

/*
 * We want to make sure that amount of RAM required to uncompress an extent is
 * reasonable, so we limit the total size in ram of a compressed extent to
 * 128k.  This is a crucial number because it also controls how easily we can
 * spread reads across cpus for decompression.
 *
 * We also want to make sure the amount of IO required to do a random read is
 * reasonably small, so we limit the size of a compressed extent to 128k.
 */

/* Maximum length of compressed data stored on disk */
pub const BTRFS_MAX_COMPRESSED: usize = SZ_128K;
pub const BTRFS_MAX_COMPRESSED_PAGES: usize = BTRFS_MAX_COMPRESSED / PAGE_SIZE;
const _: () = assert!(BTRFS_MAX_COMPRESSED % PAGE_SIZE == 0);

/* The max size for a single worker to compress. */
pub const BTRFS_COMPRESSION_CHUNK_SIZE: usize = SZ_512K;

/* Maximum size of data before compression */
pub const BTRFS_MAX_UNCOMPRESSED: usize = SZ_128K;

pub const BTRFS_ZLIB_DEFAULT_LEVEL: i32 = 3;

#[repr(C)]
pub struct compressed_bio {
	/* starting offset in the inode for our pages */
	pub start: u64,
	/* Number of bytes in the inode we're working on */
	pub len: ::core::ffi::c_uint,
	/* The compression algorithm for this bio */
	pub compress_type: u8,
	/* Whether this is a write for writeback. */
	pub writeback: bool,
	/* For reads, this is the bio we are copying the data into. */
	pub orig_bbio: *mut btrfs_bio,
	/* Must be last. */
	pub bbio: btrfs_bio,
}

#[inline]
pub unsafe fn cb_to_fs_info(cb: *const compressed_bio) -> *mut btrfs_fs_info {
	(*(*cb).bbio.inode).root.fs_info
}

/* @range_end must be exclusive. */
#[inline]
pub unsafe fn btrfs_calc_input_length(folio: *mut folio, range_end: u64, cur: u64) -> u32 {
	/* @cur must be inside the folio. */
	debug_assert!(folio_pos(folio) <= cur);
	debug_assert!(cur < folio_next_pos(folio));
	umin(range_end, folio_next_pos(folio)).wrapping_sub(cur) as u32
}

extern "C" {
	pub fn btrfs_alloc_compress_wsm(fs_info: *mut btrfs_fs_info) -> i32;
	pub fn btrfs_free_compress_wsm(fs_info: *mut btrfs_fs_info);
	pub fn btrfs_init_compress() -> i32;
	pub fn btrfs_exit_compress();
	pub fn btrfs_compress_level_valid(typ: ::core::ffi::c_uint, level: i32) -> bool;
	pub fn btrfs_decompress(typ: i32, data_in: *const u8, dest_folio: *mut folio,
		dest_pgoff: ::core::ffi::c_ulong, srclen: usize, destlen: usize) -> i32;
	pub fn btrfs_decompress_buf2page(buf: *const ::core::ffi::c_char, buf_len: u32,
		cb: *mut compressed_bio, decompressed: u32) -> i32;
	pub fn btrfs_alloc_compressed_write(inode: *mut btrfs_inode, start: u64, len: u64) -> *mut compressed_bio;
	pub fn btrfs_submit_compressed_write(ordered: *mut btrfs_ordered_extent, cb: *mut compressed_bio);
	pub fn btrfs_submit_compressed_read(bbio: *mut btrfs_bio);
	pub fn btrfs_compress_str2level(typ: ::core::ffi::c_uint, s: *const ::core::ffi::c_char, level_ret: *mut i32) -> i32;
	pub fn btrfs_alloc_compr_folio(fs_info: *mut btrfs_fs_info, gfp: gfp_t) -> *mut folio;
	pub fn btrfs_free_compr_folio(folio: *mut folio);
}

#[repr(C)]
pub struct workspace_manager {
	pub idle_ws: list_head,
	pub ws_lock: spinlock_t,
	/* Number of free workspaces */
	pub free_ws: i32,
	/* Total number of allocated workspaces */
	pub total_ws: atomic_t,
	/* Waiters for a free workspace */
	pub ws_wait: wait_queue_head_t,
}

extern "C" {
	pub fn btrfs_get_workspace(fs_info: *mut btrfs_fs_info, typ: i32, level: i32) -> *mut list_head;
	pub fn btrfs_put_workspace(fs_info: *mut btrfs_fs_info, typ: i32, ws: *mut list_head);
}

#[repr(C)]
pub struct btrfs_compress_levels {
	/* Maximum level supported by the compression algorithm */
	pub min_level: i32,
	pub max_level: i32,
	pub default_level: i32,
}

pub const BTRFS_NR_WORKSPACE_MANAGERS: usize = BTRFS_NR_COMPRESS_TYPES;

extern "C" {
	pub static btrfs_heuristic_compress: btrfs_compress_levels;
	pub static btrfs_zlib_compress: btrfs_compress_levels;
	pub static btrfs_lzo_compress: btrfs_compress_levels;
	pub static btrfs_zstd_compress: btrfs_compress_levels;

	pub fn btrfs_compress_type2str(typ: btrfs_compression_type) -> *const ::core::ffi::c_char;
	pub fn btrfs_compress_is_valid_type(s: *const ::core::ffi::c_char, len: usize) -> bool;
	pub fn btrfs_compress_heuristic(inode: *mut btrfs_inode, start: u64, end: u64) -> i32;
	pub fn btrfs_compress_filemap_get_folio(mapping: *mut address_space, start: u64,
		in_folio_ret: *mut *mut folio) -> i32;
	pub fn btrfs_compress_bio(inode: *mut btrfs_inode, start: u64, len: u32,
		typ: ::core::ffi::c_uint, level: i32, write_flags: blk_opf_t) -> *mut compressed_bio;
}

#[inline]
pub unsafe fn cleanup_compressed_bio(cb: *mut compressed_bio) {
	let bio = &mut (*cb).bbio.bio as *mut bio;
	let mut fi: folio_iter = ::core::mem::zeroed();
	bio_for_each_folio_all(&mut fi, bio);
	btrfs_free_compr_folio(fi.folio);
	bio_put(bio);
}

extern "C" {
	pub fn zlib_compress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> i32;
	pub fn zlib_decompress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> i32;
	pub fn zlib_decompress(ws: *mut list_head, data_in: *const u8, dest_folio: *mut folio, dest_pgoff: ::core::ffi::c_ulong, srclen: usize, destlen: usize) -> i32;
	pub fn zlib_alloc_workspace(fs_info: *mut btrfs_fs_info, level: ::core::ffi::c_uint) -> *mut list_head;
	pub fn zlib_free_workspace(ws: *mut list_head);
	pub fn zlib_get_workspace(fs_info: *mut btrfs_fs_info, level: ::core::ffi::c_uint) -> *mut list_head;
	pub fn lzo_compress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> i32;
	pub fn lzo_decompress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> i32;
	pub fn lzo_decompress(ws: *mut list_head, data_in: *const u8, dest_folio: *mut folio, dest_pgoff: ::core::ffi::c_ulong, srclen: usize, destlen: usize) -> i32;
	pub fn lzo_alloc_workspace(fs_info: *mut btrfs_fs_info) -> *mut list_head;
	pub fn lzo_free_workspace(ws: *mut list_head);
	pub fn zstd_compress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> i32;
	pub fn zstd_decompress_bio(ws: *mut list_head, cb: *mut compressed_bio) -> i32;
	pub fn zstd_decompress(ws: *mut list_head, data_in: *const u8, dest_folio: *mut folio, dest_pgoff: ::core::ffi::c_ulong, srclen: usize, destlen: usize) -> i32;
	pub fn zstd_alloc_workspace_manager(fs_info: *mut btrfs_fs_info) -> i32;
	pub fn zstd_free_workspace_manager(fs_info: *mut btrfs_fs_info);
	pub fn zstd_alloc_workspace(fs_info: *mut btrfs_fs_info, level: i32) -> *mut list_head;
	pub fn zstd_free_workspace(ws: *mut list_head);
	pub fn zstd_get_workspace(fs_info: *mut btrfs_fs_info, level: i32) -> *mut list_head;
	pub fn zstd_put_workspace(fs_info: *mut btrfs_fs_info, ws: *mut list_head);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
