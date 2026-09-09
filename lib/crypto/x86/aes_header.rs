/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AES block cipher using AES-NI instructions
 *
 * Copyright 2026 Google LLC
 */

// Dependency supplied by the surrounding translation unit: FPU API.

static mut have_aes: static_key_false = DEFINE_STATIC_KEY_FALSE!();

extern "C" {
    fn aes128_expandkey_aesni(
        rndkeys: *mut u32,
        inv_rndkeys: *mut u32,
        in_key: *const u8,
    );
    fn aes256_expandkey_aesni(
        rndkeys: *mut u32,
        inv_rndkeys: *mut u32,
        in_key: *const u8,
    );
    fn aes_encrypt_aesni(
        rndkeys: *const u32,
        nrounds: i32,
        out: *mut u8,
        input: *const u8,
    );
    fn aes_decrypt_aesni(
        inv_rndkeys: *const u32,
        nrounds: i32,
        out: *mut u8,
        input: *const u8,
    );
}

/*
 * Expand an AES key using AES-NI if supported and usable or generic code
 * otherwise.  The expanded key format is compatible between the two cases.  The
 * outputs are @k->rndkeys (required) and @inv_k->inv_rndkeys (optional).
 *
 * We could just always use the generic key expansion code.  AES key expansion
 * is usually less performance-critical than AES en/decryption.  However,
 * there's still *some* value in speed here, as well as in non-key-dependent
 * execution time which AES-NI provides.  So, do use AES-NI to expand AES-128
 * and AES-256 keys.  (Don't bother with AES-192, as it's almost never used.)
 */
unsafe fn aes_preparekey_arch(
    k: *mut aes_enckey_arch,
    inv_k: *mut aes_invkey_arch,
    in_key: *const u8,
    key_len: i32,
    _nrounds: i32,
) {
    let rndkeys = (*k).rndkeys.as_mut_ptr();
    let inv_rndkeys = if !inv_k.is_null() {
        (*inv_k).inv_rndkeys.as_mut_ptr()
    } else {
        core::ptr::null_mut()
    };

    if static_branch_likely(&raw mut have_aes)
        && key_len != AES_KEYSIZE_192
        && irq_fpu_usable()
    {
        kernel_fpu_begin();
        if key_len == AES_KEYSIZE_128 {
            aes128_expandkey_aesni(rndkeys, inv_rndkeys, in_key);
        } else {
            aes256_expandkey_aesni(rndkeys, inv_rndkeys, in_key);
        }
        kernel_fpu_end();
    } else {
        aes_expandkey_generic(rndkeys, inv_rndkeys, in_key, key_len);
    }
}

unsafe fn aes_encrypt_arch(
    key: *const aes_enckey,
    out: *mut u8,
    input: *const u8,
) {
    if static_branch_likely(&raw mut have_aes) && irq_fpu_usable() {
        kernel_fpu_begin();
        aes_encrypt_aesni((*key).k.rndkeys.as_ptr(), (*key).nrounds, out, input);
        kernel_fpu_end();
    } else {
        aes_encrypt_generic((*key).k.rndkeys.as_ptr(), (*key).nrounds, out, input);
    }
}

unsafe fn aes_decrypt_arch(
    key: *const aes_key,
    out: *mut u8,
    input: *const u8,
) {
    if static_branch_likely(&raw mut have_aes) && irq_fpu_usable() {
        kernel_fpu_begin();
        aes_decrypt_aesni(
            (*key).inv_k.inv_rndkeys.as_ptr(),
            (*key).nrounds,
            out,
            input,
        );
        kernel_fpu_end();
    } else {
        aes_decrypt_generic(
            (*key).inv_k.inv_rndkeys.as_ptr(),
            (*key).nrounds,
            out,
            input,
        );
    }
}

// #define aes_mod_init_arch aes_mod_init_arch
unsafe fn aes_mod_init_arch() {
    if boot_cpu_has(X86_FEATURE_AES) {
        static_branch_enable(&raw mut have_aes);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
