// SPDX-License-Identifier: GPL-2.0-only
/*
 * x86-optimized CRC32 functions
 *
 * Copyright (C) 2008 Intel Corporation
 * Copyright 2012 Xyratex Technology Limited
 * Copyright 2024 Google LLC
 */

// Dependency: crc-pclmul-template.h

extern "C" {
    static mut have_crc32: StaticKey;
    static mut have_pclmulqdq: StaticKey;
    static mut have_vpclmul_avx512: StaticKey;

    fn crc32_lsb_pclmul(crc: u32, p: *const u8, len: usize, constants: *const u8) -> u32;
    fn crc32_lsb_vpclmul_avx512(crc: u32, p: *const u8, len: usize, constants: *const u8) -> u32;
    fn crc32_lsb_vpclmul_avx2(crc: u32, p: *const u8, len: usize, constants: *const u8) -> u32;
    fn crc32c_x86_3way(crc: u32, buffer: *const u8, len: usize) -> u32;
    fn crc32_le_base(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32c_base(crc: u32, p: *const u8, len: usize) -> u32;
}

// Declaration supplied by the CRC PCLMUL template.
extern "C" {
    static crc32_lsb_0xedb88320_consts: u8;
    static crc32_lsb_0x82f63b78_consts_fold_across_128_bits_consts: u8;
}

const CRC32C_PCLMUL_BREAKEVEN: usize = 512;

#[inline]
unsafe fn crc32_le_arch(mut crc: u32, p: *const u8, len: usize) -> u32 {
    // CRC_PCLMUL(crc, p, len, crc32_lsb, crc32_lsb_0xedb88320_consts,
    //            have_pclmulqdq);
    crc = crc32_le_base(crc, p, len);
    crc
}

#[inline]
unsafe fn crc32c_arch(mut crc: u32, mut p: *const u8, len: usize) -> u32 {
    let mut num_longs: usize;

    if !static_branch_likely(&have_crc32) {
        return crc32c_base(crc, p, len);
    }

    if cfg!(target_arch = "x86_64") && len >= CRC32C_PCLMUL_BREAKEVEN
        && static_branch_likely(&have_pclmulqdq) && irq_fpu_usable()
    {
        kernel_fpu_begin();
        if static_branch_likely(&have_vpclmul_avx512) {
            crc = crc32_lsb_vpclmul_avx512(
                crc, p, len, &crc32_lsb_0x82f63b78_consts_fold_across_128_bits_consts,
            );
        } else {
            crc = crc32c_x86_3way(crc, p, len);
        }
        kernel_fpu_end();
        return crc;
    }

    for num_longs = len / core::mem::size_of::<usize>(); num_longs != 0;
        num_longs -= 1
    {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!("crc32 {value}, {crc}", value = in(reg) *(p as *const usize), crc = inout(reg) crc);
        #[cfg(not(target_arch = "x86_64"))]
        core::arch::asm!("crc32l {value}, {crc}", value = in(reg) *(p as *const u32), crc = inout(reg) crc);
        p = p.add(core::mem::size_of::<usize>());
    }
    if core::mem::size_of::<usize>() > 4 && (len & 4) != 0 {
        core::arch::asm!("crc32l {value}, {crc}", value = in(reg) *(p as *const u32), crc = inout(reg) crc);
        p = p.add(4);
    }
    if (len & 2) != 0 {
        core::arch::asm!("crc32w {value}, {crc}", value = in(reg) *(p as *const u16), crc = inout(reg) crc);
        p = p.add(2);
    }
    if (len & 1) != 0 {
        core::arch::asm!("crc32b {value}, {crc}", value = in(reg) *p, crc = inout(reg) crc);
    }
    crc
}

// crc32_be_arch = crc32_be_base (not implemented on this arch)
// crc32_mod_init_arch is the architecture-specific initializer.
unsafe fn crc32_mod_init_arch() {
    if boot_cpu_has(X86_FEATURE_XMM4_2) { static_branch_enable(&mut have_crc32); }
    if boot_cpu_has(X86_FEATURE_PCLMULQDQ) {
        static_branch_enable(&mut have_pclmulqdq);
        if have_vpclmul() {
            if have_avx512() {
                static_call_update(crc32_lsb_pclmul, crc32_lsb_vpclmul_avx512);
                static_branch_enable(&mut have_vpclmul_avx512);
            } else {
                static_call_update(crc32_lsb_pclmul, crc32_lsb_vpclmul_avx2);
            }
        }
    }
}

#[inline]
unsafe fn crc32_optimizations_arch() -> u32 {
    let mut optimizations: u32 = 0;
    if static_key_enabled(&have_crc32) { optimizations |= CRC32C_OPTIMIZATION; }
    if static_key_enabled(&have_pclmulqdq) { optimizations |= CRC32_LE_OPTIMIZATION; }
    optimizations
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
