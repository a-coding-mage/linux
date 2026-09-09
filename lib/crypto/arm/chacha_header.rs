/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ChaCha and HChaCha functions (ARM optimized)
 *
 * Copyright (C) 2016-2019 Linaro, Ltd. <ard.biesheuvel@linaro.org>
 * Copyright (C) 2015 Martin Willi
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn chacha_block_xor_neon(
        state: *const crate::chacha_state,
        dst: *mut u8,
        src: *const u8,
        nrounds: i32,
    );
    fn chacha_4block_xor_neon(
        state: *const crate::chacha_state,
        dst: *mut u8,
        src: *const u8,
        nrounds: i32,
        nbytes: u32,
    );
    fn hchacha_block_arm(
        state: *const crate::chacha_state,
        out: *mut u32,
        nrounds: i32,
    );
    fn hchacha_block_neon(
        state: *const crate::chacha_state,
        out: *mut u32,
        nrounds: i32,
    );
    fn chacha_doarm(
        dst: *mut u8,
        src: *const u8,
        bytes: u32,
        state: *const crate::chacha_state,
        nrounds: i32,
    );
    fn crypto_simd_usable() -> bool;
    fn static_branch_likely(key: *const bool) -> bool;
    fn static_branch_enable(key: *mut bool);
    fn read_cpuid_part() -> u32;
}

static mut use_neon: bool = false;

#[inline]
unsafe fn neon_usable() -> bool {
    static_branch_likely(&raw const use_neon) && crypto_simd_usable()
}

unsafe fn chacha_doneon(
    state: *mut crate::chacha_state,
    mut dst: *mut u8,
    mut src: *const u8,
    mut bytes: u32,
    nrounds: i32,
) {
    let mut buf = [0u8; crate::CHACHA_BLOCK_SIZE as usize];

    while bytes > crate::CHACHA_BLOCK_SIZE {
        let l = core::cmp::min(bytes, crate::CHACHA_BLOCK_SIZE * 4u32);

        chacha_4block_xor_neon(state, dst, src, nrounds, l);
        bytes -= l;
        src = src.add(l as usize);
        dst = dst.add(l as usize);
        (*state).x[12] = (*state).x[12].wrapping_add(l.div_ceil(crate::CHACHA_BLOCK_SIZE));
    }
    if bytes != 0 {
        let mut s = src;
        let mut d = dst;

        if bytes != crate::CHACHA_BLOCK_SIZE {
            core::ptr::copy_nonoverlapping(src, buf.as_mut_ptr(), bytes as usize);
            s = buf.as_ptr();
            d = buf.as_mut_ptr();
        }
        chacha_block_xor_neon(state, d, s, nrounds);
        if d != dst {
            core::ptr::copy_nonoverlapping(buf.as_ptr(), dst, bytes as usize);
        }
        (*state).x[12] = (*state).x[12].wrapping_add(1);
    }
}

unsafe fn hchacha_block_arch(
    state: *const crate::chacha_state,
    out: *mut u32,
    nrounds: i32,
) {
    // CONFIG_KERNEL_MODE_NEON is a build-time condition from the original source.
    if !neon_usable() {
        hchacha_block_arm(state, out, nrounds);
    } else {
        hchacha_block_neon(state, out, nrounds);
    }
}

unsafe fn chacha_crypt_arch(
    state: *mut crate::chacha_state,
    mut dst: *mut u8,
    mut src: *const u8,
    mut bytes: u32,
    nrounds: i32,
) {
    // CONFIG_KERNEL_MODE_NEON is a build-time condition from the original source.
    if !neon_usable() || bytes <= crate::CHACHA_BLOCK_SIZE {
        chacha_doarm(dst, src, bytes, state, nrounds);
        (*state).x[12] = (*state).x[12].wrapping_add(bytes.div_ceil(crate::CHACHA_BLOCK_SIZE));
        return;
    }

    loop {
        let todo = core::cmp::min(bytes, 4096u32);
        chacha_doneon(state, dst, src, todo, nrounds);
        bytes -= todo;
        src = src.add(todo as usize);
        dst = dst.add(todo as usize);
        if bytes == 0 {
            break;
        }
    }
}

unsafe fn chacha_mod_init_arch() {
    // CONFIG_KERNEL_MODE_NEON is a build-time condition from the original source.
    if (crate::elf_hwcap & crate::HWCAP_NEON) != 0 {
        match read_cpuid_part() {
            crate::ARM_CPU_PART_CORTEX_A7 | crate::ARM_CPU_PART_CORTEX_A5 => {
                /*
                 * The Cortex-A7 and Cortex-A5 do not perform well with
                 * the NEON implementation but do incredibly with
                 * the scalar one and use less power.
                 */
            }
            _ => static_branch_enable(&raw mut use_neon),
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
