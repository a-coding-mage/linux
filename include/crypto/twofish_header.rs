/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux type definitions.

pub const TF_MIN_KEY_SIZE: usize = 16;
pub const TF_MAX_KEY_SIZE: usize = 32;
pub const TF_BLOCK_SIZE: usize = 16;

#[repr(C)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

/* Structure for an expanded Twofish key.  s contains the key-dependent
 * S-boxes composed with the MDS matrix; w contains the eight "whitening"
 * subkeys, K[0] through K[7]. k holds the remaining, "round" subkeys.  Note
 * that k[i] corresponds to what the Twofish paper calls K[i+8]. */
#[repr(C)]
pub struct twofish_ctx {
    pub s: [[u32; 256]; 4],
    pub w: [u32; 8],
    pub k: [u32; 32],
}

unsafe extern "C" {
    pub fn __twofish_setkey(
        ctx: *mut twofish_ctx,
        key: *const u8,
        key_len: u32,
    ) -> i32;
    pub fn twofish_setkey(
        tfm: *mut crypto_tfm,
        key: *const u8,
        key_len: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
