/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Forward declaration: struct dir_context;

/* In-core version of the leaf and free block headers. */
#[repr(C)]
pub struct xfs_dir3_icleaf_hdr {
    pub forw: u32,
    pub back: u32,
    pub magic: u16,
    pub count: u16,
    pub stale: u16,
    // Pointer to the on-disk format entries behind the variable-size header.
    pub ents: *mut xfs_dir2_leaf_entry,
}

#[repr(C)]
pub struct xfs_dir3_icfree_hdr {
    pub magic: u32,
    pub firstdb: u32,
    pub nvalid: u32,
    pub nused: u32,
    // Pointer to the on-disk format entries behind the variable-size header.
    pub bests: *mut __be16,
}

extern "C" {
    pub fn xfs_ascii_ci_hashname(name: *const xfs_name) -> xfs_dahash_t;
    pub fn xfs_ascii_ci_compname(args: *mut xfs_da_args, name: *const u8, len: i32) -> xfs_dacmp;
    pub fn xfs_dir2_grow_inode(args: *mut xfs_da_args, space: i32, dbp: *mut xfs_dir2_db_t) -> i32;
    pub fn xfs_dir_cilookup_result(args: *mut xfs_da_args, name: *const u8, len: i32) -> i32;

    pub fn xfs_dir3_block_read(tp: *mut xfs_trans, dp: *mut xfs_inode, owner: xfs_ino_t, bpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_dir2_block_addname(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_block_lookup(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_block_removename(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_block_replace(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_leaf_to_block(args: *mut xfs_da_args, lbp: *mut xfs_buf, dbp: *mut xfs_buf) -> i32;

    pub fn xfs_dir2_data_bestfree_p(mp: *mut xfs_mount, hdr: *mut xfs_dir2_data_hdr) -> *mut xfs_dir2_data_free;
    pub fn xfs_dir2_data_entry_tag_p(mp: *mut xfs_mount, dep: *mut xfs_dir2_data_entry) -> *mut __be16;
    pub fn xfs_dir2_data_get_ftype(mp: *mut xfs_mount, dep: *mut xfs_dir2_data_entry) -> u8;
    pub fn xfs_dir2_data_put_ftype(mp: *mut xfs_mount, dep: *mut xfs_dir2_data_entry, ftype: u8);
    // In the non-DEBUG build this macro expands to nothing.
    pub fn __xfs_dir3_data_check(dp: *mut xfs_inode, bp: *mut xfs_buf) -> xfs_failaddr_t;
    pub fn xfs_dir3_data_read(tp: *mut xfs_trans, dp: *mut xfs_inode, owner: xfs_ino_t, bno: xfs_dablk_t, flags: u32, bpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_dir3_data_readahead(dp: *mut xfs_inode, bno: xfs_dablk_t, flags: u32) -> i32;
    pub fn xfs_dir2_data_freeinsert(hdr: *mut xfs_dir2_data_hdr, bf: *mut xfs_dir2_data_free, dup: *mut xfs_dir2_data_unused, loghead: *mut i32) -> *mut xfs_dir2_data_free;
    pub fn xfs_dir3_data_init(args: *mut xfs_da_args, blkno: xfs_dir2_db_t, bpp: *mut *mut xfs_buf) -> i32;

    pub fn xfs_dir2_leaf_hdr_from_disk(mp: *mut xfs_mount, to: *mut xfs_dir3_icleaf_hdr, from: *mut xfs_dir2_leaf);
    pub fn xfs_dir2_leaf_hdr_to_disk(mp: *mut xfs_mount, to: *mut xfs_dir2_leaf, from: *mut xfs_dir3_icleaf_hdr);
    pub fn xfs_dir3_leaf_read(tp: *mut xfs_trans, dp: *mut xfs_inode, owner: xfs_ino_t, fbno: xfs_dablk_t, bpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_dir3_leafn_read(tp: *mut xfs_trans, dp: *mut xfs_inode, owner: xfs_ino_t, fbno: xfs_dablk_t, bpp: *mut *mut xfs_buf) -> i32;
    pub fn xfs_dir2_block_to_leaf(args: *mut xfs_da_args, dbp: *mut xfs_buf) -> i32;
    pub fn xfs_dir2_leaf_addname(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir3_leaf_compact(args: *mut xfs_da_args, leafhdr: *mut xfs_dir3_icleaf_hdr, bp: *mut xfs_buf);
    pub fn xfs_dir3_leaf_compact_x1(leafhdr: *mut xfs_dir3_icleaf_hdr, ents: *mut xfs_dir2_leaf_entry, indexp: *mut i32, lowstalep: *mut i32, highstalep: *mut i32, lowlogp: *mut i32, highlogp: *mut i32);
    pub fn xfs_dir3_leaf_get_buf(args: *mut xfs_da_args, bno: xfs_dir2_db_t, bpp: *mut *mut xfs_buf, magic: u16) -> i32;
    pub fn xfs_dir3_leaf_log_ents(args: *mut xfs_da_args, hdr: *mut xfs_dir3_icleaf_hdr, bp: *mut xfs_buf, first: i32, last: i32);
    pub fn xfs_dir3_leaf_log_header(args: *mut xfs_da_args, bp: *mut xfs_buf);
    pub fn xfs_dir2_leaf_lookup(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_leaf_removename(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_leaf_replace(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_leaf_search_hash(args: *mut xfs_da_args, lbp: *mut xfs_buf) -> i32;
    pub fn xfs_dir2_leaf_trim_data(args: *mut xfs_da_args, lbp: *mut xfs_buf, db: xfs_dir2_db_t) -> i32;
    pub fn xfs_dir3_leaf_find_entry(leafhdr: *mut xfs_dir3_icleaf_hdr, ents: *mut xfs_dir2_leaf_entry, index: i32, compact: i32, lowstale: i32, highstale: i32, lfloglow: *mut i32, lfloghigh: *mut i32) -> *mut xfs_dir2_leaf_entry;
    pub fn xfs_dir2_node_to_leaf(state: *mut xfs_da_state) -> i32;
    pub fn xfs_dir3_leaf_check_int(mp: *mut xfs_mount, hdr: *mut xfs_dir3_icleaf_hdr, leaf: *mut xfs_dir2_leaf, expensive_checks: bool) -> xfs_failaddr_t;

    pub fn xfs_dir2_free_hdr_from_disk(mp: *mut xfs_mount, to: *mut xfs_dir3_icfree_hdr, from: *mut xfs_dir2_free);
    pub fn xfs_dir2_leaf_to_node(args: *mut xfs_da_args, lbp: *mut xfs_buf) -> i32;
    pub fn xfs_dir2_leaf_lasthash(dp: *mut xfs_inode, bp: *mut xfs_buf, count: *mut i32) -> xfs_dahash_t;
    pub fn xfs_dir2_leafn_lookup_int(bp: *mut xfs_buf, args: *mut xfs_da_args, indexp: *mut i32, state: *mut xfs_da_state) -> i32;
    pub fn xfs_dir2_leafn_order(dp: *mut xfs_inode, leaf1_bp: *mut xfs_buf, leaf2_bp: *mut xfs_buf) -> i32;
    pub fn xfs_dir2_leafn_split(state: *mut xfs_da_state, oldblk: *mut xfs_da_state_blk, newblk: *mut xfs_da_state_blk) -> i32;
    pub fn xfs_dir2_leafn_toosmall(state: *mut xfs_da_state, action: *mut i32) -> i32;
    pub fn xfs_dir2_leafn_unbalance(state: *mut xfs_da_state, drop_blk: *mut xfs_da_state_blk, save_blk: *mut xfs_da_state_blk);
    pub fn xfs_dir2_node_addname(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_node_lookup(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_node_removename(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_node_replace(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_node_trim_free(args: *mut xfs_da_args, fo: xfs_fileoff_t, rvalp: *mut i32) -> i32;
    pub fn xfs_dir2_free_read(tp: *mut xfs_trans, dp: *mut xfs_inode, owner: xfs_ino_t, fbno: xfs_dablk_t, bpp: *mut *mut xfs_buf) -> i32;

    pub fn xfs_dir2_sf_get_ino(mp: *mut xfs_mount, hdr: *mut xfs_dir2_sf_hdr, sfep: *mut xfs_dir2_sf_entry) -> xfs_ino_t;
    pub fn xfs_dir2_sf_get_parent_ino(hdr: *mut xfs_dir2_sf_hdr) -> xfs_ino_t;
    pub fn xfs_dir2_sf_put_parent_ino(hdr: *mut xfs_dir2_sf_hdr, ino: xfs_ino_t);
    pub fn xfs_dir2_sf_get_ftype(mp: *mut xfs_mount, sfep: *mut xfs_dir2_sf_entry) -> u8;
    pub fn xfs_dir2_sf_nextentry(mp: *mut xfs_mount, hdr: *mut xfs_dir2_sf_hdr, sfep: *mut xfs_dir2_sf_entry) -> *mut xfs_dir2_sf_entry;
    pub fn xfs_dir2_block_sfsize(dp: *mut xfs_inode, block: *mut xfs_dir2_data_hdr, sfhp: *mut xfs_dir2_sf_hdr) -> i32;
    pub fn xfs_dir2_block_to_sf(args: *mut xfs_da_args, bp: *mut xfs_buf, size: i32, sfhp: *mut xfs_dir2_sf_hdr_t) -> i32;
    pub fn xfs_dir2_sf_addname(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_sf_create(args: *mut xfs_da_args, pino: xfs_ino_t) -> i32;
    pub fn xfs_dir2_sf_lookup(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_sf_removename(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_sf_replace(args: *mut xfs_da_args) -> i32;
    pub fn xfs_dir2_sf_verify(mp: *mut xfs_mount, sfp: *mut xfs_dir2_sf_hdr, size: i64) -> xfs_failaddr_t;
    pub fn xfs_dir2_sf_entsize(mp: *mut xfs_mount, hdr: *mut xfs_dir2_sf_hdr, len: i32) -> i32;
    pub fn xfs_dir2_sf_put_ino(mp: *mut xfs_mount, hdr: *mut xfs_dir2_sf_hdr, sfep: *mut xfs_dir2_sf_entry, ino: xfs_ino_t);
    pub fn xfs_dir2_sf_put_ftype(mp: *mut xfs_mount, sfep: *mut xfs_dir2_sf_entry, ftype: u8);
    pub fn xfs_readdir(tp: *mut xfs_trans, dp: *mut xfs_inode, ctx: *mut dir_context, bufsize: usize) -> i32;
}

#[inline]
pub unsafe fn xfs_dir2_data_unusedsize(len: u32) -> u32 {
    round_up(len, XFS_DIR2_DATA_ALIGN)
}

#[inline]
pub unsafe fn xfs_dir2_data_entsize(mp: *mut xfs_mount, namelen: u32) -> u32 {
    let mut len = offsetof::<xfs_dir2_data_entry>("name") + namelen + core::mem::size_of::<xfs_dir2_data_off_t>() as u32;
    if xfs_has_ftype(mp) { len += core::mem::size_of::<u8>() as u32; }
    round_up(len, XFS_DIR2_DATA_ALIGN)
}

extern "C" {
    pub fn xfs_dir2_hashname(mp: *mut xfs_mount, name: *const xfs_name) -> xfs_dahash_t;
    pub fn xfs_dir2_compname(args: *mut xfs_da_args, name: *const u8, len: i32) -> xfs_dacmp;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
