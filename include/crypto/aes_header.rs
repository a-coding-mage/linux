/* SPDX-License-Identifier: GPL-2.0 */
/* Common values for AES algorithms. */

pub const AES_MIN_KEY_SIZE: usize = 16;
pub const AES_MAX_KEY_SIZE: usize = 32;
pub const AES_KEYSIZE_128: usize = 16;
pub const AES_KEYSIZE_192: usize = 24;
pub const AES_KEYSIZE_256: usize = 32;
pub const AES_BLOCK_SIZE: usize = 16;
pub const AES_MAX_KEYLENGTH: usize = 15 * 16;
pub const AES_MAX_KEYLENGTH_U32: usize = AES_MAX_KEYLENGTH / core::mem::size_of::<u32>();

#[repr(C)]
pub struct p8_aes_key {
    pub rndkeys: [u32; AES_MAX_KEYLENGTH_U32],
    pub nrounds: core::ffi::c_int,
}

#[repr(C)]
pub union aes_enckey_arch {
    pub rndkeys: [u32; AES_MAX_KEYLENGTH_U32],
    pub spe_enc_key: [u32; AES_MAX_KEYLENGTH_U32],
    pub p8: p8_aes_key,
    pub raw_key: [u8; AES_MAX_KEY_SIZE],
    pub sparc_rndkeys: [u64; AES_MAX_KEYLENGTH / core::mem::size_of::<u64>()],
}

#[repr(C)]
pub union aes_invkey_arch {
    pub inv_rndkeys: [u32; AES_MAX_KEYLENGTH_U32],
    pub spe_dec_key: [u32; AES_MAX_KEYLENGTH_U32],
    pub p8: p8_aes_key,
}

#[repr(C)]
pub struct aes_enckey {
    pub len: u32,
    pub nrounds: u32,
    pub padding: [u32; 2],
    pub k: aes_enckey_arch,
}

#[repr(C)]
pub struct aes_key {
    pub aes_enckey: aes_enckey,
    pub inv_k: aes_invkey_arch,
}

#[repr(C)]
pub struct crypto_aes_ctx {
    pub key_enc: [u32; AES_MAX_KEYLENGTH_U32],
    pub key_dec: [u32; AES_MAX_KEYLENGTH_U32],
    pub key_length: u32,
}

#[inline]
pub fn aes_check_keylen(keylen: usize) -> core::ffi::c_int {
    match keylen {
        AES_KEYSIZE_128 | AES_KEYSIZE_192 | AES_KEYSIZE_256 => 0,
        _ => -22, // -EINVAL
    }
}

extern "C" {
    pub fn aes_expandkey(ctx: *mut crypto_aes_ctx, in_key: *const u8, key_len: u32) -> core::ffi::c_int;

    /* CONFIG_ARM64 declarations */
    pub fn ce_aes_expandkey(ctx: *mut crypto_aes_ctx, in_key: *const u8, key_len: u32) -> core::ffi::c_int;
    pub fn neon_aes_ecb_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int);
    pub fn neon_aes_ecb_decrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int);
    pub fn neon_aes_cbc_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int, iv: *mut u8);
    pub fn neon_aes_cbc_decrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int, iv: *mut u8);
    pub fn neon_aes_cbc_cts_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, iv: *const u8);
    pub fn neon_aes_cbc_cts_decrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, iv: *const u8);
    pub fn neon_aes_ctr_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, ctr: *mut u8);
    pub fn neon_aes_xctr_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, ctr: *mut u8, byte_ctr: core::ffi::c_int);
    pub fn neon_aes_xts_encrypt(out: *mut u8, input: *const u8, rk1: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, rk2: *const u32, iv: *mut u8, first: core::ffi::c_int);
    pub fn neon_aes_xts_decrypt(out: *mut u8, input: *const u8, rk1: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, rk2: *const u32, iv: *mut u8, first: core::ffi::c_int);
    pub fn neon_aes_essiv_cbc_encrypt(out: *mut u8, input: *const u8, rk1: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int, iv: *mut u8, rk2: *const u32);
    pub fn neon_aes_essiv_cbc_decrypt(out: *mut u8, input: *const u8, rk1: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int, iv: *mut u8, rk2: *const u32);
    pub fn ce_aes_ecb_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int);
    pub fn ce_aes_ecb_decrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int);
    pub fn ce_aes_cbc_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int, iv: *mut u8);
    pub fn ce_aes_cbc_decrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int, iv: *mut u8);
    pub fn ce_aes_cbc_cts_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, iv: *const u8);
    pub fn ce_aes_cbc_cts_decrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, iv: *const u8);
    pub fn ce_aes_ctr_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, ctr: *mut u8);
    pub fn ce_aes_xctr_encrypt(out: *mut u8, input: *const u8, rk: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, ctr: *mut u8, byte_ctr: core::ffi::c_int);
    pub fn ce_aes_xts_encrypt(out: *mut u8, input: *const u8, rk1: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, rk2: *const u32, iv: *mut u8, first: core::ffi::c_int);
    pub fn ce_aes_xts_decrypt(out: *mut u8, input: *const u8, rk1: *const u32, rounds: core::ffi::c_int, bytes: core::ffi::c_int, rk2: *const u32, iv: *mut u8, first: core::ffi::c_int);
    pub fn ce_aes_essiv_cbc_encrypt(out: *mut u8, input: *const u8, rk1: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int, iv: *mut u8, rk2: *const u32);
    pub fn ce_aes_essiv_cbc_decrypt(out: *mut u8, input: *const u8, rk1: *const u32, rounds: core::ffi::c_int, blocks: core::ffi::c_int, iv: *mut u8, rk2: *const u32);
    pub fn ce_aes_mac_update(input: *const u8, rk: *const u32, rounds: core::ffi::c_int, blocks: usize, dg: *mut u8, enc_before: core::ffi::c_int, enc_after: core::ffi::c_int);

    /* CONFIG_PPC declarations */
    pub fn ppc_expand_key_128(key_enc: *mut u32, key: *const u8);
    pub fn ppc_expand_key_192(key_enc: *mut u32, key: *const u8);
    pub fn ppc_expand_key_256(key_enc: *mut u32, key: *const u8);
    pub fn ppc_generate_decrypt_key(key_dec: *mut u32, key_enc: *mut u32, key_len: u32);
    pub fn ppc_encrypt_ecb(out: *mut u8, input: *const u8, key_enc: *mut u32, rounds: u32, bytes: u32);
    pub fn ppc_decrypt_ecb(out: *mut u8, input: *const u8, key_dec: *mut u32, rounds: u32, bytes: u32);
    pub fn ppc_encrypt_cbc(out: *mut u8, input: *const u8, key_enc: *mut u32, rounds: u32, bytes: u32, iv: *mut u8);
    pub fn ppc_decrypt_cbc(out: *mut u8, input: *const u8, key_dec: *mut u32, rounds: u32, bytes: u32, iv: *mut u8);
    pub fn ppc_crypt_ctr(out: *mut u8, input: *const u8, key_enc: *mut u32, rounds: u32, bytes: u32, iv: *mut u8);
    pub fn ppc_encrypt_xts(out: *mut u8, input: *const u8, key_enc: *mut u32, rounds: u32, bytes: u32, iv: *mut u8, key_twk: *mut u32);
    pub fn ppc_decrypt_xts(out: *mut u8, input: *const u8, key_dec: *mut u32, rounds: u32, bytes: u32, iv: *mut u8, key_twk: *mut u32);
    pub fn aes_p8_set_encrypt_key(user_key: *const u8, bits: core::ffi::c_int, key: *mut p8_aes_key) -> core::ffi::c_int;
    pub fn aes_p8_set_decrypt_key(user_key: *const u8, bits: core::ffi::c_int, key: *mut p8_aes_key) -> core::ffi::c_int;
    pub fn aes_p8_encrypt(input: *const u8, out: *mut u8, key: *const p8_aes_key);
    pub fn aes_p8_decrypt(input: *const u8, out: *mut u8, key: *const p8_aes_key);
    pub fn aes_p8_cbc_encrypt(input: *const u8, out: *mut u8, len: usize, key: *const p8_aes_key, iv: *mut u8, enc: bool);
    pub fn aes_p8_ctr32_encrypt_blocks(input: *const u8, out: *mut u8, len: usize, key: *const p8_aes_key, iv: *const u8);
    pub fn aes_p8_xts_encrypt(input: *const u8, out: *mut u8, len: usize, key1: *const p8_aes_key, key2: *const p8_aes_key, iv: *mut u8);
    pub fn aes_p8_xts_decrypt(input: *const u8, out: *mut u8, len: usize, key1: *const p8_aes_key, key2: *const p8_aes_key, iv: *mut u8);

    /* CONFIG_SPARC64 declarations */
    pub fn aes_sparc64_key_expand(in_key: *const u32, output_key: *mut u64, key_len: u32);
    pub fn aes_sparc64_load_encrypt_keys_128(key: *const u64);
    pub fn aes_sparc64_load_encrypt_keys_192(key: *const u64);
    pub fn aes_sparc64_load_encrypt_keys_256(key: *const u64);
    pub fn aes_sparc64_load_decrypt_keys_128(key: *const u64);
    pub fn aes_sparc64_load_decrypt_keys_192(key: *const u64);
    pub fn aes_sparc64_load_decrypt_keys_256(key: *const u64);
    pub fn aes_sparc64_ecb_encrypt_128(key: *const u64, input: *const u64, output: *mut u64, len: u32);
    pub fn aes_sparc64_ecb_encrypt_192(key: *const u64, input: *const u64, output: *mut u64, len: u32);
    pub fn aes_sparc64_ecb_encrypt_256(key: *const u64, input: *const u64, output: *mut u64, len: u32);
    pub fn aes_sparc64_ecb_decrypt_128(key: *const u64, input: *const u64, output: *mut u64, len: u32);
    pub fn aes_sparc64_ecb_decrypt_192(key: *const u64, input: *const u64, output: *mut u64, len: u32);
    pub fn aes_sparc64_ecb_decrypt_256(key: *const u64, input: *const u64, output: *mut u64, len: u32);
    pub fn aes_sparc64_cbc_encrypt_128(key: *const u64, input: *const u64, output: *mut u64, len: u32, iv: *mut u64);
    pub fn aes_sparc64_cbc_encrypt_192(key: *const u64, input: *const u64, output: *mut u64, len: u32, iv: *mut u64);
    pub fn aes_sparc64_cbc_encrypt_256(key: *const u64, input: *const u64, output: *mut u64, len: u32, iv: *mut u64);
    pub fn aes_sparc64_cbc_decrypt_128(key: *const u64, input: *const u64, output: *mut u64, len: u32, iv: *mut u64);
    pub fn aes_sparc64_cbc_decrypt_192(key: *const u64, input: *const u64, output: *mut u64, len: u32, iv: *mut u64);
    pub fn aes_sparc64_cbc_decrypt_256(key: *const u64, input: *const u64, output: *mut u64, len: u32, iv: *mut u64);
    pub fn aes_sparc64_ctr_crypt_128(key: *const u64, input: *const u64, output: *mut u64, len: u32, iv: *mut u64);
    pub fn aes_sparc64_ctr_crypt_192(key: *const u64, input: *const u64, output: *mut u64, len: u32, iv: *mut u64);
    pub fn aes_sparc64_ctr_crypt_256(key: *const u64, input: *const u64, output: *mut u64, len: u32, iv: *mut u64);

    pub fn aes_preparekey(key: *mut aes_key, in_key: *const u8, key_len: usize) -> core::ffi::c_int;
    pub fn aes_prepareenckey(key: *mut aes_enckey, in_key: *const u8, key_len: usize) -> core::ffi::c_int;
    pub fn aes_encrypt(key: *const aes_enckey, out: *mut u8, input: *const u8);
    pub fn aes_decrypt(key: *const aes_key, out: *mut u8, input: *const u8);
    pub static crypto_aes_sbox: [u8; 0];
    pub static crypto_aes_inv_sbox: [u8; 0];
    pub static aes_enc_tab: [u32; 256];
    pub static aes_dec_tab: [u32; 256];
    pub fn aescfb_encrypt(key: *const aes_enckey, dst: *mut u8, src: *const u8, len: core::ffi::c_int, iv: *const u8);
    pub fn aescfb_decrypt(key: *const aes_enckey, dst: *mut u8, src: *const u8, len: core::ffi::c_int, iv: *const u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
