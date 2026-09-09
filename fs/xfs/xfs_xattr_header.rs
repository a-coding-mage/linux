// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// The following types are supplied by the surrounding translation unit.
pub enum xfs_attr_update {}
pub enum xfs_da_args {}
pub enum xattr_handler {}

extern "C" {
    pub fn xfs_attr_change(args: *mut xfs_da_args, op: xfs_attr_update) -> ::core::ffi::c_int;

    pub static xfs_xattr_handlers: [*const xattr_handler; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
