// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RISC-V optimized most-significant-bit-first CRC64
 *
 * Copyright 2025 Google LLC
 */

// Dependency supplied by crc-clmul.h and crc-clmul-template.h.

pub type crc_t = u64;
pub const LSB_CRC: i32 = 0;

#[repr(C)]
pub struct crc_clmul_consts {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn crc_clmul(
        crc: u64,
        p: *const core::ffi::c_void,
        len: usize,
        consts: *const crc_clmul_consts,
    ) -> u64;
}

pub unsafe fn crc64_msb_clmul(
    crc: u64,
    p: *const core::ffi::c_void,
    len: usize,
    consts: *const crc_clmul_consts,
) -> u64 {
    unsafe { crc_clmul(crc, p, len, consts) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
