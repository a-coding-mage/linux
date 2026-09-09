/* SPDX-License-Identifier: GPL-2.0-or-later */

// linux/types.h dependency is represented by Rust's u8 type.

pub const ARIA_AESNI_PARALLEL_BLOCKS: usize = 16;
pub const ARIA_AESNI_PARALLEL_BLOCK_SIZE: usize =
    ARIA_BLOCK_SIZE * ARIA_AESNI_PARALLEL_BLOCKS;

pub const ARIA_AESNI_AVX2_PARALLEL_BLOCKS: usize = 32;
pub const ARIA_AESNI_AVX2_PARALLEL_BLOCK_SIZE: usize =
    ARIA_BLOCK_SIZE * ARIA_AESNI_AVX2_PARALLEL_BLOCKS;

pub const ARIA_GFNI_AVX512_PARALLEL_BLOCKS: usize = 64;
pub const ARIA_GFNI_AVX512_PARALLEL_BLOCK_SIZE: usize =
    ARIA_BLOCK_SIZE * ARIA_GFNI_AVX512_PARALLEL_BLOCKS;

extern "C" {
    pub fn aria_aesni_avx_encrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx_decrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx_ctr_crypt_16way(
        ctx: *const core::ffi::c_void,
        dst: *mut u8,
        src: *const u8,
        keystream: *mut u8,
        iv: *mut u8,
    );
    pub fn aria_aesni_avx_gfni_encrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx_gfni_decrypt_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx_gfni_ctr_crypt_16way(
        ctx: *const core::ffi::c_void,
        dst: *mut u8,
        src: *const u8,
        keystream: *mut u8,
        iv: *mut u8,
    );

    pub fn aria_aesni_avx2_encrypt_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx2_decrypt_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx2_ctr_crypt_32way(
        ctx: *const core::ffi::c_void,
        dst: *mut u8,
        src: *const u8,
        keystream: *mut u8,
        iv: *mut u8,
    );
    pub fn aria_aesni_avx2_gfni_encrypt_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx2_gfni_decrypt_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn aria_aesni_avx2_gfni_ctr_crypt_32way(
        ctx: *const core::ffi::c_void,
        dst: *mut u8,
        src: *const u8,
        keystream: *mut u8,
        iv: *mut u8,
    );
}

#[repr(C)]
pub struct aria_avx_ops {
    pub aria_encrypt_16way: Option<unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8)>,
    pub aria_decrypt_16way: Option<unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8)>,
    pub aria_ctr_crypt_16way:
        Option<unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8, *mut u8, *mut u8)>,
    pub aria_encrypt_32way: Option<unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8)>,
    pub aria_decrypt_32way: Option<unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8)>,
    pub aria_ctr_crypt_32way:
        Option<unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8, *mut u8, *mut u8)>,
    pub aria_encrypt_64way: Option<unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8)>,
    pub aria_decrypt_64way: Option<unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8)>,
    pub aria_ctr_crypt_64way:
        Option<unsafe extern "C" fn(*const core::ffi::c_void, *mut u8, *const u8, *mut u8, *mut u8)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
