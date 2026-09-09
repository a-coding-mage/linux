// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RISC-V optimized CRC64 functions
 *
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

pub type u8 = core::primitive::u8;
pub type u64 = core::primitive::u64;

pub const RISCV_ISA_EXT_ZBC: i32 = 0;

unsafe extern "C" {
    fn riscv_has_extension_likely(extension: i32) -> bool;

    static crc64_msb_0x42f0e1eba9ea3693_consts: c_void;
    static crc64_lsb_0x9a6c9329ac4bc9b5_consts: c_void;

    fn crc64_msb_clmul(
        crc: u64,
        p: *const u8,
        len: usize,
        constants: *const c_void,
    ) -> u64;
    fn crc64_lsb_clmul(
        crc: u64,
        p: *const u8,
        len: usize,
        constants: *const c_void,
    ) -> u64;
    fn crc64_be_generic(crc: u64, p: *const u8, len: usize) -> u64;
    fn crc64_nvme_generic(crc: u64, p: *const u8, len: usize) -> u64;
}

#[inline]
pub unsafe fn crc64_be_arch(crc: u64, p: *const u8, len: usize) -> u64 {
    if riscv_has_extension_likely(RISCV_ISA_EXT_ZBC) {
        return crc64_msb_clmul(
            crc,
            p,
            len,
            &crc64_msb_0x42f0e1eba9ea3693_consts as *const c_void,
        );
    }
    crc64_be_generic(crc, p, len)
}

#[inline]
pub unsafe fn crc64_nvme_arch(crc: u64, p: *const u8, len: usize) -> u64 {
    if riscv_has_extension_likely(RISCV_ISA_EXT_ZBC) {
        return crc64_lsb_clmul(
            crc,
            p,
            len,
            &crc64_lsb_0x9a6c9329ac4bc9b5_consts as *const c_void,
        );
    }
    crc64_nvme_generic(crc, p, len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
