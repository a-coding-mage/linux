/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015 Markus Stockhausen <stockhausen@collogia.de>
 * Copyright (C) 2015 International Business Machines Inc.
 * Copyright 2026 Google LLC
 */

// External kernel dependencies supplied by other translation units.

#[cfg(feature = "CONFIG_SPE")]
extern "C" {
    pub fn ppc_expand_key_128(out: *mut u32, in_key: *const u8);
    pub fn ppc_expand_key_192(out: *mut u32, in_key: *const u8);
    pub fn ppc_expand_key_256(out: *mut u32, in_key: *const u8);
    pub fn ppc_generate_decrypt_key(out: *mut u32, in_key: *const u32, key_len: i32);
    pub fn ppc_encrypt_aes(out: *mut u8, input: *const u8, key_enc: *const u32, rounds: u32);
    pub fn ppc_decrypt_aes(out: *mut u8, input: *const u8, key_dec: *const u32, rounds: u32);
}

#[cfg(feature = "CONFIG_SPE")]
unsafe fn spe_begin() {
    // disable preemption and save users SPE registers if required
    preempt_disable();
    enable_kernel_spe();
}

#[cfg(feature = "CONFIG_SPE")]
unsafe fn spe_end() {
    disable_kernel_spe();
    // reenable preemption
    preempt_enable();
}

#[cfg(feature = "CONFIG_SPE")]
unsafe fn aes_preparekey_arch(
    k: *mut aes_enckey_arch,
    inv_k: *mut aes_invkey_arch,
    in_key: *const u8,
    key_len: i32,
    _nrounds: i32,
) {
    if key_len == AES_KEYSIZE_128 {
        ppc_expand_key_128((*k).spe_enc_key.as_mut_ptr(), in_key);
    } else if key_len == AES_KEYSIZE_192 {
        ppc_expand_key_192((*k).spe_enc_key.as_mut_ptr(), in_key);
    } else {
        ppc_expand_key_256((*k).spe_enc_key.as_mut_ptr(), in_key);
    }

    if !inv_k.is_null() {
        ppc_generate_decrypt_key(
            (*inv_k).spe_dec_key.as_mut_ptr(),
            (*k).spe_enc_key.as_ptr(),
            key_len,
        );
    }
}

#[cfg(feature = "CONFIG_SPE")]
unsafe fn aes_encrypt_arch(
    key: *const aes_enckey,
    out: *mut u8,
    input: *const u8,
) {
    spe_begin();
    ppc_encrypt_aes(out, input, (*key).k.spe_enc_key.as_ptr(), (*key).nrounds / 2 - 1);
    spe_end();
}

#[cfg(feature = "CONFIG_SPE")]
unsafe fn aes_decrypt_arch(
    key: *const aes_key,
    out: *mut u8,
    input: *const u8,
) {
    spe_begin();
    ppc_decrypt_aes(out, input, (*key).inv_k.spe_dec_key.as_ptr(), (*key).nrounds / 2 - 1);
    spe_end();
}

#[cfg(not(feature = "CONFIG_SPE"))]
unsafe fn is_vsx_format(key: *const p8_aes_key) -> bool {
    (*key).nrounds != 0
}

/*
 * Convert a round key from VSX to generic format by reflecting all 16 bytes (if
 * little endian) or reflecting the bytes in each 4-byte word (if big endian),
 * and (if apply_inv_mix=true) applying InvMixColumn to each column.
 *
 * It would be nice if the VSX and generic key formats would be compatible.  But
 * that's very difficult to do, with the assembly code having been borrowed from
 * OpenSSL and also targeted to POWER8 rather than POWER9.
 *
 * Fortunately, this conversion should only be needed in extremely rare cases,
 * possibly not at all in practice.  It's just included for full correctness.
 */
#[cfg(not(feature = "CONFIG_SPE"))]
unsafe fn rndkey_from_vsx(out: *mut u32, input: *const u32, apply_inv_mix: bool) {
    let be = IS_ENABLED(CONFIG_CPU_BIG_ENDIAN);
    let mut k0 = swab32(*input.add(0));
    let mut k1 = swab32(*input.add(1));
    let mut k2 = swab32(*input.add(2));
    let mut k3 = swab32(*input.add(3));

    if apply_inv_mix {
        k0 = inv_mix_columns(k0);
        k1 = inv_mix_columns(k1);
        k2 = inv_mix_columns(k2);
        k3 = inv_mix_columns(k3);
    }
    *out.add(0) = if be { k0 } else { k3 };
    *out.add(1) = if be { k1 } else { k2 };
    *out.add(2) = if be { k2 } else { k1 };
    *out.add(3) = if be { k3 } else { k0 };
}

#[cfg(not(feature = "CONFIG_SPE"))]
unsafe fn aes_preparekey_arch(
    k: *mut aes_enckey_arch,
    inv_k: *mut aes_invkey_arch,
    in_key: *const u8,
    key_len: i32,
    _nrounds: i32,
) {
    let keybits = 8 * key_len;
    let mut ret: i32;

    if static_branch_likely(&have_vec_crypto) && likely(may_use_simd()) {
        preempt_disable();
        pagefault_disable();
        enable_kernel_vsx();
        ret = aes_p8_set_encrypt_key(in_key, keybits, &mut (*k).p8);
        // aes_p8_set_encrypt_key() should never fail here, since the key length was already validated.
        WARN_ON_ONCE(ret);
        if !inv_k.is_null() {
            ret = aes_p8_set_decrypt_key(in_key, keybits, &mut (*inv_k).p8);
            // ... and likewise for aes_p8_set_decrypt_key().
            WARN_ON_ONCE(ret);
        }
        disable_kernel_vsx();
        pagefault_enable();
        preempt_enable();
    } else {
        aes_expandkey_generic(
            (*k).rndkeys.as_mut_ptr(),
            if inv_k.is_null() { core::ptr::null_mut() } else { (*inv_k).inv_rndkeys.as_mut_ptr() },
            in_key,
            key_len,
        );
        // Mark the key as using the generic format.
        (*k).p8.nrounds = 0;
        if !inv_k.is_null() {
            (*inv_k).p8.nrounds = 0;
        }
    }
}

#[cfg(not(feature = "CONFIG_SPE"))]
unsafe fn aes_encrypt_arch(key: *const aes_enckey, out: *mut u8, input: *const u8) {
    if static_branch_likely(&have_vec_crypto)
        && likely(is_vsx_format(&(*key).k.p8) && may_use_simd())
    {
        preempt_disable();
        pagefault_disable();
        enable_kernel_vsx();
        aes_p8_encrypt(input, out, &(*key).k.p8);
        disable_kernel_vsx();
        pagefault_enable();
        preempt_enable();
    } else if unlikely(is_vsx_format(&(*key).k.p8)) {
        // This handles the extremely rare case where VSX preparation is followed by non-VSX encryption.
        let mut rndkeys = [0u32; AES_MAX_KEYLENGTH_U32];
        let mut i = 0;
        while i < 4 * ((*key).nrounds + 1) {
            rndkey_from_vsx(rndkeys.as_mut_ptr().add(i), (*key).k.p8.rndkeys.as_ptr().add(i), false);
            i += 4;
        }
        aes_encrypt_generic(rndkeys.as_ptr(), (*key).nrounds, out, input);
    } else {
        aes_encrypt_generic((*key).k.rndkeys.as_ptr(), (*key).nrounds, out, input);
    }
}

#[cfg(not(feature = "CONFIG_SPE"))]
unsafe fn aes_decrypt_arch(key: *const aes_key, out: *mut u8, input: *const u8) {
    if static_branch_likely(&have_vec_crypto)
        && likely(is_vsx_format(&(*key).inv_k.p8) && may_use_simd())
    {
        preempt_disable();
        pagefault_disable();
        enable_kernel_vsx();
        aes_p8_decrypt(input, out, &(*key).inv_k.p8);
        disable_kernel_vsx();
        pagefault_enable();
        preempt_enable();
    } else if unlikely(is_vsx_format(&(*key).inv_k.p8)) {
        // This handles the extremely rare case where VSX preparation is followed by non-VSX decryption.
        let mut inv_rndkeys = [0u32; AES_MAX_KEYLENGTH_U32];
        let mut i = 0;
        rndkey_from_vsx(inv_rndkeys.as_mut_ptr(), (*key).inv_k.p8.rndkeys.as_ptr(), false);
        while i < 4 * (*key).nrounds - 4 {
            i += 4;
            rndkey_from_vsx(inv_rndkeys.as_mut_ptr().add(i), (*key).inv_k.p8.rndkeys.as_ptr().add(i), true);
        }
        rndkey_from_vsx(inv_rndkeys.as_mut_ptr().add(i + 4), (*key).inv_k.p8.rndkeys.as_ptr().add(i + 4), false);
        aes_decrypt_generic(inv_rndkeys.as_ptr(), (*key).nrounds, out, input);
    } else {
        aes_decrypt_generic((*key).inv_k.inv_rndkeys.as_ptr(), (*key).nrounds, out, input);
    }
}

#[cfg(not(feature = "CONFIG_SPE"))]
unsafe fn aes_mod_init_arch() {
    if cpu_has_feature(CPU_FTR_ARCH_207S)
        && (cur_cpu_spec.cpu_user_features2 & PPC_FEATURE2_VEC_CRYPTO) != 0
    {
        static_branch_enable(&have_vec_crypto);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
