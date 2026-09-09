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

// Opaque types declared by the surrounding C interface.
#[repr(C)]
pub struct dlm_ls {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct dlm_rcom {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct dlm_rsb {
    _opaque: [u8; 0],
}

unsafe extern "C" {
    pub fn dlm_wait_function(
        ls: *mut dlm_ls,
        testfn: Option<unsafe extern "C" fn(ls: *mut dlm_ls) -> ::std::os::raw::c_int>,
    ) -> ::std::os::raw::c_int;
    pub fn dlm_recover_status(ls: *mut dlm_ls) -> u32;
    pub fn dlm_set_recover_status(ls: *mut dlm_ls, status: u32);
    pub fn dlm_recover_members_wait(ls: *mut dlm_ls, seq: u64) -> ::std::os::raw::c_int;
    pub fn dlm_recover_directory_wait(ls: *mut dlm_ls, seq: u64) -> ::std::os::raw::c_int;
    pub fn dlm_recover_locks_wait(ls: *mut dlm_ls, seq: u64) -> ::std::os::raw::c_int;
    pub fn dlm_recover_done_wait(ls: *mut dlm_ls, seq: u64) -> ::std::os::raw::c_int;
    pub fn dlm_recover_masters(
        ls: *mut dlm_ls,
        seq: u64,
        root_list: *const list_head,
    ) -> ::std::os::raw::c_int;
    pub fn dlm_recover_master_reply(
        ls: *mut dlm_ls,
        rc: *const dlm_rcom,
    ) -> ::std::os::raw::c_int;
    pub fn dlm_recover_locks(
        ls: *mut dlm_ls,
        seq: u64,
        root_list: *const list_head,
    ) -> ::std::os::raw::c_int;
    pub fn dlm_recovered_lock(r: *mut dlm_rsb);
    pub fn dlm_clear_inactive(ls: *mut dlm_ls);
    pub fn dlm_recover_rsbs(ls: *mut dlm_ls, root_list: *const list_head);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
