// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RISC-V optimized least-significant-bit-first CRC64
 *
 * Copyright 2025 Google LLC
 */

// Dependency supplied by crc-clmul.h.

pub type CrcT = u64;
pub const LSB_CRC: i32 = 1;

// The crc-clmul-template.h include supplies the implementation and declaration
// of crc_clmul; it remains an external dependency of this translation unit.
#[repr(C)]
pub struct CrcClmulConsts {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn crc_clmul(
        crc: u64,
        p: *const core::ffi::c_void,
        len: usize,
        consts: *const CrcClmulConsts,
    ) -> u64;
}

pub unsafe fn crc64_lsb_clmul(
    crc: u64,
    p: *const core::ffi::c_void,
    len: usize,
    consts: *const CrcClmulConsts,
) -> u64 {
    unsafe { crc_clmul(crc, p, len, consts) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
