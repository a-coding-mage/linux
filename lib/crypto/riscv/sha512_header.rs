/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-512 and SHA-384 using the RISC-V vector crypto extensions
 *
 * Copyright (C) 2023 VRULL GmbH
 * Author: Heiko Stuebner <heiko.stuebner@vrull.eu>
 *
 * Copyright (C) 2023 SiFive, Inc.
 * Author: Jerry Shih <jerry.shih@sifive.com>
 */

// C dependencies: asm/simd.h and asm/vector.h.

// DEFINE_STATIC_KEY_FALSE(have_extensions), __ro_after_init.
extern "C" {
    static mut have_extensions: StaticKey;

    fn sha512_transform_zvknhb_zvkb(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );

    fn sha512_blocks_generic(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );

    fn static_branch_likely(key: *const StaticKey) -> bool;
    fn may_use_simd() -> bool;
    fn kernel_vector_begin();
    fn kernel_vector_end();
    fn riscv_isa_extension_available(cpu: *const core::ffi::c_void, extension: u32) -> bool;
    fn riscv_vector_vlen() -> usize;
    fn static_branch_enable(key: *mut StaticKey);
}

// StaticKey, sha512_block_state, ZVKNHB, and ZVKB are supplied by the
// translated kernel dependencies.

unsafe fn sha512_blocks(
    state: *mut sha512_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(&have_extensions) && may_use_simd() {
        kernel_vector_begin();
        sha512_transform_zvknhb_zvkb(state, data, nblocks);
        kernel_vector_end();
    } else {
        sha512_blocks_generic(state, data, nblocks);
    }
}

// #define sha512_mod_init_arch sha512_mod_init_arch
unsafe fn sha512_mod_init_arch() {
    if riscv_isa_extension_available(core::ptr::null(), ZVKNHB)
        && riscv_isa_extension_available(core::ptr::null(), ZVKB)
        && riscv_vector_vlen() >= 128
    {
        static_branch_enable(&mut have_extensions);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
