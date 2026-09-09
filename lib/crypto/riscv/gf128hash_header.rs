/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * GHASH, RISC-V optimized
 *
 * Copyright (C) 2023 VRULL GmbH
 * Copyright (C) 2023 SiFive, Inc.
 * Copyright 2026 Google LLC
 */

// C dependencies supplied by the surrounding kernel/Rust translation.

static mut have_zvkg: static_key_false = static_key_false { _opaque: [] };

extern "C" {
    fn ghash_zvkg(
        accumulator: *mut u8,
        key: *const u8,
        data: *const u8,
        nblocks: usize,
    );
}

// #define ghash_preparekey_arch ghash_preparekey_arch
unsafe fn ghash_preparekey_arch(
    key: *mut ghash_key,
    raw_key: *const u8,
) {
    /* Save key in POLYVAL format for fallback */
    ghash_key_to_polyval(raw_key, core::ptr::addr_of_mut!((*key).h));

    /* Save key in GHASH format for zvkg */
    memcpy(
        (*key).h_raw.as_mut_ptr(),
        raw_key,
        GHASH_BLOCK_SIZE,
    );
}

// #define ghash_blocks_arch ghash_blocks_arch
unsafe fn ghash_blocks_arch(
    acc: *mut polyval_elem,
    key: *const ghash_key,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(core::ptr::addr_of!(have_zvkg)) && likely(may_use_simd()) {
        let mut ghash_acc = [0u8; GHASH_BLOCK_SIZE];

        polyval_acc_to_ghash(acc, ghash_acc.as_mut_ptr());

        kernel_vector_begin();
        ghash_zvkg(ghash_acc.as_mut_ptr(), (*key).h_raw.as_ptr(), data, nblocks);
        kernel_vector_end();

        ghash_acc_to_polyval(ghash_acc.as_ptr(), acc);
        memzero_explicit(ghash_acc.as_mut_ptr(), core::mem::size_of_val(&ghash_acc));
    } else {
        ghash_blocks_generic(acc, core::ptr::addr_of!((*key).h), data, nblocks);
    }
}

// #define gf128hash_mod_init_arch gf128hash_mod_init_arch
unsafe fn gf128hash_mod_init_arch() {
    if riscv_isa_extension_available(core::ptr::null_mut(), ZVKG)
        && riscv_vector_vlen() >= 128
    {
        static_branch_enable(core::ptr::addr_of_mut!(have_zvkg));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
