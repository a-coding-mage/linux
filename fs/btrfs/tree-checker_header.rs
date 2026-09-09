/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) Qu Wenruo 2017.  All rights reserved.
 */

// Translated from btrfs/tree-checker.h.  Linux and UAPI types/constants are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct extent_buffer;
#[repr(C)]
pub struct btrfs_fs_info;
#[repr(C)]
pub struct btrfs_chunk;
#[repr(C)]
pub struct btrfs_key;

/* All the extra info needed to verify the parentness of a tree block. */
#[repr(C)]
pub struct btrfs_tree_parent_check {
	/*
	 * The owner check against the tree block.
	 *
	 * Can be 0 to skip the owner check.
	 */
	pub owner_root: u64,

	/*
	 * Expected transid, can be 0 to skip the check, but such skip
	 * should only be utilized for backref walk related code.
	 */
	pub transid: u64,

	/*
	 * The expected first key.
	 *
	 * This check can be skipped if @has_first_key is false, such skip
	 * can happen for case where we don't have the parent node key,
	 * e.g. reading the tree root, doing backref walk.
	 */
	pub first_key: btrfs_key,
	pub has_first_key: bool,

	/* The expected level. Should always be set. */
	pub level: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum btrfs_tree_block_status {
	BTRFS_TREE_BLOCK_CLEAN,
	BTRFS_TREE_BLOCK_INVALID_NRITEMS,
	BTRFS_TREE_BLOCK_INVALID_PARENT_KEY,
	BTRFS_TREE_BLOCK_BAD_KEY_ORDER,
	BTRFS_TREE_BLOCK_INVALID_LEVEL,
	BTRFS_TREE_BLOCK_INVALID_FREE_SPACE,
	BTRFS_TREE_BLOCK_INVALID_OFFSETS,
	BTRFS_TREE_BLOCK_INVALID_BLOCKPTR,
	BTRFS_TREE_BLOCK_INVALID_ITEM,
	BTRFS_TREE_BLOCK_INVALID_OWNER,
	BTRFS_TREE_BLOCK_WRITTEN_NOT_SET,
}

pub const BTRFS_BLOCK_GROUP_VALID: u64 =
	BTRFS_BLOCK_GROUP_TYPE_MASK | BTRFS_BLOCK_GROUP_PROFILE_MASK | BTRFS_BLOCK_GROUP_REMAPPED;

/*
 * Exported simply for btrfs-progs which wants to have the
 * btrfs_tree_block_status return codes.
 */
extern "C" {
	pub fn __btrfs_check_leaf(leaf: *mut extent_buffer) -> btrfs_tree_block_status;
	pub fn __btrfs_check_node(node: *mut extent_buffer) -> btrfs_tree_block_status;

	pub fn btrfs_check_leaf(leaf: *mut extent_buffer) -> i32;
	pub fn btrfs_check_node(node: *mut extent_buffer) -> i32;

	pub fn btrfs_check_chunk_valid(
		fs_info: *const btrfs_fs_info,
		leaf: *const extent_buffer,
		chunk: *const btrfs_chunk,
		logical: u64,
		sectorsize: u32,
	) -> i32;
	pub fn btrfs_check_eb_owner(eb: *const extent_buffer, root_owner: u64) -> i32;
	pub fn btrfs_verify_level_key(
		eb: *mut extent_buffer,
		check: *const btrfs_tree_parent_check,
	) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
