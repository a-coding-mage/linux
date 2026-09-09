/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright 2025 Google LLC */

// Dependency: crc-clmul-consts.h supplies the C-compatible declaration of
// `struct crc_clmul_consts`.
#[repr(C)]
pub struct crc_clmul_consts {
    _private: [u8; 0],
}

extern "C" {
    pub fn crc16_msb_clmul(
        crc: u16,
        p: *const core::ffi::c_void,
        len: usize,
        consts: *const crc_clmul_consts,
    ) -> u16;

    pub fn crc32_msb_clmul(
        crc: u32,
        p: *const core::ffi::c_void,
        len: usize,
        consts: *const crc_clmul_consts,
    ) -> u32;

    pub fn crc32_lsb_clmul(
        crc: u32,
        p: *const core::ffi::c_void,
        len: usize,
        consts: *const crc_clmul_consts,
    ) -> u32;

    // C build condition: CONFIG_64BIT.
    #[cfg(target_pointer_width = "64")]
    pub fn crc64_msb_clmul(
        crc: u64,
        p: *const core::ffi::c_void,
        len: usize,
        consts: *const crc_clmul_consts,
    ) -> u64;

    #[cfg(target_pointer_width = "64")]
    pub fn crc64_lsb_clmul(
        crc: u64,
        p: *const core::ffi::c_void,
        len: usize,
        consts: *const crc_clmul_consts,
    ) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
