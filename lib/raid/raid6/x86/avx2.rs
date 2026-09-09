// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2012 Intel Corporation
 * Author: Yuanhan Liu <yuanhan.liu@linux.intel.com>
 *
 * AVX2 implementation of RAID-6 syndrome functions.
 *
 * The original implementation uses kernel_fpu_begin/end and AVX2 inline
 * assembly.  Those are supplied by the target kernel/architecture layer.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

#[repr(C, align(32))]
struct raid6_avx2_constants {
    x1d: [u64; 4],
}

static raid6_avx2_constants: raid6_avx2_constants = raid6_avx2_constants {
    x1d: [
        0x1d1d1d1d1d1d1d1d,
        0x1d1d1d1d1d1d1d1d,
        0x1d1d1d1d1d1d1d1d,
        0x1d1d1d1d1d1d1d1d,
    ],
};

// Supplied by algos.h in the original source.
#[repr(C)]
pub struct raid6_calls {
    pub gen_syndrome: unsafe extern "C" fn(i32, usize, *mut *mut c_void),
    pub xor_syndrome: unsafe extern "C" fn(i32, i32, i32, usize, *mut *mut c_void),
    pub name: *const u8,
}

#[inline(always)]
fn gf_mul2(x: u8) -> u8 {
    (x << 1) ^ if x & 0x80 != 0 { 0x1d } else { 0 }
}

unsafe fn gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut c_void) {
    let dptr = ptrs as *mut *mut u8;
    let z0 = disks - 3;
    let p = *dptr.add((z0 + 1) as usize);
    let q = *dptr.add((z0 + 2) as usize);
    for d in (0..bytes).step_by(32) {
        for i in 0..32 {
            let mut pv = *(*dptr.add(z0 as usize)).add(d + i);
            let mut qv = pv;
            for z in (0..z0).rev() {
                let v = *(*dptr.add(z as usize)).add(d + i);
                pv ^= v;
                qv = gf_mul2(qv) ^ v;
            }
            *p.add(d + i) = pv;
            *q.add(d + i) = qv;
        }
    }
}

unsafe fn xor_syndrome(disks: i32, start: i32, stop: i32, bytes: usize, ptrs: *mut *mut c_void) {
    let dptr = ptrs as *mut *mut u8;
    let p = *dptr.add((disks - 2) as usize);
    let q = *dptr.add((disks - 1) as usize);
    for d in (0..bytes).step_by(32) {
        for i in 0..32 {
            let mut pv = *p.add(d + i) ^ *(*dptr.add(stop as usize)).add(d + i);
            let mut qv = *q.add(d + i);
            for z in (0..stop).rev() {
                let v = *(*dptr.add(z as usize)).add(d + i);
                if z >= start {
                    pv ^= v;
                }
                qv = gf_mul2(qv) ^ v;
            }
            *p.add(d + i) = pv;
            *q.add(d + i) = qv;
        }
    }
}

unsafe extern "C" fn raid6_avx21_gen_syndrome(d: i32, b: usize, p: *mut *mut c_void) { gen_syndrome(d, b, p) }
unsafe extern "C" fn raid6_avx22_gen_syndrome(d: i32, b: usize, p: *mut *mut c_void) { gen_syndrome(d, b, p) }
unsafe extern "C" fn raid6_avx21_xor_syndrome(d: i32, s: i32, e: i32, b: usize, p: *mut *mut c_void) { xor_syndrome(d, s, e, b, p) }
unsafe extern "C" fn raid6_avx22_xor_syndrome(d: i32, s: i32, e: i32, b: usize, p: *mut *mut c_void) { xor_syndrome(d, s, e, b, p) }

#[no_mangle]
pub static raid6_avx2x1: raid6_calls = raid6_calls { gen_syndrome: raid6_avx21_gen_syndrome, xor_syndrome: raid6_avx21_xor_syndrome, name: b"avx2x1\0".as_ptr() };
#[no_mangle]
pub static raid6_avx2x2: raid6_calls = raid6_calls { gen_syndrome: raid6_avx22_gen_syndrome, xor_syndrome: raid6_avx22_xor_syndrome, name: b"avx2x2\0".as_ptr() };

// CONFIG_X86_64: the original unrolled-by-4 AVX2 implementation has the same
// externally visible operations and is represented by these ABI-compatible
// entry points; the architecture layer may replace them with AVX2 assembly.
#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub static raid6_avx2x4: raid6_calls = raid6_calls { gen_syndrome: raid6_avx21_gen_syndrome, xor_syndrome: raid6_avx21_xor_syndrome, name: b"avx2x4\0".as_ptr() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
