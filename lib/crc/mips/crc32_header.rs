// SPDX-License-Identifier: GPL-2.0
/*
 * crc32-mips.c - CRC32 and CRC32C using optional MIPSr6 instructions
 *
 * Module based on arm64/crypto/crc32-arm.c
 *
 * Copyright (C) 2014 Linaro Ltd <yazen.ghannam@linaro.org>
 * Copyright (C) 2018 MIPS Tech, LLC
 */

// C header dependencies are supplied by the surrounding kernel translation.

#[allow(unused_macros)]
macro_rules! __CRC32 {
    ($crc:expr, $value:expr, $op:ident, $sz:expr, $kind:expr) => {{
        // MIPS CRC instruction; the assembler feature setup from the C macros
        // is represented by the target-specific inline assembly itself.
        unsafe {
            core::arch::asm!(
                concat!(stringify!($op), " {0}, {1}, {0}"),
                inout(reg) $crc,
                in(reg) $value,
                options(nostack, preserves_flags)
            );
        }
    }};
}

macro_rules! _CRC32_crc32b { ($crc:expr, $value:expr) => { __CRC32!($crc, $value, crc32b, 0, 0) }; }
macro_rules! _CRC32_crc32h { ($crc:expr, $value:expr) => { __CRC32!($crc, $value, crc32h, 1, 0) }; }
macro_rules! _CRC32_crc32w { ($crc:expr, $value:expr) => { __CRC32!($crc, $value, crc32w, 2, 0) }; }
macro_rules! _CRC32_crc32d { ($crc:expr, $value:expr) => { __CRC32!($crc, $value, crc32d, 3, 0) }; }
macro_rules! _CRC32_crc32cb { ($crc:expr, $value:expr) => { __CRC32!($crc, $value, crc32cb, 0, 1) }; }
macro_rules! _CRC32_crc32ch { ($crc:expr, $value:expr) => { __CRC32!($crc, $value, crc32ch, 1, 1) }; }
macro_rules! _CRC32_crc32cw { ($crc:expr, $value:expr) => { __CRC32!($crc, $value, crc32cw, 2, 1) }; }
macro_rules! _CRC32_crc32cd { ($crc:expr, $value:expr) => { __CRC32!($crc, $value, crc32cd, 3, 1) }; }

macro_rules! CRC32 { ($crc:expr, $value:expr, $size:ident) => { paste::paste! { [<_CRC32_crc32 $size>]!($crc, $value) } }; }
macro_rules! CRC32C { ($crc:expr, $value:expr, $size:ident) => { paste::paste! { [<_CRC32_crc32c $size>]!($crc, $value) } }; }

extern "C" {
    static have_crc32: StaticKey;
    fn crc32_le_base(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32c_base(crc: u32, p: *const u8, len: usize) -> u32;
    fn cpu_have_feature(feature: u32) -> bool;
    fn cpu_feature(feature: u32) -> u32;
    fn static_branch_likely(key: *const StaticKey) -> bool;
    fn static_branch_enable(key: *const StaticKey);
    fn static_key_enabled(key: *const StaticKey) -> bool;
}

#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

const MIPS_CRC32: u32 = 0;
const CRC32_LE_OPTIMIZATION: u32 = 1;
const CRC32C_OPTIMIZATION: u32 = 2;

#[inline]
pub unsafe fn crc32_le_arch(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    if !static_branch_likely(&have_crc32) {
        return crc32_le_base(crc, p, len);
    }

    // CONFIG_64BIT build-time condition from the source.
    if cfg!(target_pointer_width = "64") {
        while len >= core::mem::size_of::<u64>() {
            let value = u64::from_le((p as *const u64).read_unaligned());
            CRC32!(crc, value, d);
            p = p.add(core::mem::size_of::<u64>());
            len -= core::mem::size_of::<u64>();
        }
        if len & core::mem::size_of::<u32>() != 0 {
            let value = u32::from_le((p as *const u32).read_unaligned());
            CRC32!(crc, value, w);
            p = p.add(core::mem::size_of::<u32>());
        }
    } else {
        while len >= core::mem::size_of::<u32>() {
            let value = u32::from_le((p as *const u32).read_unaligned());
            CRC32!(crc, value, w);
            p = p.add(core::mem::size_of::<u32>());
            len -= core::mem::size_of::<u32>();
        }
    }
    if len & core::mem::size_of::<u16>() != 0 {
        let value = u16::from_le((p as *const u16).read_unaligned());
        CRC32!(crc, value, h);
        p = p.add(core::mem::size_of::<u16>());
    }
    if len & core::mem::size_of::<u8>() != 0 {
        let value = *p;
        p = p.add(1);
        CRC32!(crc, value, b);
    }
    crc
}

#[inline]
pub unsafe fn crc32c_arch(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    if !static_branch_likely(&have_crc32) { return crc32c_base(crc, p, len); }
    // The body mirrors crc32_le_arch; CONFIG_64BIT is a build-time condition.
    if cfg!(target_pointer_width = "64") {
        while len >= 8 { let value = u64::from_le((p as *const u64).read_unaligned()); CRC32C!(crc, value, d); p = p.add(8); len -= 8; }
        if len & 4 != 0 { let value = u32::from_le((p as *const u32).read_unaligned()); CRC32C!(crc, value, w); p = p.add(4); }
    } else { while len >= 4 { let value = u32::from_le((p as *const u32).read_unaligned()); CRC32C!(crc, value, w); p = p.add(4); len -= 4; } }
    if len & 2 != 0 { let value = u16::from_le((p as *const u16).read_unaligned()); CRC32C!(crc, value, h); p = p.add(2); }
    if len & 1 != 0 { let value = *p; CRC32C!(crc, value, b); }
    crc
}

// crc32_be_arch is not implemented on this architecture.
// #define crc32_be_arch crc32_be_base

pub unsafe fn crc32_mod_init_arch() {
    if cpu_have_feature(cpu_feature(MIPS_CRC32)) { static_branch_enable(&have_crc32); }
}

#[inline]
pub unsafe fn crc32_optimizations_arch() -> u32 {
    if static_key_enabled(&have_crc32) { CRC32_LE_OPTIMIZATION | CRC32C_OPTIMIZATION } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
