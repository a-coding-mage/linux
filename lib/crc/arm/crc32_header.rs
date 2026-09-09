// SPDX-License-Identifier: GPL-2.0-only
/*
 * Accelerated CRC32(C) using ARM CRC, NEON and Crypto Extensions instructions
 *
 * Copyright (C) 2016 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// The following names are supplied by the surrounding kernel translation.
use core::ffi::c_void;

const PMULL_MIN_LEN: usize = 64; /* min size of buffer for pmull functions */

// C: static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_crc32);
// C: static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_pmull);
extern "C" {
    static have_crc32: c_void;
    static have_pmull: c_void;

    fn crc32_pmull_le(buf: *const u8, len: u32, init_crc: u32) -> u32;
    fn crc32_armv8_le(init_crc: u32, buf: *const u8, len: u32) -> u32;
    fn crc32c_pmull_le(buf: *const u8, len: u32, init_crc: u32) -> u32;
    fn crc32c_armv8_le(init_crc: u32, buf: *const u8, len: u32) -> u32;

    fn static_branch_likely(key: *const c_void) -> bool;
    fn may_use_simd() -> bool;
    fn crc32_le_base(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32c_base(crc: u32, p: *const u8, len: usize) -> u32;
    fn static_branch_enable(key: *const c_void);
    static elf_hwcap2: usize;
}

#[inline]
unsafe fn crc32_le_scalar(mut crc: u32, p: *const u8, len: usize) -> u32 {
    if static_branch_likely(&have_crc32) {
        crc = crc32_armv8_le(crc, p, len as u32);
    } else {
        crc = crc32_le_base(crc, p, len);
    }
    crc
}

#[inline]
unsafe fn crc32_le_arch(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    if len >= PMULL_MIN_LEN + 15
        && static_branch_likely(&have_pmull)
        && may_use_simd()
    {
        let mut n = (!(p as usize)).wrapping_add(1) & 15;

        /* align p to 16-byte boundary */
        if n != 0 {
            crc = crc32_le_scalar(crc, p, n);
            p = p.add(n);
            len -= n;
        }
        n = len & !15;
        // scoped_ksimd()
        crc = crc32_pmull_le(p, n as u32, crc);
        p = p.add(n);
        len -= n;
    }
    crc32_le_scalar(crc, p, len)
}

#[inline]
unsafe fn crc32c_scalar(mut crc: u32, p: *const u8, len: usize) -> u32 {
    if static_branch_likely(&have_crc32) {
        crc = crc32c_armv8_le(crc, p, len as u32);
    } else {
        crc = crc32c_base(crc, p, len);
    }
    crc
}

#[inline]
unsafe fn crc32c_arch(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    if len >= PMULL_MIN_LEN + 15
        && static_branch_likely(&have_pmull)
        && may_use_simd()
    {
        let mut n = (!(p as usize)).wrapping_add(1) & 15;

        /* align p to 16-byte boundary */
        if n != 0 {
            crc = crc32c_scalar(crc, p, n);
            p = p.add(n);
            len -= n;
        }
        n = len & !15;
        // scoped_ksimd()
        crc = crc32c_pmull_le(p, n as u32, crc);
        p = p.add(n);
        len -= n;
    }
    crc32c_scalar(crc, p, len)
}

// #define crc32_be_arch crc32_be_base /* not implemented on this arch */
// #define crc32_mod_init_arch crc32_mod_init_arch
unsafe fn crc32_mod_init_arch() {
    const HWCAP2_CRC32: usize = 1 << 7;
    const HWCAP2_PMULL: usize = 1 << 1;
    if elf_hwcap2 & HWCAP2_CRC32 != 0 {
        static_branch_enable(&have_crc32);
    }
    if elf_hwcap2 & HWCAP2_PMULL != 0 {
        static_branch_enable(&have_pmull);
    }
}

#[inline]
unsafe fn crc32_optimizations_arch() -> u32 {
    const HWCAP2_CRC32: usize = 1 << 7;
    const HWCAP2_PMULL: usize = 1 << 1;
    const CRC32_LE_OPTIMIZATION: u32 = 1;
    const CRC32C_OPTIMIZATION: u32 = 2;
    if elf_hwcap2 & (HWCAP2_CRC32 | HWCAP2_PMULL) != 0 {
        return CRC32_LE_OPTIMIZATION | CRC32C_OPTIMIZATION;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
