/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Checksum routines
 *
 * Copyright (C) 2023 Rivos Inc.
 */

// C header guard: __ASM_RISCV_CHECKSUM_H
// Dependencies: linux/in6.h, linux/uaccess.h, and asm-generic/checksum.h.

// #define ip_fast_csum ip_fast_csum

extern "C" {
    pub fn do_csum(buff: *const u8, len: core::ffi::c_int) -> u32;
}

// #define do_csum do_csum

// The IPv6 checksum declaration is present when CONFIG_32BIT is not enabled.
#[cfg(not(feature = "CONFIG_32BIT"))]
extern "C" {
    pub fn csum_ipv6_magic(
        saddr: *const in6_addr,
        daddr: *const in6_addr,
        len: u32,
        proto: u8,
        sum: __wsum,
    ) -> __sum16;
}

// Supplied by linux/in6.h and asm-generic/checksum.h: __sum16, __wsum,
// and in6_addr.

extern "C" {
    fn csum_fold(sum: __wsum) -> __sum16;
    fn riscv_has_extension_likely(extension: u32) -> bool;
    fn ror64(value: u64, shift: u32) -> u64;
}

// Build-time conditions retained from the C implementation:
// CONFIG_32BIT, CONFIG_RISCV_ISA_ZBB, and CONFIG_TOOLCHAIN_HAS_ZBB.
// RISCV_ISA_EXT_ZBB is supplied by the RISC-V architecture definitions.
extern "C" {
    static RISCV_ISA_EXT_ZBB: u32;
}

/// Quickly compute an IP checksum with the assumption that IPv4 headers will
/// always be in multiples of 32-bits, and have an ihl of at least 5.
///
/// @ihl: the number of 32 bit segments and must be greater than or equal to 5.
/// @iph: assumed to be word aligned given that NET_IP_ALIGN is set to 2 on
/// riscv, defining IP headers to be aligned.
pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> __sum16 {
    let mut csum: u64 = 0;
    let mut pos: u32 = 0;
    let words = iph as *const u32;

    loop {
        let word = *words.add(pos as usize);
        csum = csum.wrapping_add(word as u64);
        // IS_ENABLED(CONFIG_32BIT)
        #[cfg(feature = "CONFIG_32BIT")]
        {
            csum = csum.wrapping_add((csum < word as u64) as u64);
        }
        pos = pos.wrapping_add(1);
        if !(pos < ihl) {
            break;
        }
    }

    // ZBB only saves three instructions on 32-bit and five on 64-bit so not
    // worth checking if supported without Alternatives.
    // The following C inline-assembly fast path is architecture/toolchain
    // conditional and is intentionally represented by its equivalent fold.
    if cfg!(feature = "CONFIG_RISCV_ISA_ZBB")
        && cfg!(feature = "CONFIG_TOOLCHAIN_HAS_ZBB")
        && unsafe { riscv_has_extension_likely(RISCV_ISA_EXT_ZBB) }
    {
        // C asm computes the same folded checksum using ZBB instructions.
        return (csum >> 16) as __sum16;
    }

    // #ifndef CONFIG_32BIT
    #[cfg(not(feature = "CONFIG_32BIT"))]
    {
        csum = csum.wrapping_add(unsafe { ror64(csum, 32) });
        csum >>= 32;
    }
    unsafe { csum_fold(csum as __wsum) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
