// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2002 H. Peter Anvin - All Rights Reserved
 *
 * SSE-2 implementation of RAID-6 syndrome functions
 */

use core::ffi::c_void;

#[repr(C, align(16))]
struct Raid6SseConstants { x1d: [u64; 2] }

static RAID6_SSE_CONSTANTS: Raid6SseConstants = Raid6SseConstants {
    x1d: [0x1d1d1d1d1d1d1d1d, 0x1d1d1d1d1d1d1d1d],
};

// The surrounding kernel supplies these declarations and the raid6_calls ABI.
#[repr(C)]
pub struct Raid6Calls {
    pub gen_syndrome: unsafe extern "C" fn(i32, usize, *mut *mut c_void),
    pub xor_syndrome: unsafe extern "C" fn(i32, i32, i32, usize, *mut *mut c_void),
    pub name: *const u8,
}

#[inline]
unsafe fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut r = 0u8;
    for _ in 0..8 {
        if b & 1 != 0 { r ^= a; }
        let hi = a & 0x80;
        a = a.wrapping_shl(1);
        if hi != 0 { a ^= 0x1d; }
        b >>= 1;
    }
    r
}

unsafe fn gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut c_void, step: usize) {
    let dptr = ptrs as *mut *mut u8;
    let z0 = disks - 3;
    let p = *dptr.add((z0 + 1) as usize);
    let q = *dptr.add((z0 + 2) as usize);
    let mut d = 0;
    while d < bytes {
        let mut i = 0;
        while i < step && d + i < bytes {
            let mut pv = 0u8;
            let mut qv = 0u8;
            let mut z = z0;
            while z >= 0 {
                let v = *(*dptr.add(z as usize)).add(d + i);
                pv ^= v;
                qv = gf_mul(qv, 2) ^ v;
                z -= 1;
            }
            *p.add(d + i) = pv;
            *q.add(d + i) = qv;
            i += 1;
        }
        d += step;
    }
}

unsafe fn xor_syndrome(disks: i32, start: i32, stop: i32, bytes: usize,
                       ptrs: *mut *mut c_void, step: usize) {
    let dptr = ptrs as *mut *mut u8;
    let p = *dptr.add((disks - 2) as usize);
    let q = *dptr.add((disks - 1) as usize);
    let mut d = 0;
    while d < bytes {
        let mut i = 0;
        while i < step && d + i < bytes {
            let mut pv = *p.add(d + i);
            let mut qv = *q.add(d + i);
            let mut z = stop;
            while z >= start {
                let v = *(*dptr.add(z as usize)).add(d + i);
                pv ^= v;
                qv = gf_mul(qv, 2) ^ v;
                z -= 1;
            }
            z = start - 1;
            while z >= 0 {
                qv = gf_mul(qv, 2);
                z -= 1;
            }
            *p.add(d + i) = pv;
            *q.add(d + i) = *q.add(d + i) ^ qv;
            i += 1;
        }
        d += step;
    }
}

unsafe extern "C" fn raid6_sse21_gen_syndrome(d: i32, b: usize, p: *mut *mut c_void) { gen_syndrome(d, b, p, 16); }
unsafe extern "C" fn raid6_sse22_gen_syndrome(d: i32, b: usize, p: *mut *mut c_void) { gen_syndrome(d, b, p, 32); }
unsafe extern "C" fn raid6_sse24_gen_syndrome(d: i32, b: usize, p: *mut *mut c_void) { gen_syndrome(d, b, p, 64); }
unsafe extern "C" fn raid6_sse21_xor_syndrome(d: i32, s: i32, e: i32, b: usize, p: *mut *mut c_void) { xor_syndrome(d, s, e, b, p, 16); }
unsafe extern "C" fn raid6_sse22_xor_syndrome(d: i32, s: i32, e: i32, b: usize, p: *mut *mut c_void) { xor_syndrome(d, s, e, b, p, 32); }
unsafe extern "C" fn raid6_sse24_xor_syndrome(d: i32, s: i32, e: i32, b: usize, p: *mut *mut c_void) { xor_syndrome(d, s, e, b, p, 64); }

#[no_mangle]
pub static raid6_sse2x1: Raid6Calls = Raid6Calls { gen_syndrome: raid6_sse21_gen_syndrome, xor_syndrome: raid6_sse21_xor_syndrome, name: b"sse2x1\0".as_ptr() };
#[no_mangle]
pub static raid6_sse2x2: Raid6Calls = Raid6Calls { gen_syndrome: raid6_sse22_gen_syndrome, xor_syndrome: raid6_sse22_xor_syndrome, name: b"sse2x2\0".as_ptr() };
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub static raid6_sse2x4: Raid6Calls = Raid6Calls { gen_syndrome: raid6_sse24_gen_syndrome, xor_syndrome: raid6_sse24_xor_syndrome, name: b"sse2x4\0".as_ptr() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
