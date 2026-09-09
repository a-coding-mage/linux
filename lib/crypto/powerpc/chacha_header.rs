/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ChaCha stream cipher (P10 accelerated)
 *
 * Copyright 2023- IBM Corp. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    pub fn chacha_p10le_8x(
        state: *const chacha_state,
        dst: *mut u8,
        src: *const u8,
        len: core::ffi::c_uint,
        nrounds: core::ffi::c_int,
    );
}

// Equivalent local representation of DEFINE_STATIC_KEY_FALSE(have_p10).
#[repr(C)]
pub struct StaticKeyFalse {
    pub enabled: bool,
}

#[no_mangle]
pub static mut have_p10: StaticKeyFalse = StaticKeyFalse { enabled: false };

unsafe fn vsx_begin() {
    preempt_disable();
    enable_kernel_vsx();
}

unsafe fn vsx_end() {
    disable_kernel_vsx();
    preempt_enable();
}

unsafe fn chacha_p10_do_8x(
    state: *mut chacha_state,
    mut dst: *mut u8,
    mut src: *const u8,
    mut bytes: core::ffi::c_uint,
    nrounds: core::ffi::c_int,
) {
    let l: core::ffi::c_uint = bytes & !0x0ff;

    if l > 0 {
        chacha_p10le_8x(state, dst, src, l, nrounds);
        bytes -= l;
        src = src.add(l as usize);
        dst = dst.add(l as usize);
        (*state).x[12] = (*state).x[12].wrapping_add(l / CHACHA_BLOCK_SIZE);
    }

    if bytes > 0 {
        chacha_crypt_generic(state, dst, src, bytes, nrounds);
    }
}

// #define hchacha_block_arch hchacha_block_generic /* not implemented yet */

unsafe fn chacha_crypt_arch(
    state: *mut chacha_state,
    mut dst: *mut u8,
    mut src: *const u8,
    mut bytes: core::ffi::c_uint,
    nrounds: core::ffi::c_int,
) {
    if !static_branch_likely(&raw const have_p10)
        || bytes <= CHACHA_BLOCK_SIZE
        || !crypto_simd_usable()
    {
        return chacha_crypt_generic(state, dst, src, bytes, nrounds);
    }

    loop {
        let todo: core::ffi::c_uint = if bytes < SZ_4K { bytes } else { SZ_4K };

        vsx_begin();
        chacha_p10_do_8x(state, dst, src, todo, nrounds);
        vsx_end();

        bytes -= todo;
        src = src.add(todo as usize);
        dst = dst.add(todo as usize);
        if bytes == 0 {
            break;
        }
    }
}

// #define chacha_mod_init_arch chacha_mod_init_arch
unsafe fn chacha_mod_init_arch() {
    if cpu_has_feature(CPU_FTR_ARCH_31) {
        static_branch_enable(&raw mut have_p10);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
