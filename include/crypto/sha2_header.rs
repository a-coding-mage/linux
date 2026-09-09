/* SPDX-License-Identifier: GPL-2.0 */
/* Common values for SHA-2 algorithms. */

/* `u8`, `u32`, `u64`, `size_t`, and `bool` are supplied by the including crate. */

pub const SHA224_DIGEST_SIZE: usize = 28;
pub const SHA224_BLOCK_SIZE: usize = 64;
pub const SHA256_DIGEST_SIZE: usize = 32;
pub const SHA256_BLOCK_SIZE: usize = 64;
pub const SHA256_STATE_WORDS: usize = 8;
pub const SHA384_DIGEST_SIZE: usize = 48;
pub const SHA384_BLOCK_SIZE: usize = 128;
pub const SHA512_DIGEST_SIZE: usize = 64;
pub const SHA512_BLOCK_SIZE: usize = 128;
pub const SHA512_STATE_SIZE: usize = 80;

pub const SHA224_H0: u32 = 0xc1059ed8;
pub const SHA224_H1: u32 = 0x367cd507;
pub const SHA224_H2: u32 = 0x3070dd17;
pub const SHA224_H3: u32 = 0xf70e5939;
pub const SHA224_H4: u32 = 0xffc00b31;
pub const SHA224_H5: u32 = 0x68581511;
pub const SHA224_H6: u32 = 0x64f98fa7;
pub const SHA224_H7: u32 = 0xbefa4fa4;
pub const SHA256_H0: u32 = 0x6a09e667;
pub const SHA256_H1: u32 = 0xbb67ae85;
pub const SHA256_H2: u32 = 0x3c6ef372;
pub const SHA256_H3: u32 = 0xa54ff53a;
pub const SHA256_H4: u32 = 0x510e527f;
pub const SHA256_H5: u32 = 0x9b05688c;
pub const SHA256_H6: u32 = 0x1f83d9ab;
pub const SHA256_H7: u32 = 0x5be0cd19;
pub const SHA384_H0: u64 = 0xcbbb9d5dc1059ed8;
pub const SHA384_H1: u64 = 0x629a292a367cd507;
pub const SHA384_H2: u64 = 0x9159015a3070dd17;
pub const SHA384_H3: u64 = 0x152fecd8f70e5939;
pub const SHA384_H4: u64 = 0x67332667ffc00b31;
pub const SHA384_H5: u64 = 0x8eb44a8768581511;
pub const SHA384_H6: u64 = 0xdb0c2e0d64f98fa7;
pub const SHA384_H7: u64 = 0x47b5481dbefa4fa4;
pub const SHA512_H0: u64 = 0x6a09e667f3bcc908;
pub const SHA512_H1: u64 = 0xbb67ae8584caa73b;
pub const SHA512_H2: u64 = 0x3c6ef372fe94f82b;
pub const SHA512_H3: u64 = 0xa54ff53a5f1d36f1;
pub const SHA512_H4: u64 = 0x510e527fade682d1;
pub const SHA512_H5: u64 = 0x9b05688c2b3e6c1f;
pub const SHA512_H6: u64 = 0x1f83d9abfb41bd6b;
pub const SHA512_H7: u64 = 0x5be0cd19137e2179;

extern "C" {
    pub static sha224_zero_message_hash: [u8; SHA224_DIGEST_SIZE];
    pub static sha256_zero_message_hash: [u8; SHA256_DIGEST_SIZE];
    pub static sha384_zero_message_hash: [u8; SHA384_DIGEST_SIZE];
    pub static sha512_zero_message_hash: [u8; SHA512_DIGEST_SIZE];
}

#[repr(C)]
pub struct crypto_sha256_state { pub state: [u32; SHA256_STATE_WORDS], pub count: u64 }

#[inline]
pub unsafe fn sha224_block_init(sctx: *mut crypto_sha256_state) {
    (*sctx).state = [SHA224_H0, SHA224_H1, SHA224_H2, SHA224_H3, SHA224_H4, SHA224_H5, SHA224_H6, SHA224_H7];
    (*sctx).count = 0;
}
#[inline]
pub unsafe fn sha256_block_init(sctx: *mut crypto_sha256_state) {
    (*sctx).state = [SHA256_H0, SHA256_H1, SHA256_H2, SHA256_H3, SHA256_H4, SHA256_H5, SHA256_H6, SHA256_H7];
    (*sctx).count = 0;
}

#[repr(C)]
pub struct sha256_state { pub ctx: crypto_sha256_state, pub buf: [u8; SHA256_BLOCK_SIZE] }
#[repr(C)]
pub struct sha512_state { pub state: [u64; SHA512_DIGEST_SIZE / 8], pub count: [u64; 2], pub buf: [u8; SHA512_BLOCK_SIZE] }
#[repr(C)]
pub struct sha256_block_state { pub h: [u32; SHA256_STATE_WORDS] }
#[repr(C, align(8))]
pub struct __sha256_ctx { pub state: sha256_block_state, pub bytecount: u64, pub buf: [u8; SHA256_BLOCK_SIZE] }
#[repr(C)]
pub struct __hmac_sha256_key { pub istate: sha256_block_state, pub ostate: sha256_block_state }
#[repr(C)]
pub struct __hmac_sha256_ctx { pub sha_ctx: __sha256_ctx, pub ostate: sha256_block_state }
#[repr(C)] pub struct sha224_ctx { pub ctx: __sha256_ctx }
#[repr(C)] pub struct hmac_sha224_key { pub key: __hmac_sha256_key }
#[repr(C)] pub struct hmac_sha224_ctx { pub ctx: __hmac_sha256_ctx }
#[repr(C)] pub struct sha256_ctx { pub ctx: __sha256_ctx }
#[repr(C)] pub struct hmac_sha256_key { pub key: __hmac_sha256_key }
#[repr(C)] pub struct hmac_sha256_ctx { pub ctx: __hmac_sha256_ctx }

#[repr(C)] pub struct sha512_block_state { pub h: [u64; 8] }
#[repr(C, align(8))]
pub struct __sha512_ctx { pub state: sha512_block_state, pub bytecount_lo: u64, pub bytecount_hi: u64, pub buf: [u8; SHA512_BLOCK_SIZE] }
#[repr(C)] pub struct __hmac_sha512_key { pub istate: sha512_block_state, pub ostate: sha512_block_state }
#[repr(C)] pub struct __hmac_sha512_ctx { pub sha_ctx: __sha512_ctx, pub ostate: sha512_block_state }
#[repr(C)] pub struct sha384_ctx { pub ctx: __sha512_ctx }
#[repr(C)] pub struct hmac_sha384_key { pub key: __hmac_sha512_key }
#[repr(C)] pub struct hmac_sha384_ctx { pub ctx: __hmac_sha512_ctx }
#[repr(C)] pub struct sha512_ctx { pub ctx: __sha512_ctx }
#[repr(C)] pub struct hmac_sha512_key { pub key: __hmac_sha512_key }
#[repr(C)] pub struct hmac_sha512_ctx { pub ctx: __hmac_sha512_ctx }

extern "C" {
    pub fn __sha256_update(ctx: *mut __sha256_ctx, data: *const u8, len: usize);
    pub fn __hmac_sha256_init(ctx: *mut __hmac_sha256_ctx, key: *const __hmac_sha256_key);
    pub fn sha224_init(ctx: *mut sha224_ctx);
    pub fn sha224_final(ctx: *mut sha224_ctx, out: *mut u8);
    pub fn sha224(data: *const u8, len: usize, out: *mut u8);
    pub fn hmac_sha224_preparekey(key: *mut hmac_sha224_key, raw_key: *const u8, raw_key_len: usize);
    pub fn hmac_sha224_init_usingrawkey(ctx: *mut hmac_sha224_ctx, raw_key: *const u8, raw_key_len: usize);
    pub fn hmac_sha224_final(ctx: *mut hmac_sha224_ctx, out: *mut u8);
    pub fn hmac_sha224(key: *const hmac_sha224_key, data: *const u8, data_len: usize, out: *mut u8);
    pub fn hmac_sha224_usingrawkey(raw_key: *const u8, raw_key_len: usize, data: *const u8, data_len: usize, out: *mut u8);
    pub fn sha256_init(ctx: *mut sha256_ctx);
    pub fn sha256_final(ctx: *mut sha256_ctx, out: *mut u8);
    pub fn sha256(data: *const u8, len: usize, out: *mut u8);
    pub fn sha256_finup_2x(ctx: *const sha256_ctx, data1: *const u8, data2: *const u8, len: usize, out1: *mut u8, out2: *mut u8);
    pub fn sha256_finup_2x_is_optimized() -> bool;
    pub fn hmac_sha256_preparekey(key: *mut hmac_sha256_key, raw_key: *const u8, raw_key_len: usize);
    pub fn hmac_sha256_init_usingrawkey(ctx: *mut hmac_sha256_ctx, raw_key: *const u8, raw_key_len: usize);
    pub fn hmac_sha256_final(ctx: *mut hmac_sha256_ctx, out: *mut u8);
    pub fn hmac_sha256(key: *const hmac_sha256_key, data: *const u8, data_len: usize, out: *mut u8);
    pub fn hmac_sha256_usingrawkey(raw_key: *const u8, raw_key_len: usize, data: *const u8, data_len: usize, out: *mut u8);
    pub fn __sha512_update(ctx: *mut __sha512_ctx, data: *const u8, len: usize);
    pub fn __hmac_sha512_init(ctx: *mut __hmac_sha512_ctx, key: *const __hmac_sha512_key);
    pub fn sha384_init(ctx: *mut sha384_ctx);
    pub fn sha384_final(ctx: *mut sha384_ctx, out: *mut u8);
    pub fn sha384(data: *const u8, len: usize, out: *mut u8);
    pub fn hmac_sha384_preparekey(key: *mut hmac_sha384_key, raw_key: *const u8, raw_key_len: usize);
    pub fn hmac_sha384_init_usingrawkey(ctx: *mut hmac_sha384_ctx, raw_key: *const u8, raw_key_len: usize);
    pub fn hmac_sha384_final(ctx: *mut hmac_sha384_ctx, out: *mut u8);
    pub fn hmac_sha384(key: *const hmac_sha384_key, data: *const u8, data_len: usize, out: *mut u8);
    pub fn hmac_sha384_usingrawkey(raw_key: *const u8, raw_key_len: usize, data: *const u8, data_len: usize, out: *mut u8);
    pub fn sha512_init(ctx: *mut sha512_ctx);
    pub fn sha512_final(ctx: *mut sha512_ctx, out: *mut u8);
    pub fn sha512(data: *const u8, len: usize, out: *mut u8);
    pub fn hmac_sha512_preparekey(key: *mut hmac_sha512_key, raw_key: *const u8, raw_key_len: usize);
    pub fn hmac_sha512_init_usingrawkey(ctx: *mut hmac_sha512_ctx, raw_key: *const u8, raw_key_len: usize);
    pub fn hmac_sha512_final(ctx: *mut hmac_sha512_ctx, out: *mut u8);
    pub fn hmac_sha512(key: *const hmac_sha512_key, data: *const u8, data_len: usize, out: *mut u8);
    pub fn hmac_sha512_usingrawkey(raw_key: *const u8, raw_key_len: usize, data: *const u8, data_len: usize, out: *mut u8);
}

#[inline] pub unsafe fn sha224_update(ctx: *mut sha224_ctx, data: *const u8, len: usize) { __sha256_update(&mut (*ctx).ctx, data, len); }
#[inline] pub unsafe fn hmac_sha224_init(ctx: *mut hmac_sha224_ctx, key: *const hmac_sha224_key) { __hmac_sha256_init(&mut (*ctx).ctx, &(*key).key); }
#[inline] pub unsafe fn hmac_sha224_update(ctx: *mut hmac_sha224_ctx, data: *const u8, data_len: usize) { __sha256_update(&mut (*ctx).ctx.sha_ctx, data, data_len); }
#[inline] pub unsafe fn sha256_update(ctx: *mut sha256_ctx, data: *const u8, len: usize) { __sha256_update(&mut (*ctx).ctx, data, len); }
#[inline] pub unsafe fn hmac_sha256_init(ctx: *mut hmac_sha256_ctx, key: *const hmac_sha256_key) { __hmac_sha256_init(&mut (*ctx).ctx, &(*key).key); }
#[inline] pub unsafe fn hmac_sha256_update(ctx: *mut hmac_sha256_ctx, data: *const u8, data_len: usize) { __sha256_update(&mut (*ctx).ctx.sha_ctx, data, data_len); }
#[inline] pub unsafe fn sha384_update(ctx: *mut sha384_ctx, data: *const u8, len: usize) { __sha512_update(&mut (*ctx).ctx, data, len); }
#[inline] pub unsafe fn hmac_sha384_init(ctx: *mut hmac_sha384_ctx, key: *const hmac_sha384_key) { __hmac_sha512_init(&mut (*ctx).ctx, &(*key).key); }
#[inline] pub unsafe fn hmac_sha384_update(ctx: *mut hmac_sha384_ctx, data: *const u8, data_len: usize) { __sha512_update(&mut (*ctx).ctx.sha_ctx, data, data_len); }
#[inline] pub unsafe fn sha512_update(ctx: *mut sha512_ctx, data: *const u8, len: usize) { __sha512_update(&mut (*ctx).ctx, data, len); }
#[inline] pub unsafe fn hmac_sha512_init(ctx: *mut hmac_sha512_ctx, key: *const hmac_sha512_key) { __hmac_sha512_init(&mut (*ctx).ctx, &(*key).key); }
#[inline] pub unsafe fn hmac_sha512_update(ctx: *mut hmac_sha512_ctx, data: *const u8, data_len: usize) { __sha512_update(&mut (*ctx).ctx.sha_ctx, data, data_len); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
