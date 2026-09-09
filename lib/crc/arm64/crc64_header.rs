/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * CRC64 using ARM64 PMULL instructions
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    pub fn crc64_nvme_neon(crc: u64, p: *const u8, len: usize) -> u64;
    pub fn crc64_nvme_generic(crc: u64, p: *const u8, len: usize) -> u64;
    pub fn cpu_have_named_feature(feature: i32) -> bool;
    pub fn may_use_simd() -> bool;
}

// #define crc64_be_arch crc64_be_generic
pub use crc64_be_generic as crc64_be_arch;

extern "C" {
    pub fn crc64_be_generic(crc: u64, p: *const u8, len: usize) -> u64;
}

// The PMULL feature identifier is supplied by the ARM64 CPU-feature headers.
unsafe extern "C" {
    static PMULL: i32;
}

#[inline]
pub unsafe fn crc64_nvme_arch(mut crc: u64, mut p: *const u8, mut len: usize) -> u64 {
    if len >= 128
        && cpu_have_named_feature(PMULL)
        && may_use_simd()
    {
        let chunk = len & !15;

        // scoped_ksimd() protects the SIMD use for the duration of this call.
        crc = crc64_nvme_neon(crc, p, chunk);

        p = p.add(chunk);
        len &= 15;
    }
    crc64_nvme_generic(crc, p, len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
