/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SM3 using the RISC-V vector crypto extensions
 *
 * Copyright (C) 2023 VRULL GmbH
 * Author: Heiko Stuebner <heiko.stuebner@vrull.eu>
 *
 * Copyright (C) 2023 SiFive, Inc.
 * Author: Jerry Shih <jerry.shih@sifive.com>
 */

// C dependencies: <asm/simd.h>, <asm/vector.h>

/// Opaque declaration supplied by the SM3 implementation.
#[repr(C)]
pub struct sm3_block_state {
    _private: [u8; 0],
}

/// Opaque declaration supplied by the kernel static-key implementation.
#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

static mut have_extensions: StaticKey = StaticKey { _private: [] };

extern "C" {
    pub fn sm3_transform_zvksh_zvkb(
        state: *mut sm3_block_state,
        data: *const u8,
        nblocks: usize,
    );

    fn sm3_blocks_generic(
        state: *mut sm3_block_state,
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

// Build-time architecture constants supplied by the RISC-V vector headers.
extern "C" {
    static ZVKSH: u32;
    static ZVKB: u32;
}

fn sm3_blocks(state: *mut sm3_block_state, data: *const u8, nblocks: usize) {
    unsafe {
        if static_branch_likely(&have_extensions) && may_use_simd() {
            kernel_vector_begin();
            sm3_transform_zvksh_zvkb(state, data, nblocks);
            kernel_vector_end();
        } else {
            sm3_blocks_generic(state, data, nblocks);
        }
    }
}

// The self-referential C macro `#define sm3_mod_init_arch sm3_mod_init_arch`
// preserves the architecture-specific initializer name.
fn sm3_mod_init_arch() {
    unsafe {
        if riscv_isa_extension_available(core::ptr::null(), ZVKSH)
            && riscv_isa_extension_available(core::ptr::null(), ZVKB)
            && riscv_vector_vlen() >= 128
        {
            static_branch_enable(&raw mut have_extensions);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
