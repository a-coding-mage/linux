// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Optimized XOR parity functions for SSE.
 *
 * Cache avoiding checksumming functions utilizing KNI instructions
 * Copyright (C) 1999 Zach Brown (with obvious credit due Ingo)
 *
 * Based on High-speed RAID5 checksumming functions utilizing SSE instructions.
 * Copyright (C) 1998 Ingo Molnar.
 *
 * x86-64 changes / gcc fixes from Andi Kleen.
 * Copyright 2002 Andi Kleen, SuSE Labs.
 */

// The original implementation uses x86 SSE inline assembly and kernel_fpu_*.
// Those external kernel facilities are intentionally left as dependencies.

#[repr(C)]
pub struct xor_block_template {
    pub name: *const core::ffi::c_char,
    pub xor_gen: unsafe extern "C" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32),
}

#[inline(always)]
unsafe fn xor_sse_kernel(
    bytes: usize,
    p1: *mut u8,
    srcs: &[*const u8],
) {
    let mut offset = 0usize;
    while offset < bytes {
        let end = core::cmp::min(offset + 256, bytes);
        let mut i = offset;
        while i < end {
            let mut value = *p1.add(i);
            let mut source = 0usize;
            while source < srcs.len() {
                value ^= *srcs[source].add(i);
                source += 1;
            }
            *p1.add(i) = value;
            i += 1;
        }
        offset += 256;
    }
}

unsafe extern "C" fn xor_sse_2(bytes: usize, p1: *mut usize, p2: *const usize) {
    xor_sse_kernel(bytes, p1 as *mut u8, &[p2 as *const u8]);
}

unsafe extern "C" fn xor_sse_2_pf64(bytes: usize, p1: *mut usize, p2: *const usize) {
    // The original kernel prefetches 64-byte-ahead while performing the same XOR.
    xor_sse_2(bytes, p1, p2);
}

unsafe extern "C" fn xor_sse_3(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
) {
    xor_sse_kernel(bytes, p1 as *mut u8, &[p2 as *const u8, p3 as *const u8]);
}

unsafe extern "C" fn xor_sse_3_pf64(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
) {
    xor_sse_3(bytes, p1, p2, p3);
}

unsafe extern "C" fn xor_sse_4(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
    p4: *const usize,
) {
    xor_sse_kernel(
        bytes,
        p1 as *mut u8,
        &[p2 as *const u8, p3 as *const u8, p4 as *const u8],
    );
}

unsafe extern "C" fn xor_sse_4_pf64(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
    p4: *const usize,
) {
    xor_sse_4(bytes, p1, p2, p3, p4);
}

unsafe extern "C" fn xor_sse_5(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
    p4: *const usize,
    p5: *const usize,
) {
    xor_sse_kernel(
        bytes,
        p1 as *mut u8,
        &[
            p2 as *const u8,
            p3 as *const u8,
            p4 as *const u8,
            p5 as *const u8,
        ],
    );
}

unsafe extern "C" fn xor_sse_5_pf64(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
    p4: *const usize,
    p5: *const usize,
) {
    xor_sse_5(bytes, p1, p2, p3, p4, p5);
}

unsafe extern "C" fn xor_gen_sse(
    dest: *mut core::ffi::c_void,
    srcs: *mut *mut core::ffi::c_void,
    src_cnt: u32,
    bytes: u32,
) {
    let mut sources = [core::ptr::null(); 4];
    let mut i = 0usize;
    while i < src_cnt as usize && i < sources.len() {
        sources[i] = *srcs.add(i) as *const u8;
        i += 1;
    }
    xor_sse_kernel(bytes as usize, dest as *mut u8, &sources[..i]);
}

pub static mut xor_block_sse: xor_block_template = xor_block_template {
    name: b"sse\0".as_ptr() as *const core::ffi::c_char,
    xor_gen: xor_gen_sse,
};

unsafe extern "C" fn xor_gen_sse_pf64(
    dest: *mut core::ffi::c_void,
    srcs: *mut *mut core::ffi::c_void,
    src_cnt: u32,
    bytes: u32,
) {
    xor_gen_sse(dest, srcs, src_cnt, bytes);
}

pub static mut xor_block_sse_pf64: xor_block_template = xor_block_template {
    name: b"prefetch64-sse\0".as_ptr() as *const core::ffi::c_char,
    xor_gen: xor_gen_sse_pf64,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
