/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * GHASH and POLYVAL, x86_64 optimized
 *
 * Copyright 2025 Google LLC
 */

// C dependencies supplied by the surrounding kernel translation unit.

pub const NUM_H_POWERS: usize = 8;

// `DEFINE_STATIC_KEY_FALSE` and `__ro_after_init` are kernel build-time
// constructs; these declarations preserve the corresponding local symbols.
extern "C" {
    static mut have_pclmul: StaticKey;
    static mut have_pclmul_avx: StaticKey;

    fn polyval_mul_pclmul(a: *mut polyval_elem, b: *const polyval_elem);
    fn polyval_mul_pclmul_avx(a: *mut polyval_elem, b: *const polyval_elem);
    fn ghash_blocks_pclmul(
        acc: *mut polyval_elem,
        key: *const polyval_elem,
        data: *const u8,
        nblocks: usize,
    );
    fn polyval_blocks_pclmul_avx(
        acc: *mut polyval_elem,
        key: *const polyval_key,
        data: *const u8,
        nblocks: usize,
    );
}

// The following types and functions are provided by the generic GHASH/POLYVAL
// implementation and kernel headers.
extern "C" {
    type StaticKey;
    type polyval_elem;
    type polyval_key;
    type ghash_key;

    fn polyval_mul_generic(a: *mut polyval_elem, b: *const polyval_elem);
    fn ghash_blocks_generic(
        acc: *mut polyval_elem,
        key: *const polyval_elem,
        data: *const u8,
        nblocks: usize,
    );
    fn polyval_blocks_generic(
        acc: *mut polyval_elem,
        key: *const polyval_elem,
        data: *const u8,
        nblocks: usize,
    );
}

// `POLYVAL_BLOCK_SIZE` and `GHASH_BLOCK_SIZE` are supplied by the dependency
// headers.

#[inline]
pub unsafe fn polyval_preparekey_arch(
    key: *mut polyval_key,
    raw_key: *const u8,
) {
    // static_assert(ARRAY_SIZE(key->h_powers) == NUM_H_POWERS);
    core::ptr::copy_nonoverlapping(
        raw_key,
        (*key).h_powers.as_mut_ptr().add(NUM_H_POWERS - 1) as *mut u8,
        POLYVAL_BLOCK_SIZE,
    );
    if static_branch_likely(&have_pclmul_avx) && irq_fpu_usable() {
        kernel_fpu_begin();
        for i in (0..=(NUM_H_POWERS - 2)).rev() {
            (*key).h_powers[i] = (*key).h_powers[i + 1];
            polyval_mul_pclmul_avx(
                &mut (*key).h_powers[i],
                &(*key).h_powers[NUM_H_POWERS - 1],
            );
        }
        kernel_fpu_end();
    } else {
        for i in (0..=(NUM_H_POWERS - 2)).rev() {
            (*key).h_powers[i] = (*key).h_powers[i + 1];
            polyval_mul_generic(
                &mut (*key).h_powers[i],
                &(*key).h_powers[NUM_H_POWERS - 1],
            );
        }
    }
}

#[inline]
unsafe fn polyval_mul_x86(a: *mut polyval_elem, b: *const polyval_elem) {
    if static_branch_likely(&have_pclmul) && irq_fpu_usable() {
        kernel_fpu_begin();
        if static_branch_likely(&have_pclmul_avx) {
            polyval_mul_pclmul_avx(a, b);
        } else {
            polyval_mul_pclmul(a, b);
        }
        kernel_fpu_end();
    } else {
        polyval_mul_generic(a, b);
    }
}

#[inline]
pub unsafe fn ghash_mul_arch(acc: *mut polyval_elem, key: *const ghash_key) {
    polyval_mul_x86(acc, &(*key).h);
}

#[inline]
pub unsafe fn polyval_mul_arch(acc: *mut polyval_elem, key: *const polyval_key) {
    polyval_mul_x86(acc, &(*key).h_powers[NUM_H_POWERS - 1]);
}

#[inline]
pub unsafe fn ghash_blocks_arch(
    acc: *mut polyval_elem,
    key: *const ghash_key,
    mut data: *const u8,
    mut nblocks: usize,
) {
    if static_branch_likely(&have_pclmul) && irq_fpu_usable() {
        loop {
            let n = core::cmp::min(nblocks, 4096 / GHASH_BLOCK_SIZE);
            kernel_fpu_begin();
            ghash_blocks_pclmul(acc, &(*key).h, data, n);
            kernel_fpu_end();
            data = data.add(n * GHASH_BLOCK_SIZE);
            nblocks -= n;
            if nblocks == 0 { break; }
        }
    } else {
        ghash_blocks_generic(acc, &(*key).h, data, nblocks);
    }
}

#[inline]
pub unsafe fn polyval_blocks_arch(
    acc: *mut polyval_elem,
    key: *const polyval_key,
    mut data: *const u8,
    mut nblocks: usize,
) {
    if static_branch_likely(&have_pclmul_avx) && irq_fpu_usable() {
        loop {
            let n = core::cmp::min(nblocks, 4096 / POLYVAL_BLOCK_SIZE);
            kernel_fpu_begin();
            polyval_blocks_pclmul_avx(acc, key, data, n);
            kernel_fpu_end();
            data = data.add(n * POLYVAL_BLOCK_SIZE);
            nblocks -= n;
            if nblocks == 0 { break; }
        }
    } else {
        polyval_blocks_generic(acc, &(*key).h_powers[NUM_H_POWERS - 1], data, nblocks);
    }
}

pub unsafe fn gf128hash_mod_init_arch() {
    if boot_cpu_has(X86_FEATURE_PCLMULQDQ) {
        static_branch_enable(&mut have_pclmul);
        if boot_cpu_has(X86_FEATURE_AVX) {
            static_branch_enable(&mut have_pclmul_avx);
        }
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
