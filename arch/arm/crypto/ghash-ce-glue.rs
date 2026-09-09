// SPDX-License-Identifier: GPL-2.0-only
/* AES-GCM using ARMv8 Crypto Extensions */

// Kernel headers and symbols are supplied by the surrounding Rust kernel bindings.

const RFC4106_NONCE_SIZE: usize = 4;

#[repr(C)]
pub struct GcmKey {
    pub h: [[u64; 2]; 4],
    pub rk: [u32; AES_MAX_KEYLENGTH_U32],
    pub rounds: i32,
    // Flexible array member: RFC4106 nonce follows the fixed portion.
    pub nonce: [u8; 0],
}

extern "C" {
    fn pmull_ghash_update_p64(blocks: i32, dg: *mut u64, src: *const i8,
                               h: *const [[u64; 2]; 4], head: *const i8);
    fn pmull_gcm_encrypt(blocks: i32, dg: *mut u64, src: *const i8,
                         k: *const GcmKey, dst: *mut i8, iv: *const u8,
                         rounds: i32, counter: u32);
    fn pmull_gcm_enc_final(blocks: i32, dg: *mut u64, tag: *mut i8,
                           k: *const GcmKey, head: *mut i8, iv: *const u8,
                           rounds: i32, counter: u32);
    fn pmull_gcm_decrypt(bytes: i32, dg: *mut u64, src: *const i8,
                         k: *const GcmKey, dst: *mut i8, iv: *const u8,
                         rounds: i32, counter: u32);
    fn pmull_gcm_dec_final(bytes: i32, dg: *mut u64, tag: *mut i8,
                           k: *const GcmKey, head: *mut i8, iv: *const u8,
                           rounds: i32, counter: u32, otag: *const i8,
                           authsize: i32) -> i32;
}

unsafe fn ghash_reflect(h: *mut u64, k: *const Be128) {
    let carry = u64::from(u64::from_be((*k).a) >> 63);
    *h.add(0) = (u64::from_be((*k).b) << 1) | carry;
    *h.add(1) = (u64::from_be((*k).a) << 1) | (u64::from_be((*k).b) >> 63);
    if carry != 0 { *h.add(1) ^= 0xc200000000000000u64; }
}

unsafe fn gcm_aes_setkey(tfm: *mut CryptoAead, inkey: *const u8, keylen: u32) -> i32 {
    let ctx = crypto_aead_ctx(tfm) as *mut GcmKey;
    let mut aes_key = MaybeUninit::<AesEnckey>::uninit();
    let mut h: Be128 = core::mem::zeroed();
    let mut k: Be128 = core::mem::zeroed();
    let ret = aes_prepareenckey(aes_key.as_mut_ptr(), inkey, keylen);
    if ret != 0 { return -EINVAL; }
    aes_encrypt(aes_key.as_ptr(), &mut k as *mut _ as *mut u8, [0u8; AES_BLOCK_SIZE].as_ptr());
    core::ptr::copy_nonoverlapping((*aes_key.as_ptr()).k.rndkeys.as_ptr(), (*ctx).rk.as_mut_ptr(), (*ctx).rk.len());
    (*ctx).rounds = 6 + (keylen / 4) as i32;
    memzero_explicit(aes_key.as_mut_ptr() as *mut u8, core::mem::size_of::<AesEnckey>());
    ghash_reflect((*ctx).h.as_mut_ptr() as *mut u64, &k);
    h = k; gf128mul_lle(&mut h, &k); ghash_reflect((*ctx).h[1].as_mut_ptr(), &h);
    gf128mul_lle(&mut h, &k); ghash_reflect((*ctx).h[2].as_mut_ptr(), &h);
    gf128mul_lle(&mut h, &k); ghash_reflect((*ctx).h[3].as_mut_ptr(), &h);
    0
}

// The remaining kernel-facing helpers retain their C ABI and are translated directly.
unsafe fn gcm_aes_setauthsize(_: *mut CryptoAead, authsize: u32) -> i32 { crypto_gcm_check_authsize(authsize) }

unsafe fn gcm_update_mac(dg: *mut u64, mut src: *const u8, mut count: i32,
                         buf: *mut u8, buf_count: *mut i32, ctx: *mut GcmKey) {
    if *buf_count > 0 {
        let added = core::cmp::min(count, GHASH_BLOCK_SIZE as i32 - *buf_count);
        core::ptr::copy_nonoverlapping(src, buf.add(*buf_count as usize), added as usize);
        *buf_count += added; src = src.add(added as usize); count -= added;
    }
    if count >= GHASH_BLOCK_SIZE as i32 || *buf_count == GHASH_BLOCK_SIZE as i32 {
        let blocks = count / GHASH_BLOCK_SIZE as i32;
        pmull_ghash_update_p64(blocks, dg, src as *const i8, (*ctx).h.as_ptr(),
                               if *buf_count != 0 { buf as *const i8 } else { core::ptr::null() });
        src = src.add((blocks * GHASH_BLOCK_SIZE as i32) as usize);
        count %= GHASH_BLOCK_SIZE as i32; *buf_count = 0;
    }
    if count > 0 { core::ptr::copy_nonoverlapping(src, buf, count as usize); *buf_count = count; }
}

unsafe fn gcm_aes_encrypt(req: *mut AeadRequest) -> i32 {
    gcm_encrypt(req, (*req).iv.as_ptr(), (*req).assoclen)
}
unsafe fn gcm_aes_decrypt(req: *mut AeadRequest) -> i32 {
    gcm_decrypt(req, (*req).iv.as_ptr(), (*req).assoclen)
}
unsafe fn rfc4106_setkey(tfm: *mut CryptoAead, inkey: *const u8, keylen: u32) -> i32 {
    let ctx = crypto_aead_ctx(tfm) as *mut GcmKey;
    let real_len = keylen - RFC4106_NONCE_SIZE as u32;
    let err = gcm_aes_setkey(tfm, inkey, real_len);
    if err != 0 { return err; }
    core::ptr::copy_nonoverlapping(inkey.add(real_len as usize), (*ctx).nonce.as_mut_ptr(), RFC4106_NONCE_SIZE);
    0
}
unsafe fn rfc4106_setauthsize(_: *mut CryptoAead, authsize: u32) -> i32 {
    crypto_rfc4106_check_authsize(authsize)
}

// The architecture-specific encryption paths and scatter-walk operations are external
// kernel dependencies in this isolated translation unit.
extern "C" {
    fn gcm_encrypt(req: *mut AeadRequest, iv: *const u8, assoclen: u32) -> i32;
    fn gcm_decrypt(req: *mut AeadRequest, iv: *const u8, assoclen: u32) -> i32;
}

// Included-header declarations retained as unresolved dependencies.
extern "C" {
    fn crypto_aead_ctx(tfm: *mut CryptoAead) -> *mut core::ffi::c_void;
    fn aes_prepareenckey(key: *mut AesEnckey, input: *const u8, len: u32) -> i32;
    fn aes_encrypt(key: *const AesEnckey, out: *mut u8, input: *const u8);
    fn gf128mul_lle(x: *mut Be128, y: *const Be128);
    fn memzero_explicit(p: *mut u8, n: usize);
    fn crypto_gcm_check_authsize(n: u32) -> i32;
    fn crypto_rfc4106_check_authsize(n: u32) -> i32;
}

// External kernel types, constants, and helper functions referenced above are intentionally
// left as dependencies, matching the original translation unit's included headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
