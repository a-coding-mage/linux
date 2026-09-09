/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * AES optimized using the CP Assist for Cryptographic Functions (CPACF)
 *
 * Copyright 2026 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation:
// asm/cpacf.h, linux/cpufeature.h, AES types/constants, and helper routines.

static mut have_cpacf_aes128: StaticKey = DEFINE_STATIC_KEY_FALSE!();
static mut have_cpacf_aes192: StaticKey = DEFINE_STATIC_KEY_FALSE!();
static mut have_cpacf_aes256: StaticKey = DEFINE_STATIC_KEY_FALSE!();

/*
 * When the CPU supports CPACF AES for the requested key length, we need only
 * save a copy of the raw AES key, as that's what the CPACF instructions need.
 *
 * When unsupported, fall back to the generic key expansion and en/decryption.
 */
unsafe fn aes_preparekey_arch(
    k: *mut aes_enckey_arch,
    inv_k: *mut aes_invkey_arch,
    in_key: *const u8,
    key_len: i32,
    nrounds: i32,
) {
    let _ = nrounds;
    if key_len == AES_KEYSIZE_128 {
        if static_branch_likely(&raw const have_cpacf_aes128) {
            memcpy((*k).raw_key.as_mut_ptr(), in_key, AES_KEYSIZE_128);
            return;
        }
    } else if key_len == AES_KEYSIZE_192 {
        if static_branch_likely(&raw const have_cpacf_aes192) {
            memcpy((*k).raw_key.as_mut_ptr(), in_key, AES_KEYSIZE_192);
            return;
        }
    } else if static_branch_likely(&raw const have_cpacf_aes256) {
        memcpy((*k).raw_key.as_mut_ptr(), in_key, AES_KEYSIZE_256);
        return;
    }
    aes_expandkey_generic(
        (*k).rndkeys.as_mut_ptr(),
        if !inv_k.is_null() {
            (*inv_k).inv_rndkeys.as_mut_ptr()
        } else {
            core::ptr::null_mut()
        },
        in_key,
        key_len,
    );
}

unsafe fn aes_crypt_s390(
    key: *const aes_enckey,
    out: *mut u8,
    input: *const u8,
    decrypt: i32,
) -> bool {
    if (*key).len == AES_KEYSIZE_128 {
        if static_branch_likely(&raw const have_cpacf_aes128) {
            cpacf_km(CPACF_KM_AES_128 | decrypt, (*key).k.raw_key.as_ptr(), out, input, AES_BLOCK_SIZE);
            return true;
        }
    } else if (*key).len == AES_KEYSIZE_192 {
        if static_branch_likely(&raw const have_cpacf_aes192) {
            cpacf_km(CPACF_KM_AES_192 | decrypt, (*key).k.raw_key.as_ptr(), out, input, AES_BLOCK_SIZE);
            return true;
        }
    } else if static_branch_likely(&raw const have_cpacf_aes256) {
        cpacf_km(CPACF_KM_AES_256 | decrypt, (*key).k.raw_key.as_ptr(), out, input, AES_BLOCK_SIZE);
        return true;
    }
    false
}

unsafe fn aes_encrypt_arch(key: *const aes_enckey, out: *mut u8, input: *const u8) {
    if likely(aes_crypt_s390(key, out, input, 0)) {
        return;
    }
    aes_encrypt_generic((*key).k.rndkeys.as_ptr(), (*key).nrounds, out, input);
}

unsafe fn aes_decrypt_arch(key: *const aes_key, out: *mut u8, input: *const u8) {
    if likely(aes_crypt_s390(key as *const aes_enckey, out, input, CPACF_DECRYPT)) {
        return;
    }
    aes_decrypt_generic((*key).inv_k.inv_rndkeys.as_ptr(), (*key).nrounds, out, input);
}

// #define aes_mod_init_arch aes_mod_init_arch
unsafe fn aes_mod_init_arch() {
    if cpu_have_feature(S390_CPU_FEATURE_MSA) {
        let mut km_functions: cpacf_mask_t = core::mem::zeroed();

        cpacf_query(CPACF_KM, &mut km_functions);
        if cpacf_test_func(&km_functions, CPACF_KM_AES_128) {
            static_branch_enable(&raw mut have_cpacf_aes128);
        }
        if cpacf_test_func(&km_functions, CPACF_KM_AES_192) {
            static_branch_enable(&raw mut have_cpacf_aes192);
        }
        if cpacf_test_func(&km_functions, CPACF_KM_AES_256) {
            static_branch_enable(&raw mut have_cpacf_aes256);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
