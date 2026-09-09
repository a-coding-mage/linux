/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000,2002,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/*
 * We generate this then sort it, attr_list() must return things in hash-order.
 */
#[repr(C)]
pub struct xfs_attr_sf_sort {
    pub entno: u8,       /* entry number in original list */
    pub namelen: u8,     /* length of name value (no null) */
    pub valuelen: u8,    /* length of value */
    pub flags: u8,       /* flags bits (see xfs_attr_leaf.h) */
    pub hash: xfs_dahash_t, /* this entry's hash value */
    pub name: *mut u8,   /* name value, pointer into buffer */
    pub value: *mut core::ffi::c_void,
}

pub type xfs_attr_sf_sort_t = xfs_attr_sf_sort;

pub const XFS_ATTR_SF_ENTSIZE_MAX: usize =
    (1usize << (NBBY * core::mem::size_of::<u8>())) - 1;

/* space name/value uses */
#[inline]
pub unsafe fn xfs_attr_sf_entsize_byname(nlen: u8, vlen: u8) -> i32 {
    (core::mem::size_of::<xfs_attr_sf_entry>() + nlen as usize + vlen as usize) as i32
}

/* space an entry uses */
#[inline]
pub unsafe fn xfs_attr_sf_entsize(sfep: *mut xfs_attr_sf_entry) -> i32 {
    struct_size(
        sfep,
        (*sfep).nameval,
        (*sfep).namelen as usize + (*sfep).valuelen as usize,
    )
}

/* first entry in the SF attr fork */
#[inline]
pub unsafe fn xfs_attr_sf_firstentry(
    hdr: *mut xfs_attr_sf_hdr,
) -> *mut xfs_attr_sf_entry {
    hdr.add(1) as *mut xfs_attr_sf_entry
}

/* next entry after sfep */
#[inline]
pub unsafe fn xfs_attr_sf_nextentry(
    sfep: *mut xfs_attr_sf_entry,
) -> *mut xfs_attr_sf_entry {
    (sfep as *mut u8).add(xfs_attr_sf_entsize(sfep) as usize) as *mut xfs_attr_sf_entry
}

/* pointer to the space after the last entry, e.g. for adding a new one */
#[inline]
pub unsafe fn xfs_attr_sf_endptr(
    sf: *mut xfs_attr_sf_hdr,
) -> *mut xfs_attr_sf_entry {
    (sf as *mut u8).add(be16_to_cpu((*sf).totsize) as usize) as *mut xfs_attr_sf_entry
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
