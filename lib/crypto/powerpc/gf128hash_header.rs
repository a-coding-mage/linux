/* SPDX-License-Identifier: GPL-2.0 */
/*
 * GHASH routines supporting VMX instructions on the Power 8
 *
 * Copyright (C) 2015, 2019 International Business Machines Inc.
 * Copyright (C) 2014 - 2018 Linaro Ltd.
 * Copyright 2026 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    static have_vec_crypto: StaticKey;

    fn gcm_init_p8(htable: *mut [[u64; 2]; 4], h: *const u8);
    fn gcm_gmult_p8(xi: *mut u8, htable: *const [[u64; 2]; 4]);
    fn gcm_ghash_p8(
        xi: *mut u8,
        htable: *const [[u64; 2]; 4],
        input: *const u8,
        len: usize,
    );
}

// The concrete definition is provided by the kernel's jump-label support.
#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

pub unsafe fn ghash_preparekey_arch(
    key: *mut ghash_key,
    raw_key: *const u8,
) {
    ghash_key_to_polyval(raw_key, &mut (*key).h);

    if static_branch_likely(&have_vec_crypto) && likely(may_use_simd()) {
        preempt_disable();
        pagefault_disable();
        enable_kernel_vsx();
        gcm_init_p8((*key).htable.as_mut_ptr(), raw_key);
        disable_kernel_vsx();
        pagefault_enable();
        preempt_enable();
    } else {
        /* This reproduces gcm_init_p8() on both LE and BE systems. */
        (*key).htable[0][0] = 0;
        (*key).htable[0][1] = 0xc200000000000000;

        (*key).htable[1][0] = 0;
        (*key).htable[1][1] = le64_to_cpu((*key).h.lo);

        (*key).htable[2][0] = le64_to_cpu((*key).h.lo);
        (*key).htable[2][1] = le64_to_cpu((*key).h.hi);

        (*key).htable[3][0] = le64_to_cpu((*key).h.hi);
        (*key).htable[3][1] = 0;
    }
}

pub unsafe fn ghash_mul_arch(
    acc: *mut polyval_elem,
    key: *const ghash_key,
) {
    if static_branch_likely(&have_vec_crypto) && likely(may_use_simd()) {
        let mut ghash_acc = [0u8; GHASH_BLOCK_SIZE];

        polyval_acc_to_ghash(acc, ghash_acc.as_mut_ptr());

        preempt_disable();
        pagefault_disable();
        enable_kernel_vsx();
        gcm_gmult_p8(ghash_acc.as_mut_ptr(), (*key).htable.as_ptr());
        disable_kernel_vsx();
        pagefault_enable();
        preempt_enable();

        ghash_acc_to_polyval(ghash_acc.as_ptr(), acc);
        memzero_explicit(ghash_acc.as_mut_ptr(), ghash_acc.len());
    } else {
        polyval_mul_generic(acc, &(*key).h);
    }
}

pub unsafe fn ghash_blocks_arch(
    acc: *mut polyval_elem,
    key: *const ghash_key,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(&have_vec_crypto) && likely(may_use_simd()) {
        let mut ghash_acc = [0u8; GHASH_BLOCK_SIZE];

        polyval_acc_to_ghash(acc, ghash_acc.as_mut_ptr());

        preempt_disable();
        pagefault_disable();
        enable_kernel_vsx();
        gcm_ghash_p8(
            ghash_acc.as_mut_ptr(),
            (*key).htable.as_ptr(),
            data,
            nblocks * GHASH_BLOCK_SIZE,
        );
        disable_kernel_vsx();
        pagefault_enable();
        preempt_enable();

        ghash_acc_to_polyval(ghash_acc.as_ptr(), acc);
        memzero_explicit(ghash_acc.as_mut_ptr(), ghash_acc.len());
    } else {
        ghash_blocks_generic(acc, &(*key).h, data, nblocks);
    }
}

pub unsafe fn gf128hash_mod_init_arch() {
    if cpu_has_feature(CPU_FTR_ARCH_207S)
        && ((*cur_cpu_spec).cpu_user_features2 & PPC_FEATURE2_VEC_CRYPTO) != 0
    {
        static_branch_enable(&have_vec_crypto);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
