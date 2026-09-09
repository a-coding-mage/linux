/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) 2005-2011 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

// C header guard: __MEMBER_DOT_H__

use core::ffi::c_int;

// Types defined by other headers.
#[repr(C)]
pub struct dlm_ls {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_recover {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_rcom {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_member {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_slot {
    _private: [u8; 0],
}

extern "C" {
    pub fn dlm_ls_stop(ls: *mut dlm_ls) -> c_int;
    pub fn dlm_ls_start(ls: *mut dlm_ls) -> c_int;
    pub fn dlm_clear_members(ls: *mut dlm_ls);
    pub fn dlm_clear_members_gone(ls: *mut dlm_ls);
    pub fn dlm_recover_members(
        ls: *mut dlm_ls,
        rv: *mut dlm_recover,
        neg_out: *mut c_int,
    ) -> c_int;
    pub fn dlm_is_removed(ls: *mut dlm_ls, nodeid: c_int) -> c_int;
    pub fn dlm_is_member(ls: *mut dlm_ls, nodeid: c_int) -> c_int;
    pub fn dlm_slots_version(h: *const dlm_header) -> c_int;
    pub fn dlm_slot_save(ls: *mut dlm_ls, rc: *mut dlm_rcom, memb: *mut dlm_member);
    pub fn dlm_slots_copy_out(ls: *mut dlm_ls, rc: *mut dlm_rcom);
    pub fn dlm_slots_copy_in(ls: *mut dlm_ls) -> c_int;
    pub fn dlm_slots_assign(
        ls: *mut dlm_ls,
        num_slots: *mut c_int,
        slots_size: *mut c_int,
        slots_out: *mut *mut dlm_slot,
        gen_out: *mut u32,
    ) -> c_int;
    pub fn dlm_lsop_recover_done(ls: *mut dlm_ls);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
