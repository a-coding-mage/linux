/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * GHASH and POLYVAL, arm64 optimized
 *
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::{ghash_key, polyval_elem, polyval_key};

pub const NUM_H_POWERS: usize = 8;

static mut have_asimd: crate::StaticKey = crate::StaticKey::new_false();
static mut have_pmull: crate::StaticKey = crate::StaticKey::new_false();

extern "C" {
    fn pmull_ghash_update_p8(
        blocks: usize,
        dg: *mut polyval_elem,
        src: *const u8,
        h: *const polyval_elem,
    );
    fn polyval_mul_pmull(a: *mut polyval_elem, b: *const polyval_elem);
    fn polyval_blocks_pmull(
        acc: *mut polyval_elem,
        key: *const polyval_key,
        data: *const u8,
        nblocks: usize,
    );
    fn polyval_mul_generic(a: *mut polyval_elem, b: *const polyval_elem);
    fn ghash_blocks_generic(
        acc: *mut polyval_elem,
        h: *const polyval_elem,
        data: *const u8,
        nblocks: usize,
    );
    fn polyval_blocks_generic(
        acc: *mut polyval_elem,
        h: *const polyval_elem,
        data: *const u8,
        nblocks: usize,
    );
    fn may_use_simd() -> bool;
    fn static_branch_likely(key: *const crate::StaticKey) -> bool;
    fn static_branch_enable(key: *mut crate::StaticKey);
    fn cpu_have_named_feature(feature: u32) -> bool;
}

pub const ASIMD: u32 = crate::ASIMD;
pub const PMULL: u32 = crate::PMULL;

pub unsafe fn polyval_preparekey_arch(key: *mut polyval_key, raw_key: *const u8) {
    // static_assert(ARRAY_SIZE(key->h_powers) == NUM_H_POWERS);
    core::ptr::copy_nonoverlapping(
        raw_key,
        (*key).h_powers.as_mut_ptr().add(NUM_H_POWERS - 1) as *mut u8,
        crate::POLYVAL_BLOCK_SIZE,
    );
    if static_branch_likely(&have_pmull) && may_use_simd() {
        for i in (0..=(NUM_H_POWERS - 2)).rev() {
            (*key).h_powers[i] = (*key).h_powers[i + 1];
            polyval_mul_pmull(
                &mut (*key).h_powers[i],
                &(*key).h_powers[NUM_H_POWERS - 1],
            );
        }
    } else {
        for i in (0..=(NUM_H_POWERS - 2)).rev() {
            (*key).h_powers[i] = (*key).h_powers[i + 1];
            polyval_mul_generic(
                &mut (*key).h_powers[i],
                &(*key).h_powers[NUM_H_POWERS - 1],
            );
        }
    }
}

unsafe fn polyval_mul_arm64(a: *mut polyval_elem, b: *const polyval_elem) {
    if static_branch_likely(&have_asimd) && may_use_simd() {
        static const ZEROES: [u8; crate::GHASH_BLOCK_SIZE] = [0; crate::GHASH_BLOCK_SIZE];
        if static_branch_likely(&have_pmull) {
            polyval_mul_pmull(a, b);
        } else {
            /*
             * Note that this is indeed equivalent to a
             * POLYVAL multiplication, since it takes the
             * accumulator and key in POLYVAL format, and
             * byte-swapping a block of zeroes is a no-op.
             */
            pmull_ghash_update_p8(1, a, ZEROES.as_ptr(), b);
        }
    } else {
        polyval_mul_generic(a, b);
    }
}

pub unsafe fn ghash_mul_arch(acc: *mut polyval_elem, key: *const ghash_key) {
    polyval_mul_arm64(acc, &(*key).h);
}

pub unsafe fn polyval_mul_arch(acc: *mut polyval_elem, key: *const polyval_key) {
    polyval_mul_arm64(acc, &(*key).h_powers[NUM_H_POWERS - 1]);
}

pub unsafe fn ghash_blocks_arch(
    acc: *mut polyval_elem,
    key: *const ghash_key,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(&have_asimd) && may_use_simd() {
        pmull_ghash_update_p8(nblocks, acc, data, &(*key).h);
    } else {
        ghash_blocks_generic(acc, &(*key).h, data, nblocks);
    }
}

pub unsafe fn polyval_blocks_arch(
    acc: *mut polyval_elem,
    key: *const polyval_key,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(&have_pmull) && may_use_simd() {
        polyval_blocks_pmull(acc, key, data, nblocks);
    } else {
        polyval_blocks_generic(acc, &(*key).h_powers[NUM_H_POWERS - 1], data, nblocks);
    }
}

pub unsafe fn gf128hash_mod_init_arch() {
    if cpu_have_named_feature(ASIMD) {
        static_branch_enable(&mut have_asimd);
        if cpu_have_named_feature(PMULL) {
            static_branch_enable(&mut have_pmull);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
