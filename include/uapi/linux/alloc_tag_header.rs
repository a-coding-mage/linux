/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * alloc_tag IOCTL API definition
 *
 * Copyright (C) 2026 Google, LLC.  All rights reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// Dependency intent: the C header includes <linux/types.h>.

/*
 * Function, file and module names often have the same prefixes, therefore
 * when filtering by these criteria, we compare the last 64 characters to
 * minimize the chances of name collisions
 */
pub const ALLOCINFO_STR_SIZE: usize = 64;

#[repr(C)]
pub struct allocinfo_content_id {
    pub id: __u64,
}

#[repr(C)]
pub struct allocinfo_tag {
    /* Longer names are trimmed */
    pub modname: [core::ffi::c_char; ALLOCINFO_STR_SIZE],
    pub function: [core::ffi::c_char; ALLOCINFO_STR_SIZE],
    pub filename: [core::ffi::c_char; ALLOCINFO_STR_SIZE],
    pub lineno: __u64,
}

/* The alignment ensures 32-bit compatible interfaces are not broken */
#[repr(C, align(8))]
pub struct allocinfo_counter {
    pub bytes: __u64,
    pub calls: __u64,
    pub accurate: __u8,
}

#[repr(C)]
pub struct allocinfo_tag_data {
    pub tag: allocinfo_tag,
    pub counter: allocinfo_counter,
}

#[repr(i32)]
pub enum allocinfo_filter_kind {
    ALLOCINFO_FILTER_MODNAME,
    ALLOCINFO_FILTER_FUNCTION,
    ALLOCINFO_FILTER_FILENAME,
    ALLOCINFO_FILTER_LINENO,
    ALLOCINFO_FILTER_INACCURATE,
    ALLOCINFO_FILTER_MIN_SIZE,
    ALLOCINFO_FILTER_MAX_SIZE,
    __ALLOCINFO_FILTER_LAST = ALLOCINFO_FILTER_MAX_SIZE as isize,
}

pub const ALLOCINFO_FILTER_MASK_MODNAME: __u64 =
    1u64 << (allocinfo_filter_kind::ALLOCINFO_FILTER_MODNAME as u32);
pub const ALLOCINFO_FILTER_MASK_FUNCTION: __u64 =
    1u64 << (allocinfo_filter_kind::ALLOCINFO_FILTER_FUNCTION as u32);
pub const ALLOCINFO_FILTER_MASK_FILENAME: __u64 =
    1u64 << (allocinfo_filter_kind::ALLOCINFO_FILTER_FILENAME as u32);
pub const ALLOCINFO_FILTER_MASK_LINENO: __u64 =
    1u64 << (allocinfo_filter_kind::ALLOCINFO_FILTER_LINENO as u32);
pub const ALLOCINFO_FILTER_MASK_INACCURATE: __u64 =
    1u64 << (allocinfo_filter_kind::ALLOCINFO_FILTER_INACCURATE as u32);
pub const ALLOCINFO_FILTER_MASK_MIN_SIZE: __u64 =
    1u64 << (allocinfo_filter_kind::ALLOCINFO_FILTER_MIN_SIZE as u32);
pub const ALLOCINFO_FILTER_MASK_MAX_SIZE: __u64 =
    1u64 << (allocinfo_filter_kind::ALLOCINFO_FILTER_MAX_SIZE as u32);

pub const ALLOCINFO_FILTER_MASKS: __u64 =
    (1u64 << ((allocinfo_filter_kind::__ALLOCINFO_FILTER_LAST as u32) + 1)) - 1;

#[repr(C)]
pub struct allocinfo_filter {
    pub mask: __u64, /* bitmask of the filter fields used */
    pub fields: allocinfo_tag,
    pub min_size: __u64,
    pub max_size: __u64,
    /* filter criteria only; see allocinfo_counter.accurate for actual accuracy */
    pub inaccurate: __u64,
}

#[repr(C)]
pub struct allocinfo_get_at {
    /* inputs */
    pub pos: __u64,
    pub filter: allocinfo_filter,
    /* output */
    pub data: allocinfo_tag_data,
}

pub const _ALLOCINFO_IOC_CONTENT_ID: u32 = 0;
pub const _ALLOCINFO_IOC_GET_AT: u32 = 1;
pub const _ALLOCINFO_IOC_GET_NEXT: u32 = 2;

pub const ALLOCINFO_IOC_BASE: u32 = 0xA6;

// These ioctl encodings depend on the platform-provided _IOR/_IOWR macros.
pub const ALLOCINFO_IOC_CONTENT_ID: usize =
    _IOR!(ALLOCINFO_IOC_BASE, _ALLOCINFO_IOC_CONTENT_ID, allocinfo_content_id);
pub const ALLOCINFO_IOC_GET_AT: usize =
    _IOWR!(ALLOCINFO_IOC_BASE, _ALLOCINFO_IOC_GET_AT, allocinfo_get_at);
pub const ALLOCINFO_IOC_GET_NEXT: usize =
    _IOR!(ALLOCINFO_IOC_BASE, _ALLOCINFO_IOC_GET_NEXT, allocinfo_tag_data);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
