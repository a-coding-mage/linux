/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * BLAKE2b digest algorithm, NEON accelerated
 *
 * Copyright 2020 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation unit:
// `StaticKeyFalse`, `Blake2bCtx`, `u8`, `u32`, `size_t`, `elf_hwcap`,
// `HWCAP_NEON`, `SZ_4K`, `BLAKE2B_BLOCK_SIZE`, `static_branch_likely`,
// `may_use_simd`, `scoped_ksimd`, `static_branch_enable`, and the generic
// compression routine.

static mut have_neon: StaticKeyFalse = StaticKeyFalse::new();

extern "C" {
    fn blake2b_compress_neon(
        ctx: *mut Blake2bCtx,
        data: *const u8,
        nblocks: size_t,
        inc: u32,
    );
}

unsafe fn blake2b_compress(
    ctx: *mut Blake2bCtx,
    mut data: *const u8,
    mut nblocks: size_t,
    inc: u32,
) {
    if !static_branch_likely(&have_neon) || !may_use_simd() {
        blake2b_compress_generic(ctx, data, nblocks, inc);
        return;
    }
    loop {
        let blocks: size_t = core::cmp::min(
            nblocks,
            SZ_4K / BLAKE2B_BLOCK_SIZE,
        );

        scoped_ksimd! {
            blake2b_compress_neon(ctx, data, blocks, inc);
        }

        data = data.add(blocks * BLAKE2B_BLOCK_SIZE);
        nblocks -= blocks;
        if nblocks == 0 {
            break;
        }
    }
}

// #define blake2b_mod_init_arch blake2b_mod_init_arch
fn blake2b_mod_init_arch() {
    unsafe {
        if elf_hwcap & HWCAP_NEON != 0 {
            static_branch_enable(&mut have_neon);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
