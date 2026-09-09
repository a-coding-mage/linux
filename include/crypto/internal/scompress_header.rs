/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Synchronous Compression operations
 *
 * Copyright 2015 LG Electronics Inc.
 * Copyright (c) 2016, Intel Corporation
 * Author: Giovanni Cabiddu <giovanni.cabiddu@intel.com>
 */

// Dependency supplied by crypto/internal/acompress.h.

#[repr(C)]
pub struct crypto_scomp {
    pub base: crypto_tfm,
}

/**
 * struct scomp_alg - synchronous compression algorithm
 *
 * @compress: Function performs a compress operation
 * @decompress: Function performs a de-compress operation
 * @streams: Per-cpu memory for algorithm
 * @calg: Cmonn algorithm data structure shared with acomp
 * @COMP_ALG_COMMON: see struct comp_alg_common
 */
#[repr(C)]
pub struct scomp_alg {
    pub compress: Option<unsafe extern "C" fn(
        tfm: *mut crypto_scomp,
        src: *const u8,
        slen: u32,
        dst: *mut u8,
        dlen: *mut u32,
        ctx: *mut core::ffi::c_void,
    ) -> i32>,
    pub decompress: Option<unsafe extern "C" fn(
        tfm: *mut crypto_scomp,
        src: *const u8,
        slen: u32,
        dst: *mut u8,
        dlen: *mut u32,
        ctx: *mut core::ffi::c_void,
    ) -> i32>,
    pub streams: crypto_acomp_streams,
    // The C COMP_ALG_COMMON macro expands to the common algorithm fields.
    pub calg: comp_alg_common,
}

#[inline]
pub unsafe fn __crypto_scomp_alg(alg: *mut crypto_alg) -> *mut scomp_alg {
    // C: container_of(alg, struct scomp_alg, base)
    container_of!(alg, scomp_alg, base)
}

#[inline]
pub unsafe fn __crypto_scomp_tfm(tfm: *mut crypto_tfm) -> *mut crypto_scomp {
    // C: container_of(tfm, struct crypto_scomp, base)
    container_of!(tfm, crypto_scomp, base)
}

#[inline]
pub unsafe fn crypto_scomp_tfm(tfm: *mut crypto_scomp) -> *mut crypto_tfm {
    &mut (*tfm).base
}

#[inline]
pub unsafe fn crypto_free_scomp(tfm: *mut crypto_scomp) {
    crypto_destroy_tfm(tfm.cast(), crypto_scomp_tfm(tfm));
}

#[inline]
pub unsafe fn crypto_scomp_alg(tfm: *mut crypto_scomp) -> *mut scomp_alg {
    __crypto_scomp_alg((*crypto_scomp_tfm(tfm)).__crt_alg)
}

#[inline]
pub unsafe fn crypto_scomp_compress(
    tfm: *mut crypto_scomp,
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: *mut u32,
    ctx: *mut core::ffi::c_void,
) -> i32 {
    ((*crypto_scomp_alg(tfm)).compress.unwrap())(tfm, src, slen, dst, dlen, ctx)
}

#[inline]
pub unsafe fn crypto_scomp_decompress(
    tfm: *mut crypto_scomp,
    src: *const u8,
    slen: u32,
    dst: *mut u8,
    dlen: *mut u32,
    ctx: *mut core::ffi::c_void,
) -> i32 {
    ((*crypto_scomp_alg(tfm)).decompress.unwrap())(tfm, src, slen, dst, dlen, ctx)
}

/** Register synchronous compression algorithm. */
pub unsafe extern "C" fn crypto_register_scomp(alg: *mut scomp_alg) -> i32;

/** Unregister synchronous compression algorithm. */
pub unsafe extern "C" fn crypto_unregister_scomp(alg: *mut scomp_alg);

pub unsafe extern "C" fn crypto_register_scomps(alg: *mut scomp_alg, count: i32) -> i32;
pub unsafe extern "C" fn crypto_unregister_scomps(alg: *mut scomp_alg, count: i32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
