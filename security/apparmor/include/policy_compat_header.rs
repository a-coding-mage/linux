// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// Code to provide backwards compatibility with older policy versions,
// by converting/mapping older policy formats into the newer internal
// formats.
//
// Copyright 2022 Canonical Ltd.

// Requires: "policy.h" - defines aa_policydb and u32

pub const K_ABI_MASK: u32 = 0x3ff;
pub const FORCE_COMPLAIN_FLAG: u32 = 0x800;

pub const fn VERSION_LT(x: u32, y: u32) -> bool {
    (x & K_ABI_MASK) < (y & K_ABI_MASK)
}

pub const fn VERSION_LE(x: u32, y: u32) -> bool {
    (x & K_ABI_MASK) <= (y & K_ABI_MASK)
}

pub const fn VERSION_GT(x: u32, y: u32) -> bool {
    (x & K_ABI_MASK) > (y & K_ABI_MASK)
}

pub const v5: u32 = 5; // base version
pub const v6: u32 = 6; // per entry policydb mediation check
pub const v7: u32 = 7;
pub const v8: u32 = 8; // full network masking
pub const v9: u32 = 9; // xbits are used as permission bits in policydb

extern "C" {
    pub fn aa_compat_map_xmatch(policy: *mut aa_policydb) -> i32;
    pub fn aa_compat_map_policy(policy: *mut aa_policydb, version: u32) -> i32;
    pub fn aa_compat_map_file(policy: *mut aa_policydb) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
