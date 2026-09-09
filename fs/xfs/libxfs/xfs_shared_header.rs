// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * Copyright (c) 2013 Red Hat, Inc.
 * All Rights Reserved.
 */

/*
 * Definitions shared between kernel and userspace that don't fit into any other
 * header file that is shared with userspace.
 */

#[repr(C)]
pub struct xfs_ifork {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_buf {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_buf_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_trans {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_btree_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfs_trans_res {
    _private: [u8; 0],
}

/* Buffer verifier operations are widely used, including userspace tools. */
extern "C" {
    pub static xfs_agf_buf_ops: xfs_buf_ops;
    pub static xfs_agfl_buf_ops: xfs_buf_ops;
    pub static xfs_agi_buf_ops: xfs_buf_ops;
    pub static xfs_attr3_leaf_buf_ops: xfs_buf_ops;
    pub static xfs_attr3_rmt_buf_ops: xfs_buf_ops;
    pub static xfs_bmbt_buf_ops: xfs_buf_ops;
    pub static xfs_bnobt_buf_ops: xfs_buf_ops;
    pub static xfs_cntbt_buf_ops: xfs_buf_ops;
    pub static xfs_da3_node_buf_ops: xfs_buf_ops;
    pub static xfs_dquot_buf_ops: xfs_buf_ops;
    pub static xfs_dquot_buf_ra_ops: xfs_buf_ops;
    pub static xfs_finobt_buf_ops: xfs_buf_ops;
    pub static xfs_inobt_buf_ops: xfs_buf_ops;
    pub static xfs_inode_buf_ops: xfs_buf_ops;
    pub static xfs_inode_buf_ra_ops: xfs_buf_ops;
    pub static xfs_refcountbt_buf_ops: xfs_buf_ops;
    pub static xfs_rmapbt_buf_ops: xfs_buf_ops;
    pub static xfs_rtbitmap_buf_ops: xfs_buf_ops;
    pub static xfs_rtsummary_buf_ops: xfs_buf_ops;
    pub static xfs_rtbuf_ops: xfs_buf_ops;
    pub static xfs_rtsb_buf_ops: xfs_buf_ops;
    pub static xfs_rtrefcountbt_buf_ops: xfs_buf_ops;
    pub static xfs_rtrmapbt_buf_ops: xfs_buf_ops;
    pub static xfs_sb_buf_ops: xfs_buf_ops;
    pub static xfs_sb_quiet_buf_ops: xfs_buf_ops;
    pub static xfs_symlink_buf_ops: xfs_buf_ops;

    /* btree ops */
    pub static xfs_bnobt_ops: xfs_btree_ops;
    pub static xfs_cntbt_ops: xfs_btree_ops;
    pub static xfs_inobt_ops: xfs_btree_ops;
    pub static xfs_finobt_ops: xfs_btree_ops;
    pub static xfs_bmbt_ops: xfs_btree_ops;
    pub static xfs_refcountbt_ops: xfs_btree_ops;
    pub static xfs_rmapbt_ops: xfs_btree_ops;
    pub static xfs_rmapbt_mem_ops: xfs_btree_ops;
    pub static xfs_rtrmapbt_ops: xfs_btree_ops;
    pub static xfs_rtrmapbt_mem_ops: xfs_btree_ops;
    pub static xfs_rtrefcountbt_ops: xfs_btree_ops;

    pub fn xfs_log_calc_unit_res(mp: *mut xfs_mount, unit_bytes: i32) -> i32;
    pub fn xfs_log_calc_minimum_size(mp: *mut xfs_mount) -> i32;
    pub fn xfs_log_get_max_trans_res(mp: *mut xfs_mount, max_resp: *mut xfs_trans_res);
}

#[inline]
pub unsafe fn xfs_btree_is_bno(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_bnobt_ops }
#[inline]
pub unsafe fn xfs_btree_is_cnt(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_cntbt_ops }
#[inline]
pub unsafe fn xfs_btree_is_bmap(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_bmbt_ops }
#[inline]
pub unsafe fn xfs_btree_is_ino(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_inobt_ops }
#[inline]
pub unsafe fn xfs_btree_is_fino(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_finobt_ops }
#[inline]
pub unsafe fn xfs_btree_is_refcount(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_refcountbt_ops }
#[inline]
pub unsafe fn xfs_btree_is_rmap(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_rmapbt_ops }

/* CONFIG_XFS_BTREE_IN_MEM controls these declarations in the C build. */
#[cfg(feature = "CONFIG_XFS_BTREE_IN_MEM")]
#[inline]
pub unsafe fn xfs_btree_is_mem_rmap(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_rmapbt_mem_ops }
#[cfg(not(feature = "CONFIG_XFS_BTREE_IN_MEM"))]
#[inline]
pub unsafe fn xfs_btree_is_mem_rmap(_ops: *const xfs_btree_ops) -> bool { false }
#[cfg(feature = "CONFIG_XFS_BTREE_IN_MEM")]
#[inline]
pub unsafe fn xfs_btree_is_mem_rtrmap(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_rtrmapbt_mem_ops }
#[cfg(not(feature = "CONFIG_XFS_BTREE_IN_MEM"))]
#[inline]
pub unsafe fn xfs_btree_is_mem_rtrmap(_ops: *const xfs_btree_ops) -> bool { false }

#[inline]
pub unsafe fn xfs_btree_is_rtrmap(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_rtrmapbt_ops }
#[inline]
pub unsafe fn xfs_btree_is_rtrefcount(ops: *const xfs_btree_ops) -> bool { ops == &raw const xfs_rtrefcountbt_ops }

pub const XFS_TRANS_DIRTY: u32 = 1u32 << 0;
pub const XFS_TRANS_SB_DIRTY: u32 = 1u32 << 1;
pub const XFS_TRANS_PERM_LOG_RES: u32 = 1u32 << 2;
pub const XFS_TRANS_SYNC: u32 = 1u32 << 3;
pub const XFS_TRANS_RESERVE: u32 = 1u32 << 4;
pub const XFS_TRANS_NO_WRITECOUNT: u32 = 1u32 << 5;
pub const XFS_TRANS_RES_FDBLKS: u32 = 1u32 << 6;
pub const XFS_TRANS_HAS_INTENT_DONE: u32 = 1u32 << 7;
pub const XFS_TRANS_LOWMODE: u32 = 1u32 << 8;
pub const XFS_TRANS_RTBITMAP_LOCKED: u32 = 1u32 << 9;

pub const XFS_TRANS_SB_ICOUNT: u32 = 0x00000001;
pub const XFS_TRANS_SB_IFREE: u32 = 0x00000002;
pub const XFS_TRANS_SB_FDBLOCKS: u32 = 0x00000004;
pub const XFS_TRANS_SB_RES_FDBLOCKS: u32 = 0x00000008;
pub const XFS_TRANS_SB_FREXTENTS: u32 = 0x00000010;
pub const XFS_TRANS_SB_RES_FREXTENTS: u32 = 0x00000020;
pub const XFS_TRANS_SB_DBLOCKS: u32 = 0x00000040;
pub const XFS_TRANS_SB_AGCOUNT: u32 = 0x00000080;
pub const XFS_TRANS_SB_IMAXPCT: u32 = 0x00000100;
pub const XFS_TRANS_SB_REXTSIZE: u32 = 0x00000200;
pub const XFS_TRANS_SB_RBMBLOCKS: u32 = 0x00000400;
pub const XFS_TRANS_SB_RBLOCKS: u32 = 0x00000800;
pub const XFS_TRANS_SB_REXTENTS: u32 = 0x00001000;
pub const XFS_TRANS_SB_REXTSLOG: u32 = 0x00002000;
pub const XFS_TRANS_SB_RGCOUNT: u32 = 0x00004000;

pub const XFS_AGF_REF: i32 = 4;
pub const XFS_AGI_REF: i32 = 4;
pub const XFS_AGFL_REF: i32 = 3;
pub const XFS_INO_BTREE_REF: i32 = 3;
pub const XFS_ALLOC_BTREE_REF: i32 = 2;
pub const XFS_BMAP_BTREE_REF: i32 = 2;
pub const XFS_RMAP_BTREE_REF: i32 = 2;
pub const XFS_DIR_BTREE_REF: i32 = 2;
pub const XFS_INO_REF: i32 = 2;
pub const XFS_ATTR_BTREE_REF: i32 = 1;
pub const XFS_DQUOT_REF: i32 = 1;
pub const XFS_REFC_BTREE_REF: i32 = 1;
pub const XFS_SSB_REF: i32 = 0;

/* Computed inode geometry for the filesystem. */
#[repr(C)]
pub struct xfs_ino_geometry {
    /* Maximum inode count in this filesystem. */
    pub maxicount: u64,
    /* Actual inode cluster buffer size, in bytes. */
    pub inode_cluster_size: u32,
    /* Desired inode cluster buffer size, in bytes. */
    pub inode_cluster_size_raw: u32,
    /* Inode cluster sizes, adjusted to be at least 1 fsb. */
    pub inodes_per_cluster: u32,
    pub blocks_per_cluster: u32,
    /* Inode cluster alignment. */
    pub cluster_align: u32,
    pub cluster_align_inodes: u32,
    pub inoalign_mask: u32,
    pub inobt_mxr: [u32; 2],
    pub inobt_mnr: [u32; 2],
    pub inobt_maxlevels: u32,
    /* Size of inode allocations under normal operation. */
    pub ialloc_inos: u32,
    pub ialloc_blks: u32,
    /* Minimum inode blocks for a sparse allocation. */
    pub ialloc_min_blks: u32,
    /* stripe unit inode alignment */
    pub ialloc_align: u32,
    pub agino_log: u32,
    /* precomputed default inode attribute fork offset */
    pub attr_fork_offset: u32,
    /* precomputed value for di_flags2 */
    pub new_diflags2: u64,
    /* minimum folio order of a page cache allocation */
    pub min_folio_order: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
