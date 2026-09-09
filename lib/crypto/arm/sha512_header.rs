/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arm32-optimized SHA-512 block function
 *
 * Copyright 2025 Google LLC
 */

// C dependencies: <asm/neon.h> and <asm/simd.h>.

/// Opaque declaration corresponding to `struct sha512_block_state`.
#[repr(C)]
pub struct sha512_block_state {
    _private: [u8; 0],
}

// `DEFINE_STATIC_KEY_FALSE(have_neon)`; the kernel static-key machinery is
// supplied by the surrounding build.
static mut HAVE_NEON: bool = false;

extern "C" {
    /// C declaration: `asmlinkage void sha512_block_data_order(...)`.
    fn sha512_block_data_order(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );

    /// C declaration: `asmlinkage void sha512_block_data_order_neon(...)`.
    fn sha512_block_data_order_neon(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );

    // Kernel helpers/macros supplied by the included architecture headers.
    fn may_use_simd() -> bool;
    fn cpu_has_neon() -> bool;
}

unsafe fn sha512_blocks(
    state: *mut sha512_block_state,
    data: *const u8,
    nblocks: usize,
) {
    // `IS_ENABLED(CONFIG_KERNEL_MODE_NEON)`, `static_branch_likely`, and
    // `likely` retain their kernel build/runtime conditional intent here.
    if cfg!(feature = "CONFIG_KERNEL_MODE_NEON")
        && unsafe { HAVE_NEON }
        && unsafe { may_use_simd() }
    {
        // C's `scoped_ksimd()` scope protects this NEON call. The equivalent
        // kernel SIMD scope is provided by the surrounding Rust integration.
        unsafe { sha512_block_data_order_neon(state, data, nblocks) };
    } else {
        unsafe { sha512_block_data_order(state, data, nblocks) };
    }
}

#[cfg(feature = "CONFIG_KERNEL_MODE_NEON")]
unsafe fn sha512_mod_init_arch() {
    if unsafe { cpu_has_neon() } {
        // Equivalent of `static_branch_enable(&have_neon)`.
        unsafe { HAVE_NEON = true };
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
