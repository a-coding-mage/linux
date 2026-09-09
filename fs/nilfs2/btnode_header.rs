/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS B-tree node cache
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Seiji Kihara.
 * Revised by Ryusuke Konishi.
 */

// Dependencies supplied by the surrounding kernel translation.

/**
 * struct nilfs_btnode_chkey_ctxt - change key context
 * @oldkey: old key of block's moving content
 * @newkey: new key for block's content
 * @bh: buffer head of old buffer
 * @newbh: buffer head of new buffer
 */
#[repr(C)]
pub struct nilfs_btnode_chkey_ctxt {
	pub oldkey: __u64,
	pub newkey: __u64,
	pub bh: *mut buffer_head,
	pub newbh: *mut buffer_head,
}

extern "C" {
	pub fn nilfs_init_btnc_inode(btnc_inode: *mut inode);
	pub fn nilfs_btnode_cache_clear(arg1: *mut address_space);
	pub fn nilfs_btnode_create_block(
		btnc: *mut address_space,
		blocknr: __u64,
	) -> *mut buffer_head;
	pub fn nilfs_btnode_submit_block(
		arg1: *mut address_space,
		arg2: __u64,
		arg3: sector_t,
		arg4: blk_opf_t,
		arg5: *mut *mut buffer_head,
		arg6: *mut sector_t,
	) -> ::core::ffi::c_int;
	pub fn nilfs_btnode_delete(arg1: *mut buffer_head);
	pub fn nilfs_btnode_prepare_change_key(
		arg1: *mut address_space,
		arg2: *mut nilfs_btnode_chkey_ctxt,
	) -> ::core::ffi::c_int;
	pub fn nilfs_btnode_commit_change_key(
		arg1: *mut address_space,
		arg2: *mut nilfs_btnode_chkey_ctxt,
	);
	pub fn nilfs_btnode_abort_change_key(
		arg1: *mut address_space,
		arg2: *mut nilfs_btnode_chkey_ctxt,
	);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
