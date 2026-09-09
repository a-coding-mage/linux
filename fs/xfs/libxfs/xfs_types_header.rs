// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

pub type prid_t = u32; // project ID

pub type xfs_agblock_t = u32;
pub type xfs_rgblock_t = u32;
pub type xfs_agino_t = u32;
pub type xfs_extlen_t = u32;
pub type xfs_rtxlen_t = u32;
pub type xfs_agnumber_t = u32;
pub type xfs_rgnumber_t = u32;
pub type xfs_extnum_t = u64;
pub type xfs_aextnum_t = u32;
pub type xfs_fsize_t = i64;
pub type xfs_ufsize_t = u64;

pub type xfs_suminfo_t = i32;
pub type xfs_rtsumoff_t = u32;
pub type xfs_rtword_t = u32;
pub type xfs_lsn_t = i64;
pub type xfs_csn_t = i64;
pub type xfs_dablk_t = u32;
pub type xfs_dahash_t = u32;
pub type xfs_fsblock_t = u64;
pub type xfs_rfsblock_t = u64;
pub type xfs_rtblock_t = u64;
pub type xfs_fileoff_t = u64;
pub type xfs_filblks_t = u64;
pub type xfs_rtxnum_t = u64;
pub type xfs_rtbxlen_t = u64;
pub type xfs_srtblock_t = i64;

/* New verifiers return the instruction address of the failing check; NULL is ok. */
pub type xfs_failaddr_t = *mut core::ffi::c_void;

pub const NULLFSBLOCK: xfs_fsblock_t = u64::MAX;
pub const NULLRFSBLOCK: xfs_rfsblock_t = u64::MAX;
pub const NULLRTBLOCK: xfs_rtblock_t = u64::MAX;
pub const NULLFILEOFF: xfs_fileoff_t = u64::MAX;
pub const NULLAGBLOCK: xfs_agblock_t = u32::MAX;
pub const NULLRGBLOCK: xfs_rgblock_t = u32::MAX;
pub const NULLAGNUMBER: xfs_agnumber_t = u32::MAX;
pub const NULLRGNUMBER: xfs_rgnumber_t = u32::MAX;
pub const NULLCOMMITLSN: xfs_lsn_t = -1;
pub const NULLFSINO: xfs_ino_t = xfs_ino_t::MAX;
pub const NULLAGINO: xfs_agino_t = u32::MAX;

pub const XFS_MIN_BLOCKSIZE_LOG: i32 = 9;
pub const XFS_MAX_BLOCKSIZE_LOG: i32 = 16;
pub const XFS_MIN_BLOCKSIZE: i32 = 1 << XFS_MIN_BLOCKSIZE_LOG;
pub const XFS_MAX_BLOCKSIZE: i32 = 1 << XFS_MAX_BLOCKSIZE_LOG;
pub const XFS_MIN_CRC_BLOCKSIZE: i32 = 1 << (XFS_MIN_BLOCKSIZE_LOG + 1);
pub const XFS_MIN_SECTORSIZE_LOG: i32 = 9;
pub const XFS_MAX_SECTORSIZE_LOG: i32 = 15;
pub const XFS_MIN_SECTORSIZE: i32 = 1 << XFS_MIN_SECTORSIZE_LOG;
pub const XFS_MAX_SECTORSIZE: i32 = 1 << XFS_MAX_SECTORSIZE_LOG;

pub const XFS_STAGING_FORK: i32 = -1;
pub const XFS_DATA_FORK: i32 = 0;
pub const XFS_ATTR_FORK: i32 = 1;
pub const XFS_COW_FORK: i32 = 2;
pub const XFS_WHICHFORK_STRINGS: &[(i32, &str)] = &[
    (XFS_STAGING_FORK, "staging"), (XFS_DATA_FORK, "data"),
    (XFS_ATTR_FORK, "attr"), (XFS_COW_FORK, "cow"),
];

pub const MINDBTPTRS: i32 = 3;
pub const MINABTPTRS: i32 = 2;
pub const MAXNAMELEN: i32 = 256;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum xfs_lookup_t { XFS_LOOKUP_EQi, XFS_LOOKUP_LEi, XFS_LOOKUP_GEi }
pub const XFS_AG_BTREE_CMP_FORMAT_STR: &[(xfs_lookup_t, &str)] = &[
    (xfs_lookup_t::XFS_LOOKUP_EQi, "eq"), (xfs_lookup_t::XFS_LOOKUP_LEi, "le"),
    (xfs_lookup_t::XFS_LOOKUP_GEi, "ge"),
];

#[repr(C)]
pub struct xfs_name { pub name: *const u8, pub len: i32, pub type_: i32 }
pub type xfs_dqid_t = u32;
pub const XFS_NBBYLOG: i32 = 3;
pub const XFS_WORDLOG: i32 = 2;
pub const XFS_SUMINFOLOG: i32 = 2;
pub const XFS_NBWORDLOG: i32 = XFS_NBBYLOG + XFS_WORDLOG;
pub const XFS_NBWORD: i32 = 1 << XFS_NBWORDLOG;
pub const XFS_WORDMASK: i32 = (1 << XFS_WORDLOG) - 1;

#[repr(C)]
pub struct xfs_iext_cursor { pub leaf: *mut xfs_iext_leaf, pub pos: i32 }

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum xfs_exntst_t { XFS_EXT_NORM, XFS_EXT_UNWRITTEN }
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xfs_bmbt_irec { pub br_startoff: xfs_fileoff_t, pub br_startblock: xfs_fsblock_t, pub br_blockcount: xfs_filblks_t, pub br_state: xfs_exntst_t }
pub type xfs_bmbt_irec_t = xfs_bmbt_irec;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum xfs_refc_domain { XFS_REFC_DOMAIN_SHARED = 0, XFS_REFC_DOMAIN_COW }
pub const XFS_REFC_DOMAIN_STRINGS: &[(xfs_refc_domain, &str)] = &[(xfs_refc_domain::XFS_REFC_DOMAIN_SHARED, "shared"), (xfs_refc_domain::XFS_REFC_DOMAIN_COW, "cow")];
#[repr(C)]
pub struct xfs_refcount_irec { pub rc_startblock: xfs_agblock_t, pub rc_blockcount: xfs_extlen_t, pub rc_refcount: xfs_nlink_t, pub rc_domain: xfs_refc_domain }

pub const XFS_RMAP_ATTR_FORK: u32 = 1 << 0;
pub const XFS_RMAP_BMBT_BLOCK: u32 = 1 << 1;
pub const XFS_RMAP_UNWRITTEN: u32 = 1 << 2;
pub const XFS_RMAP_KEY_FLAGS: u32 = XFS_RMAP_ATTR_FORK | XFS_RMAP_BMBT_BLOCK;
pub const XFS_RMAP_REC_FLAGS: u32 = XFS_RMAP_UNWRITTEN;
#[repr(C)]
pub struct xfs_rmap_irec { pub rm_startblock: xfs_agblock_t, pub rm_blockcount: xfs_extlen_t, pub rm_owner: u64, pub rm_offset: u64, pub rm_flags: u32 }

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum xfs_ag_resv_type { XFS_AG_RESV_NONE = 0, XFS_AG_RESV_AGFL, XFS_AG_RESV_METADATA, XFS_AG_RESV_RMAPBT, XFS_AG_RESV_IGNORE, XFS_AG_RESV_METAFILE }
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum xbtree_recpacking { XBTREE_RECPACKING_EMPTY = 0, XBTREE_RECPACKING_SPARSE, XBTREE_RECPACKING_FULL }
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum xfs_group_type { XG_TYPE_AG, XG_TYPE_RTG, XG_TYPE_MAX }
pub const XG_TYPE_STRINGS: &[(xfs_group_type, &str)] = &[(xfs_group_type::XG_TYPE_AG, "ag"), (xfs_group_type::XG_TYPE_RTG, "rtg")];
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum xfs_free_counter { XC_FREE_BLOCKS, XC_FREE_RTEXTENTS, XC_FREE_RTAVAILABLE, XC_FREE_NR }
pub const XFS_FREECOUNTER_STR: &[(xfs_free_counter, &str)] = &[(xfs_free_counter::XC_FREE_BLOCKS, "blocks"), (xfs_free_counter::XC_FREE_RTEXTENTS, "rtextents"), (xfs_free_counter::XC_FREE_RTAVAILABLE, "rtavailable")];

#[repr(C)] pub struct xfs_mount;
unsafe extern "C" {
    pub fn xfs_verify_fsbno(mp: *mut xfs_mount, fsbno: xfs_fsblock_t) -> bool;
    pub fn xfs_verify_fsbext(mp: *mut xfs_mount, fsbno: xfs_fsblock_t, len: xfs_fsblock_t) -> bool;
    pub fn xfs_verify_ino(mp: *mut xfs_mount, ino: xfs_ino_t) -> bool;
    pub fn xfs_is_sb_inum(mp: *mut xfs_mount, ino: xfs_ino_t) -> bool;
    pub fn xfs_verify_dir_ino(mp: *mut xfs_mount, ino: xfs_ino_t) -> bool;
    pub fn xfs_verify_rtbno(mp: *mut xfs_mount, rtbno: xfs_rtblock_t) -> bool;
    pub fn xfs_verify_rtbext(mp: *mut xfs_mount, rtbno: xfs_rtblock_t, len: xfs_filblks_t) -> bool;
    pub fn xfs_verify_icount(mp: *mut xfs_mount, icount: u64) -> bool;
    pub fn xfs_verify_dablk(mp: *mut xfs_mount, off: xfs_fileoff_t) -> bool;
    pub fn xfs_icount_range(mp: *mut xfs_mount, min: *mut u64, max: *mut u64);
    pub fn xfs_verify_fileoff(mp: *mut xfs_mount, off: xfs_fileoff_t) -> bool;
    pub fn xfs_verify_fileext(mp: *mut xfs_mount, off: xfs_fileoff_t, len: xfs_fileoff_t) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
