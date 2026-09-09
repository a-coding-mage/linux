// SPDX-License-Identifier: GPL-2.0-only
/* CRC32c (Castagnoli), sparc64 crc32c opcode accelerated
 *
 * This is based largely upon arch/x86/crypto/crc32c-intel.c
 *
 * Copyright (C) 2008 Intel Corporation
 * Authors: Austin Zhang <austin_zhang@linux.intel.com>
 *          Kent Liu <kent.liu@intel.com>
 */

// C dependencies: <asm/pstate.h>, <asm/elf.h>

// The concrete static-key type and initializer are supplied by the kernel
// environment corresponding to DEFINE_STATIC_KEY_FALSE.
extern "C" {
    static have_crc32c_opcode: StaticKeyFalse;
    static sparc64_elf_hwcap: ::core::ffi::c_ulong;
}

// #define crc32_le_arch crc32_le_base /* not implemented on this arch */
// #define crc32_be_arch crc32_be_base /* not implemented on this arch */

extern "C" {
    fn crc32c_sparc64(crcp: *mut u32, data: *const u64, len: usize);
    fn crc32c_base(crc: u32, data: *const u8, len: usize) -> u32;
    fn static_branch_likely(key: *const StaticKeyFalse) -> bool;
    fn static_branch_enable(key: *mut StaticKeyFalse);
    fn static_key_enabled(key: *const StaticKeyFalse) -> bool;
    fn pr_info(format: *const ::core::ffi::c_char, ...);
}

#[inline]
pub unsafe fn crc32c_arch(mut crc: u32, mut data: *const u8, mut len: usize) -> u32 {
    let mut n = (-(data as usize) as usize) & 7;

    if !static_branch_likely(&have_crc32c_opcode) {
        return crc32c_base(crc, data, len);
    }

    if n != 0 {
        /* Data isn't 8-byte aligned.  Align it. */
        n = if n < len { n } else { len };
        crc = crc32c_base(crc, data, n);
        data = data.add(n);
        len -= n;
    }
    n = len & !7usize;
    if n != 0 {
        crc32c_sparc64(&mut crc, data as *const u64, n);
        data = data.add(n);
        len -= n;
    }
    if len != 0 {
        crc = crc32c_base(crc, data, len);
    }
    crc
}

// #define crc32_mod_init_arch crc32_mod_init_arch
unsafe fn crc32_mod_init_arch() {
    let mut cfr: ::core::ffi::c_ulong;

    if (sparc64_elf_hwcap & HWCAP_SPARC_CRYPTO) == 0 {
        return;
    }

    ::core::arch::asm!("rd %asr26, {0}", out(reg) cfr);
    if (cfr & CFR_CRC32C) == 0 {
        return;
    }

    static_branch_enable(&mut have_crc32c_opcode);
    pr_info(b"Using sparc64 crc32c opcode optimized CRC32C implementation\0".as_ptr() as *const _);
}

#[inline]
unsafe fn crc32_optimizations_arch() -> u32 {
    if static_key_enabled(&have_crc32c_opcode) {
        return CRC32C_OPTIMIZATION;
    }
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
