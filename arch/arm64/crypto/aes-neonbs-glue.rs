// SPDX-License-Identifier: GPL-2.0-only
/*
 * Bit sliced AES using NEON instructions
 *
 * Copyright (C) 2016 - 2017 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// Kernel headers and module metadata are supplied by the surrounding kernel Rust environment.

extern "C" {
    fn aesbs_convert_key(out: *mut u8, rk: *const u32, rounds: i32);
    fn aesbs_ecb_encrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32);
    fn aesbs_ecb_decrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32);
    fn aesbs_cbc_decrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32, iv: *mut u8);
    fn aesbs_ctr_encrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32, iv: *mut u8);
    fn aesbs_xts_encrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32, iv: *mut u8);
    fn aesbs_xts_decrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32, iv: *mut u8);
}

#[repr(C, align(16))]
pub struct AesbsCtx {
    pub rk: [u8; 13 * (8 * AES_BLOCK_SIZE) + 32],
    pub rounds: i32,
}

#[repr(C)]
pub struct AesbsCbcCtrCtx {
    pub key: AesbsCtx,
    pub enc: [u32; AES_MAX_KEYLENGTH_U32],
}

#[repr(C)]
pub struct AesbsXtsCtx {
    pub key: AesbsCtx,
    pub twkey: [u32; AES_MAX_KEYLENGTH_U32],
    pub cts: CryptoAesCtx,
}

// External kernel types, constants, and helpers are provided by the surrounding translation unit.
extern "C" {
    fn kmalloc(size: usize, flags: u32) -> *mut CryptoAesCtx;
    fn kfree_sensitive(ptr: *mut CryptoAesCtx);
    fn aes_expandkey(ctx: *mut CryptoAesCtx, key: *const u8, key_len: usize) -> i32;
    fn crypto_skcipher_ctx(tfm: *mut CryptoSkcipher) -> *mut core::ffi::c_void;
    fn crypto_skcipher_reqtfm(req: *mut SkcipherRequest) -> *mut CryptoSkcipher;
    fn skcipher_walk_virt(walk: *mut SkcipherWalk, req: *mut SkcipherRequest, atomic: bool) -> i32;
    fn skcipher_walk_done(walk: *mut SkcipherWalk, nbytes: usize) -> i32;
    fn neon_aes_cbc_encrypt(dst: *mut u8, src: *const u8, enc: *const u32, rounds: i32, blocks: usize, iv: *mut u8);
    fn neon_aes_ctr_encrypt(dst: *mut u8, src: *const u8, enc: *const u32, rounds: i32, nbytes: i32, iv: *mut u8);
    fn neon_aes_ecb_encrypt(dst: *mut u8, src: *const u8, key: *const u32, rounds: i32, blocks: i32);
    fn neon_aes_xts_encrypt(out: *mut u8, input: *const u8, key: *const u32, rounds: i32, nbytes: i32, twkey: *const u32, iv: *mut u8, first: i32);
    fn neon_aes_xts_decrypt(out: *mut u8, input: *const u8, key: *const u32, rounds: i32, nbytes: i32, twkey: *const u32, iv: *mut u8, first: i32);
}

pub const AES_BLOCK_SIZE: usize = 16;
pub const AES_MAX_KEYLENGTH_U32: usize = 60 / 4;
pub const AES_MIN_KEY_SIZE: usize = 16;
pub const AES_MAX_KEY_SIZE: usize = 32;

#[repr(C)] pub struct CryptoAesCtx { pub key_enc: [u32; AES_MAX_KEYLENGTH_U32], pub key_dec: [u32; AES_MAX_KEYLENGTH_U32] }
#[repr(C)] pub struct CryptoSkcipher;
#[repr(C)] pub struct SkcipherRequest { pub cryptlen: usize, pub src: *mut Scatterlist, pub dst: *mut Scatterlist, pub iv: *mut u8 }
#[repr(C)] pub struct Scatterlist;
#[repr(C)] pub struct SkcipherWalk { pub nbytes: usize, pub total: usize, pub stride: usize, pub src: WalkBuffer, pub dst: WalkBuffer, pub iv: *mut u8 }
#[repr(C)] pub struct WalkBuffer { pub virt: VirtAddr }
#[repr(C)] pub struct VirtAddr { pub addr: *mut u8 }

unsafe fn aesbs_setkey(tfm: *mut CryptoSkcipher, in_key: *const u8, key_len: usize) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm) as *mut AesbsCtx;
    let rk = kmalloc(core::mem::size_of::<CryptoAesCtx>(), 0);
    if rk.is_null() { return -12; }
    let mut err = aes_expandkey(rk, in_key, key_len);
    if err == 0 {
        (*ctx).rounds = 6 + (key_len / 4) as i32;
        aesbs_convert_key((*ctx).rk.as_mut_ptr(), (*rk).key_enc.as_ptr(), (*ctx).rounds);
    }
    kfree_sensitive(rk);
    err
}

// Kernel registration and request-walk entry points.
extern "C" {
    fn ecb_encrypt(req: *mut SkcipherRequest) -> i32;
    fn ecb_decrypt(req: *mut SkcipherRequest) -> i32;
    fn cbc_encrypt(req: *mut SkcipherRequest) -> i32;
    fn cbc_decrypt(req: *mut SkcipherRequest) -> i32;
    fn ctr_encrypt(req: *mut SkcipherRequest) -> i32;
    fn xts_encrypt(req: *mut SkcipherRequest) -> i32;
    fn xts_decrypt(req: *mut SkcipherRequest) -> i32;
}

// Module registration: ecb(aes), cbc(aes), ctr(aes), and xts(aes), priority 250.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
