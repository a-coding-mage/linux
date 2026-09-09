/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AES block cipher, optimized for ARM
 *
 * Copyright (C) 2017 Linaro Ltd.
 * Copyright 2026 Google LLC
 */

// `asmlinkage` is a C calling-convention annotation; the declarations retain
// the C ABI for the externally supplied ARM implementations.
unsafe extern "C" {
    fn aes_expandkey_generic(
        rndkeys: *mut u32,
        inv_rndkeys: *mut u32,
        in_key: *const u8,
        key_len: ::core::ffi::c_int,
    );
    pub fn __aes_arm_encrypt(
        rk: *const u32,
        rounds: ::core::ffi::c_int,
        input: *const u8,
        out: *mut u8,
    );
    pub fn __aes_arm_decrypt(
        inv_rk: *const u32,
        rounds: ::core::ffi::c_int,
        input: *const u8,
        out: *mut u8,
    );
}

unsafe fn aes_preparekey_arch(
    k: *mut union aes_enckey_arch,
    inv_k: *mut union aes_invkey_arch,
    in_key: *const u8,
    key_len: ::core::ffi::c_int,
    _nrounds: ::core::ffi::c_int,
) {
    aes_expandkey_generic(
        unsafe { (*k).rndkeys.as_mut_ptr() },
        if !inv_k.is_null() {
            unsafe { (*inv_k).inv_rndkeys.as_mut_ptr() }
        } else {
            ::core::ptr::null_mut()
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
    if !IS_ENABLED(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS)
        && !IS_ALIGNED((out as usize) | (input as usize), 4)
    {
        let mut bounce_buf = [0u8; AES_BLOCK_SIZE];

        unsafe {
            ::core::ptr::copy_nonoverlapping(input, bounce_buf.as_mut_ptr(), AES_BLOCK_SIZE);
            __aes_arm_encrypt(
                (*key).k.rndkeys.as_ptr(),
                (*key).nrounds,
                bounce_buf.as_ptr(),
                bounce_buf.as_mut_ptr(),
            );
            ::core::ptr::copy_nonoverlapping(bounce_buf.as_ptr(), out, AES_BLOCK_SIZE);
        }
        return;
    }
    unsafe {
        __aes_arm_encrypt(
            (*key).k.rndkeys.as_ptr(),
            (*key).nrounds,
            input,
            out,
        );
    }
}

unsafe fn aes_decrypt_arch(
    key: *const aes_key,
    out: *mut u8,
    input: *const u8,
) {
    if !IS_ENABLED(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS)
        && !IS_ALIGNED((out as usize) | (input as usize), 4)
    {
        let mut bounce_buf = [0u8; AES_BLOCK_SIZE];

        unsafe {
            ::core::ptr::copy_nonoverlapping(input, bounce_buf.as_mut_ptr(), AES_BLOCK_SIZE);
            __aes_arm_decrypt(
                (*key).inv_k.inv_rndkeys.as_ptr(),
                (*key).nrounds,
                bounce_buf.as_ptr(),
                bounce_buf.as_mut_ptr(),
            );
            ::core::ptr::copy_nonoverlapping(bounce_buf.as_ptr(), out, AES_BLOCK_SIZE);
        }
        return;
    }
    unsafe {
        __aes_arm_decrypt(
            (*key).inv_k.inv_rndkeys.as_ptr(),
            (*key).nrounds,
            input,
            out,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
