// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the kernel crypto, module, unaligned-access, and
// xxhash subsystems are referenced here but not implemented in this file.

pub const XXHASH64_BLOCK_SIZE: usize = 32;
pub const XXHASH64_DIGEST_SIZE: usize = 8;

#[repr(C)]
pub struct xxhash64_tfm_ctx {
    pub seed: u64,
}

#[repr(C)]
pub struct xxhash64_desc_ctx {
    pub xxhstate: xxh64_state,
}

#[repr(C)]
pub struct xxh64_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_shash {
    _private: [u8; 0],
}

#[repr(C)]
pub struct shash_desc {
    pub tfm: *mut crypto_shash,
    _private: [u8; 0],
}

extern "C" {
    fn crypto_shash_ctx(tfm: *mut crypto_shash) -> *mut xxhash64_tfm_ctx;
    fn shash_desc_ctx(desc: *mut shash_desc) -> *mut xxhash64_desc_ctx;
    fn xxh64_reset(state: *mut xxh64_state, seed: u64);
    fn xxh64_update(state: *mut xxh64_state, input: *const u8, length: usize);
    fn xxh64_digest(state: *const xxh64_state) -> u64;
    fn xxh64(input: *const u8, length: usize, seed: u64) -> u64;
    fn crypto_register_shash(alg: *mut shash_alg) -> i32;
    fn crypto_unregister_shash(alg: *mut shash_alg);
}

#[inline]
unsafe fn get_unaligned_le64(ptr: *const u8) -> u64 {
    u64::from_le_bytes(*(ptr as *const [u8; 8]))
}

#[inline]
unsafe fn put_unaligned_le64(value: u64, ptr: *mut u8) {
    *(ptr as *mut [u8; 8]) = value.to_le_bytes();
}

unsafe fn xxhash64_setkey(
    tfm: *mut crypto_shash,
    key: *const u8,
    keylen: u32,
) -> i32 {
    let tctx = crypto_shash_ctx(tfm);

    if keylen as usize != core::mem::size_of::<u64>() {
        return -22; // -EINVAL
    }
    (*tctx).seed = get_unaligned_le64(key);
    0
}

unsafe fn xxhash64_init(desc: *mut shash_desc) -> i32 {
    let tctx = crypto_shash_ctx((*desc).tfm);
    let dctx = shash_desc_ctx(desc);

    xxh64_reset(&mut (*dctx).xxhstate, (*tctx).seed);

    0
}

unsafe fn xxhash64_update(
    desc: *mut shash_desc,
    data: *const u8,
    length: u32,
) -> i32 {
    let dctx = shash_desc_ctx(desc);

    xxh64_update(&mut (*dctx).xxhstate, data, length as usize);

    0
}

unsafe fn xxhash64_final(desc: *mut shash_desc, out: *mut u8) -> i32 {
    let dctx = shash_desc_ctx(desc);

    put_unaligned_le64(xxh64_digest(&(*dctx).xxhstate), out);

    0
}

unsafe fn xxhash64_digest(
    desc: *mut shash_desc,
    data: *const u8,
    length: u32,
    out: *mut u8,
) -> i32 {
    let tctx = crypto_shash_ctx((*desc).tfm);

    put_unaligned_le64(xxh64(data, length as usize, (*tctx).seed), out);

    0
}

#[repr(C)]
pub struct shash_alg {
    pub digestsize: usize,
    pub setkey: Option<unsafe extern "C" fn(*mut crypto_shash, *const u8, u32) -> i32>,
    pub init: Option<unsafe extern "C" fn(*mut shash_desc) -> i32>,
    pub update: Option<unsafe extern "C" fn(*mut shash_desc, *const u8, u32) -> i32>,
    pub final_: Option<unsafe extern "C" fn(*mut shash_desc, *mut u8) -> i32>,
    pub digest: Option<unsafe extern "C" fn(*mut shash_desc, *const u8, u32, *mut u8) -> i32>,
    pub descsize: usize,
    pub base: crypto_alg,
}

#[repr(C)]
pub struct crypto_alg {
    pub cra_name: *const core::ffi::c_char,
    pub cra_driver_name: *const core::ffi::c_char,
    pub cra_priority: i32,
    pub cra_flags: u32,
    pub cra_blocksize: usize,
    pub cra_ctxsize: usize,
    pub cra_module: *mut core::ffi::c_void,
}

// CRYPTO_ALG_OPTIONAL_KEY and THIS_MODULE are supplied by the kernel headers.
pub const CRYPTO_ALG_OPTIONAL_KEY: u32 = 0;

#[no_mangle]
pub static mut alg: shash_alg = shash_alg {
    digestsize: XXHASH64_DIGEST_SIZE,
    setkey: Some(xxhash64_setkey),
    init: Some(xxhash64_init),
    update: Some(xxhash64_update),
    final_: Some(xxhash64_final),
    digest: Some(xxhash64_digest),
    descsize: core::mem::size_of::<xxhash64_desc_ctx>(),
    base: crypto_alg {
        cra_name: b"xxhash64\0".as_ptr() as *const core::ffi::c_char,
        cra_driver_name: b"xxhash64-generic\0".as_ptr() as *const core::ffi::c_char,
        cra_priority: 100,
        cra_flags: CRYPTO_ALG_OPTIONAL_KEY,
        cra_blocksize: XXHASH64_BLOCK_SIZE,
        cra_ctxsize: core::mem::size_of::<xxhash64_tfm_ctx>(),
        cra_module: core::ptr::null_mut(),
    },
};

unsafe fn xxhash_mod_init() -> i32 {
    crypto_register_shash(&mut alg)
}

unsafe fn xxhash_mod_fini() {
    crypto_unregister_shash(&mut alg);
}

// module_init(xxhash_mod_init);
// module_exit(xxhash_mod_fini);
// MODULE_AUTHOR("Nikolay Borisov <nborisov@suse.com>");
// MODULE_DESCRIPTION("xxhash calculations wrapper for lib/xxhash.c");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_CRYPTO("xxhash64");
// MODULE_ALIAS_CRYPTO("xxhash64-generic");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
