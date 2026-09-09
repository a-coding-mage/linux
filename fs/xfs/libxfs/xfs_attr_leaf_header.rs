/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000,2002-2003,2005 Silicon Graphics, Inc.
 * Copyright (c) 2013 Red Hat, Inc.
 * All Rights Reserved.
 */

// C header dependencies and conditional compilation intent are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct xfs_attr3_icleaf_hdr {
    pub forw: u32,
    pub back: u32,
    pub magic: u16,
    pub count: u16,
    pub usedbytes: u16,
    /*
     * Firstused is 32-bit here instead of 16-bit like the on-disk variant
     * to support maximum fsb size of 64k without overflow issues throughout
     * the attr code. Instead, the overflow condition is handled on
     * conversion to/from disk.
     */
    pub firstused: u32,
    pub holes: __u8,
    pub freemap: [xfs_attr3_icleaf_hdr_freemap; XFS_ATTR_LEAF_MAPSIZE],
}

#[repr(C)]
pub struct xfs_attr3_icleaf_hdr_freemap {
    pub base: u16,
    pub size: u16,
}

/*========================================================================
 * Function prototypes for the kernel.
 *========================================================================*/

/*
 * Internal routines when attribute fork size < XFS_LITINO(mp).
 */
extern "C" {
    pub fn xfs_attr_shortform_create(args: *mut xfs_da_args);
    pub fn xfs_attr_shortform_replace(args: *mut xfs_da_args) -> ::core::ffi::c_int;
    pub fn xfs_attr_shortform_add(args: *mut xfs_da_args, forkoff: ::core::ffi::c_int);
    pub fn xfs_attr_shortform_getvalue(args: *mut xfs_da_args) -> ::core::ffi::c_int;
    pub fn xfs_attr_shortform_to_leaf(args: *mut xfs_da_args) -> ::core::ffi::c_int;
    pub fn xfs_attr_sf_removename(args: *mut xfs_da_args) -> ::core::ffi::c_int;
    pub fn xfs_attr_sf_findname(args: *mut xfs_da_args) -> *mut xfs_attr_sf_entry;
    pub fn xfs_attr_shortform_allfit(bp: *mut xfs_buf, dp: *mut xfs_inode) -> ::core::ffi::c_int;
    pub fn xfs_attr_shortform_bytesfit(dp: *mut xfs_inode, bytes: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn xfs_attr_shortform_verify(sfp: *mut xfs_attr_sf_hdr, size: usize) -> xfs_failaddr_t;
    pub fn xfs_attr_fork_remove(ip: *mut xfs_inode, tp: *mut xfs_trans);

    /* Internal routines when attribute fork size == XFS_LBSIZE(mp). */
    pub fn xfs_attr3_leaf_to_node(args: *mut xfs_da_args) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_to_shortform(bp: *mut xfs_buf, args: *mut xfs_da_args, forkoff: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_clearflag(args: *mut xfs_da_args) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_setflag(args: *mut xfs_da_args) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_flipflags(args: *mut xfs_da_args) -> ::core::ffi::c_int;

    /* Routines used for growing the Btree. */
    pub fn xfs_attr3_leaf_split(state: *mut xfs_da_state, oldblk: *mut xfs_da_state_blk, newblk: *mut xfs_da_state_blk) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_lookup_int(leaf: *mut xfs_buf, args: *mut xfs_da_args) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_getvalue(bp: *mut xfs_buf, args: *mut xfs_da_args) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_add(leaf_buffer: *mut xfs_buf, args: *mut xfs_da_args) -> bool;
    pub fn xfs_attr3_leaf_remove(leaf_buffer: *mut xfs_buf, args: *mut xfs_da_args) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_list_int(bp: *mut xfs_buf, context: *mut xfs_attr_list_context) -> ::core::ffi::c_int;

    /* Routines used for shrinking the Btree. */
    pub fn xfs_attr3_leaf_init(tp: *mut xfs_trans, dp: *mut xfs_inode, blkno: xfs_dablk_t) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_toosmall(state: *mut xfs_da_state, retval: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_unbalance(state: *mut xfs_da_state, drop_blk: *mut xfs_da_state_blk, save_blk: *mut xfs_da_state_blk);

    /* Utility routines. */
    pub fn xfs_attr_leaf_lasthash(bp: *mut xfs_buf, count: *mut ::core::ffi::c_int) -> xfs_dahash_t;
    pub fn xfs_attr_leaf_order(leaf1_bp: *mut xfs_buf, leaf2_bp: *mut xfs_buf) -> ::core::ffi::c_int;
    pub fn xfs_attr_leaf_newentsize(args: *mut xfs_da_args, local: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_read(tp: *mut xfs_trans, dp: *mut xfs_inode, owner: xfs_ino_t, bno: xfs_dablk_t, bpp: *mut *mut xfs_buf) -> ::core::ffi::c_int;
    pub fn xfs_attr3_leaf_hdr_from_disk(geo: *mut xfs_da_geometry, to: *mut xfs_attr3_icleaf_hdr, from: *mut xfs_attr_leafblock);
    pub fn xfs_attr3_leaf_hdr_to_disk(geo: *mut xfs_da_geometry, to: *mut xfs_attr_leafblock, from: *mut xfs_attr3_icleaf_hdr);
    pub fn xfs_attr3_leaf_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
