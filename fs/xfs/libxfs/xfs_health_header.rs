/* SPDX-License-Identifier: GPL-2.0+ */
/* In-Core Filesystem Health Assessments. */

// Forward declarations supplied by the XFS implementation.
pub enum xfs_group {}
pub enum xfs_mount {}
pub enum xfs_perag {}
pub enum xfs_inode {}
pub enum xfs_fsop_geom {}
pub enum xfs_btree_cur {}
pub enum xfs_da_args {}
pub enum xfs_rtgroup {}
pub enum xfs_ag_geometry {}
pub enum xfs_rtgroup_geometry {}
pub enum xfs_bulkstat {}

pub const XFS_SICK_FS_COUNTERS: u32 = 1 << 0;
pub const XFS_SICK_FS_UQUOTA: u32 = 1 << 1;
pub const XFS_SICK_FS_GQUOTA: u32 = 1 << 2;
pub const XFS_SICK_FS_PQUOTA: u32 = 1 << 3;
pub const XFS_SICK_FS_QUOTACHECK: u32 = 1 << 4;
pub const XFS_SICK_FS_NLINKS: u32 = 1 << 5;
pub const XFS_SICK_FS_METADIR: u32 = 1 << 6;
pub const XFS_SICK_FS_METAPATH: u32 = 1 << 7;

pub const XFS_SICK_RG_SUPER: u32 = 1 << 0;
pub const XFS_SICK_RG_BITMAP: u32 = 1 << 1;
pub const XFS_SICK_RG_SUMMARY: u32 = 1 << 2;
pub const XFS_SICK_RG_RMAPBT: u32 = 1 << 3;
pub const XFS_SICK_RG_REFCNTBT: u32 = 1 << 4;

pub const XFS_SICK_AG_SB: u32 = 1 << 0;
pub const XFS_SICK_AG_AGF: u32 = 1 << 1;
pub const XFS_SICK_AG_AGFL: u32 = 1 << 2;
pub const XFS_SICK_AG_AGI: u32 = 1 << 3;
pub const XFS_SICK_AG_BNOBT: u32 = 1 << 4;
pub const XFS_SICK_AG_CNTBT: u32 = 1 << 5;
pub const XFS_SICK_AG_INOBT: u32 = 1 << 6;
pub const XFS_SICK_AG_FINOBT: u32 = 1 << 7;
pub const XFS_SICK_AG_RMAPBT: u32 = 1 << 8;
pub const XFS_SICK_AG_REFCNTBT: u32 = 1 << 9;
pub const XFS_SICK_AG_INODES: u32 = 1 << 10;

pub const XFS_SICK_INO_CORE: u32 = 1 << 0;
pub const XFS_SICK_INO_BMBTD: u32 = 1 << 1;
pub const XFS_SICK_INO_BMBTA: u32 = 1 << 2;
pub const XFS_SICK_INO_BMBTC: u32 = 1 << 3;
pub const XFS_SICK_INO_DIR: u32 = 1 << 4;
pub const XFS_SICK_INO_XATTR: u32 = 1 << 5;
pub const XFS_SICK_INO_SYMLINK: u32 = 1 << 6;
pub const XFS_SICK_INO_PARENT: u32 = 1 << 7;
pub const XFS_SICK_INO_BMBTD_ZAPPED: u32 = 1 << 8;
pub const XFS_SICK_INO_BMBTA_ZAPPED: u32 = 1 << 9;
pub const XFS_SICK_INO_DIR_ZAPPED: u32 = 1 << 10;
pub const XFS_SICK_INO_SYMLINK_ZAPPED: u32 = 1 << 11;
pub const XFS_SICK_INO_FORGET: u32 = 1 << 12;
pub const XFS_SICK_INO_DIRTREE: u32 = 1 << 13;

pub const XFS_SICK_FS_PRIMARY: u32 = XFS_SICK_FS_COUNTERS | XFS_SICK_FS_UQUOTA | XFS_SICK_FS_GQUOTA | XFS_SICK_FS_PQUOTA | XFS_SICK_FS_QUOTACHECK | XFS_SICK_FS_NLINKS | XFS_SICK_FS_METADIR | XFS_SICK_FS_METAPATH;
pub const XFS_SICK_RG_PRIMARY: u32 = XFS_SICK_RG_SUPER | XFS_SICK_RG_BITMAP | XFS_SICK_RG_SUMMARY | XFS_SICK_RG_RMAPBT | XFS_SICK_RG_REFCNTBT;
pub const XFS_SICK_AG_PRIMARY: u32 = XFS_SICK_AG_SB | XFS_SICK_AG_AGF | XFS_SICK_AG_AGFL | XFS_SICK_AG_AGI | XFS_SICK_AG_BNOBT | XFS_SICK_AG_CNTBT | XFS_SICK_AG_INOBT | XFS_SICK_AG_FINOBT | XFS_SICK_AG_RMAPBT | XFS_SICK_AG_REFCNTBT;
pub const XFS_SICK_INO_PRIMARY: u32 = XFS_SICK_INO_CORE | XFS_SICK_INO_BMBTD | XFS_SICK_INO_BMBTA | XFS_SICK_INO_BMBTC | XFS_SICK_INO_DIR | XFS_SICK_INO_XATTR | XFS_SICK_INO_SYMLINK | XFS_SICK_INO_PARENT | XFS_SICK_INO_DIRTREE;
pub const XFS_SICK_INO_ZAPPED: u32 = XFS_SICK_INO_BMBTD_ZAPPED | XFS_SICK_INO_BMBTA_ZAPPED | XFS_SICK_INO_DIR_ZAPPED | XFS_SICK_INO_SYMLINK_ZAPPED;
pub const XFS_SICK_FS_SECONDARY: u32 = 0;
pub const XFS_SICK_RG_SECONDARY: u32 = 0;
pub const XFS_SICK_AG_SECONDARY: u32 = 0;
pub const XFS_SICK_INO_SECONDARY: u32 = XFS_SICK_INO_FORGET;
pub const XFS_SICK_FS_INDIRECT: u32 = 0;
pub const XFS_SICK_RG_INDIRECT: u32 = 0;
pub const XFS_SICK_AG_INDIRECT: u32 = XFS_SICK_AG_INODES;
pub const XFS_SICK_INO_INDIRECT: u32 = 0;
pub const XFS_SICK_FS_ALL: u32 = XFS_SICK_FS_PRIMARY | XFS_SICK_FS_SECONDARY | XFS_SICK_FS_INDIRECT;
pub const XFS_SICK_RG_ALL: u32 = XFS_SICK_RG_PRIMARY | XFS_SICK_RG_SECONDARY | XFS_SICK_RG_INDIRECT;
pub const XFS_SICK_AG_ALL: u32 = XFS_SICK_AG_PRIMARY | XFS_SICK_AG_SECONDARY | XFS_SICK_AG_INDIRECT;
pub const XFS_SICK_INO_ALL: u32 = XFS_SICK_INO_PRIMARY | XFS_SICK_INO_SECONDARY | XFS_SICK_INO_INDIRECT | XFS_SICK_INO_ZAPPED;

extern "C" {
    pub fn xfs_fs_mark_sick(mp: *mut xfs_mount, mask: u32);
    pub fn xfs_fs_mark_corrupt(mp: *mut xfs_mount, mask: u32);
    pub fn xfs_fs_mark_healthy(mp: *mut xfs_mount, mask: u32);
    pub fn xfs_fs_measure_sickness(mp: *mut xfs_mount, sick: *mut u32, checked: *mut u32);
    pub fn xfs_rgno_mark_sick(mp: *mut xfs_mount, rgno: xfs_rgnumber_t, mask: u32);
    pub fn xfs_agno_mark_sick(mp: *mut xfs_mount, agno: xfs_agnumber_t, mask: u32);
    pub fn xfs_group_mark_sick(xg: *mut xfs_group, mask: u32);
    pub fn xfs_group_mark_corrupt(xg: *mut xfs_group, mask: u32);
    pub fn xfs_group_mark_healthy(xg: *mut xfs_group, mask: u32);
    pub fn xfs_group_measure_sickness(xg: *mut xfs_group, sick: *mut u32, checked: *mut u32);
    pub fn xfs_inode_mark_sick(ip: *mut xfs_inode, mask: u32);
    pub fn xfs_inode_mark_corrupt(ip: *mut xfs_inode, mask: u32);
    pub fn xfs_inode_mark_healthy(ip: *mut xfs_inode, mask: u32);
    pub fn xfs_inode_measure_sickness(ip: *mut xfs_inode, sick: *mut u32, checked: *mut u32);
    pub fn xfs_health_unmount(mp: *mut xfs_mount);
    pub fn xfs_bmap_mark_sick(ip: *mut xfs_inode, whichfork: i32);
    pub fn xfs_btree_mark_sick(cur: *mut xfs_btree_cur);
    pub fn xfs_dirattr_mark_sick(ip: *mut xfs_inode, whichfork: i32);
    pub fn xfs_da_mark_sick(args: *mut xfs_da_args);
    pub fn xfs_fsop_geom_health(mp: *mut xfs_mount, geo: *mut xfs_fsop_geom);
    pub fn xfs_ag_geom_health(pag: *mut xfs_perag, ageo: *mut xfs_ag_geometry);
    pub fn xfs_rtgroup_geom_health(rtg: *mut xfs_rtgroup, rgeo: *mut xfs_rtgroup_geometry);
    pub fn xfs_bulkstat_health(ip: *mut xfs_inode, bs: *mut xfs_bulkstat);
    pub fn xfs_healthmon_inode_mask(sick_mask: u32) -> u32;
    pub fn xfs_healthmon_rtgroup_mask(sick_mask: u32) -> u32;
    pub fn xfs_healthmon_perag_mask(sick_mask: u32) -> u32;
    pub fn xfs_healthmon_fs_mask(sick_mask: u32) -> u32;
}

// xfs_ag_mark_sick, xfs_ag_has_sickness, xfs_ag_is_healthy,
// xfs_rtgroup_has_sickness, xfs_rtgroup_is_healthy, and xfs_metadata_is_sick
// retain their C macro intent; pag_group/rtg_group, UINT_MAX, errno constants,
// and unlikely are supplied by other translated headers.

pub unsafe fn xfs_fs_has_sickness(mp: *mut xfs_mount, mask: u32) -> bool {
    let mut sick = 0u32;
    let mut checked = 0u32;
    xfs_fs_measure_sickness(mp, &mut sick, &mut checked);
    (sick & mask) != 0
}

pub unsafe fn xfs_group_has_sickness(xg: *mut xfs_group, mask: u32) -> bool {
    let mut sick = 0u32;
    let mut checked = 0u32;
    xfs_group_measure_sickness(xg, &mut sick, &mut checked);
    (sick & mask) != 0
}

pub unsafe fn xfs_inode_has_sickness(ip: *mut xfs_inode, mask: u32) -> bool {
    let mut sick = 0u32;
    let mut checked = 0u32;
    xfs_inode_measure_sickness(ip, &mut sick, &mut checked);
    (sick & mask) != 0
}

pub unsafe fn xfs_fs_is_healthy(mp: *mut xfs_mount) -> bool {
    !xfs_fs_has_sickness(mp, u32::MAX)
}

pub unsafe fn xfs_inode_is_healthy(ip: *mut xfs_inode) -> bool {
    !xfs_inode_has_sickness(ip, u32::MAX)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
