// SPDX-License-Identifier: GPL-2.0
/*
 * Type definitions for the multi-level security (MLS) policy.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/*
 * Updated: Trusted Computer Solutions, Inc. <dgoeddel@trustedcs.com>
 *          Support for enhanced MLS infrastructure.
 *          Copyright (C) 2004-2005 Trusted Computer Solutions, Inc.
 */

// C dependencies: "security.h" and "ebitmap.h".

unsafe extern "C" {
    pub fn ebitmap_equal(e1: *const ebitmap, e2: *const ebitmap) -> core::ffi::c_int;
    pub fn ebitmap_contains(
        e1: *const ebitmap,
        e2: *const ebitmap,
        startbit: u32,
    ) -> core::ffi::c_int;
}

#[repr(C)]
pub struct mls_level {
    pub sens: u32,       /* sensitivity */
    pub cat: ebitmap,    /* category set */
}

#[repr(C)]
pub struct mls_range {
    pub level: [mls_level; 2], /* low == level[0], high == level[1] */
}

#[inline]
pub unsafe fn mls_level_eq(
    l1: *const mls_level,
    l2: *const mls_level,
) -> core::ffi::c_int {
    (((*l1).sens == (*l2).sens)
        && ebitmap_equal(&raw const (*l1).cat, &raw const (*l2).cat) != 0)
        as core::ffi::c_int
}

#[inline]
pub unsafe fn mls_level_dom(
    l1: *const mls_level,
    l2: *const mls_level,
) -> core::ffi::c_int {
    (((*l1).sens >= (*l2).sens)
        && ebitmap_contains(&raw const (*l1).cat, &raw const (*l2).cat, 0) != 0)
        as core::ffi::c_int
}

#[inline]
pub unsafe fn mls_level_incomp(
    l1: *const mls_level,
    l2: *const mls_level,
) -> core::ffi::c_int {
    (mls_level_dom(l1, l2) == 0 && mls_level_dom(l2, l1) == 0) as core::ffi::c_int
}

#[inline]
pub unsafe fn mls_level_between(
    l1: *const mls_level,
    l2: *const mls_level,
    l3: *const mls_level,
) -> core::ffi::c_int {
    (mls_level_dom(l1, l2) != 0 && mls_level_dom(l3, l1) != 0) as core::ffi::c_int
}

#[inline]
pub unsafe fn mls_range_contains(
    r1: *const mls_range,
    r2: *const mls_range,
) -> core::ffi::c_int {
    (mls_level_dom(&raw const (*r2).level[0], &raw const (*r1).level[0]) != 0
        && mls_level_dom(&raw const (*r1).level[1], &raw const (*r2).level[1]) != 0)
        as core::ffi::c_int
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
