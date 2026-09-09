/* SPDX-License-Identifier: GPL-2.0 */
/*
 * OpenSSL/Cryptogams accelerated Poly1305 transform for ARM
 *
 * Copyright (C) 2019 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

// C dependencies supplied by the surrounding kernel translation unit:
// asm/hwcap.h, asm/simd.h, linux/cpufeature.h, linux/jump_label.h,
// linux/kernel.h

extern "C" {
    pub fn poly1305_block_init(
        state: *mut poly1305_block_state,
        raw_key: *const u8,
    );
    pub fn poly1305_blocks_arm(
        state: *mut poly1305_block_state,
        src: *const u8,
        len: u32,
        hibit: u32,
    );
    pub fn poly1305_blocks_neon(
        state: *mut poly1305_block_state,
        src: *const u8,
        len: u32,
        hibit: u32,
    );
    pub fn poly1305_emit(
        state: *const poly1305_state,
        digest: *mut u8,
        nonce: *const u32,
    );
}

// `DEFINE_STATIC_KEY_FALSE(have_neon)`; initialized read-only after init.
static mut HAVE_NEON: bool = false;

unsafe fn poly1305_blocks(
    state: *mut poly1305_block_state,
    mut src: *const u8,
    mut len: usize,
    padbit: u32,
) {
    // CONFIG_KERNEL_MODE_NEON is a build-time kernel condition.  The
    // static-branch and SIMD-availability checks retain the source intent.
    if cfg!(feature = "CONFIG_KERNEL_MODE_NEON")
        && HAVE_NEON
        && may_use_simd()
    {
        loop {
            let todo = core::cmp::min(len, 4096usize);

            // `scoped_ksimd()` scopes the kernel SIMD context around this call.
            poly1305_blocks_neon(state, src, todo as u32, padbit);

            len -= todo;
            src = src.add(todo);
            if len == 0 {
                break;
            }
        }
    } else {
        poly1305_blocks_arm(state, src, len as u32, padbit);
    }
}

// CONFIG_KERNEL_MODE_NEON
#[cfg(feature = "CONFIG_KERNEL_MODE_NEON")]
unsafe fn poly1305_mod_init_arch() {
    if elf_hwcap & HWCAP_NEON != 0 {
        HAVE_NEON = true;
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
