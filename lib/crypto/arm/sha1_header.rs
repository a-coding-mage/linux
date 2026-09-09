/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-1 optimized for ARM
 *
 * Copyright 2025 Google LLC
 */

// C dependency: <asm/simd.h>

// These names and types are supplied by the surrounding kernel translation.
extern "C" {
    fn sha1_block_data_order(state: *mut sha1_block_state, data: *const u8, nblocks: usize);
    fn sha1_transform_neon(state: *mut sha1_block_state, data: *const u8, nblocks: usize);
    fn sha1_ce_transform(state: *mut sha1_block_state, data: *const u8, nblocks: usize);

    fn static_branch_likely(key: *mut StaticKey) -> bool;
    fn static_branch_enable(key: *mut StaticKey);
    fn may_use_simd() -> bool;
}

// Opaque kernel type corresponding to DEFINE_STATIC_KEY_FALSE.
#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

static mut have_neon: StaticKey = StaticKey { _private: [] };
static mut have_ce: StaticKey = StaticKey { _private: [] };

// Supplied by the surrounding kernel translation.
#[repr(C)]
pub struct sha1_block_state {
    _private: [u8; 0],
}

#[inline]
unsafe fn sha1_blocks(
    state: *mut sha1_block_state,
    data: *const u8,
    nblocks: usize,
) {
    // CONFIG_KERNEL_MODE_NEON is a build-time condition from the C source.
    if cfg!(feature = "CONFIG_KERNEL_MODE_NEON")
        && static_branch_likely(&raw mut have_neon)
        && may_use_simd()
    {
        // C scoped_ksimd() scope.
        if static_branch_likely(&raw mut have_ce) {
            sha1_ce_transform(state, data, nblocks);
        } else {
            sha1_transform_neon(state, data, nblocks);
        }
    } else {
        sha1_block_data_order(state, data, nblocks);
    }
}

// CONFIG_KERNEL_MODE_NEON is a build-time condition from the C source.
#[cfg(feature = "CONFIG_KERNEL_MODE_NEON")]
unsafe fn sha1_mod_init_arch() {
    extern "C" {
        static elf_hwcap: usize;
        static elf_hwcap2: usize;
    }

    // HWCAP_NEON and HWCAP2_SHA1 are supplied by the surrounding kernel translation.
    if elf_hwcap & HWCAP_NEON != 0 {
        static_branch_enable(&raw mut have_neon);
        if elf_hwcap2 & HWCAP2_SHA1 != 0 {
            static_branch_enable(&raw mut have_ce);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
