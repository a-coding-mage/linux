// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * This is an implementation of the ChaCha20Poly1305 AEAD construction.
 *
 * Information: https://tools.ietf.org/html/rfc8439
 */

// Declarations supplied by the corresponding crypto and kernel dependencies.

unsafe fn chacha_load_key(k: *mut u32, input: *const u8) {
    *k.add(0) = get_unaligned_le32(input);
    *k.add(1) = get_unaligned_le32(input.add(4));
    *k.add(2) = get_unaligned_le32(input.add(8));
    *k.add(3) = get_unaligned_le32(input.add(12));
    *k.add(4) = get_unaligned_le32(input.add(16));
    *k.add(5) = get_unaligned_le32(input.add(20));
    *k.add(6) = get_unaligned_le32(input.add(24));
    *k.add(7) = get_unaligned_le32(input.add(28));
}

unsafe fn xchacha_init(chacha_state: *mut chacha_state, key: *const u8, nonce: *const u8) {
    let mut k = [0u32; CHACHA_KEY_WORDS];
    let mut iv = [0u8; CHACHA_IV_SIZE];
    core::ptr::write_bytes(iv.as_mut_ptr(), 0, 8);
    core::ptr::copy_nonoverlapping(nonce.add(16), iv.as_mut_ptr().add(8), 8);
    chacha_load_key(k.as_mut_ptr(), key);
    // Compute the subkey given the original key and first 128 nonce bits
    chacha_init(chacha_state, k.as_ptr(), nonce);
    hchacha_block(chacha_state, k.as_mut_ptr(), 20);
    chacha_init(chacha_state, k.as_ptr(), iv.as_ptr());
    memzero_explicit(k.as_mut_ptr().cast(), core::mem::size_of_val(&k));
    memzero_explicit(iv.as_mut_ptr().cast(), core::mem::size_of_val(&iv));
}

unsafe fn __chacha20poly1305_encrypt(dst: *mut u8, src: *const u8, src_len: usize,
                                     ad: *const u8, ad_len: usize,
                                     chacha_state: *mut chacha_state) {
    let pad0 = page_address(ZERO_PAGE(0));
    let mut poly1305_state: poly1305_desc_ctx = core::mem::zeroed();
    let mut b = [0u8; POLY1305_KEY_SIZE];
    chacha20_crypt(chacha_state, b.as_mut_ptr(), pad0, POLY1305_KEY_SIZE);
    poly1305_init(&mut poly1305_state, b.as_ptr());
    poly1305_update(&mut poly1305_state, ad, ad_len);
    if ad_len & 0xf != 0 { poly1305_update(&mut poly1305_state, pad0, 0x10 - (ad_len & 0xf)); }
    chacha20_crypt(chacha_state, dst, src, src_len);
    poly1305_update(&mut poly1305_state, dst, src_len);
    if src_len & 0xf != 0 { poly1305_update(&mut poly1305_state, pad0, 0x10 - (src_len & 0xf)); }
    let lens = [ad_len.to_le() as u64, src_len.to_le() as u64];
    poly1305_update(&mut poly1305_state, lens.as_ptr().cast(), 16);
    poly1305_final(&mut poly1305_state, dst.add(src_len));
    chacha_zeroize_state(chacha_state);
    memzero_explicit(b.as_mut_ptr().cast(), core::mem::size_of_val(&b));
}

pub unsafe fn chacha20poly1305_encrypt(dst: *mut u8, src: *const u8, src_len: usize,
                                        ad: *const u8, ad_len: usize, nonce: u64,
                                        key: *const u8) {
    let mut state: chacha_state = core::mem::zeroed();
    let mut k = [0u32; CHACHA_KEY_WORDS];
    let iv = [0u64, nonce.to_le()];
    chacha_load_key(k.as_mut_ptr(), key);
    chacha_init(&mut state, k.as_ptr(), iv.as_ptr().cast());
    __chacha20poly1305_encrypt(dst, src, src_len, ad, ad_len, &mut state);
    memzero_explicit(iv.as_ptr().cast_mut().cast(), 16);
    memzero_explicit(k.as_mut_ptr().cast(), core::mem::size_of_val(&k));
}

pub unsafe fn xchacha20poly1305_encrypt(dst: *mut u8, src: *const u8, src_len: usize,
                                         ad: *const u8, ad_len: usize, nonce: *const u8,
                                         key: *const u8) {
    let mut state: chacha_state = core::mem::zeroed();
    xchacha_init(&mut state, key, nonce);
    __chacha20poly1305_encrypt(dst, src, src_len, ad, ad_len, &mut state);
}

unsafe fn __chacha20poly1305_decrypt(dst: *mut u8, src: *const u8, src_len: usize,
                                     ad: *const u8, ad_len: usize,
                                     state: *mut chacha_state) -> bool {
    if src_len < POLY1305_DIGEST_SIZE { return false; }
    let pad0 = page_address(ZERO_PAGE(0));
    let mut ps: poly1305_desc_ctx = core::mem::zeroed();
    let mut block0 = [0u8; POLY1305_KEY_SIZE];
    let mut mac = [0u8; POLY1305_DIGEST_SIZE];
    chacha20_crypt(state, block0.as_mut_ptr(), pad0, POLY1305_KEY_SIZE);
    poly1305_init(&mut ps, block0.as_ptr());
    poly1305_update(&mut ps, ad, ad_len);
    if ad_len & 0xf != 0 { poly1305_update(&mut ps, pad0, 0x10 - (ad_len & 0xf)); }
    let dst_len = src_len - POLY1305_DIGEST_SIZE;
    poly1305_update(&mut ps, src, dst_len);
    if dst_len & 0xf != 0 { poly1305_update(&mut ps, pad0, 0x10 - (dst_len & 0xf)); }
    let lens = [ad_len.to_le() as u64, dst_len.to_le() as u64];
    poly1305_update(&mut ps, lens.as_ptr().cast(), 16);
    poly1305_final(&mut ps, mac.as_mut_ptr());
    let ret = crypto_memneq(mac.as_ptr(), src.add(dst_len), POLY1305_DIGEST_SIZE) == 0;
    if ret { chacha20_crypt(state, dst, src, dst_len); }
    memzero_explicit(mac.as_mut_ptr().cast(), POLY1305_DIGEST_SIZE);
    ret
}

pub unsafe fn chacha20poly1305_decrypt(dst: *mut u8, src: *const u8, src_len: usize,
                                        ad: *const u8, ad_len: usize, nonce: u64,
                                        key: *const u8) -> bool {
    let mut state: chacha_state = core::mem::zeroed();
    let mut k = [0u32; CHACHA_KEY_WORDS];
    let iv = [0u64, nonce.to_le()];
    chacha_load_key(k.as_mut_ptr(), key);
    chacha_init(&mut state, k.as_ptr(), iv.as_ptr().cast());
    let ret = __chacha20poly1305_decrypt(dst, src, src_len, ad, ad_len, &mut state);
    chacha_zeroize_state(&mut state);
    ret
}

pub unsafe fn xchacha20poly1305_decrypt(dst: *mut u8, src: *const u8, src_len: usize,
                                         ad: *const u8, ad_len: usize, nonce: *const u8,
                                         key: *const u8) -> bool {
    let mut state: chacha_state = core::mem::zeroed();
    xchacha_init(&mut state, key, nonce);
    __chacha20poly1305_decrypt(dst, src, src_len, ad, ad_len, &mut state)
}

// Scatterlist entry points are declarations of the corresponding low-level kernel path;
// their implementation depends on kernel scatterlist iterator facilities unavailable here.
extern "C" {
    pub fn chacha20poly1305_encrypt_sg_inplace(src: *mut scatterlist, src_len: usize,
                                                ad: *const u8, ad_len: usize, nonce: u64,
                                                key: *const u8) -> bool;
    pub fn chacha20poly1305_decrypt_sg_inplace(src: *mut scatterlist, src_len: usize,
                                                ad: *const u8, ad_len: usize, nonce: u64,
                                                key: *const u8) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
