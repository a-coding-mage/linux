/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Cryptographic utilities
 *
 * Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependency intent from linux/unaligned.h, linux/compiler_attributes.h, and
// linux/types.h is preserved through the low-level Rust equivalents below.

extern "C" {
    pub fn __crypto_xor(dst: *mut u8, src1: *const u8, src2: *const u8, size: u32);
    pub fn __crypto_memneq(a: *const core::ffi::c_void, b: *const core::ffi::c_void, size: usize) -> usize;
}

#[inline]
pub unsafe fn crypto_xor(dst: *mut u8, src: *const u8, mut size: u32) {
    // CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS and __builtin_constant_p(size)
    // are build/compiler conditions in the original C implementation.
    if cfg!(feature = "have_efficient_unaligned_access")
        && (size as usize % core::mem::size_of::<usize>() == 0)
    {
        let mut d = dst as *mut usize;
        let mut s = src as *const usize;
        while size > 0 {
            let l = core::ptr::read_unaligned(d) ^ core::ptr::read_unaligned(s);
            s = s.add(1);
            core::ptr::write_unaligned(d, l);
            d = d.add(1);
            size -= core::mem::size_of::<usize>() as u32;
        }
    } else {
        __crypto_xor(dst, dst, src, size);
    }
}

#[inline]
pub unsafe fn crypto_xor_cpy(
    dst: *mut u8,
    src1: *const u8,
    src2: *const u8,
    mut size: u32,
) {
    // CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS and __builtin_constant_p(size)
    // are build/compiler conditions in the original C implementation.
    if cfg!(feature = "have_efficient_unaligned_access")
        && (size as usize % core::mem::size_of::<usize>() == 0)
    {
        let mut d = dst as *mut usize;
        let mut s1 = src1 as *const usize;
        let mut s2 = src2 as *const usize;
        while size > 0 {
            let l = core::ptr::read_unaligned(s1) ^ core::ptr::read_unaligned(s2);
            s1 = s1.add(1);
            s2 = s2.add(1);
            core::ptr::write_unaligned(d, l);
            d = d.add(1);
            size -= core::mem::size_of::<usize>() as u32;
        }
    } else {
        __crypto_xor(dst, src1, src2, size);
    }
}

#[inline]
pub unsafe fn crypto_memneq(
    a: *const core::ffi::c_void,
    b: *const core::ffi::c_void,
    size: usize,
) -> i32 {
    if __crypto_memneq(a, b, size) != 0 { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
