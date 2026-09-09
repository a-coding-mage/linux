// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/*
 * Components of space reservations.
 */

/* Worst case number of bmaps that can be held in a block. */
#[macro_export]
macro_rules! XFS_MAX_CONTIG_BMAPS_PER_BLOCK { ($mp:expr) => { (($mp).m_bmap_dmxr[0]) - (($mp).m_bmap_dmnr[0]) }; }

/* Worst case number of realtime rmaps that can be held in a block. */
#[macro_export]
macro_rules! XFS_MAX_CONTIG_RTRMAPS_PER_BLOCK { ($mp:expr) => { (($mp).m_rtrmap_mxr[0]) - (($mp).m_rtrmap_mnr[0]) }; }

/* Adding one realtime rmap could split every level to the top of the tree. */
#[macro_export]
macro_rules! XFS_RTRMAPADD_SPACE_RES { ($mp:expr) => { ($mp).m_rtrmap_maxlevels }; }

/* Blocks we might need to add "b" realtime rmaps to a tree. */
#[macro_export]
macro_rules! XFS_NRTRMAPADD_SPACE_RES { ($mp:expr, $b:expr) => {
    ((($b) + XFS_MAX_CONTIG_RTRMAPS_PER_BLOCK!($mp) - 1) /
        XFS_MAX_CONTIG_RTRMAPS_PER_BLOCK!($mp)) * XFS_RTRMAPADD_SPACE_RES!($mp)
}; }

/* Worst case number of rmaps that can be held in a block. */
#[macro_export]
macro_rules! XFS_MAX_CONTIG_RMAPS_PER_BLOCK { ($mp:expr) => { (($mp).m_rmap_mxr[0]) - (($mp).m_rmap_mnr[0]) }; }

/* Adding one rmap could split every level up to the top of the tree. */
#[macro_export]
macro_rules! XFS_RMAPADD_SPACE_RES { ($mp:expr) => { ($mp).m_rmap_maxlevels }; }

/*
 * Note that we historically set m_rmap_maxlevels to 9 when reflink is enabled,
 * so we must preserve this behavior to avoid changing the transaction space
 * reservations and minimum log size calculations for existing filesystems.
 */
pub const XFS_OLD_REFLINK_RMAP_MAXLEVELS: i32 = 9;

/* Blocks we might need to add "b" rmaps to a tree. */
#[macro_export]
macro_rules! XFS_NRMAPADD_SPACE_RES { ($mp:expr, $b:expr) => {
    ((($b) + XFS_MAX_CONTIG_RMAPS_PER_BLOCK!($mp) - 1) /
        XFS_MAX_CONTIG_RMAPS_PER_BLOCK!($mp)) * XFS_RMAPADD_SPACE_RES!($mp)
}; }

#[macro_export]
macro_rules! XFS_MAX_CONTIG_EXTENTS_PER_BLOCK { ($mp:expr) => { (($mp).m_alloc_mxr[0]) - (($mp).m_alloc_mnr[0]) }; }
#[macro_export]
macro_rules! XFS_EXTENTADD_SPACE_RES { ($mp:expr, $w:expr) => { XFS_BM_MAXLEVELS!($mp, $w) - 1 }; }
#[macro_export]
macro_rules! XFS_NEXTENTADD_SPACE_RES { ($mp:expr, $b:expr, $w:expr) => {
    ((($b) + XFS_MAX_CONTIG_EXTENTS_PER_BLOCK!($mp) - 1) /
        XFS_MAX_CONTIG_EXTENTS_PER_BLOCK!($mp)) * XFS_EXTENTADD_SPACE_RES!($mp, $w)
}; }

/* Blocks we might need to add "b" mappings & rmappings to a file. */
#[macro_export]
macro_rules! XFS_SWAP_RMAP_SPACE_RES { ($mp:expr, $b:expr, $w:expr) => {
    XFS_NEXTENTADD_SPACE_RES!($mp, $b, $w) + XFS_NRMAPADD_SPACE_RES!($mp, $b)
}; }

#[macro_export]
macro_rules! XFS_DAENTER_1B { ($mp:expr, $w:expr) => { if ($w) == XFS_DATA_FORK { ($mp).m_dir_geo.fsbcount } else { 1 } }; }
#[macro_export]
macro_rules! XFS_DAENTER_DBS { ($mp:expr, $w:expr) => { XFS_DA_NODE_MAXDEPTH + (if ($w) == XFS_DATA_FORK { 2 } else { 0 }) }; }
#[macro_export]
macro_rules! XFS_DAENTER_BLOCKS { ($mp:expr, $w:expr) => { XFS_DAENTER_1B!($mp, $w) * XFS_DAENTER_DBS!($mp, $w) }; }
#[macro_export]
macro_rules! XFS_DAENTER_BMAP1B { ($mp:expr, $w:expr) => { XFS_NEXTENTADD_SPACE_RES!($mp, XFS_DAENTER_1B!($mp, $w), $w) }; }
#[macro_export]
macro_rules! XFS_DAENTER_BMAPS { ($mp:expr, $w:expr) => { XFS_DAENTER_DBS!($mp, $w) * XFS_DAENTER_BMAP1B!($mp, $w) }; }
#[macro_export]
macro_rules! XFS_DAENTER_SPACE_RES { ($mp:expr, $w:expr) => { XFS_DAENTER_BLOCKS!($mp, $w) + XFS_DAENTER_BMAPS!($mp, $w) }; }
#[macro_export]
macro_rules! XFS_DAREMOVE_SPACE_RES { ($mp:expr, $w:expr) => { XFS_DAENTER_BMAPS!($mp, $w) }; }
#[macro_export]
macro_rules! XFS_DIRENTER_MAX_SPLIT { ($mp:expr, $nl:expr) => { 1 }; }
#[macro_export]
macro_rules! XFS_DIRENTER_SPACE_RES { ($mp:expr, $nl:expr) => { XFS_DAENTER_SPACE_RES!($mp, XFS_DATA_FORK) * XFS_DIRENTER_MAX_SPLIT!($mp, $nl) }; }
#[macro_export]
macro_rules! XFS_DIRREMOVE_SPACE_RES { ($mp:expr) => { XFS_DAREMOVE_SPACE_RES!($mp, XFS_DATA_FORK) }; }
#[macro_export]
macro_rules! XFS_IALLOC_SPACE_RES { ($mp:expr) => { M_IGEO!($mp).ialloc_blks + (if xfs_has_finobt!($mp) { 2 } else { 1 }) * M_IGEO!($mp).inobt_maxlevels }; }

/* Space reservation values for various transactions. */
#[macro_export]
macro_rules! XFS_ADDAFORK_SPACE_RES { ($mp:expr) => { ($mp).m_dir_geo.fsbcount + XFS_DAENTER_BMAP1B!($mp, XFS_DATA_FORK) }; }
#[macro_export]
macro_rules! XFS_ATTRRM_SPACE_RES { ($mp:expr) => { XFS_DAREMOVE_SPACE_RES!($mp, XFS_ATTR_FORK) }; }
/* This macro is not used - see inline code in xfs_attr_set */
#[macro_export]
macro_rules! XFS_ATTRSET_SPACE_RES { ($mp:expr, $v:expr) => { XFS_DAENTER_SPACE_RES!($mp, XFS_ATTR_FORK) + XFS_B_TO_FSB!($mp, $v) }; }
#[macro_export]
macro_rules! XFS_DIOSTRAT_SPACE_RES { ($mp:expr, $v:expr) => { XFS_EXTENTADD_SPACE_RES!($mp, XFS_DATA_FORK) + ($v) }; }
#[macro_export]
macro_rules! XFS_GROWFS_SPACE_RES { ($mp:expr) => { 2 * ($mp).m_alloc_maxlevels }; }
#[macro_export]
macro_rules! XFS_GROWFSRT_SPACE_RES { ($mp:expr, $b:expr) => { ($b) + XFS_EXTENTADD_SPACE_RES!($mp, XFS_DATA_FORK) }; }
#[macro_export]
macro_rules! XFS_QM_DQALLOC_SPACE_RES { ($mp:expr) => { XFS_EXTENTADD_SPACE_RES!($mp, XFS_DATA_FORK) + XFS_DQUOT_CLUSTER_SIZE_FSB }; }
#[macro_export]
macro_rules! XFS_QM_QINOCREATE_SPACE_RES { ($mp:expr) => { XFS_IALLOC_SPACE_RES!($mp) }; }
#[macro_export]
macro_rules! XFS_IFREE_SPACE_RES { ($mp:expr) => { if xfs_has_finobt!($mp) { M_IGEO!($mp).inobt_maxlevels } else { 0 } }; }

extern "C" {
    pub fn xfs_parent_calc_space_res(mp: *mut xfs_mount, namelen: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn xfs_create_space_res(mp: *mut xfs_mount, namelen: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn xfs_mkdir_space_res(mp: *mut xfs_mount, namelen: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn xfs_link_space_res(mp: *mut xfs_mount, namelen: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn xfs_symlink_space_res(mp: *mut xfs_mount, namelen: ::core::ffi::c_uint, fsblocks: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn xfs_remove_space_res(mp: *mut xfs_mount, namelen: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn xfs_rename_space_res(mp: *mut xfs_mount, src_namelen: ::core::ffi::c_uint, target_exists: bool, target_namelen: ::core::ffi::c_uint, has_whiteout: bool) -> ::core::ffi::c_uint;
}

#[repr(C)]
pub struct xfs_mount { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
