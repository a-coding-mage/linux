// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RISC-V optimized CRC32 functions
 *
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the surrounding implementation:
// asm/hwcap.h, asm/alternative-macros.h, and crc-clmul.h.

extern "C" {
    fn riscv_has_extension_likely(extension: u32) -> bool;

    fn crc32_lsb_clmul(
        crc: u32,
        p: *const u8,
        len: usize,
        constants: *const u32,
    ) -> u32;
    fn crc32_msb_clmul(
        crc: u32,
        p: *const u8,
        len: usize,
        constants: *const u32,
    ) -> u32;
    fn crc32_le_base(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32_be_base(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32c_base(crc: u32, p: *const u8, len: usize) -> u32;

    static crc32_lsb_0xedb88320_consts: u32;
    static crc32_msb_0x04c11db7_consts: u32;
    static crc32_lsb_0x82f63b78_consts: u32;
}

// Build-time symbols supplied by the surrounding implementation.
const RISCV_ISA_EXT_ZBC: u32 = 0;
const CRC32_LE_OPTIMIZATION: u32 = 0;
const CRC32_BE_OPTIMIZATION: u32 = 0;
const CRC32C_OPTIMIZATION: u32 = 0;

unsafe fn crc32_le_arch(crc: u32, p: *const u8, len: usize) -> u32 {
    if riscv_has_extension_likely(RISCV_ISA_EXT_ZBC) {
        return crc32_lsb_clmul(crc, p, len, &crc32_lsb_0xedb88320_consts);
    }
    crc32_le_base(crc, p, len)
}

unsafe fn crc32_be_arch(crc: u32, p: *const u8, len: usize) -> u32 {
    if riscv_has_extension_likely(RISCV_ISA_EXT_ZBC) {
        return crc32_msb_clmul(crc, p, len, &crc32_msb_0x04c11db7_consts);
    }
    crc32_be_base(crc, p, len)
}

unsafe fn crc32c_arch(crc: u32, p: *const u8, len: usize) -> u32 {
    if riscv_has_extension_likely(RISCV_ISA_EXT_ZBC) {
        return crc32_lsb_clmul(crc, p, len, &crc32_lsb_0x82f63b78_consts);
    }
    crc32c_base(crc, p, len)
}

unsafe fn crc32_optimizations_arch() -> u32 {
    if riscv_has_extension_likely(RISCV_ISA_EXT_ZBC) {
        return CRC32_LE_OPTIMIZATION
            | CRC32_BE_OPTIMIZATION
            | CRC32C_OPTIMIZATION;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
