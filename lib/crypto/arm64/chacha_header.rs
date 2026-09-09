/*
 * ChaCha and HChaCha functions (ARM64 optimized)
 *
 * Copyright (C) 2016 - 2017 Linaro, Ltd. <ard.biesheuvel@linaro.org>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * Based on:
 * ChaCha20 256-bit cipher algorithm, RFC7539, SIMD glue code
 *
 * Copyright (C) 2015 Martin Willi
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation; either version 2 of the
 * License, or (at your option) any later version.
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn chacha_block_xor_neon(state: *const chacha_state, dst: *mut u8, src: *const u8, nrounds: i32);
    fn chacha_4block_xor_neon(
        state: *const chacha_state,
        dst: *mut u8,
        src: *const u8,
        nrounds: i32,
        bytes: i32,
    );
    fn hchacha_block_neon(state: *const chacha_state, out: *mut u32, nrounds: i32);
}

static mut have_neon: StaticKey = StaticKey;

unsafe fn chacha_doneon(
    state: *mut chacha_state,
    mut dst: *mut u8,
    mut src: *const u8,
    mut bytes: usize,
    nrounds: i32,
) {
    while bytes != 0 {
        let l = core::cmp::min(bytes, CHACHA_BLOCK_SIZE * 5);

        if l <= CHACHA_BLOCK_SIZE {
            let mut buf = [0u8; CHACHA_BLOCK_SIZE];

            core::ptr::copy_nonoverlapping(src, buf.as_mut_ptr(), l);
            chacha_block_xor_neon(state, buf.as_mut_ptr(), buf.as_ptr(), nrounds);
            core::ptr::copy_nonoverlapping(buf.as_ptr(), dst, l);
            (*state).x[12] = (*state).x[12].wrapping_add(1);
            break;
        }
        chacha_4block_xor_neon(state, dst, src, nrounds, l as i32);
        bytes -= l;
        src = src.add(l);
        dst = dst.add(l);
        (*state).x[12] = (*state).x[12].wrapping_add((l.div_ceil(CHACHA_BLOCK_SIZE)) as _);
    }
}

unsafe fn hchacha_block_arch(
    state: *const chacha_state,
    out: *mut u32,
    nrounds: i32,
) {
    if !static_branch_likely(&have_neon) || !crypto_simd_usable() {
        hchacha_block_generic(state, out, nrounds);
    } else {
        // scoped_ksimd() protects the SIMD operation in the kernel.
        hchacha_block_neon(state, out, nrounds);
    }
}

unsafe fn chacha_crypt_arch(
    state: *mut chacha_state,
    dst: *mut u8,
    src: *const u8,
    bytes: usize,
    nrounds: i32,
) {
    if !static_branch_likely(&have_neon)
        || bytes <= CHACHA_BLOCK_SIZE
        || !crypto_simd_usable()
    {
        return chacha_crypt_generic(state, dst, src, bytes, nrounds);
    }

    // scoped_ksimd() protects the SIMD operation in the kernel.
    chacha_doneon(state, dst, src, bytes, nrounds);
}

// #define chacha_mod_init_arch chacha_mod_init_arch
unsafe fn chacha_mod_init_arch() {
    if cpu_have_named_feature(ASIMD) {
        static_branch_enable(&mut have_neon);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
