// SPDX-License-Identifier: GPL-2.0-only
/* Glue code for CAMELLIA encryption optimized for sparc64 crypto opcodes. */

// Kernel headers and architecture headers are supplied by the surrounding
// kernel translation unit.

pub const CAMELLIA_MIN_KEY_SIZE: usize = 16;
pub const CAMELLIA_MAX_KEY_SIZE: usize = 32;
pub const CAMELLIA_BLOCK_SIZE: usize = 16;
pub const CAMELLIA_TABLE_BYTE_LEN: usize = 272;

#[repr(C)]
pub struct camellia_sparc64_ctx {
    pub encrypt_key: [u64; CAMELLIA_TABLE_BYTE_LEN / core::mem::size_of::<u64>()],
    pub decrypt_key: [u64; CAMELLIA_TABLE_BYTE_LEN / core::mem::size_of::<u64>()],
    pub key_len: i32,
}

extern "C" {
    pub fn camellia_sparc64_key_expand(in_key: *const u32, encrypt_key: *mut u64,
        key_len: u32, decrypt_key: *mut u64);
    pub fn camellia_sparc64_crypt(key: *const u64, input: *const u32,
        output: *mut u32, key_len: u32);
    pub fn camellia_sparc64_load_keys(key: *const u64, key_len: u32);
    pub fn camellia_sparc64_ecb_crypt_3_grand_rounds(input: *const u64, output: *mut u64,
        len: u32, key: *const u64);
    pub fn camellia_sparc64_ecb_crypt_4_grand_rounds(input: *const u64, output: *mut u64,
        len: u32, key: *const u64);
    pub fn camellia_sparc64_cbc_encrypt_3_grand_rounds(input: *const u64, output: *mut u64,
        len: u32, key: *const u64, iv: *mut u64);
    pub fn camellia_sparc64_cbc_encrypt_4_grand_rounds(input: *const u64, output: *mut u64,
        len: u32, key: *const u64, iv: *mut u64);
    pub fn camellia_sparc64_cbc_decrypt_3_grand_rounds(input: *const u64, output: *mut u64,
        len: u32, key: *const u64, iv: *mut u64);
    pub fn camellia_sparc64_cbc_decrypt_4_grand_rounds(input: *const u64, output: *mut u64,
        len: u32, key: *const u64, iv: *mut u64);
}

// The following framework types and helpers are provided by the kernel.
#[allow(non_camel_case_types)] pub enum crypto_tfm {}
#[allow(non_camel_case_types)] pub enum crypto_skcipher {}
#[allow(non_camel_case_types)] pub enum skcipher_request {}
extern "C" {
    fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut camellia_sparc64_ctx;
    fn crypto_skcipher_tfm(tfm: *mut crypto_skcipher) -> *mut crypto_tfm;
    fn crypto_skcipher_ctx(tfm: *mut crypto_skcipher) -> *const camellia_sparc64_ctx;
}

unsafe fn camellia_set_key(tfm: *mut crypto_tfm, in_key: *const u8, key_len: u32) -> i32 {
    if key_len != 16 && key_len != 24 && key_len != 32 { return -22; }
    let ctx = &mut *crypto_tfm_ctx(tfm);
    ctx.key_len = key_len as i32;
    camellia_sparc64_key_expand(in_key as *const u32, ctx.encrypt_key.as_mut_ptr(),
        key_len, ctx.decrypt_key.as_mut_ptr());
    0
}

unsafe fn camellia_set_key_skcipher(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 {
    camellia_set_key(crypto_skcipher_tfm(tfm), key, len)
}

unsafe fn camellia_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    let ctx = &*crypto_tfm_ctx(tfm);
    camellia_sparc64_crypt(ctx.encrypt_key.as_ptr(), src as *const u32, dst as *mut u32,
        ctx.key_len as u32);
}

unsafe fn camellia_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    let ctx = &*crypto_tfm_ctx(tfm);
    camellia_sparc64_crypt(ctx.decrypt_key.as_ptr(), src as *const u32, dst as *mut u32,
        ctx.key_len as u32);
}

// skcipher_walk and kernel registration objects retain their original ABI
// role; their concrete definitions are supplied by the kernel environment.
extern "C" {
    fn ecb_encrypt(req: *mut skcipher_request) -> i32;
    fn ecb_decrypt(req: *mut skcipher_request) -> i32;
    fn cbc_encrypt(req: *mut skcipher_request) -> i32;
    fn cbc_decrypt(req: *mut skcipher_request) -> i32;
}

// Equivalent registration records (field values mirror the C source).
#[repr(C)]
pub struct crypto_alg {
    pub name: *const u8, pub driver_name: *const u8, pub priority: i32,
    pub blocksize: usize, pub ctxsize: usize, pub alignmask: usize,
    pub min_keysize: usize, pub max_keysize: usize,
    pub setkey: Option<unsafe fn(*mut crypto_tfm, *const u8, u32) -> i32>,
    pub encrypt: Option<unsafe fn(*mut crypto_tfm, *mut u8, *const u8)>,
    pub decrypt: Option<unsafe fn(*mut crypto_tfm, *mut u8, *const u8)>,
}

#[allow(non_upper_case_globals)]
pub static mut cipher_alg: crypto_alg = crypto_alg {
    name: b"camellia\0".as_ptr(), driver_name: b"camellia-sparc64\0".as_ptr(),
    priority: 0, blocksize: CAMELLIA_BLOCK_SIZE, ctxsize: core::mem::size_of::<camellia_sparc64_ctx>(),
    alignmask: 3, min_keysize: CAMELLIA_MIN_KEY_SIZE, max_keysize: CAMELLIA_MAX_KEY_SIZE,
    setkey: Some(camellia_set_key), encrypt: Some(camellia_encrypt), decrypt: Some(camellia_decrypt),
};

// Module initialization/finalization and crop_devid.c integration are retained
// as external kernel integration points.
extern "C" {
    fn camellia_sparc64_mod_init() -> i32;
    fn camellia_sparc64_mod_fini();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
