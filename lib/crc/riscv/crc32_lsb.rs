// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RISC-V optimized least-significant-bit-first CRC32
 *
 * Copyright 2025 Google LLC
 */

// Dependency supplied by the C crc-clmul.h header.
pub type u32 = core::primitive::u32;

pub type crc_t = u32;
pub const LSB_CRC: i32 = 1;

// The C crc-clmul-template.h include supplies the crc_clmul implementation.
// This opaque declaration represents struct crc_clmul_consts from that header.
#[repr(C)]
pub enum crc_clmul_consts {}

unsafe extern "C" {
    fn crc_clmul(
        crc: u32,
        p: *const core::ffi::c_void,
        len: usize,
        consts: *const crc_clmul_consts,
    ) -> u32;
}

pub unsafe fn crc32_lsb_clmul(
    crc: u32,
    p: *const core::ffi::c_void,
    len: usize,
    consts: *const crc_clmul_consts,
) -> u32 {
    unsafe { crc_clmul(crc, p, len, consts) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
