/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AES accelerated using the sparc64 aes opcodes
 *
 * Copyright (C) 2008, Intel Corp.
 * Copyright (c) 2010, Intel Corporation.
 * Copyright 2026 Google LLC
 */

// C dependencies: <asm/fpumacro.h>, <asm/opcodes.h>, <asm/pstate.h>,
// <asm/elf.h>, and the generic AES/kernel declarations.

static mut have_aes_opcodes: StaticKey = StaticKey::new_false();

// EXPORT_SYMBOL_GPL declarations are retained as public ABI declarations.

unsafe extern "C" {
    pub fn aes_sparc64_key_expand(key: *const u32, rndkeys: *mut u64, key_len: i32);
    pub fn aes_sparc64_load_encrypt_keys_128();
    pub fn aes_sparc64_load_encrypt_keys_192();
    pub fn aes_sparc64_load_encrypt_keys_256();
    pub fn aes_sparc64_load_decrypt_keys_128();
    pub fn aes_sparc64_load_decrypt_keys_192();
    pub fn aes_sparc64_load_decrypt_keys_256();
    pub fn aes_sparc64_ecb_encrypt_128();
    pub fn aes_sparc64_ecb_encrypt_192();
    pub fn aes_sparc64_ecb_encrypt_256();
    pub fn aes_sparc64_ecb_decrypt_128();
    pub fn aes_sparc64_ecb_decrypt_192();
    pub fn aes_sparc64_ecb_decrypt_256();
    pub fn aes_sparc64_cbc_encrypt_128();
    pub fn aes_sparc64_cbc_encrypt_192();
    pub fn aes_sparc64_cbc_encrypt_256();
    pub fn aes_sparc64_cbc_decrypt_128();
    pub fn aes_sparc64_cbc_decrypt_192();
    pub fn aes_sparc64_cbc_decrypt_256();
    pub fn aes_sparc64_ctr_crypt_128();
    pub fn aes_sparc64_ctr_crypt_192();
    pub fn aes_sparc64_ctr_crypt_256();

    pub fn aes_sparc64_encrypt_128(key: *const u64, input: *const u32, output: *mut u32);
    pub fn aes_sparc64_encrypt_192(key: *const u64, input: *const u32, output: *mut u32);
    pub fn aes_sparc64_encrypt_256(key: *const u64, input: *const u32, output: *mut u32);
    pub fn aes_sparc64_decrypt_128(key: *const u64, input: *const u32, output: *mut u32);
    pub fn aes_sparc64_decrypt_192(key: *const u64, input: *const u32, output: *mut u32);
    pub fn aes_sparc64_decrypt_256(key: *const u64, input: *const u32, output: *mut u32);
}

unsafe fn aes_preparekey_arch(
    k: *mut aes_enckey_arch,
    inv_k: *mut aes_invkey_arch,
    in_key: *const u8,
    key_len: i32,
    _nrounds: i32,
) {
    if static_branch_likely(&raw mut have_aes_opcodes) {
        let mut aligned_key = [0u32; AES_MAX_KEY_SIZE / 4];

        if is_aligned(in_key as usize, 4) {
            aes_sparc64_key_expand(in_key as *const u32, (*k).sparc_rndkeys.as_mut_ptr(), key_len);
        } else {
            core::ptr::copy_nonoverlapping(in_key, aligned_key.as_mut_ptr() as *mut u8, key_len as usize);
            aes_sparc64_key_expand(aligned_key.as_ptr(), (*k).sparc_rndkeys.as_mut_ptr(), key_len);
            core::ptr::write_bytes(aligned_key.as_mut_ptr() as *mut u8, 0, key_len as usize);
        }
        // inv_k is intentionally unused: SPARC64 uses the encryption round keys for both directions.
        let _ = inv_k;
    } else {
        aes_expandkey_generic((*k).rndkeys.as_mut_ptr(), if inv_k.is_null() { core::ptr::null_mut() } else { (*inv_k).inv_rndkeys.as_mut_ptr() }, in_key, key_len);
    }
}

unsafe fn aes_sparc64_encrypt(key: *const aes_enckey, input: *const u32, output: *mut u32) {
    if (*key).len == AES_KEYSIZE_128 { aes_sparc64_encrypt_128((*key).k.sparc_rndkeys.as_ptr(), input, output); }
    else if (*key).len == AES_KEYSIZE_192 { aes_sparc64_encrypt_192((*key).k.sparc_rndkeys.as_ptr(), input, output); }
    else { aes_sparc64_encrypt_256((*key).k.sparc_rndkeys.as_ptr(), input, output); }
}

unsafe fn aes_encrypt_arch(key: *const aes_enckey, out: *mut u8, input: *const u8) {
    let mut bounce_buf = [0u32; AES_BLOCK_SIZE / 4];
    if static_branch_likely(&raw mut have_aes_opcodes) {
        if is_aligned((input as usize) | (out as usize), 4) { aes_sparc64_encrypt(key, input as *const u32, out as *mut u32); }
        else { core::ptr::copy_nonoverlapping(input, bounce_buf.as_mut_ptr() as *mut u8, AES_BLOCK_SIZE); aes_sparc64_encrypt(key, bounce_buf.as_ptr(), bounce_buf.as_mut_ptr()); core::ptr::copy_nonoverlapping(bounce_buf.as_ptr() as *const u8, out, AES_BLOCK_SIZE); }
    } else { aes_encrypt_generic((*key).k.rndkeys.as_ptr(), (*key).nrounds, out, input); }
}

unsafe fn aes_sparc64_decrypt(key: *const aes_key, input: *const u32, output: *mut u32) {
    if (*key).len == AES_KEYSIZE_128 { aes_sparc64_decrypt_128((*key).k.sparc_rndkeys.as_ptr(), input, output); }
    else if (*key).len == AES_KEYSIZE_192 { aes_sparc64_decrypt_192((*key).k.sparc_rndkeys.as_ptr(), input, output); }
    else { aes_sparc64_decrypt_256((*key).k.sparc_rndkeys.as_ptr(), input, output); }
}

unsafe fn aes_decrypt_arch(key: *const aes_key, out: *mut u8, input: *const u8) {
    let mut bounce_buf = [0u32; AES_BLOCK_SIZE / 4];
    if static_branch_likely(&raw mut have_aes_opcodes) {
        if is_aligned((input as usize) | (out as usize), 4) { aes_sparc64_decrypt(key, input as *const u32, out as *mut u32); }
        else { core::ptr::copy_nonoverlapping(input, bounce_buf.as_mut_ptr() as *mut u8, AES_BLOCK_SIZE); aes_sparc64_decrypt(key, bounce_buf.as_ptr(), bounce_buf.as_mut_ptr()); core::ptr::copy_nonoverlapping(bounce_buf.as_ptr() as *const u8, out, AES_BLOCK_SIZE); }
    } else { aes_decrypt_generic((*key).inv_k.inv_rndkeys.as_ptr(), (*key).nrounds, out, input); }
}

// #define aes_mod_init_arch aes_mod_init_arch
unsafe fn aes_mod_init_arch() {
    let mut cfr: c_ulong;
    if (sparc64_elf_hwcap & HWCAP_SPARC_CRYPTO) == 0 { return; }
    core::arch::asm!("rd %asr26, {0}", out(reg) cfr);
    if (cfr & CFR_AES) == 0 { return; }
    static_branch_enable(&raw mut have_aes_opcodes);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
