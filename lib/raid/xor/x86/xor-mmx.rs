// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Optimized XOR parity functions for MMX.
 *
 * Copyright (C) 1998 Ingo Molnar.
 */

use core::ffi::c_void;

extern "C" {
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
}

#[repr(C)]
pub struct xor_block_template {
    pub name: *const u8,
    pub xor_gen: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, u32, u32),
}

unsafe fn xor_kernel(bytes: usize, p1: *mut usize, srcs: &[*const usize]) {
    let words = bytes / core::mem::size_of::<usize>();
    let mut i = 0usize;
    while i < words {
        let mut v = *p1.add(i);
        for src in srcs {
            v ^= *(*src).add(i);
        }
        *p1.add(i) = v;
        i += 1;
    }
}

unsafe extern "C" fn xor_pII_mmx_2(bytes: usize, p1: *mut usize, p2: *const usize) {
    xor_kernel(bytes, p1, &[p2]);
}

unsafe extern "C" fn xor_pII_mmx_3(bytes: usize, p1: *mut usize, p2: *const usize, p3: *const usize) {
    xor_kernel(bytes, p1, &[p2, p3]);
}

unsafe extern "C" fn xor_pII_mmx_4(bytes: usize, p1: *mut usize, p2: *const usize, p3: *const usize, p4: *const usize) {
    xor_kernel(bytes, p1, &[p2, p3, p4]);
}

unsafe extern "C" fn xor_pII_mmx_5(bytes: usize, p1: *mut usize, p2: *const usize, p3: *const usize, p4: *const usize, p5: *const usize) {
    xor_kernel(bytes, p1, &[p2, p3, p4, p5]);
}

unsafe extern "C" fn xor_p5_mmx_2(bytes: usize, p1: *mut usize, p2: *const usize) {
    xor_kernel(bytes, p1, &[p2]);
}

unsafe extern "C" fn xor_p5_mmx_3(bytes: usize, p1: *mut usize, p2: *const usize, p3: *const usize) {
    xor_kernel(bytes, p1, &[p2, p3]);
}

unsafe extern "C" fn xor_p5_mmx_4(bytes: usize, p1: *mut usize, p2: *const usize, p3: *const usize, p4: *const usize) {
    xor_kernel(bytes, p1, &[p2, p3, p4]);
}

unsafe extern "C" fn xor_p5_mmx_5(bytes: usize, p1: *mut usize, p2: *const usize, p3: *const usize, p4: *const usize, p5: *const usize) {
    xor_kernel(bytes, p1, &[p2, p3, p4, p5]);
}

unsafe fn xor_gen_inner(dest: *mut c_void, srcs: *mut *mut c_void, src_cnt: u32, bytes: u32) {
    let p1 = dest as *mut usize;
    let mut inputs = [core::ptr::null::<usize>(); 4];
    let mut i = 0usize;
    while i < src_cnt as usize && i < inputs.len() {
        inputs[i] = *srcs.add(i) as *const usize;
        i += 1;
    }
    xor_kernel(bytes as usize, p1, &inputs[..src_cnt as usize]);
}

unsafe extern "C" fn xor_gen_pII_mmx(dest: *mut c_void, srcs: *mut *mut c_void, src_cnt: u32, bytes: u32) {
    kernel_fpu_begin();
    xor_gen_inner(dest, srcs, src_cnt, bytes);
    kernel_fpu_end();
}

#[no_mangle]
pub static mut xor_block_pII_mmx: xor_block_template = xor_block_template {
    name: b"pII_mmx\0".as_ptr(),
    xor_gen: xor_gen_pII_mmx,
};

unsafe extern "C" fn xor_gen_p5_mmx(dest: *mut c_void, srcs: *mut *mut c_void, src_cnt: u32, bytes: u32) {
    kernel_fpu_begin();
    xor_gen_inner(dest, srcs, src_cnt, bytes);
    kernel_fpu_end();
}

#[no_mangle]
pub static mut xor_block_p5_mmx: xor_block_template = xor_block_template {
    name: b"p5_mmx\0".as_ptr(),
    xor_gen: xor_gen_p5_mmx,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
