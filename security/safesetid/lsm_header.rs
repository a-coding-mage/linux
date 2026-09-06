// SPDX-License-Identifier: GPL-2.0
/*
 * SafeSetID Linux Security Module
 *
 * Author: Micah Morton <mortonm@chromium.org>
 *
 * Copyright (C) 2018 The Chromium OS Authors.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2, as
 * published by the Free Software Foundation.
 *
 */

// C dependencies:
// #include <linux/types.h>
// #include <linux/uidgid.h>
// #include <linux/hashtable.h>

// Flag indicating whether initialization completed.
// C declaration used __initdata storage annotation.
unsafe extern "C" {
    pub static mut safesetid_initialized: core::ffi::c_int;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum sid_policy_type {
    SIDPOL_DEFAULT,     // source ID is unaffected by policy
    SIDPOL_CONSTRAINED, // source ID is affected by policy
    SIDPOL_ALLOWED,     // target ID explicitly allowed
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kid_t {
    pub uid: kuid_t,
    pub gid: kgid_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum setid_type {
    UID,
    GID,
}

/*
 * Hash table entry to store safesetid policy signifying that 'src_id'
 * can set*id to 'dst_id'.
 */
#[repr(C)]
pub struct setid_rule {
    pub next: hlist_node,
    pub src_id: kid_t,
    pub dst_id: kid_t,

    // Flag to signal if rule is for UID's or GID's.
    pub type_: setid_type,
}

pub const SETID_HASH_BITS: usize = 8; // 256 buckets in hash table

// Extension of INVALID_UID/INVALID_GID for kid_t type.
pub const INVALID_ID: kid_t = kid_t { uid: INVALID_UID };

#[repr(C)]
pub struct setid_ruleset {
    // DECLARE_HASHTABLE(rules, SETID_HASH_BITS);
    pub rules: [hlist_head; 1usize << SETID_HASH_BITS],
    pub policy_str: *mut core::ffi::c_char,
    pub rcu: rcu_head,

    // Flag to signal if ruleset is for UID's or GID's.
    pub type_: setid_type,
}

unsafe extern "C" {
    pub fn _setid_policy_lookup(
        policy: *mut setid_ruleset,
        src: kid_t,
        dst: kid_t,
    ) -> sid_policy_type;
}

// C declarations used __rcu pointer annotation.
unsafe extern "C" {
    pub static mut safesetid_setuid_rules: *mut setid_ruleset;
    pub static mut safesetid_setgid_rules: *mut setid_ruleset;

    pub fn safesetid_init_securityfs() -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
