/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * CRC64 using ARM PMULL instructions
 */

// C dependency: <asm/simd.h>

extern "C" {
    static mut have_pmull: bool;

    fn crc64_nvme_neon(crc: u64, p: *const u8, len: usize) -> u64;
    fn crc64_nvme_generic(crc: u64, p: *const u8, len: usize) -> u64;
    fn may_use_simd() -> bool;
    static mut elf_hwcap2: usize;
}

// C macro alias: crc64_be_arch crc64_be_generic
pub use crc64_be_generic as crc64_be_arch;

// External dependency supplied by the surrounding translation.
extern "C" {
    fn crc64_be_generic(crc: u64, p: *const u8, len: usize) -> u64;
}

#[inline]
pub unsafe fn crc64_nvme_arch(mut crc: u64, mut p: *const u8, mut len: usize) -> u64 {
    if len >= 128 && have_pmull && may_use_simd() {
        loop {
            let chunk = core::cmp::min(len & !15, 4096);

            crc = crc64_nvme_neon(crc, p, chunk);

            p = p.add(chunk);
            len -= chunk;
            if len < 128 {
                break;
            }
        }
    }
    crc64_nvme_generic(crc, p, len)
}

// C macro self-reference: crc64_mod_init_arch crc64_mod_init_arch
pub unsafe fn crc64_mod_init_arch() {
    const HWCAP2_PMULL: usize = 1 << 1;

    if elf_hwcap2 & HWCAP2_PMULL != 0 {
        have_pmull = true;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
