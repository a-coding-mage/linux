/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * localalloc.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// Declarations supplied by other translation units.
#[repr(C)]
pub enum ocfs2_super {}

#[repr(C)]
pub enum ocfs2_dinode {}

#[repr(C)]
pub enum ocfs2_alloc_context {}

#[repr(C)]
pub enum handle_t {}

#[repr(C)]
pub enum work_struct {}

extern "C" {
    pub fn ocfs2_load_local_alloc(osb: *mut ocfs2_super) -> ::std::os::raw::c_int;

    pub fn ocfs2_shutdown_local_alloc(osb: *mut ocfs2_super);

    pub fn ocfs2_la_set_sizes(osb: *mut ocfs2_super, requested_mb: ::std::os::raw::c_int);
    pub fn ocfs2_la_default_mb(osb: *mut ocfs2_super) -> ::std::os::raw::c_uint;

    pub fn ocfs2_begin_local_alloc_recovery(
        osb: *mut ocfs2_super,
        node_num: ::std::os::raw::c_int,
        alloc_copy: *mut *mut ocfs2_dinode,
    ) -> ::std::os::raw::c_int;

    pub fn ocfs2_complete_local_alloc_recovery(
        osb: *mut ocfs2_super,
        alloc: *mut ocfs2_dinode,
    ) -> ::std::os::raw::c_int;

    pub fn ocfs2_alloc_should_use_local(
        osb: *mut ocfs2_super,
        bits: u64,
    ) -> ::std::os::raw::c_int;

    pub fn ocfs2_reserve_local_alloc_bits(
        osb: *mut ocfs2_super,
        bits_wanted: u32,
        ac: *mut ocfs2_alloc_context,
    ) -> ::std::os::raw::c_int;

    pub fn ocfs2_claim_local_alloc_bits(
        osb: *mut ocfs2_super,
        handle: *mut handle_t,
        ac: *mut ocfs2_alloc_context,
        bits_wanted: u32,
        bit_off: *mut u32,
        num_bits: *mut u32,
    ) -> ::std::os::raw::c_int;

    pub fn ocfs2_free_local_alloc_bits(
        osb: *mut ocfs2_super,
        handle: *mut handle_t,
        ac: *mut ocfs2_alloc_context,
        bit_off: u32,
        num_bits: u32,
    ) -> ::std::os::raw::c_int;

    pub fn ocfs2_local_alloc_seen_free_bits(
        osb: *mut ocfs2_super,
        num_clusters: ::std::os::raw::c_uint,
    );
    pub fn ocfs2_la_enable_worker(work: *mut work_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
