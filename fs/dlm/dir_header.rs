/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
 *******************************************************************************
 **
 **  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
 **  Copyright (C) 2004-2005 Red Hat, Inc.  All rights reserved.
 **
 **
 *******************************************************************************
 ******************************************************************************/

// The following types are supplied by the corresponding external dependencies.

extern "C" {
    pub fn dlm_dir_nodeid(rsb: *mut dlm_rsb) -> ::std::os::raw::c_int;
    pub fn dlm_hash2nodeid(ls: *mut dlm_ls, hash: u32) -> ::std::os::raw::c_int;
    pub fn dlm_recover_dir_nodeid(ls: *mut dlm_ls, root_list: *const list_head);
    pub fn dlm_recover_directory(ls: *mut dlm_ls, seq: u64) -> ::std::os::raw::c_int;
    pub fn dlm_copy_master_names(
        ls: *mut dlm_ls,
        inbuf: *const ::std::os::raw::c_char,
        inlen: ::std::os::raw::c_int,
        outbuf: *mut ::std::os::raw::c_char,
        outlen: ::std::os::raw::c_int,
        nodeid: ::std::os::raw::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
