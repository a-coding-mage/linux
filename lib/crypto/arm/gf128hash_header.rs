/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * GHASH, arm optimized
 *
 * Copyright 2026 Google LLC
 */

// C dependencies supplied by the surrounding kernel translation unit:
// asm/hwcap.h, asm/neon.h, and asm/simd.h.

static mut have_neon: StaticKeyFalse = StaticKeyFalse;

unsafe extern "C" {
    fn pmull_ghash_update_p8(
        blocks: usize,
        dg: *mut polyval_elem,
        src: *const u8,
        h: *const polyval_elem,
    );
}

static unsafe fn ghash_blocks_arch(
    acc: *mut polyval_elem,
    key: *const ghash_key,
    mut data: *const u8,
    mut nblocks: usize,
) {
    if static_branch_likely(unsafe { &mut have_neon }) && may_use_simd() {
        loop {
            /* Allow rescheduling every 4 KiB. */
            let n: usize = core::cmp::min(nblocks, 4096 / GHASH_BLOCK_SIZE);

            scoped_ksimd!(pmull_ghash_update_p8(
                n,
                acc,
                data,
                unsafe { core::ptr::addr_of!((*key).h) },
            ));
            data = unsafe { data.add(n * GHASH_BLOCK_SIZE) };
            nblocks -= n;
            if nblocks == 0 {
                break;
            }
        }
    } else {
        unsafe {
            ghash_blocks_generic(
                acc,
                core::ptr::addr_of!((*key).h),
                data,
                nblocks,
            );
        }
    }
}

static unsafe fn gf128hash_mod_init_arch() {
    if elf_hwcap & HWCAP_NEON != 0 {
        unsafe { static_branch_enable(&mut have_neon) };
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
