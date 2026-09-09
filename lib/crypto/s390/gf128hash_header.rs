/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * GHASH optimized using the CP Assist for Cryptographic Functions (CPACF)
 *
 * Copyright 2026 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation are referenced
// here but are not defined in this header translation.

static mut have_cpacf_ghash: StaticKeyFalse = DEFINE_STATIC_KEY_FALSE!();

pub unsafe fn ghash_preparekey_arch(
    key: *mut ghash_key,
    raw_key: *const u8,
) {
    /* Save key in POLYVAL format for fallback */
    ghash_key_to_polyval(raw_key, core::ptr::addr_of_mut!((*key).h));

    /* Save key in GHASH format for CPACF_KIMD_GHASH */
    core::ptr::copy_nonoverlapping(
        raw_key,
        (*key).h_raw.as_mut_ptr(),
        GHASH_BLOCK_SIZE,
    );
}

pub unsafe fn ghash_blocks_arch(
    acc: *mut polyval_elem,
    key: *const ghash_key,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(core::ptr::addr_of!(have_cpacf_ghash)) {
        /*
         * CPACF_KIMD_GHASH requires the accumulator and key in a single
         * buffer, each using the GHASH convention.
         */
        let mut ctx = [[0u8; GHASH_BLOCK_SIZE]; 2];

        polyval_acc_to_ghash(acc, ctx[0].as_mut_ptr());
        core::ptr::copy_nonoverlapping(
            (*key).h_raw.as_ptr(),
            ctx[1].as_mut_ptr(),
            GHASH_BLOCK_SIZE,
        );

        cpacf_kimd(
            CPACF_KIMD_GHASH,
            ctx.as_mut_ptr() as *mut u8,
            data,
            nblocks.wrapping_mul(GHASH_BLOCK_SIZE),
        );

        ghash_acc_to_polyval(ctx[0].as_ptr(), acc);
        memzero_explicit(ctx.as_mut_ptr() as *mut u8, core::mem::size_of_val(&ctx));
    } else {
        ghash_blocks_generic(acc, core::ptr::addr_of!((*key).h), data, nblocks);
    }
}

pub unsafe fn gf128hash_mod_init_arch() {
    if cpu_have_feature(S390_CPU_FEATURE_MSA)
        && cpacf_query_func(CPACF_KIMD, CPACF_KIMD_GHASH)
    {
        static_branch_enable(core::ptr::addr_of_mut!(have_cpacf_ghash));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
