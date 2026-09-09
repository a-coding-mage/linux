// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Glue Code for 3-way parallel assembler optimized version of Twofish
 *
 * Copyright (c) 2011 Jussi Kivilinna <jussi.kivilinna@mbnet.fi>
 */

// C dependencies supplied by the surrounding kernel translation.

extern "C" {
    pub fn __twofish_enc_blk_3way(ctx: *const core::ffi::c_void, dst: *mut u8,
                                   src: *const u8, decrypt: bool);
    pub fn twofish_dec_blk_3way(ctx: *const core::ffi::c_void, dst: *mut u8,
                                src: *const u8);
    fn twofish_setkey(base: *mut crypto_tfm, key: *const u8, keylen: u32) -> i32;
    fn twofish_enc_blk(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn twofish_dec_blk(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize)
        -> *mut core::ffi::c_void;
    fn crypto_xor(dst: *mut u8, src: *const u8, n: usize);
    fn crypto_register_skciphers(algs: *mut skcipher_alg, n: usize) -> i32;
    fn crypto_unregister_skciphers(algs: *mut skcipher_alg, n: usize);
    fn printk(level: *const u8, fmt: *const u8, ...);
}

type u8_ = u8;

#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_skcipher {
    pub base: crypto_tfm,
}

#[repr(C)]
pub struct skcipher_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct twofish_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_alg {
    pub cra_name: *const u8,
    pub cra_driver_name: *const u8,
    pub cra_priority: u32,
    pub cra_blocksize: u32,
    pub cra_ctxsize: usize,
    pub cra_module: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct skcipher_alg {
    pub base: crypto_alg,
    pub min_keysize: u32,
    pub max_keysize: u32,
    pub ivsize: u32,
    pub setkey: Option<unsafe extern "C" fn(*mut crypto_skcipher, *const u8, u32) -> i32>,
    pub encrypt: Option<unsafe extern "C" fn(*mut skcipher_request) -> i32>,
    pub decrypt: Option<unsafe extern "C" fn(*mut skcipher_request) -> i32>,
}

const TF_BLOCK_SIZE: usize = 16;
const TF_MIN_KEY_SIZE: u32 = 16;
const TF_MAX_KEY_SIZE: u32 = 32;
const X86_VENDOR_INTEL: u32 = 0;
const INTEL_ATOM_BONNELL: u32 = 0;
const INTEL_ATOM_BONNELL_MID: u32 = 1;
const INTEL_ATOM_SALTWELL: u32 = 2;
const ENODEV: i32 = 19;

#[repr(C)]
struct BootCpuData {
    x86_vendor: u32,
    x86_vfm: u32,
    x86: u32,
}

extern "C" {
    static boot_cpu_data: BootCpuData;
}

static mut force: i32 = 0;

unsafe extern "C" fn twofish_setkey_skcipher(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    twofish_setkey(&mut (*tfm).base, key, keylen)
}

#[inline]
unsafe fn twofish_enc_blk_3way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8) {
    __twofish_enc_blk_3way(ctx, dst, src, false);
}

#[no_mangle]
pub unsafe extern "C" fn twofish_dec_blk_cbc_3way(
    ctx: *const core::ffi::c_void,
    dst: *mut u8,
    src: *const u8,
) {
    let mut buf = [[0u8; TF_BLOCK_SIZE]; 2];
    let mut s = src;

    if dst == src {
        s = memcpy(buf.as_mut_ptr() as *mut core::ffi::c_void,
                   src as *const core::ffi::c_void, core::mem::size_of_val(&buf)) as *const u8;
    }
    twofish_dec_blk_3way(ctx, dst, src);
    crypto_xor(dst.add(TF_BLOCK_SIZE), s, core::mem::size_of_val(&buf));
}

unsafe extern "C" fn ecb_encrypt(req: *mut skcipher_request) -> i32 {
    // ECB_WALK_START(req, TF_BLOCK_SIZE, -1);
    // ECB_BLOCK(3, twofish_enc_blk_3way);
    // ECB_BLOCK(1, twofish_enc_blk);
    // ECB_WALK_END();
    ecb_walk_start(req, TF_BLOCK_SIZE, -1);
    ecb_block(3, twofish_enc_blk_3way);
    ecb_block(1, twofish_enc_blk);
    ecb_walk_end()
}

unsafe extern "C" fn ecb_decrypt(req: *mut skcipher_request) -> i32 {
    ecb_walk_start(req, TF_BLOCK_SIZE, -1);
    ecb_block(3, twofish_dec_blk_3way);
    ecb_block(1, twofish_dec_blk);
    ecb_walk_end()
}

unsafe extern "C" fn cbc_encrypt(req: *mut skcipher_request) -> i32 {
    cbc_walk_start(req, TF_BLOCK_SIZE, -1);
    cbc_enc_block(twofish_enc_blk);
    cbc_walk_end()
}

unsafe extern "C" fn cbc_decrypt(req: *mut skcipher_request) -> i32 {
    cbc_walk_start(req, TF_BLOCK_SIZE, -1);
    cbc_dec_block(3, twofish_dec_blk_cbc_3way);
    cbc_dec_block(1, twofish_dec_blk);
    cbc_walk_end()
}

extern "C" {
    fn ecb_walk_start(req: *mut skcipher_request, blocksize: usize, n: i32);
    fn ecb_block(n: i32, f: unsafe fn(*const core::ffi::c_void, *mut u8, *const u8));
    fn ecb_walk_end() -> i32;
    fn cbc_walk_start(req: *mut skcipher_request, blocksize: usize, n: i32);
    fn cbc_enc_block(f: unsafe fn(*const core::ffi::c_void, *mut u8, *const u8));
    fn cbc_dec_block(n: i32, f: unsafe fn(*const core::ffi::c_void, *mut u8, *const u8));
    fn cbc_walk_end() -> i32;
}

static mut tf_skciphers: [skcipher_alg; 2] = [
    skcipher_alg {
        base: crypto_alg { cra_name: b"ecb(twofish)\0".as_ptr(), cra_driver_name: b"ecb-twofish-3way\0".as_ptr(), cra_priority: 300, cra_blocksize: TF_BLOCK_SIZE as u32, cra_ctxsize: core::mem::size_of::<twofish_ctx>(), cra_module: core::ptr::null_mut() },
        min_keysize: TF_MIN_KEY_SIZE, max_keysize: TF_MAX_KEY_SIZE, ivsize: 0,
        setkey: Some(twofish_setkey_skcipher), encrypt: Some(ecb_encrypt), decrypt: Some(ecb_decrypt),
    },
    skcipher_alg {
        base: crypto_alg { cra_name: b"cbc(twofish)\0".as_ptr(), cra_driver_name: b"cbc-twofish-3way\0".as_ptr(), cra_priority: 300, cra_blocksize: TF_BLOCK_SIZE as u32, cra_ctxsize: core::mem::size_of::<twofish_ctx>(), cra_module: core::ptr::null_mut() },
        min_keysize: TF_MIN_KEY_SIZE, max_keysize: TF_MAX_KEY_SIZE, ivsize: TF_BLOCK_SIZE as u32,
        setkey: Some(twofish_setkey_skcipher), encrypt: Some(cbc_encrypt), decrypt: Some(cbc_decrypt),
    },
];

unsafe fn is_blacklisted_cpu() -> bool {
    if boot_cpu_data.x86_vendor != X86_VENDOR_INTEL { return false; }
    match boot_cpu_data.x86_vfm {
        INTEL_ATOM_BONNELL | INTEL_ATOM_BONNELL_MID | INTEL_ATOM_SALTWELL => return true,
        _ => {}
    }
    if boot_cpu_data.x86 == 0x0f { return true; }
    false
}

unsafe extern "C" fn twofish_3way_init() -> i32 {
    if force == 0 && is_blacklisted_cpu() { return -ENODEV; }
    crypto_register_skciphers(tf_skciphers.as_mut_ptr(), tf_skciphers.len())
}

unsafe extern "C" fn twofish_3way_fini() {
    crypto_unregister_skciphers(tf_skciphers.as_mut_ptr(), tf_skciphers.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
