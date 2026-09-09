/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * heartbeat.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// C header guard: OCFS2_HEARTBEAT_H

use core::ffi::c_void;

// Definitions supplied by the corresponding OCFS2 dependencies.
#[repr(C)]
pub struct ocfs2_super {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ocfs2_node_map {
    _private: [u8; 0],
}

extern "C" {
    pub fn ocfs2_init_node_maps(osb: *mut ocfs2_super);

    pub fn ocfs2_do_node_down(node_num: ::core::ffi::c_int, data: *mut c_void);

    /* node map functions - used to keep track of mounted and in-recovery
     * nodes. */
    pub fn ocfs2_node_map_set_bit(
        osb: *mut ocfs2_super,
        map: *mut ocfs2_node_map,
        bit: ::core::ffi::c_int,
    );
    pub fn ocfs2_node_map_clear_bit(
        osb: *mut ocfs2_super,
        map: *mut ocfs2_node_map,
        bit: ::core::ffi::c_int,
    );
    pub fn ocfs2_node_map_test_bit(
        osb: *mut ocfs2_super,
        map: *mut ocfs2_node_map,
        bit: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
