// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the kernel and other translation units are kept as
// external symbols/types here.

use core::ffi::c_void;

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type c_int = core::ffi::c_int;

#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sm4_ctx {
    pub rkey_enc: [u32; 32],
    pub rkey_dec: [u32; 32],
}

#[repr(C)]
pub struct crypto_alg_cipher {
    pub cia_min_keysize: u32,
    pub cia_max_keysize: u32,
    pub cia_setkey: Option<unsafe extern "C" fn(*mut crypto_tfm, *const u8, u32) -> c_int>,
    pub cia_encrypt: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
    pub cia_decrypt: Option<unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8)>,
}

#[repr(C)]
pub union crypto_alg_u {
    pub cipher: crypto_alg_cipher,
}

#[repr(C)]
pub struct crypto_alg {
    pub cra_name: *const u8,
    pub cra_driver_name: *const u8,
    pub cra_priority: c_int,
    pub cra_flags: u32,
    pub cra_blocksize: u32,
    pub cra_ctxsize: usize,
    pub cra_module: *mut c_void,
    pub cra_u: crypto_alg_u,
}

extern "C" {
    pub fn sm4_ce_do_crypt(rk: *const u32, out: *mut c_void, input: *const c_void);
    pub fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut c_void;
    pub fn sm4_expandkey(ctx: *mut sm4_ctx, key: *const u8, key_len: u32) -> c_int;
    pub fn sm4_crypt_block(rk: *const u32, out: *mut u8, input: *const u8);
    pub fn crypto_simd_usable() -> bool;
    pub fn crypto_register_alg(alg: *mut crypto_alg) -> c_int;
    pub fn crypto_unregister_alg(alg: *mut crypto_alg);
}

// Kernel constants/macros supplied by the included headers.
extern "C" {
    static mut THIS_MODULE: c_void;
}

const SM4_BLOCK_SIZE: u32 = 16;
const SM4_KEY_SIZE: u32 = 16;
const CRYPTO_ALG_TYPE_CIPHER: u32 = 0;

unsafe fn sm4_ce_setkey(tfm: *mut crypto_tfm, key: *const u8, key_len: u32) -> c_int {
    let ctx = crypto_tfm_ctx(tfm) as *mut sm4_ctx;
    sm4_expandkey(ctx, key, key_len)
}

unsafe fn sm4_ce_encrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) {
    let ctx = crypto_tfm_ctx(tfm) as *const sm4_ctx;

    if !crypto_simd_usable() {
        sm4_crypt_block((*ctx).rkey_enc.as_ptr(), out, input);
    } else {
        // scoped_ksimd() acquires the kernel SIMD context around this call.
        sm4_ce_do_crypt((*ctx).rkey_enc.as_ptr(), out as *mut c_void, input as *const c_void);
    }
}

unsafe fn sm4_ce_decrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) {
    let ctx = crypto_tfm_ctx(tfm) as *const sm4_ctx;

    if !crypto_simd_usable() {
        sm4_crypt_block((*ctx).rkey_dec.as_ptr(), out, input);
    } else {
        // scoped_ksimd() acquires the kernel SIMD context around this call.
        sm4_ce_do_crypt((*ctx).rkey_dec.as_ptr(), out as *mut c_void, input as *const c_void);
    }
}

static mut sm4_ce_alg: crypto_alg = crypto_alg {
    cra_name: b"sm4\0".as_ptr(),
    cra_driver_name: b"sm4-ce\0".as_ptr(),
    cra_priority: 300,
    cra_flags: CRYPTO_ALG_TYPE_CIPHER,
    cra_blocksize: SM4_BLOCK_SIZE,
    cra_ctxsize: core::mem::size_of::<sm4_ctx>(),
    cra_module: core::ptr::addr_of_mut!(THIS_MODULE),
    cra_u: crypto_alg_u {
        cipher: crypto_alg_cipher {
            cia_min_keysize: SM4_KEY_SIZE,
            cia_max_keysize: SM4_KEY_SIZE,
            cia_setkey: Some(sm4_ce_setkey),
            cia_encrypt: Some(sm4_ce_encrypt),
            cia_decrypt: Some(sm4_ce_decrypt),
        },
    },
};

unsafe fn sm4_ce_mod_init() -> c_int {
    crypto_register_alg(core::ptr::addr_of_mut!(sm4_ce_alg))
}

unsafe fn sm4_ce_mod_fini() {
    crypto_unregister_alg(core::ptr::addr_of_mut!(sm4_ce_alg));
}

// module_cpu_feature_match(SM4, sm4_ce_mod_init);
// module_exit(sm4_ce_mod_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
