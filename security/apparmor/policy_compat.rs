// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor functions for unpacking policy loaded
 * from userspace.
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2022 Canonical Ltd.
 *
 * Code to provide backwards compatibility with older policy versions,
 * by converting/mapping older policy formats into the newer internal
 * formats.
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

// Dependencies: linux/ctype.h, linux/errno.h, include/lib.h, include/policy_unpack.h, include/policy_compat.h

use std::ffi::c_void;

pub type u32 = u32;
pub type u16 = u16;
pub type aa_state_t = u32;

#[repr(C)]
pub struct aa_dfa {
    pub tables: *mut *mut c_void,
}

#[repr(C)]
pub struct aa_perms {
    pub allow: u32,
    pub deny: u32,
    pub prompt: u32,
    pub audit: u32,
    pub quiet: u32,
    pub xindex: u32,
}

#[repr(C)]
pub struct aa_policydb {
    pub perms: *mut aa_perms,
    pub dfa: *mut aa_dfa,
    pub size: u32,
}

extern "C" {
    pub static AA_X_UNSAFE: u32;
    pub static AA_X_INHERIT: u32;
    pub static AA_X_UNCONFINED: u32;
    pub static AA_X_NAME: u32;
    pub static AA_X_CHILD: u32;
    pub static AA_X_TABLE: u32;
    pub static AA_MAY_GETATTR: u32;
    pub static AA_MAY_OPEN: u32;
    pub static AA_MAY_SETATTR: u32;
    pub static AA_MAY_CREATE: u32;
    pub static AA_MAY_DELETE: u32;
    pub static AA_MAY_CHMOD: u32;
    pub static AA_MAY_CHOWN: u32;
    pub static AA_MAY_LINK: u32;
    pub static AA_MAY_LOCK: u32;
    pub static AA_LINK_SUBSET: u32;
    pub static AA_EXEC_MMAP: u32;
    pub static AA_MAY_CHANGE_PROFILE: u32;
    pub static AA_MAY_ONEXEC: u32;
    pub static MAY_READ: u32;
    pub static MAY_WRITE: u32;
    pub static YYTD_ID_BASE: u32;
    pub static ACCEPT_FLAG_OWNER: u32;
    pub static v8: u32;

    fn ACCEPT_TABLE(dfa: *const aa_dfa) -> *const u32;
    fn ACCEPT_TABLE2(dfa: *const aa_dfa) -> *const u32;
    fn AA_BUG(cond: bool) -> ();
    fn AA_DEBUG(level: u32, msg: *const u8, ...) -> ();
    fn kvzalloc_objs(count: usize) -> *mut aa_perms;
    fn VERSION_LE(v: u32, cmp: u32) -> bool;
    fn VERSION_GT(v: u32, cmp: u32) -> bool;
}

const ENOMEM: i32 = -12;

fn dfa_map_xindex(mask: u16) -> u32 {
    let old_index = (mask >> 10) & 0xf;
    let mut index: u32 = 0;

    if mask & 0x100 != 0 {
        unsafe { index |= AA_X_UNSAFE; }
    }
    if mask & 0x200 != 0 {
        unsafe { index |= AA_X_INHERIT; }
    }
    if mask & 0x80 != 0 {
        unsafe { index |= AA_X_UNCONFINED; }
    }

    if old_index == 1 {
        unsafe { index |= AA_X_UNCONFINED; }
    } else if old_index == 2 {
        unsafe { index |= AA_X_NAME; }
    } else if old_index == 3 {
        unsafe { index |= AA_X_NAME | AA_X_CHILD; }
    } else if old_index != 0 {
        unsafe {
            index |= AA_X_TABLE;
            index |= old_index - 4;
        }
    }

    index
}

unsafe fn dfa_user_allow(dfa: *const aa_dfa, state: aa_state_t) -> u32 {
    let val = *ACCEPT_TABLE(dfa).add(state as usize);
    (val & 0x7f) | (val & 0x80000000)
}

unsafe fn dfa_user_xbits(dfa: *const aa_dfa, state: aa_state_t) -> u32 {
    ((*ACCEPT_TABLE(dfa).add(state as usize)) >> 7) & 0x7f
}

unsafe fn dfa_user_audit(dfa: *const aa_dfa, state: aa_state_t) -> u32 {
    (*ACCEPT_TABLE2(dfa).add(state as usize)) & 0x7f
}

unsafe fn dfa_user_quiet(dfa: *const aa_dfa, state: aa_state_t) -> u32 {
    ((*ACCEPT_TABLE2(dfa).add(state as usize)) >> 7) & 0x7f
}

unsafe fn dfa_user_xindex(dfa: *const aa_dfa, state: aa_state_t) -> u32 {
    dfa_map_xindex(((*ACCEPT_TABLE(dfa).add(state as usize)) & 0x3fff) as u16)
}

unsafe fn dfa_other_allow(dfa: *const aa_dfa, state: aa_state_t) -> u32 {
    let val = *ACCEPT_TABLE(dfa).add(state as usize);
    (((val) >> 14) & 0x7f) | (val & 0x80000000)
}

unsafe fn dfa_other_xbits(dfa: *const aa_dfa, state: aa_state_t) -> u32 {
    (((*ACCEPT_TABLE(dfa).add(state as usize)) >> 7) >> 14) & 0x7f
}

unsafe fn dfa_other_audit(dfa: *const aa_dfa, state: aa_state_t) -> u32 {
    ((*ACCEPT_TABLE2(dfa).add(state as usize)) >> 14) & 0x7f
}

unsafe fn dfa_other_quiet(dfa: *const aa_dfa, state: aa_state_t) -> u32 {
    (((*ACCEPT_TABLE2(dfa).add(state as usize)) >> 7) >> 14) & 0x7f
}

unsafe fn dfa_other_xindex(dfa: *const aa_dfa, state: aa_state_t) -> u32 {
    dfa_map_xindex(((*ACCEPT_TABLE(dfa).add(state as usize)) >> 14) & 0x3fff as u16)
}

unsafe fn map_old_perms(old: u32) -> u32 {
    let mut new = old & 0xf;

    if old & MAY_READ != 0 {
        new |= AA_MAY_GETATTR | AA_MAY_OPEN;
    }
    if old & MAY_WRITE != 0 {
        new |= AA_MAY_SETATTR | AA_MAY_CREATE | AA_MAY_DELETE |
               AA_MAY_CHMOD | AA_MAY_CHOWN | AA_MAY_OPEN;
    }
    if old & 0x10 != 0 {
        new |= AA_MAY_LINK;
    }
    if old & 0x20 != 0 {
        new |= AA_MAY_LOCK | AA_LINK_SUBSET;
    }
    if old & 0x40 != 0 {
        new |= AA_EXEC_MMAP;
    }

    new
}

unsafe fn compute_fperms_allow(perms: *mut aa_perms, dfa: *const aa_dfa, state: aa_state_t) {
    (*perms).allow |= AA_MAY_GETATTR;

    if *ACCEPT_TABLE(dfa).add(state as usize) & 0x80000000 != 0 {
        (*perms).allow |= AA_MAY_CHANGE_PROFILE;
    }
    if *ACCEPT_TABLE(dfa).add(state as usize) & 0x40000000 != 0 {
        (*perms).allow |= AA_MAY_ONEXEC;
    }
}

unsafe fn compute_fperms_user(dfa: *const aa_dfa, state: aa_state_t) -> aa_perms {
    let mut perms: aa_perms = std::mem::zeroed();

    perms.allow = map_old_perms(dfa_user_allow(dfa, state));
    perms.audit = map_old_perms(dfa_user_audit(dfa, state));
    perms.quiet = map_old_perms(dfa_user_quiet(dfa, state));
    perms.xindex = dfa_user_xindex(dfa, state);

    compute_fperms_allow(&mut perms, dfa, state);

    perms
}

unsafe fn compute_fperms_other(dfa: *const aa_dfa, state: aa_state_t) -> aa_perms {
    let mut perms: aa_perms = std::mem::zeroed();

    perms.allow = map_old_perms(dfa_other_allow(dfa, state));
    perms.audit = map_old_perms(dfa_other_audit(dfa, state));
    perms.quiet = map_old_perms(dfa_other_quiet(dfa, state));
    perms.xindex = dfa_other_xindex(dfa, state);

    compute_fperms_allow(&mut perms, dfa, state);

    perms
}

unsafe fn compute_fperms(dfa: *const aa_dfa, size: *mut u32) -> *mut aa_perms {
    AA_BUG(dfa.is_null());

    let state_count = *(*(*dfa).tables.add(YYTD_ID_BASE as usize) as *const u32) as usize;
    let table = kvzalloc_objs(state_count * 2);
    if table.is_null() {
        return std::ptr::null_mut();
    }
    *size = (state_count * 2) as u32;

    for state in 0..state_count {
        *table.add(state * 2) = compute_fperms_user(dfa, state as aa_state_t);
        *table.add(state * 2 + 1) = compute_fperms_other(dfa, state as aa_state_t);
    }

    table
}

unsafe fn compute_xmatch_perms(xmatch: *const aa_dfa, size: *mut u32) -> *mut aa_perms {
    AA_BUG(xmatch.is_null());

    let state_count = *(*(*xmatch).tables.add(YYTD_ID_BASE as usize) as *const u32) as usize;
    let perms = kvzalloc_objs(state_count);
    if perms.is_null() {
        return std::ptr::null_mut();
    }
    *size = state_count as u32;

    for state in 1..state_count {
        (*perms.add(state)).allow = dfa_user_allow(xmatch, state as aa_state_t);
    }

    perms
}

fn map_other(x: u32) -> u32 {
    ((x & 0x3) << 8) |
    ((x & 0x1c) << 18) |
    ((x & 0x60) << 19)
}

fn map_xbits(x: u32) -> u32 {
    ((x & 0x1) << 7) |
    ((x & 0x7e) << 9)
}

unsafe fn compute_perms_entry(dfa: *const aa_dfa, state: aa_state_t, version: u32) -> aa_perms {
    let mut perms: aa_perms = std::mem::zeroed();

    perms.allow = dfa_user_allow(dfa, state);
    perms.audit = dfa_user_audit(dfa, state);
    perms.quiet = dfa_user_quiet(dfa, state);

    perms.allow |= map_other(dfa_other_allow(dfa, state));
    if VERSION_LE(version, v8) {
        perms.allow |= AA_MAY_LOCK;
    } else {
        perms.allow |= map_xbits(dfa_user_xbits(dfa, state));
    }

    perms.audit |= map_other(dfa_other_audit(dfa, state));
    perms.quiet |= map_other(dfa_other_quiet(dfa, state));
    if VERSION_GT(version, v8) {
        perms.quiet |= map_xbits(dfa_other_xbits(dfa, state));
    }

    perms
}

unsafe fn compute_perms(dfa: *const aa_dfa, version: u32, size: *mut u32) -> *mut aa_perms {
    AA_BUG(dfa.is_null());

    let state_count = *(*(*dfa).tables.add(YYTD_ID_BASE as usize) as *const u32) as usize;
    let table = kvzalloc_objs(state_count);
    if table.is_null() {
        return std::ptr::null_mut();
    }
    *size = state_count as u32;

    for state in 1..state_count {
        *table.add(state) = compute_perms_entry(dfa, state as aa_state_t, version);
        // AA_DEBUG call with format string - external logging dependency
    }
    table
}

unsafe fn remap_dfa_accept(dfa: *mut aa_dfa, factor: u32) {
    AA_BUG(dfa.is_null());

    let state_count = *(*(*dfa).tables.add(YYTD_ID_BASE as usize) as *const u32) as usize;

    for state in 0..state_count {
        let accept_table = ACCEPT_TABLE(dfa as *const aa_dfa) as *mut u32;
        *accept_table.add(state) = (state as u32) * factor;
        let accept_table2 = ACCEPT_TABLE2(dfa as *const aa_dfa) as *mut u32;
        *accept_table2.add(state) = if factor > 1 { ACCEPT_FLAG_OWNER } else { 0 };
    }
}

pub extern "C" fn aa_compat_map_xmatch(policy: *mut aa_policydb) -> i32 {
    unsafe {
        (*policy).perms = compute_xmatch_perms((*policy).dfa, &mut (*policy).size);
        if (*policy).perms.is_null() {
            return ENOMEM;
        }

        remap_dfa_accept((*policy).dfa, 1);
    }

    0
}

pub extern "C" fn aa_compat_map_policy(policy: *mut aa_policydb, version: u32) -> i32 {
    unsafe {
        (*policy).perms = compute_perms((*policy).dfa, version, &mut (*policy).size);
        if (*policy).perms.is_null() {
            return ENOMEM;
        }

        remap_dfa_accept((*policy).dfa, 1);
    }

    0
}

pub extern "C" fn aa_compat_map_file(policy: *mut aa_policydb) -> i32 {
    unsafe {
        (*policy).perms = compute_fperms((*policy).dfa, &mut (*policy).size);
        if (*policy).perms.is_null() {
            return ENOMEM;
        }

        remap_dfa_accept((*policy).dfa, 2);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
