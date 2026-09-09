/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * suballoc.h
 *
 * Defines sub allocator api
 *
 * Copyright (C) 2003, 2004 Oracle.  All rights reserved.
 */

// C header guard: _CHAINALLOC_H_

pub struct ocfs2_suballoc_result;

pub type group_search_t = unsafe extern "C" fn(
    *mut inode,
    *mut buffer_head,
    u32, /* bits_wanted */
    u32, /* min_bits */
    u64, /* max_block */
    *mut ocfs2_suballoc_result, /* found bits */
) -> ::core::ffi::c_int;

#[repr(C)]
pub struct ocfs2_alloc_context {
    pub ac_inode: *mut inode,    /* which bitmap are we allocating from? */
    pub ac_bh: *mut buffer_head, /* file entry bh */
    pub ac_alloc_slot: u32,      /* which slot are we allocating from? */
    pub ac_bits_wanted: u32,
    pub ac_bits_given: u32,
    pub ac_which: u32,

    /* these are used by the chain search */
    pub ac_chain: u16,
    pub ac_disable_chain_relink: ::core::ffi::c_int,
    pub ac_group_search: Option<group_search_t>,

    pub ac_last_group: u64,
    pub ac_max_block: u64, /* Highest block number to allocate. 0 is
                              the same as ~0 - unlimited */

    pub ac_find_loc_only: ::core::ffi::c_int, /* hack for reflink operation ordering */
    pub ac_find_loc_priv: *mut ocfs2_suballoc_result, /* */

    pub ac_resv: *mut ocfs2_alloc_reservation,
}

pub const OCFS2_AC_USE_LOCAL: u32 = 1;
pub const OCFS2_AC_USE_MAIN: u32 = 2;
pub const OCFS2_AC_USE_INODE: u32 = 3;
pub const OCFS2_AC_USE_META: u32 = 4;
pub const OCFS2_AC_USE_MAIN_DISCONTIG: u32 = 5;

unsafe extern "C" {
    pub fn ocfs2_init_steal_slots(osb: *mut ocfs2_super);
    pub fn ocfs2_free_alloc_context(ac: *mut ocfs2_alloc_context);

    pub fn ocfs2_reserve_new_metadata(
        osb: *mut ocfs2_super,
        root_el: *mut ocfs2_extent_list,
        ac: *mut *mut ocfs2_alloc_context,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_reserve_new_metadata_blocks(
        osb: *mut ocfs2_super,
        blocks: ::core::ffi::c_int,
        ac: *mut *mut ocfs2_alloc_context,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_reserve_new_inode(
        osb: *mut ocfs2_super,
        ac: *mut *mut ocfs2_alloc_context,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_reserve_clusters(
        osb: *mut ocfs2_super,
        bits_wanted: u32,
        ac: *mut *mut ocfs2_alloc_context,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_alloc_dinode_update_counts(
        inode: *mut inode, handle: *mut handle_t, di_bh: *mut buffer_head,
        num_bits: u32, chain: u16,
    ) -> ::core::ffi::c_int;
    pub fn ocfs2_rollback_alloc_dinode_counts(
        inode: *mut inode, di_bh: *mut buffer_head, num_bits: u32, chain: u16,
    );
    pub fn ocfs2_find_max_contig_free_bits(bitmap: *mut ::core::ffi::c_void, total_bits: u16, start: u16) -> u16;
    pub fn ocfs2_block_group_set_bits(
        handle: *mut handle_t, alloc_inode: *mut inode, bg: *mut ocfs2_group_desc,
        group_bh: *mut buffer_head, bit_off: ::core::ffi::c_uint,
        num_bits: ::core::ffi::c_uint, max_contig_bits: ::core::ffi::c_uint,
        fastpath: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_claim_metadata(handle: *mut handle_t, ac: *mut ocfs2_alloc_context, bits_wanted: u32, suballoc_loc: *mut u64, suballoc_bit_start: *mut u16, num_bits: *mut u32, blkno_start: *mut u64) -> ::core::ffi::c_int;
    pub fn ocfs2_claim_new_inode(handle: *mut handle_t, dir: *mut inode, parent_fe_bh: *mut buffer_head, ac: *mut ocfs2_alloc_context, suballoc_loc: *mut u64, suballoc_bit: *mut u16, fe_blkno: *mut u64) -> ::core::ffi::c_int;
    pub fn ocfs2_claim_clusters(handle: *mut handle_t, ac: *mut ocfs2_alloc_context, min_clusters: u32, cluster_start: *mut u32, num_clusters: *mut u32) -> ::core::ffi::c_int;
    pub fn __ocfs2_claim_clusters(handle: *mut handle_t, ac: *mut ocfs2_alloc_context, min_clusters: u32, max_clusters: u32, cluster_start: *mut u32, num_clusters: *mut u32) -> ::core::ffi::c_int;

    pub fn ocfs2_free_suballoc_bits(handle: *mut handle_t, alloc_inode: *mut inode, alloc_bh: *mut buffer_head, start_bit: ::core::ffi::c_uint, bg_blkno: u64, count: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ocfs2_free_dinode(handle: *mut handle_t, inode_alloc_inode: *mut inode, inode_alloc_bh: *mut buffer_head, di: *mut ocfs2_dinode) -> ::core::ffi::c_int;
    pub fn ocfs2_free_clusters(handle: *mut handle_t, bitmap_inode: *mut inode, bitmap_bh: *mut buffer_head, start_blk: u64, num_clusters: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ocfs2_release_clusters(handle: *mut handle_t, bitmap_inode: *mut inode, bitmap_bh: *mut buffer_head, start_blk: u64, num_clusters: ::core::ffi::c_uint) -> ::core::ffi::c_int;

    pub fn ocfs2_reserve_cluster_bitmap_bits(osb: *mut ocfs2_super, ac: *mut ocfs2_alloc_context) -> ::core::ffi::c_int;
    pub fn ocfs2_free_ac_resource(ac: *mut ocfs2_alloc_context);
    pub fn ocfs2_which_cluster_group(inode: *mut inode, cluster: u32) -> u64;
    pub fn ocfs2_check_group_descriptor(sb: *mut super_block, di: *mut ocfs2_dinode, bh: *mut buffer_head) -> ::core::ffi::c_int;
    pub fn ocfs2_read_group_descriptor(inode: *mut inode, di: *mut ocfs2_dinode, gd_blkno: u64, bh: *mut *mut buffer_head) -> ::core::ffi::c_int;
    pub fn ocfs2_lock_allocators(inode: *mut inode, et: *mut ocfs2_extent_tree, clusters_to_add: u32, extents_to_split: u32, data_ac: *mut *mut ocfs2_alloc_context, meta_ac: *mut *mut ocfs2_alloc_context) -> ::core::ffi::c_int;
    pub fn ocfs2_test_inode_bit(osb: *mut ocfs2_super, blkno: u64, res: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_find_new_inode_loc(dir: *mut inode, parent_fe_bh: *mut buffer_head, ac: *mut ocfs2_alloc_context, fe_blkno: *mut u64) -> ::core::ffi::c_int;
    pub fn ocfs2_claim_new_inode_at_loc(handle: *mut handle_t, dir: *mut inode, ac: *mut ocfs2_alloc_context, suballoc_loc: *mut u64, suballoc_bit: *mut u16, di_blkno: u64) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn ocfs2_alloc_context_bits_left(ac: *mut ocfs2_alloc_context) -> ::core::ffi::c_int {
    (*ac).ac_bits_wanted as ::core::ffi::c_int - (*ac).ac_bits_given as ::core::ffi::c_int
}

#[inline]
pub unsafe fn ocfs2_which_suballoc_group(block: u64, bit: ::core::ffi::c_uint) -> u64 {
    block.wrapping_sub(bit as u64)
}

#[inline]
pub unsafe fn ocfs2_cluster_from_desc(osb: *mut ocfs2_super, bg_blkno: u64) -> u32 {
    /* This should work for all block group descriptors as only
     * the 1st group descriptor of the cluster bitmap is
     * different. */
    if bg_blkno == (*osb).first_cluster_group_blkno { return 0; }
    /* the rest of the block groups are located at the beginning
     * of their 1st cluster, so a direct translation just
     * works. */
    ocfs2_blocks_to_clusters((*osb).sb, bg_blkno)
}

#[inline]
pub unsafe fn ocfs2_is_cluster_bitmap(inode: *mut inode) -> ::core::ffi::c_int {
    let osb = OCFS2_SB((*inode).i_sb);
    (osb.bitmap_blkno == OCFS2_I(inode).ip_blkno) as ::core::ffi::c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
