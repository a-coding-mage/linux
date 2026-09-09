// SPDX-License-Identifier: GPL-2.0-only
/*
 * Accelerated CRC-T10DIF using ARM NEON and Crypto Extensions instructions
 *
 * Copyright (C) 2016 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// C dependency: <asm/simd.h>

// C: static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_neon);
// C: static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_pmull);
static mut have_neon: bool = false;
static mut have_pmull: bool = false;

pub const CRC_T10DIF_PMULL_CHUNK_SIZE: usize = 16u32 as usize;

extern "C" {
    pub fn crc_t10dif_pmull64(init_crc: u16, buf: *const u8, len: usize) -> u16;
    pub fn crc_t10dif_pmull8(init_crc: u16, buf: *const u8, len: usize, out: *mut u8);
    pub fn crc_t10dif_generic(crc: u16, data: *const u8, length: usize) -> u16;
    pub fn may_use_simd() -> bool;
    pub static elf_hwcap: usize;
    pub static elf_hwcap2: usize;
    pub const HWCAP_NEON: usize;
    pub const HWCAP2_PMULL: usize;
}

#[inline]
pub unsafe fn crc_t10dif_arch(crc: u16, data: *const u8, length: usize) -> u16 {
    if length >= CRC_T10DIF_PMULL_CHUNK_SIZE && may_use_simd() {
        if have_pmull {
            // C scoped_ksimd() establishes the SIMD context for this call.
            return crc_t10dif_pmull64(crc, data, length);
        } else if length > CRC_T10DIF_PMULL_CHUNK_SIZE && have_neon {
            let mut buf = [0u8; 16];

            // C scoped_ksimd() establishes the SIMD context for this call.
            crc_t10dif_pmull8(crc, data, length, buf.as_mut_ptr());

            return crc_t10dif_generic(0, buf.as_ptr(), core::mem::size_of_val(&buf));
        }
    }
    crc_t10dif_generic(crc, data, length)
}

// C: #define crc_t10dif_mod_init_arch crc_t10dif_mod_init_arch
#[inline]
pub unsafe fn crc_t10dif_mod_init_arch() {
    if (elf_hwcap & HWCAP_NEON) != 0 {
        have_neon = true;
        if (elf_hwcap2 & HWCAP2_PMULL) != 0 {
            have_pmull = true;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
