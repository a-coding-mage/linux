/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AES block cipher, optimized for ARM64
 *
 * Copyright (C) 2013 - 2017 Linaro Ltd <ard.biesheuvel@linaro.org>
 * Copyright 2026 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation are intentionally external.

static mut HAVE_NEON: bool = false;
static mut HAVE_AES: bool = false;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aes_block {
    pub b: [u8; AES_BLOCK_SIZE],
}

extern "C" {
    pub fn __aes_arm64_encrypt(rk: *const u32, out: *mut u8, input: *const u8, rounds: i32);
    pub fn __aes_arm64_decrypt(inv_rk: *const u32, out: *mut u8, input: *const u8, rounds: i32);
    pub fn __aes_ce_encrypt(rk: *const u32, out: *mut u8, input: *const u8, rounds: i32);
    pub fn __aes_ce_decrypt(inv_rk: *const u32, out: *mut u8, input: *const u8, rounds: i32);
    pub fn __aes_ce_sub(l: u32) -> u32;
    pub fn __aes_ce_invert(out: *mut aes_block, input: *const aes_block);
    pub fn neon_aes_mac_update(
        input: *const u8, rk: *const u32, rounds: i32, blocks: usize,
        dg: *mut u8, enc_before: i32, enc_after: i32,
    );
    pub fn ce_aes_mac_update(
        input: *const u8, rk: *const u32, rounds: i32, blocks: usize,
        dg: *mut u8, enc_before: i32, enc_after: i32,
    );
    pub fn aes_expandkey_generic(
        rndkeys: *mut u32, inv_rndkeys: *mut u32, in_key: *const u8, key_len: i32,
    );
    pub fn get_unaligned_le32(input: *const u8) -> u32;
    pub fn ror32(value: u32, shift: i32) -> u32;
    pub fn may_use_simd() -> bool;
    pub fn aes_check_keylen(key_len: u32) -> i32;
    pub fn cpu_have_named_feature(feature: u32) -> bool;
    pub fn static_branch_enable(key: *mut bool);
}

// Expand an AES key using the crypto extensions if supported and usable or
// generic code otherwise. The expanded key format is compatible between the two cases.
pub unsafe fn aes_expandkey_arm64(
    rndkeys: *mut u32, inv_rndkeys: *mut u32, in_key: *const u8, key_len: i32, nrounds: i32,
) {
    static RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];
    let kwords = (key_len as usize) / core::mem::size_of::<u32>();

    if !HAVE_AES || !may_use_simd() {
        aes_expandkey_generic(rndkeys, inv_rndkeys, in_key, key_len);
        return;
    }

    for i in 0..kwords {
        *rndkeys.add(i) = get_unaligned_le32(in_key.add(i * core::mem::size_of::<u32>()));
    }

    for i in 0..RCON.len() {
        let rki = rndkeys.add(i * kwords);
        let rko = rki.add(kwords);
        *rko.add(0) = ror32(__aes_ce_sub(*rki.add(kwords - 1)), 8)
            ^ RCON[i] as u32 ^ *rki.add(0);
        *rko.add(1) = *rko.add(0) ^ *rki.add(1);
        *rko.add(2) = *rko.add(1) ^ *rki.add(2);
        *rko.add(3) = *rko.add(2) ^ *rki.add(3);

        if key_len == AES_KEYSIZE_192 {
            if i >= 7 { break; }
            *rko.add(4) = *rko.add(3) ^ *rki.add(4);
            *rko.add(5) = *rko.add(4) ^ *rki.add(5);
        } else if key_len == AES_KEYSIZE_256 {
            if i >= 6 { break; }
            *rko.add(4) = __aes_ce_sub(*rko.add(3)) ^ *rki.add(4);
            *rko.add(5) = *rko.add(4) ^ *rki.add(5);
            *rko.add(6) = *rko.add(5) ^ *rki.add(6);
            *rko.add(7) = *rko.add(6) ^ *rki.add(7);
        }
    }

    if !inv_rndkeys.is_null() {
        let key_enc = rndkeys as *const aes_block;
        let key_dec = inv_rndkeys as *mut aes_block;
        let mut j = nrounds;
        *key_dec = *key_enc.add(j as usize);
        let mut i = 1usize;
        j -= 1;
        while j > 0 {
            __aes_ce_invert(key_dec.add(i), key_enc.add(j as usize));
            i += 1;
            j -= 1;
        }
        *key_dec.add(i) = *key_enc;
    }
}

pub unsafe fn aes_preparekey_arch(
    k: *mut union_aes_enckey_arch, inv_k: *mut union_aes_invkey_arch,
    in_key: *const u8, key_len: i32, nrounds: i32,
) {
    aes_expandkey_arm64((*k).rndkeys.as_mut_ptr(), if inv_k.is_null() { core::ptr::null_mut() } else { (*inv_k).inv_rndkeys.as_mut_ptr() }, in_key, key_len, nrounds);
}

pub unsafe fn ce_aes_expandkey(ctx: *mut crypto_aes_ctx, in_key: *const u8, key_len: u32) -> i32 {
    if aes_check_keylen(key_len) != 0 { return -EINVAL; }
    (*ctx).key_length = key_len;
    aes_expandkey_arm64((*ctx).key_enc.as_mut_ptr(), (*ctx).key_dec.as_mut_ptr(), in_key, key_len as i32, 6 + (key_len / 4) as i32);
    0
}

pub unsafe fn aes_encrypt_arch(key: *const aes_enckey, out: *mut u8, input: *const u8) {
    if HAVE_AES && may_use_simd() {
        __aes_ce_encrypt((*key).k.rndkeys.as_ptr(), out, input, (*key).nrounds);
    } else {
        __aes_arm64_encrypt((*key).k.rndkeys.as_ptr(), out, input, (*key).nrounds);
    }
}

pub unsafe fn aes_decrypt_arch(key: *const aes_key, out: *mut u8, input: *const u8) {
    if HAVE_AES && may_use_simd() {
        __aes_ce_decrypt((*key).inv_k.inv_rndkeys.as_ptr(), out, input, (*key).nrounds);
    } else {
        __aes_arm64_decrypt((*key).inv_k.inv_rndkeys.as_ptr(), out, input, (*key).nrounds);
    }
}

#[cfg(CONFIG_CRYPTO_LIB_AES_CBC_MACS)]
pub unsafe fn aes_cbcmac_blocks_arch(
    h: *mut u8, key: *const aes_enckey, data: *const u8, nblocks: usize,
    enc_before: bool, enc_after: bool,
) -> bool {
    if HAVE_NEON && may_use_simd() {
        if HAVE_AES {
            ce_aes_mac_update(data, (*key).k.rndkeys.as_ptr(), (*key).nrounds,
                              nblocks, h, enc_before as i32, enc_after as i32);
        } else {
            neon_aes_mac_update(data, (*key).k.rndkeys.as_ptr(), (*key).nrounds,
                                nblocks, h, enc_before as i32, enc_after as i32);
        }
        return true;
    }
    false
}

// EXPORT_SYMBOL and EXPORT_SYMBOL_NS_GPL declarations are linkage metadata in C.

pub unsafe fn aes_mod_init_arch() {
    if cpu_have_named_feature(ASIMD) {
        static_branch_enable(&mut HAVE_NEON);
        if cpu_have_named_feature(AES) { static_branch_enable(&mut HAVE_AES); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
