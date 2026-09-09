/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common values for SHA-3 algorithms
 *
 * See also Documentation/crypto/sha3.rst
 */

// linux/types.h and linux/string.h dependencies are represented by the
// corresponding Rust types and the external zeroization routine below.

pub const SHA3_224_DIGEST_SIZE: usize = 224 / 8;
pub const SHA3_224_BLOCK_SIZE: usize = 200 - 2 * SHA3_224_DIGEST_SIZE;
pub const SHA3_224_EXPORT_SIZE: usize = SHA3_STATE_SIZE + SHA3_224_BLOCK_SIZE + 1;

pub const SHA3_256_DIGEST_SIZE: usize = 256 / 8;
pub const SHA3_256_BLOCK_SIZE: usize = 200 - 2 * SHA3_256_DIGEST_SIZE;
pub const SHA3_256_EXPORT_SIZE: usize = SHA3_STATE_SIZE + SHA3_256_BLOCK_SIZE + 1;

pub const SHA3_384_DIGEST_SIZE: usize = 384 / 8;
pub const SHA3_384_BLOCK_SIZE: usize = 200 - 2 * SHA3_384_DIGEST_SIZE;
pub const SHA3_384_EXPORT_SIZE: usize = SHA3_STATE_SIZE + SHA3_384_BLOCK_SIZE + 1;

pub const SHA3_512_DIGEST_SIZE: usize = 512 / 8;
pub const SHA3_512_BLOCK_SIZE: usize = 200 - 2 * SHA3_512_DIGEST_SIZE;
pub const SHA3_512_EXPORT_SIZE: usize = SHA3_STATE_SIZE + SHA3_512_BLOCK_SIZE + 1;

/*
 * SHAKE128 and SHAKE256 actually have variable output size, but this is used to
 * calculate the block size (rate) analogously to the above.
 */
pub const SHAKE128_DEFAULT_SIZE: usize = 128 / 8;
pub const SHAKE128_BLOCK_SIZE: usize = 200 - 2 * SHAKE128_DEFAULT_SIZE;
pub const SHAKE256_DEFAULT_SIZE: usize = 256 / 8;
pub const SHAKE256_BLOCK_SIZE: usize = 200 - 2 * SHAKE256_DEFAULT_SIZE;

pub const SHA3_STATE_SIZE: usize = 200;

/*
 * State for the Keccak-f[1600] permutation: 25 64-bit words.
 *
 * We usually keep the state words as little-endian, to make absorbing and
 * squeezing easier.  (It means that absorbing and squeezing can just treat the
 * state as a byte array.)  The state words are converted to native-endian only
 * temporarily by implementations of the permutation that need native-endian
 * words.  Of course, that conversion is a no-op on little-endian machines.
 */
#[repr(C)]
pub union Sha3StateWords {
    pub words: [u64; SHA3_STATE_SIZE / 8],
    pub bytes: [u8; SHA3_STATE_SIZE],
    /* see comment above */
    pub native_words: [u64; SHA3_STATE_SIZE / 8],
}

#[repr(C)]
pub struct sha3_state {
    pub words: Sha3StateWords,
}

/* Internal context, shared by the digests (SHA3-*) and the XOFs (SHAKE*) */
#[repr(C)]
pub struct __sha3_ctx {
    pub state: sha3_state,
    /* Digests only: the digest size in bytes */
    pub digest_size: u8,
    /* Block size in bytes */
    pub block_size: u8,
    /* Index of next state byte to absorb into */
    pub absorb_offset: u8,
    /* XOFs only: index of next state byte to extract */
    pub squeeze_offset: u8,
}

extern "C" {
    pub fn __sha3_update(ctx: *mut __sha3_ctx, input: *const u8, in_len: usize);
    pub fn memzero_explicit(ptr: *mut core::ffi::c_void, n: usize);
}

/**
 * struct sha3_ctx - Context for SHA3-224, SHA3-256, SHA3-384, or SHA3-512
 * @ctx: private
 */
#[repr(C)]
pub struct sha3_ctx {
    pub ctx: __sha3_ctx,
}

/**
 * sha3_zeroize_ctx() - Zeroize a SHA-3 context
 * @ctx: The context to zeroize
 *
 * This is already called by sha3_final().  Call this explicitly when abandoning
 * a context without calling sha3_final().
 */
pub unsafe fn sha3_zeroize_ctx(ctx: *mut sha3_ctx) {
    memzero_explicit(ctx.cast(), core::mem::size_of::<sha3_ctx>());
}

/**
 * struct shake_ctx - Context for SHAKE128 or SHAKE256
 * @ctx: private
 */
#[repr(C)]
pub struct shake_ctx {
    pub ctx: __sha3_ctx,
}

/**
 * shake_zeroize_ctx() - Zeroize a SHAKE context
 * @ctx: The context to zeroize
 *
 * Call this after the last squeeze.
 */
pub unsafe fn shake_zeroize_ctx(ctx: *mut shake_ctx) {
    memzero_explicit(ctx.cast(), core::mem::size_of::<shake_ctx>());
}

pub unsafe fn sha3_224_init(ctx: *mut sha3_ctx) {
    *ctx = core::mem::zeroed();
    (*ctx).ctx.digest_size = SHA3_224_DIGEST_SIZE as u8;
    (*ctx).ctx.block_size = SHA3_224_BLOCK_SIZE as u8;
}

pub unsafe fn sha3_256_init(ctx: *mut sha3_ctx) {
    *ctx = core::mem::zeroed();
    (*ctx).ctx.digest_size = SHA3_256_DIGEST_SIZE as u8;
    (*ctx).ctx.block_size = SHA3_256_BLOCK_SIZE as u8;
}

pub unsafe fn sha3_384_init(ctx: *mut sha3_ctx) {
    *ctx = core::mem::zeroed();
    (*ctx).ctx.digest_size = SHA3_384_DIGEST_SIZE as u8;
    (*ctx).ctx.block_size = SHA3_384_BLOCK_SIZE as u8;
}

pub unsafe fn sha3_512_init(ctx: *mut sha3_ctx) {
    *ctx = core::mem::zeroed();
    (*ctx).ctx.digest_size = SHA3_512_DIGEST_SIZE as u8;
    (*ctx).ctx.block_size = SHA3_512_BLOCK_SIZE as u8;
}

pub unsafe fn sha3_update(ctx: *mut sha3_ctx, input: *const u8, in_len: usize) {
    __sha3_update(&mut (*ctx).ctx, input, in_len);
}

pub unsafe fn shake128_init(ctx: *mut shake_ctx) {
    *ctx = core::mem::zeroed();
    (*ctx).ctx.block_size = SHAKE128_BLOCK_SIZE as u8;
}

pub unsafe fn shake256_init(ctx: *mut shake_ctx) {
    *ctx = core::mem::zeroed();
    (*ctx).ctx.block_size = SHAKE256_BLOCK_SIZE as u8;
}

pub unsafe fn shake_update(ctx: *mut shake_ctx, input: *const u8, in_len: usize) {
    __sha3_update(&mut (*ctx).ctx, input, in_len);
}

extern "C" {
    pub fn sha3_final(ctx: *mut sha3_ctx, out: *mut u8);
    pub fn shake_squeeze(ctx: *mut shake_ctx, out: *mut u8, out_len: usize);
    pub fn sha3_224(input: *const u8, in_len: usize, out: *mut u8);
    pub fn sha3_256(input: *const u8, in_len: usize, out: *mut u8);
    pub fn sha3_384(input: *const u8, in_len: usize, out: *mut u8);
    pub fn sha3_512(input: *const u8, in_len: usize, out: *mut u8);
    pub fn shake128(input: *const u8, in_len: usize, out: *mut u8, out_len: usize);
    pub fn shake256(input: *const u8, in_len: usize, out: *mut u8, out_len: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
