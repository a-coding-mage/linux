/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/include/linux/timeriomem-rng.h
 *
 * Copyright (c) 2009 Alexander Clouter <alex@digriz.org.uk>
 */

// Translated from the C header. The original include guard is omitted.

#[repr(C)]
pub struct timeriomem_rng_data {
    pub address: *mut core::ffi::c_void,

    /* measures in usecs */
    pub period: core::ffi::c_uint,

    /* bits of entropy per 1024 bits read */
    pub quality: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
