// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RISC-V optimized most-significant-bit-first CRC16
 *
 * Copyright 2025 Google LLC
 */

// Dependency supplied by the translated crc-clmul implementation.
pub type CrcT = u16;
pub const LSB_CRC: i32 = 0;

#[repr(C)]
pub struct crc_clmul_consts {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn crc_clmul(
        crc: u16,
        p: *const core::ffi::c_void,
        len: usize,
        consts: *const crc_clmul_consts,
    ) -> u16;
}

pub unsafe fn crc16_msb_clmul(
    crc: u16,
    p: *const core::ffi::c_void,
    len: usize,
    consts: *const crc_clmul_consts,
) -> u16 {
    unsafe { crc_clmul(crc, p, len, consts) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
