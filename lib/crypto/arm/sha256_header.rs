/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-256 optimized for ARM
 *
 * Copyright 2025 Google LLC
 */

// C dependencies: <asm/neon.h> and <asm/simd.h>.

// DEFINE_STATIC_KEY_FALSE(have_neon)
static mut have_neon: StaticKey = StaticKey::new_false();
// DEFINE_STATIC_KEY_FALSE(have_ce)
static mut have_ce: StaticKey = StaticKey::new_false();

extern "C" {
    fn sha256_block_data_order(
        state: *mut sha256_block_state,
        data: *const u8,
        nblocks: usize,
    );
    fn sha256_block_data_order_neon(
        state: *mut sha256_block_state,
        data: *const u8,
        nblocks: usize,
    );
    fn sha256_ce_transform(
        state: *mut sha256_block_state,
        data: *const u8,
        nblocks: usize,
    );
}

unsafe fn sha256_blocks(
    state: *mut sha256_block_state,
    data: *const u8,
    nblocks: usize,
) {
    // CONFIG_KERNEL_MODE_NEON is a build-time condition from the C source.
    #[cfg(CONFIG_KERNEL_MODE_NEON)]
    if static_branch_likely(&have_neon) && likely(may_use_simd()) {
        // scoped_ksimd() establishes the kernel SIMD scope.
        if static_branch_likely(&have_ce) {
            sha256_ce_transform(state, data, nblocks);
        } else {
            sha256_block_data_order_neon(state, data, nblocks);
        }
    } else {
        sha256_block_data_order(state, data, nblocks);
    }

    #[cfg(not(CONFIG_KERNEL_MODE_NEON))]
    sha256_block_data_order(state, data, nblocks);
}

// #define sha256_mod_init_arch sha256_mod_init_arch
#[cfg(CONFIG_KERNEL_MODE_NEON)]
unsafe fn sha256_mod_init_arch() {
    if elf_hwcap & HWCAP_NEON != 0 {
        static_branch_enable(&mut have_neon);
        if elf_hwcap2 & HWCAP2_SHA2 != 0 {
            static_branch_enable(&mut have_ce);
        }
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
