/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AEGIS common definitions
 *
 * Copyright (c) 2018 Ondrej Mosnacek <omosnacek@gmail.com>
 * Copyright (c) 2018 Red Hat, Inc. All rights reserved.
 */

// C dependencies: <crypto/aes.h>, <linux/bitops.h>, and <linux/types.h>.

pub const AEGIS_BLOCK_SIZE: usize = 16;

#[repr(C)]
pub union aegis_block {
    pub words64: [__le64; AEGIS_BLOCK_SIZE / core::mem::size_of::<__le64>()],
    pub words32: [__le32; AEGIS_BLOCK_SIZE / core::mem::size_of::<__le32>()],
    pub bytes: [u8; AEGIS_BLOCK_SIZE],
}

#[repr(C)]
pub struct aegis_state {
    _private: [u8; 0],
}

extern "C" {
    pub static mut aegis128_have_aes_insn: ::core::ffi::c_int;

    pub fn crypto_aegis128_have_simd() -> bool;
    pub fn crypto_aegis128_update_simd(state: *mut aegis_state, msg: *const core::ffi::c_void);
    pub fn crypto_aegis128_init_simd(
        state: *mut aegis_state,
        key: *const aegis_block,
        iv: *const u8,
    );
    pub fn crypto_aegis128_encrypt_chunk_simd(
        state: *mut aegis_state,
        dst: *mut u8,
        src: *const u8,
        size: ::core::ffi::c_uint,
    );
    pub fn crypto_aegis128_decrypt_chunk_simd(
        state: *mut aegis_state,
        dst: *mut u8,
        src: *const u8,
        size: ::core::ffi::c_uint,
    );
    pub fn crypto_aegis128_final_simd(
        state: *mut aegis_state,
        tag_xor: *mut aegis_block,
        assoclen: ::core::ffi::c_uint,
        cryptlen: ::core::ffi::c_uint,
        authsize: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// The following aliases and symbols are supplied by the Linux dependencies.
pub type __le64 = u64;
pub type __le32 = u32;
extern "C" {
    pub static aes_enc_tab: [u32; 256];
    pub fn rol32(value: u32, shift: u32) -> u32;
    pub fn cpu_to_le32(value: u32) -> __le32;
}

pub const AEGIS_BLOCK_ALIGN: usize = core::mem::align_of::<aegis_block>();

#[macro_export]
macro_rules! AEGIS_ALIGNED {
    ($p:expr) => {
        (($p as usize) & (AEGIS_BLOCK_ALIGN - 1)) == 0
    };
}

#[inline(always)]
pub unsafe fn crypto_aegis_block_xor(dst: *mut aegis_block, src: *const aegis_block) {
    (*dst).words64[0] ^= (*src).words64[0];
    (*dst).words64[1] ^= (*src).words64[1];
}

#[inline(always)]
pub unsafe fn crypto_aegis_block_and(dst: *mut aegis_block, src: *const aegis_block) {
    (*dst).words64[0] &= (*src).words64[0];
    (*dst).words64[1] &= (*src).words64[1];
}

#[inline(always)]
pub unsafe fn crypto_aegis_aesenc(
    dst: *mut aegis_block,
    src: *const aegis_block,
    key: *const aegis_block,
) {
    let s = (*src).bytes;
    let t = aes_enc_tab.as_ptr();
    let d0 = *t.add(s[0] as usize)
        ^ rol32(*t.add(s[5] as usize), 8)
        ^ rol32(*t.add(s[10] as usize), 16)
        ^ rol32(*t.add(s[15] as usize), 24);
    let d1 = *t.add(s[4] as usize)
        ^ rol32(*t.add(s[9] as usize), 8)
        ^ rol32(*t.add(s[14] as usize), 16)
        ^ rol32(*t.add(s[3] as usize), 24);
    let d2 = *t.add(s[8] as usize)
        ^ rol32(*t.add(s[13] as usize), 8)
        ^ rol32(*t.add(s[2] as usize), 16)
        ^ rol32(*t.add(s[7] as usize), 24);
    let d3 = *t.add(s[12] as usize)
        ^ rol32(*t.add(s[1] as usize), 8)
        ^ rol32(*t.add(s[6] as usize), 16)
        ^ rol32(*t.add(s[11] as usize), 24);
    (*dst).words32[0] = cpu_to_le32(d0) ^ (*key).words32[0];
    (*dst).words32[1] = cpu_to_le32(d1) ^ (*key).words32[1];
    (*dst).words32[2] = cpu_to_le32(d2) ^ (*key).words32[2];
    (*dst).words32[3] = cpu_to_le32(d3) ^ (*key).words32[3];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
