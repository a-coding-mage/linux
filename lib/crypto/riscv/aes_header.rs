/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 VRULL GmbH
 * Copyright (C) 2023 SiFive, Inc.
 * Copyright 2024 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/simd.h, asm/vector.h

extern "C" {
    pub fn aes_encrypt_zvkned(
        rndkeys: *const u32,
        key_len: core::ffi::c_int,
        out: *mut u8,
        input: *const u8,
    );
    pub fn aes_decrypt_zvkned(
        rndkeys: *const u32,
        key_len: core::ffi::c_int,
        out: *mut u8,
        input: *const u8,
    );
}

// DEFINE_STATIC_KEY_FALSE(have_zvkned)
static mut have_zvkned: core::ffi::c_int = 0;

unsafe fn aes_preparekey_arch(
    k: *mut aes_enckey_arch,
    inv_k: *mut aes_invkey_arch,
    in_key: *const u8,
    key_len: core::ffi::c_int,
    _nrounds: core::ffi::c_int,
) {
    aes_expandkey_generic(
        (*k).rndkeys,
        if !inv_k.is_null() {
            (*inv_k).inv_rndkeys
        } else {
            core::ptr::null_mut()
        },
        in_key,
        key_len,
    );
}

unsafe fn aes_encrypt_arch(
    key: *const aes_enckey,
    out: *mut u8,
    input: *const u8,
) {
    if static_branch_likely(&mut have_zvkned) && likely(may_use_simd()) {
        kernel_vector_begin();
        aes_encrypt_zvkned((*key).k.rndkeys, (*key).len, out, input);
        kernel_vector_end();
    } else {
        aes_encrypt_generic((*key).k.rndkeys, (*key).nrounds, out, input);
    }
}

unsafe fn aes_decrypt_arch(
    key: *const aes_key,
    out: *mut u8,
    input: *const u8,
) {
    /*
     * Note that the Zvkned code uses the standard round keys, while the
     * fallback uses the inverse round keys.  Thus both must be present.
     */
    if static_branch_likely(&mut have_zvkned) && likely(may_use_simd()) {
        kernel_vector_begin();
        aes_decrypt_zvkned((*key).k.rndkeys, (*key).len, out, input);
        kernel_vector_end();
    } else {
        aes_decrypt_generic((*key).inv_k.inv_rndkeys, (*key).nrounds, out, input);
    }
}

// #define aes_mod_init_arch aes_mod_init_arch
unsafe fn aes_mod_init_arch() {
    if riscv_isa_extension_available(core::ptr::null_mut(), ZVKNED)
        && riscv_vector_vlen() >= 128
    {
        static_branch_enable(&mut have_zvkned);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
