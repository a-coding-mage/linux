// SPDX-License-Identifier: GPL-2.0-only
/*
 * Optimized XOR parity functions for AVX
 *
 * Copyright (C) 2012 Intel Corporation
 * Author: Jim Kukunas <james.t.kukunas@linux.intel.com>
 *
 * Based on Ingo Molnar and Zach Brown's respective MMX and SSE routines
 */

// C headers and local headers provide these external kernel definitions.

use core::arch::x86_64::{__m256, _mm256_load_ps, _mm256_store_ps, _mm256_xor_ps};
use core::mem::size_of;

extern "C" {
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn xor_gen_avx_inner(dest: *mut core::ffi::c_void, srcs: *mut *mut core::ffi::c_void,
                         src_cnt: u32, bytes: u32);
}

unsafe fn xor_avx_2(mut bytes: usize, mut p0: *mut usize, mut p1: *const usize) {
    let mut lines = bytes >> 9;
    while lines != 0 {
        macro_rules! block {
            ($i:expr) => {{
                let r = _mm256_load_ps(p1.add($i / size_of::<usize>()) as *const f32);
                let r = _mm256_xor_ps(r, _mm256_load_ps(p0.add($i / size_of::<usize>()) as *const f32));
                _mm256_store_ps(p0.add($i / size_of::<usize>()) as *mut f32, r);
            }};
        }
        macro_rules! block4 { ($i:expr) => { block!(32 * $i); block!(32 * ($i + 1)); block!(32 * ($i + 2)); block!(32 * ($i + 3)); }; }
        block4!(0); block4!(4); block4!(8); block4!(12);
        p0 = (p0 as usize + 512) as *mut usize;
        p1 = (p1 as usize + 512) as *const usize;
        lines -= 1;
    }
}

unsafe fn xor_avx_3(mut bytes: usize, mut p0: *mut usize, mut p1: *const usize, mut p2: *const usize) {
    let mut lines = bytes >> 9;
    while lines != 0 {
        macro_rules! block { ($i:expr) => {{
            let mut r = _mm256_load_ps(p2.add($i / size_of::<usize>()) as *const f32);
            r = _mm256_xor_ps(r, _mm256_load_ps(p1.add($i / size_of::<usize>()) as *const f32));
            r = _mm256_xor_ps(r, _mm256_load_ps(p0.add($i / size_of::<usize>()) as *const f32));
            _mm256_store_ps(p0.add($i / size_of::<usize>()) as *mut f32, r);
        }}; }
        macro_rules! block4 { ($i:expr) => { block!(32 * $i); block!(32 * ($i + 1)); block!(32 * ($i + 2)); block!(32 * ($i + 3)); }; }
        block4!(0); block4!(4); block4!(8); block4!(12);
        p0 = (p0 as usize + 512) as *mut usize; p1 = (p1 as usize + 512) as *const usize; p2 = (p2 as usize + 512) as *const usize;
        lines -= 1;
    }
}

unsafe fn xor_avx_4(mut bytes: usize, mut p0: *mut usize, mut p1: *const usize, mut p2: *const usize, mut p3: *const usize) {
    let mut lines = bytes >> 9;
    while lines != 0 {
        macro_rules! block { ($i:expr) => {{
            let mut r = _mm256_load_ps(p3.add($i / size_of::<usize>()) as *const f32);
            r = _mm256_xor_ps(r, _mm256_load_ps(p2.add($i / size_of::<usize>()) as *const f32));
            r = _mm256_xor_ps(r, _mm256_load_ps(p1.add($i / size_of::<usize>()) as *const f32));
            r = _mm256_xor_ps(r, _mm256_load_ps(p0.add($i / size_of::<usize>()) as *const f32));
            _mm256_store_ps(p0.add($i / size_of::<usize>()) as *mut f32, r);
        }}; }
        macro_rules! block4 { ($i:expr) => { block!(32 * $i); block!(32 * ($i + 1)); block!(32 * ($i + 2)); block!(32 * ($i + 3)); }; }
        block4!(0); block4!(4); block4!(8); block4!(12);
        p0 = (p0 as usize + 512) as *mut usize; p1 = (p1 as usize + 512) as *const usize; p2 = (p2 as usize + 512) as *const usize; p3 = (p3 as usize + 512) as *const usize;
        lines -= 1;
    }
}

unsafe fn xor_avx_5(mut bytes: usize, mut p0: *mut usize, mut p1: *const usize, mut p2: *const usize, mut p3: *const usize, mut p4: *const usize) {
    let mut lines = bytes >> 9;
    while lines != 0 {
        macro_rules! block { ($i:expr) => {{
            let mut r = _mm256_load_ps(p4.add($i / size_of::<usize>()) as *const f32);
            r = _mm256_xor_ps(r, _mm256_load_ps(p3.add($i / size_of::<usize>()) as *const f32));
            r = _mm256_xor_ps(r, _mm256_load_ps(p2.add($i / size_of::<usize>()) as *const f32));
            r = _mm256_xor_ps(r, _mm256_load_ps(p1.add($i / size_of::<usize>()) as *const f32));
            r = _mm256_xor_ps(r, _mm256_load_ps(p0.add($i / size_of::<usize>()) as *const f32));
            _mm256_store_ps(p0.add($i / size_of::<usize>()) as *mut f32, r);
        }}; }
        macro_rules! block4 { ($i:expr) => { block!(32 * $i); block!(32 * ($i + 1)); block!(32 * ($i + 2)); block!(32 * ($i + 3)); }; }
        block4!(0); block4!(4); block4!(8); block4!(12);
        p0 = (p0 as usize + 512) as *mut usize; p1 = (p1 as usize + 512) as *const usize; p2 = (p2 as usize + 512) as *const usize; p3 = (p3 as usize + 512) as *const usize; p4 = (p4 as usize + 512) as *const usize;
        lines -= 1;
    }
}

// DO_XOR_BLOCKS(avx_inner, xor_avx_2, xor_avx_3, xor_avx_4, xor_avx_5);

unsafe fn xor_gen_avx(dest: *mut core::ffi::c_void, srcs: *mut *mut core::ffi::c_void, src_cnt: u32, bytes: u32) {
    kernel_fpu_begin();
    xor_gen_avx_inner(dest, srcs, src_cnt, bytes);
    kernel_fpu_end();
}

// struct xor_block_template xor_block_avx = { .name = "avx", .xor_gen = xor_gen_avx };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
