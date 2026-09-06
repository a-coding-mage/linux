// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor security identifier (secid) definitions
//
// Copyright 2009-2018 Canonical Ltd.

// Dependencies: linux/slab.h, linux/types.h

// Forward declarations for external types
pub struct aa_label;

pub struct lsm_context;

pub struct lsm_prop;

// External type alias
pub type gfp_t = u32;

// secid value that will not be allocated
pub const AA_SECID_INVALID: u32 = 0;

// secid value that matches any other secid
pub const AA_SECID_WILDCARD: u32 = 1;

// sysctl to enable displaying mode when converting secid to secctx
extern "C" {
    pub static mut apparmor_display_secid_mode: i32;

    pub fn aa_secid_to_label(secid: u32) -> *mut aa_label;
    pub fn apparmor_secid_to_secctx(secid: u32, cp: *mut lsm_context) -> i32;
    pub fn apparmor_lsmprop_to_secctx(prop: *mut lsm_prop, cp: *mut lsm_context) -> i32;
    pub fn apparmor_secctx_to_secid(secdata: *const u8, seclen: u32, secid: *mut u32) -> i32;
    pub fn apparmor_release_secctx(cp: *mut lsm_context);
    pub fn aa_alloc_secid(label: *mut aa_label, gfp: gfp_t) -> i32;
    pub fn aa_free_secid(secid: u32);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
