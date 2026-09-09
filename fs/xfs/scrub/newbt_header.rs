// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2022-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// The C header guard __XFS_SCRUB_NEWBT_H__ is omitted in Rust.

pub struct xrep_newbt_resv {
	/* Link to list of extents that we've reserved. */
	pub list: list_head,

	pub pag: *mut xfs_perag,

	/* Auto-freeing this reservation if we don't commit. */
	pub autoreap: xfs_alloc_autoreap,

	/* AG block of the extent we reserved. */
	pub agbno: xfs_agblock_t,

	/* Length of the reservation. */
	pub len: xfs_extlen_t,

	/* How much of this reservation has been used. */
	pub used: xfs_extlen_t,
}

#[repr(C)]
pub union xrep_newbt_fake_root {
	pub afake: xbtree_afakeroot,
	pub ifake: xbtree_ifakeroot,
}

pub struct xrep_newbt {
	pub sc: *mut xfs_scrub,

	/* Custom allocation function, or NULL for xfs_alloc_vextent */
	pub alloc_vextent: Option<unsafe extern "C" fn(
		sc: *mut xfs_scrub,
		args: *mut xfs_alloc_arg,
		alloc_hint: xfs_fsblock_t,
	) -> i32>,

	/* List of extents that we've reserved. */
	pub resv_list: list_head,

	/* Fake root for new btree. */
	pub fake_root: xrep_newbt_fake_root,

	/* rmap owner of these blocks */
	pub oinfo: xfs_owner_info,

	/* btree geometry for the bulk loader */
	pub bload: xfs_btree_bload,

	/* Allocation hint */
	pub alloc_hint: xfs_fsblock_t,

	/* per-ag reservation type */
	pub resv: xfs_ag_resv_type,
}

unsafe extern "C" {
	pub fn xrep_newbt_init_bare(xnr: *mut xrep_newbt, sc: *mut xfs_scrub);
	pub fn xrep_newbt_init_ag(
		xnr: *mut xrep_newbt,
		sc: *mut xfs_scrub,
		oinfo: *const xfs_owner_info,
		alloc_hint: xfs_fsblock_t,
		resv: xfs_ag_resv_type,
	);
	pub fn xrep_newbt_init_inode(
		xnr: *mut xrep_newbt,
		sc: *mut xfs_scrub,
		whichfork: i32,
		oinfo: *const xfs_owner_info,
	) -> i32;
	pub fn xrep_newbt_init_metadir_inode(xnr: *mut xrep_newbt, sc: *mut xfs_scrub) -> i32;
	pub fn xrep_newbt_alloc_blocks(xnr: *mut xrep_newbt, nr_blocks: u64) -> i32;
	pub fn xrep_newbt_add_extent(
		xnr: *mut xrep_newbt,
		pag: *mut xfs_perag,
		agbno: xfs_agblock_t,
		len: xfs_extlen_t,
	) -> i32;
	pub fn xrep_newbt_cancel(xnr: *mut xrep_newbt);
	pub fn xrep_newbt_commit(xnr: *mut xrep_newbt) -> i32;
	pub fn xrep_newbt_claim_block(
		cur: *mut xfs_btree_cur,
		xnr: *mut xrep_newbt,
		ptr: *mut xfs_btree_ptr,
	) -> i32;
	pub fn xrep_newbt_unused_blocks(xnr: *mut xrep_newbt) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
