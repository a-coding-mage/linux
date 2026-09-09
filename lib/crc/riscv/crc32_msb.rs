// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RISC-V optimized most-significant-bit-first CRC32
 *
 * Copyright 2025 Google LLC
 */

// Dependency supplied by the C header "crc-clmul.h".
// The implementation and declarations from "crc-clmul-template.h" are
// supplied externally.

type crc_t = u32;
const LSB_CRC: i32 = 0;

#[repr(C)]
pub struct crc_clmul_consts {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn crc_clmul(
        crc: u32,
        p: *const core::ffi::c_void,
        len: usize,
        consts: *const crc_clmul_consts,
    ) -> u32;
}

pub unsafe extern "C" fn crc32_msb_clmul(
    crc: u32,
    p: *const core::ffi::c_void,
    len: usize,
    consts: *const crc_clmul_consts,
) -> u32 {
    unsafe { crc_clmul(crc, p, len, consts) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
