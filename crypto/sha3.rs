// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Crypto API support for SHA-3
 * (https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf)
 */

use core::ffi::c_void;

// C headers provide these types, constants, and functions.
#[repr(C)]
pub struct shash_desc {
    pub tfm: *mut c_void,
    pub flags: u32,
    pub __ctx: [u8; 0],
}
#[repr(C)]
pub struct sha3_ctx {
    pub _opaque: [u8; 0],
}
#[repr(C)]
pub struct shash_alg {
    pub digestsize: u32,
    pub init: Option<unsafe extern "C" fn(*mut shash_desc) -> i32>,
    pub update: Option<unsafe extern "C" fn(*mut shash_desc, *const u8, u32) -> i32>,
    pub final_: Option<unsafe extern "C" fn(*mut shash_desc, *mut u8) -> i32>,
    pub digest: Option<unsafe extern "C" fn(*mut shash_desc, *const u8, u32, *mut u8) -> i32>,
    pub export_core: Option<unsafe extern "C" fn(*mut shash_desc, *mut c_void) -> i32>,
    pub import_core: Option<unsafe extern "C" fn(*mut shash_desc, *const c_void) -> i32>,
    pub descsize: usize,
    pub base: crypto_alg,
}
#[repr(C)]
pub struct crypto_alg {
    pub cra_name: *const u8,
    pub cra_driver_name: *const u8,
    pub cra_blocksize: u32,
    pub cra_module: *mut c_void,
}

extern "C" {
    fn sha3_224_init(ctx: *mut sha3_ctx);
    fn sha3_256_init(ctx: *mut sha3_ctx);
    fn sha3_384_init(ctx: *mut sha3_ctx);
    fn sha3_512_init(ctx: *mut sha3_ctx);
    fn sha3_update(ctx: *mut sha3_ctx, data: *const u8, len: u32);
    fn sha3_final(ctx: *mut sha3_ctx, out: *mut u8);
    fn sha3_224(data: *const u8, len: u32, out: *mut u8);
    fn sha3_256(data: *const u8, len: u32, out: *mut u8);
    fn sha3_384(data: *const u8, len: u32, out: *mut u8);
    fn sha3_512(data: *const u8, len: u32, out: *mut u8);
    fn crypto_register_shashes(algs: *mut shash_alg, count: usize) -> i32;
    fn crypto_unregister_shashes(algs: *mut shash_alg, count: usize);
}

unsafe fn sha3_ctx_from_desc(desc: *mut shash_desc) -> *mut sha3_ctx {
    (desc as *mut u8).add(core::mem::size_of::<shash_desc>()) as *mut sha3_ctx
}

unsafe extern "C" fn crypto_sha3_224_init(desc: *mut shash_desc) -> i32 {
    sha3_224_init(sha3_ctx_from_desc(desc));
    0
}
unsafe extern "C" fn crypto_sha3_256_init(desc: *mut shash_desc) -> i32 {
    sha3_256_init(sha3_ctx_from_desc(desc));
    0
}
unsafe extern "C" fn crypto_sha3_384_init(desc: *mut shash_desc) -> i32 {
    sha3_384_init(sha3_ctx_from_desc(desc));
    0
}
unsafe extern "C" fn crypto_sha3_512_init(desc: *mut shash_desc) -> i32 {
    sha3_512_init(sha3_ctx_from_desc(desc));
    0
}
unsafe extern "C" fn crypto_sha3_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 {
    sha3_update(sha3_ctx_from_desc(desc), data, len);
    0
}
unsafe extern "C" fn crypto_sha3_final(desc: *mut shash_desc, out: *mut u8) -> i32 {
    sha3_final(sha3_ctx_from_desc(desc), out);
    0
}
unsafe extern "C" fn crypto_sha3_224_digest(_: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { sha3_224(data, len, out); 0 }
unsafe extern "C" fn crypto_sha3_256_digest(_: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { sha3_256(data, len, out); 0 }
unsafe extern "C" fn crypto_sha3_384_digest(_: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { sha3_384(data, len, out); 0 }
unsafe extern "C" fn crypto_sha3_512_digest(_: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 { sha3_512(data, len, out); 0 }

unsafe extern "C" fn crypto_sha3_export_core(desc: *mut shash_desc, out: *mut c_void) -> i32 {
    core::ptr::copy_nonoverlapping(sha3_ctx_from_desc(desc) as *const u8, out as *mut u8, core::mem::size_of::<sha3_ctx>());
    0
}
unsafe extern "C" fn crypto_sha3_import_core(desc: *mut shash_desc, input: *const c_void) -> i32 {
    core::ptr::copy_nonoverlapping(input as *const u8, sha3_ctx_from_desc(desc) as *mut u8, core::mem::size_of::<sha3_ctx>());
    0
}

extern "C" {
    static THIS_MODULE: c_void;
}
const SHA3_224_DIGEST_SIZE: u32 = 28;
const SHA3_256_DIGEST_SIZE: u32 = 32;
const SHA3_384_DIGEST_SIZE: u32 = 48;
const SHA3_512_DIGEST_SIZE: u32 = 64;
const SHA3_224_BLOCK_SIZE: u32 = 144;
const SHA3_256_BLOCK_SIZE: u32 = 136;
const SHA3_384_BLOCK_SIZE: u32 = 104;
const SHA3_512_BLOCK_SIZE: u32 = 72;

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const u8 }; }

static mut algs: [shash_alg; 4] = [
    shash_alg { digestsize: SHA3_224_DIGEST_SIZE, init: Some(crypto_sha3_224_init), update: Some(crypto_sha3_update), final_: Some(crypto_sha3_final), digest: Some(crypto_sha3_224_digest), export_core: Some(crypto_sha3_export_core), import_core: Some(crypto_sha3_import_core), descsize: core::mem::size_of::<sha3_ctx>(), base: crypto_alg { cra_name: cstr!("sha3-224"), cra_driver_name: cstr!("sha3-224-lib"), cra_blocksize: SHA3_224_BLOCK_SIZE, cra_module: core::ptr::addr_of!(THIS_MODULE) as *mut c_void } },
    shash_alg { digestsize: SHA3_256_DIGEST_SIZE, init: Some(crypto_sha3_256_init), update: Some(crypto_sha3_update), final_: Some(crypto_sha3_final), digest: Some(crypto_sha3_256_digest), export_core: Some(crypto_sha3_export_core), import_core: Some(crypto_sha3_import_core), descsize: core::mem::size_of::<sha3_ctx>(), base: crypto_alg { cra_name: cstr!("sha3-256"), cra_driver_name: cstr!("sha3-256-lib"), cra_blocksize: SHA3_256_BLOCK_SIZE, cra_module: core::ptr::addr_of!(THIS_MODULE) as *mut c_void } },
    shash_alg { digestsize: SHA3_384_DIGEST_SIZE, init: Some(crypto_sha3_384_init), update: Some(crypto_sha3_update), final_: Some(crypto_sha3_final), digest: Some(crypto_sha3_384_digest), export_core: Some(crypto_sha3_export_core), import_core: Some(crypto_sha3_import_core), descsize: core::mem::size_of::<sha3_ctx>(), base: crypto_alg { cra_name: cstr!("sha3-384"), cra_driver_name: cstr!("sha3-384-lib"), cra_blocksize: SHA3_384_BLOCK_SIZE, cra_module: core::ptr::addr_of!(THIS_MODULE) as *mut c_void } },
    shash_alg { digestsize: SHA3_512_DIGEST_SIZE, init: Some(crypto_sha3_512_init), update: Some(crypto_sha3_update), final_: Some(crypto_sha3_final), digest: Some(crypto_sha3_512_digest), export_core: Some(crypto_sha3_export_core), import_core: Some(crypto_sha3_import_core), descsize: core::mem::size_of::<sha3_ctx>(), base: crypto_alg { cra_name: cstr!("sha3-512"), cra_driver_name: cstr!("sha3-512-lib"), cra_blocksize: SHA3_512_BLOCK_SIZE, cra_module: core::ptr::addr_of!(THIS_MODULE) as *mut c_void } },
];

unsafe extern "C" fn crypto_sha3_mod_init() -> i32 {
    crypto_register_shashes(algs.as_mut_ptr(), algs.len())
}

unsafe extern "C" fn crypto_sha3_mod_exit() {
    crypto_unregister_shashes(algs.as_mut_ptr(), algs.len());
}

// module_init(crypto_sha3_mod_init); module_exit(crypto_sha3_mod_exit);
// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("Crypto API support for SHA-3");
// MODULE_ALIAS_CRYPTO("sha3-224"); MODULE_ALIAS_CRYPTO("sha3-224-lib");
// MODULE_ALIAS_CRYPTO("sha3-256"); MODULE_ALIAS_CRYPTO("sha3-256-lib");
// MODULE_ALIAS_CRYPTO("sha3-384"); MODULE_ALIAS_CRYPTO("sha3-384-lib");
// MODULE_ALIAS_CRYPTO("sha3-512"); MODULE_ALIAS_CRYPTO("sha3-512-lib");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
