// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor policy loading interface function definitions.
 *
 * Copyright 2013 Canonical Ltd.
 *
 * Fns to provide a checksum of policy that has been loaded this can be
 * compared to userspace policy compiles to check loaded policy is what
 * it should be.
 */

// #include <crypto/sha2.h>
// #include "include/apparmor.h"
// #include "include/crypto.h"

use std::os::raw::{c_void, c_int};

extern "C" {
    static SHA256_DIGEST_SIZE: usize;
    static aa_g_hash_policy: c_int;
    static apparmor_initialized: c_int;

    fn sha256(data: *const c_void, len: usize, hash: *mut u8);
    fn sha256_init(sctx: *mut sha256_ctx);
    fn sha256_update(sctx: *mut sha256_ctx, data: *const u8, len: usize);
    fn sha256_final(sctx: *mut sha256_ctx, hash: *mut u8);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn aa_info_message(msg: *const u8);
}

#[repr(C)]
pub struct sha256_ctx;

#[repr(C)]
pub struct aa_profile {
    pub hash: *mut u8,
}

// GFP_KERNEL is a macro from <linux/gfp.h>
const GFP_KERNEL: u32 = 0xd0810;

const ENOMEM: i32 = 12;

pub fn aa_hash_size() -> u32 {
    unsafe { SHA256_DIGEST_SIZE as u32 }
}

pub fn aa_calc_hash(data: *mut c_void, len: usize) -> *mut u8 {
    unsafe {
        let hash = kzalloc(SHA256_DIGEST_SIZE, GFP_KERNEL) as *mut u8;
        if hash.is_null() {
            return ((-ENOMEM) as isize as *mut u8);
        }
        sha256(data, len, hash);
        hash
    }
}

pub fn aa_calc_profile_hash(
    profile: *mut aa_profile,
    version: u32,
    start: *mut c_void,
    len: usize,
) -> i32 {
    unsafe {
        if aa_g_hash_policy == 0 {
            return 0;
        }

        let hash = kzalloc(SHA256_DIGEST_SIZE, GFP_KERNEL) as *mut u8;
        if hash.is_null() {
            return -ENOMEM;
        }

        (*profile).hash = hash;

        let mut sctx = std::mem::zeroed::<sha256_ctx>();
        let le32_version = version.to_le();

        sha256_init(&mut sctx);
        sha256_update(&mut sctx, &le32_version as *const u32 as *const u8, 4);
        sha256_update(&mut sctx, start as *const u8, len);
        sha256_final(&mut sctx, (*profile).hash);
        0
    }
}

pub fn init_profile_hash() -> i32 {
    unsafe {
        if apparmor_initialized != 0 {
            aa_info_message(b"AppArmor sha256 policy hashing enabled\0".as_ptr());
        }
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
