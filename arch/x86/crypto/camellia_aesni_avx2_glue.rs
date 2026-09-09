// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Glue Code for x86_64/AVX2/AES-NI assembler optimized version of Camellia
 *
 * Copyright © 2013 Jussi Kivilinna <jussi.kivilinna@mbnet.fi>
 */

// Dependencies supplied by the surrounding kernel crypto implementation are
// intentionally left external, as in the original C source.

pub const CAMELLIA_AESNI_PARALLEL_BLOCKS: usize = 16;
pub const CAMELLIA_AESNI_AVX2_PARALLEL_BLOCKS: usize = 32;

extern "C" {
    pub fn camellia_ecb_enc_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn camellia_ecb_dec_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn camellia_cbc_dec_32way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn camellia_ecb_enc_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn camellia_ecb_dec_16way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn camellia_enc_blk_2way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn camellia_dec_blk_2way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn camellia_enc_blk(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn camellia_dec_blk(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
    pub fn camellia_decrypt_cbc_2way(ctx: *const core::ffi::c_void, dst: *mut u8, src: *const u8);
}

unsafe fn camellia_setkey(
    tfm: *mut crypto_skcipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    __camellia_setkey(crypto_skcipher_ctx(tfm), key, keylen)
}

unsafe fn ecb_encrypt(req: *mut skcipher_request) -> i32 {
    ECB_WALK_START!(req, CAMELLIA_BLOCK_SIZE, CAMELLIA_AESNI_PARALLEL_BLOCKS);
    ECB_BLOCK!(CAMELLIA_AESNI_AVX2_PARALLEL_BLOCKS, camellia_ecb_enc_32way);
    ECB_BLOCK!(CAMELLIA_AESNI_PARALLEL_BLOCKS, camellia_ecb_enc_16way);
    ECB_BLOCK!(2, camellia_enc_blk_2way);
    ECB_BLOCK!(1, camellia_enc_blk);
    ECB_WALK_END!()
}

unsafe fn ecb_decrypt(req: *mut skcipher_request) -> i32 {
    ECB_WALK_START!(req, CAMELLIA_BLOCK_SIZE, CAMELLIA_AESNI_PARALLEL_BLOCKS);
    ECB_BLOCK!(CAMELLIA_AESNI_AVX2_PARALLEL_BLOCKS, camellia_ecb_dec_32way);
    ECB_BLOCK!(CAMELLIA_AESNI_PARALLEL_BLOCKS, camellia_ecb_dec_16way);
    ECB_BLOCK!(2, camellia_dec_blk_2way);
    ECB_BLOCK!(1, camellia_dec_blk);
    ECB_WALK_END!()
}

unsafe fn cbc_encrypt(req: *mut skcipher_request) -> i32 {
    CBC_WALK_START!(req, CAMELLIA_BLOCK_SIZE, -1);
    CBC_ENC_BLOCK!(camellia_enc_blk);
    CBC_WALK_END!()
}

unsafe fn cbc_decrypt(req: *mut skcipher_request) -> i32 {
    CBC_WALK_START!(req, CAMELLIA_BLOCK_SIZE, CAMELLIA_AESNI_PARALLEL_BLOCKS);
    CBC_DEC_BLOCK!(CAMELLIA_AESNI_AVX2_PARALLEL_BLOCKS, camellia_cbc_dec_32way);
    CBC_DEC_BLOCK!(CAMELLIA_AESNI_PARALLEL_BLOCKS, camellia_cbc_dec_16way);
    CBC_DEC_BLOCK!(2, camellia_decrypt_cbc_2way);
    CBC_DEC_BLOCK!(1, camellia_dec_blk);
    CBC_WALK_END!()
}

static mut camellia_algs: [skcipher_alg; 2] = [
    skcipher_alg {
        base: crypto_alg { cra_name: b"ecb(camellia)\0".as_ptr(), cra_driver_name: b"ecb-camellia-aesni-avx2\0".as_ptr(), cra_priority: 500, cra_blocksize: CAMELLIA_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<camellia_ctx>(), cra_module: THIS_MODULE },
        min_keysize: CAMELLIA_MIN_KEY_SIZE, max_keysize: CAMELLIA_MAX_KEY_SIZE, setkey: Some(camellia_setkey), encrypt: Some(ecb_encrypt), decrypt: Some(ecb_decrypt), ..unsafe { core::mem::zeroed() }
    },
    skcipher_alg {
        base: crypto_alg { cra_name: b"cbc(camellia)\0".as_ptr(), cra_driver_name: b"cbc-camellia-aesni-avx2\0".as_ptr(), cra_priority: 500, cra_blocksize: CAMELLIA_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<camellia_ctx>(), cra_module: THIS_MODULE },
        min_keysize: CAMELLIA_MIN_KEY_SIZE, max_keysize: CAMELLIA_MAX_KEY_SIZE, ivsize: CAMELLIA_BLOCK_SIZE, setkey: Some(camellia_setkey), encrypt: Some(cbc_encrypt), decrypt: Some(cbc_decrypt), ..unsafe { core::mem::zeroed() }
    },
];

unsafe fn camellia_aesni_init() -> i32 {
    let mut feature_name: *const core::ffi::c_char = core::ptr::null();
    if !boot_cpu_has(X86_FEATURE_AVX) || !boot_cpu_has(X86_FEATURE_AVX2) || !boot_cpu_has(X86_FEATURE_AES) || !boot_cpu_has(X86_FEATURE_OSXSAVE) {
        pr_info!("AVX2 or AES-NI instructions are not detected.\n");
        return -ENODEV;
    }
    if !cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, &mut feature_name) {
        pr_info!("CPU feature '%s' is not supported.\n", feature_name);
        return -ENODEV;
    }
    crypto_register_skciphers(camellia_algs.as_mut_ptr(), camellia_algs.len())
}

unsafe fn camellia_aesni_fini() {
    crypto_unregister_skciphers(camellia_algs.as_mut_ptr(), camellia_algs.len());
}

module_init!(camellia_aesni_init);
module_exit!(camellia_aesni_fini);

module_license!("GPL");
module_description!("Camellia Cipher Algorithm, AES-NI/AVX2 optimized");
module_alias_crypto!("camellia");
module_alias_crypto!("camellia-asm");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
