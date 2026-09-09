// SPDX-License-Identifier: GPL-2.0-only
/*
 * Accelerated CRC-T10DIF using arm64 NEON and Crypto Extensions instructions
 *
 * Copyright (C) 2016 - 2017 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.

// static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_asimd);
// static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_pmull);

pub const CRC_T10DIF_PMULL_CHUNK_SIZE: u32 = 16u32;

extern "C" {
    pub fn crc_t10dif_pmull_p8(
        init_crc: u16,
        buf: *const u8,
        len: usize,
        out: *mut u8,
    );
    pub fn crc_t10dif_pmull_p64(init_crc: u16, buf: *const u8, len: usize) -> u16;
    pub fn crc_t10dif_generic(crc: u16, data: *const u8, length: usize) -> u16;
}

#[inline]
pub unsafe fn crc_t10dif_arch(crc: u16, data: *const u8, length: usize) -> u16 {
    // The source condition depends on kernel SIMD and static-key facilities.
    if length >= CRC_T10DIF_PMULL_CHUNK_SIZE as usize && may_use_simd() {
        if static_branch_likely(&have_pmull) {
            return crc_t10dif_pmull_p64(crc, data, length);
        } else if length > CRC_T10DIF_PMULL_CHUNK_SIZE as usize
            && static_branch_likely(&have_asimd)
        {
            let mut buf = [0u8; 16];

            crc_t10dif_pmull_p8(crc, data, length, buf.as_mut_ptr());

            return crc_t10dif_generic(0, buf.as_ptr(), core::mem::size_of_val(&buf));
        }
    }
    crc_t10dif_generic(crc, data, length)
}

// #define crc_t10dif_mod_init_arch crc_t10dif_mod_init_arch
pub unsafe fn crc_t10dif_mod_init_arch() {
    if cpu_have_named_feature(ASIMD) {
        static_branch_enable(&have_asimd);
        if cpu_have_named_feature(PMULL) {
            static_branch_enable(&have_pmull);
        }
    }
}

// External kernel facilities referenced by this header.
extern "C" {
    static have_asimd: StaticKey;
    static have_pmull: StaticKey;
    fn may_use_simd() -> bool;
    fn static_branch_likely(key: *const StaticKey) -> bool;
    fn static_branch_enable(key: *const StaticKey);
    fn cpu_have_named_feature(feature: u32) -> bool;
}

#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

pub const ASIMD: u32 = 0;
pub const PMULL: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
