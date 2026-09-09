// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding XFS translation.
#[repr(C)]
pub struct xfs_mount {
    _private: [u8; 0],
}

extern "C" {
    fn XFS_DAENTER_SPACE_RES(mp: *mut xfs_mount, fork: u32) -> u32;
    fn XFS_NEXTENTADD_SPACE_RES(mp: *mut xfs_mount, namelen: u32, fork: u32) -> u32;
    fn XFS_IALLOC_SPACE_RES(mp: *mut xfs_mount) -> u32;
    fn XFS_DIRENTER_SPACE_RES(mp: *mut xfs_mount, namelen: u32) -> u32;
    fn XFS_DIRREMOVE_SPACE_RES(mp: *mut xfs_mount) -> u32;
    fn xfs_has_parent(mp: *mut xfs_mount) -> bool;
}

// XFS_ATTR_FORK is supplied by xfs_fs.h / the surrounding translation.
extern "C" {
    static XFS_ATTR_FORK: u32;
}

/* Calculate the disk space required to add a parent pointer. */
pub unsafe fn xfs_parent_calc_space_res(
    mp: *mut xfs_mount,
    namelen: u32,
) -> u32 {
    /*
     * Parent pointers are always the first attr in an attr tree, and never
     * larger than a block
     */
    XFS_DAENTER_SPACE_RES(mp, XFS_ATTR_FORK)
        .wrapping_add(XFS_NEXTENTADD_SPACE_RES(mp, namelen, XFS_ATTR_FORK))
}

pub unsafe fn xfs_create_space_res(
    mp: *mut xfs_mount,
    namelen: u32,
) -> u32 {
    let mut ret = XFS_IALLOC_SPACE_RES(mp)
        .wrapping_add(XFS_DIRENTER_SPACE_RES(mp, namelen));
    if xfs_has_parent(mp) {
        ret = ret.wrapping_add(xfs_parent_calc_space_res(mp, namelen));
    }
    ret
}

pub unsafe fn xfs_mkdir_space_res(
    mp: *mut xfs_mount,
    namelen: u32,
) -> u32 {
    xfs_create_space_res(mp, namelen)
}

pub unsafe fn xfs_link_space_res(
    mp: *mut xfs_mount,
    namelen: u32,
) -> u32 {
    let mut ret = XFS_DIRENTER_SPACE_RES(mp, namelen);
    if xfs_has_parent(mp) {
        ret = ret.wrapping_add(xfs_parent_calc_space_res(mp, namelen));
    }
    ret
}

pub unsafe fn xfs_symlink_space_res(
    mp: *mut xfs_mount,
    namelen: u32,
    fsblocks: u32,
) -> u32 {
    let mut ret = XFS_IALLOC_SPACE_RES(mp)
        .wrapping_add(XFS_DIRENTER_SPACE_RES(mp, namelen))
        .wrapping_add(fsblocks);
    if xfs_has_parent(mp) {
        ret = ret.wrapping_add(xfs_parent_calc_space_res(mp, namelen));
    }
    ret
}

pub unsafe fn xfs_remove_space_res(
    mp: *mut xfs_mount,
    namelen: u32,
) -> u32 {
    let mut ret = XFS_DIRREMOVE_SPACE_RES(mp);
    if xfs_has_parent(mp) {
        ret = ret.wrapping_add(xfs_parent_calc_space_res(mp, namelen));
    }
    ret
}

pub unsafe fn xfs_rename_space_res(
    mp: *mut xfs_mount,
    src_namelen: u32,
    target_exists: bool,
    target_namelen: u32,
    has_whiteout: bool,
) -> u32 {
    let mut ret = XFS_DIRREMOVE_SPACE_RES(mp)
        .wrapping_add(XFS_DIRENTER_SPACE_RES(mp, target_namelen));

    if xfs_has_parent(mp) {
        if has_whiteout {
            ret = ret.wrapping_add(xfs_parent_calc_space_res(mp, src_namelen));
        }
        ret = ret.wrapping_add(
            2u32.wrapping_mul(xfs_parent_calc_space_res(mp, target_namelen)),
        );
    }

    if target_exists {
        ret = ret.wrapping_add(xfs_parent_calc_space_res(mp, target_namelen));
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
