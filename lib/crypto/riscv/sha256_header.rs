/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-256 (RISC-V accelerated)
 *
 * Copyright (C) 2022 VRULL GmbH
 * Author: Heiko Stuebner <heiko.stuebner@vrull.eu>
 *
 * Copyright (C) 2023 SiFive, Inc.
 * Author: Jerry Shih <jerry.shih@sifive.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/simd.h and asm/vector.h.

// Build-time kernel read-only-after-init/static-key declaration.
static mut have_extensions: bool = false;

extern "C" {
    fn sha256_transform_zvknha_or_zvknhb_zvkb(
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
    if static_branch_likely(&have_extensions) && likely(may_use_simd()) {
        kernel_vector_begin();
        sha256_transform_zvknha_or_zvknhb_zvkb(state, data, nblocks);
        kernel_vector_end();
    } else {
        sha256_blocks_generic(state, data, nblocks);
    }
}

// #define sha256_mod_init_arch sha256_mod_init_arch
unsafe fn sha256_mod_init_arch() {
    /* Both zvknha and zvknhb provide the SHA-256 instructions. */
    if (riscv_isa_extension_available(core::ptr::null_mut(), ZVKNHA)
        || riscv_isa_extension_available(core::ptr::null_mut(), ZVKNHB))
        && riscv_isa_extension_available(core::ptr::null_mut(), ZVKB)
        && riscv_vector_vlen() >= 128
    {
        static_branch_enable(&mut have_extensions);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
