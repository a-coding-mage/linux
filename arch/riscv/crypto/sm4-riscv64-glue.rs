// SPDX-License-Identifier: GPL-2.0-only
/*
 * SM4 using the RISC-V vector crypto extensions
 *
 * Copyright (C) 2023 VRULL GmbH
 * Author: Heiko Stuebner <heiko.stuebner@vrull.eu>
 *
 * Copyright (C) 2023 SiFive, Inc.
 * Author: Jerry Shih <jerry.shih@sifive.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/simd.h, asm/vector.h, crypto/internal/cipher.h,
// crypto/internal/simd.h, crypto/sm4.h, linux/linkage.h, linux/module.h

extern "C" {
    fn sm4_expandkey_zvksed_zvkb(
        user_key: *const u8,
        rkey_enc: *mut u32,
        rkey_dec: *mut u32,
    );
    fn sm4_crypt_zvksed_zvkb(
        rkey: *const u32,
        input: *const u8,
        output: *mut u8,
    );
}

unsafe extern "C" {
    fn crypto_simd_usable() -> bool;
    fn kernel_vector_begin();
    fn kernel_vector_end();
    fn sm4_expandkey(ctx: *mut sm4_ctx, key: *const u8, keylen: usize) -> i32;
    fn sm4_crypt_block(rkey: *const u32, dst: *mut u8, src: *const u8);
    fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut sm4_ctx;
    fn riscv_isa_extension_available(cpu: *const core::ffi::c_void, extension: u32) -> bool;
    fn riscv_vector_vlen() -> u32;
    fn crypto_register_alg(alg: *mut crypto_alg) -> i32;
    fn crypto_unregister_alg(alg: *mut crypto_alg);
}

#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sm4_ctx {
    pub rkey_enc: [u32; SM4_RKEY_WORDS],
    pub rkey_dec: [u32; SM4_RKEY_WORDS],
}

#[repr(C)]
pub struct crypto_alg {
    pub cra_flags: u32,
    pub cra_blocksize: usize,
    pub cra_ctxsize: usize,
    pub cra_priority: u32,
    pub cra_name: *const core::ffi::c_char,
    pub cra_driver_name: *const core::ffi::c_char,
    pub cra_cipher: crypto_cipher,
    pub cra_module: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct crypto_cipher {
    pub cia_min_keysize: usize,
    pub cia_max_keysize: usize,
    pub cia_setkey: Option<unsafe extern "C" fn(*mut crypto_tfm, *const u8, u32) -> i32>,
    pub cia_encrypt: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
    pub cia_decrypt: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
}

extern "C" {
    static mut riscv64_sm4_alg: crypto_alg;
}

const SM4_KEY_SIZE: usize = 16;
const SM4_BLOCK_SIZE: usize = 16;
const SM4_RKEY_WORDS: usize = 32;
const CRYPTO_ALG_TYPE_CIPHER: u32 = 0;
const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const ZVKSED: u32 = 0;
const ZVKB: u32 = 0;

unsafe extern "C" fn riscv64_sm4_setkey(
    tfm: *mut crypto_tfm,
    key: *const u8,
    keylen: u32,
) -> i32 {
    let ctx = crypto_tfm_ctx(tfm);

    if crypto_simd_usable() {
        if keylen as usize != SM4_KEY_SIZE {
            return -EINVAL;
        }
        kernel_vector_begin();
        sm4_expandkey_zvksed_zvkb(key, (*ctx).rkey_enc.as_mut_ptr(), (*ctx).rkey_dec.as_mut_ptr());
        kernel_vector_end();
        return 0;
    }
    sm4_expandkey(ctx, key, keylen as usize)
}

unsafe extern "C" fn riscv64_sm4_encrypt(
    tfm: *mut crypto_tfm,
    dst: *mut u8,
    src: *const u8,
) {
    let ctx = crypto_tfm_ctx(tfm);

    if crypto_simd_usable() {
        kernel_vector_begin();
        sm4_crypt_zvksed_zvkb((*ctx).rkey_enc.as_ptr(), src, dst);
        kernel_vector_end();
    } else {
        sm4_crypt_block((*ctx).rkey_enc.as_ptr(), dst, src);
    }
}

unsafe extern "C" fn riscv64_sm4_decrypt(
    tfm: *mut crypto_tfm,
    dst: *mut u8,
    src: *const u8,
) {
    let ctx = crypto_tfm_ctx(tfm);

    if crypto_simd_usable() {
        kernel_vector_begin();
        sm4_crypt_zvksed_zvkb((*ctx).rkey_dec.as_ptr(), src, dst);
        kernel_vector_end();
    } else {
        sm4_crypt_block((*ctx).rkey_dec.as_ptr(), dst, src);
    }
}

static mut RISCV64_SM4_ALG: crypto_alg = crypto_alg {
    cra_flags: CRYPTO_ALG_TYPE_CIPHER,
    cra_blocksize: SM4_BLOCK_SIZE,
    cra_ctxsize: core::mem::size_of::<sm4_ctx>(),
    cra_priority: 300,
    cra_name: b"sm4\0".as_ptr() as *const core::ffi::c_char,
    cra_driver_name: b"sm4-riscv64-zvksed-zvkb\0".as_ptr() as *const core::ffi::c_char,
    cra_cipher: crypto_cipher {
        cia_min_keysize: SM4_KEY_SIZE,
        cia_max_keysize: SM4_KEY_SIZE,
        cia_setkey: Some(riscv64_sm4_setkey),
        cia_encrypt: Some(riscv64_sm4_encrypt),
        cia_decrypt: Some(riscv64_sm4_decrypt),
    },
    cra_module: core::ptr::null_mut(),
};

unsafe extern "C" fn riscv64_sm4_mod_init() -> i32 {
    if riscv_isa_extension_available(core::ptr::null(), ZVKSED)
        && riscv_isa_extension_available(core::ptr::null(), ZVKB)
        && riscv_vector_vlen() >= 128
    {
        return crypto_register_alg(&mut RISCV64_SM4_ALG);
    }

    -ENODEV
}

unsafe extern "C" fn riscv64_sm4_mod_exit() {
    crypto_unregister_alg(&mut RISCV64_SM4_ALG);
}

// Equivalent module registration and metadata:
// module_init(riscv64_sm4_mod_init);
// module_exit(riscv64_sm4_mod_exit);
// MODULE_DESCRIPTION("SM4 (RISC-V accelerated)");
// MODULE_AUTHOR("Heiko Stuebner <heiko.stuebner@vrull.eu>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_CRYPTO("sm4");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
