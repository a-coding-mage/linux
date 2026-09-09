/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * slotmap.h
 *
 * description here
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

use std::os::raw::{c_int, c_uint};

// Forward declaration supplied by the corresponding dependency.
#[repr(C)]
pub struct ocfs2_super;

extern "C" {
    pub fn ocfs2_init_slot_info(osb: *mut ocfs2_super) -> c_int;
    pub fn ocfs2_free_slot_info(osb: *mut ocfs2_super);

    pub fn ocfs2_find_slot(osb: *mut ocfs2_super) -> c_int;
    pub fn ocfs2_put_slot(osb: *mut ocfs2_super);

    pub fn ocfs2_refresh_slot_info(osb: *mut ocfs2_super) -> c_int;

    pub fn ocfs2_node_num_to_slot(osb: *mut ocfs2_super, node_num: c_uint) -> c_int;
    pub fn ocfs2_slot_to_node_num_locked(
        osb: *mut ocfs2_super,
        slot_num: c_int,
        node_num: *mut c_uint,
    ) -> c_int;

    pub fn ocfs2_clear_slot(osb: *mut ocfs2_super, slot_num: c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
