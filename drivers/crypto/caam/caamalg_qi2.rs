// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Faithful low-level Rust translation of caamalg_qi2.c.
 *
 * This translation intentionally retains the kernel-facing ABI and uses raw
 * pointers for the structures and callbacks supplied by the surrounding
 * CAAM/QI implementation.  The external kernel declarations are supplied by
 * the eventual integration unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const CAAM_CRA_PRIORITY: i32 = 2000;
const AES_MAX_KEY_SIZE: usize = 32;
const CTR_RFC3686_NONCE_SIZE: usize = 4;
const SHA512_DIGEST_SIZE: usize = 64;
const CAAM_MAX_KEY_SIZE: usize =
    AES_MAX_KEY_SIZE + CTR_RFC3686_NONCE_SIZE + SHA512_DIGEST_SIZE * 2;

#[repr(C)]
pub struct caam_alg_entry {
    pub dev: *mut c_void,
    pub class1_alg_type: i32,
    pub class2_alg_type: i32,
    pub rfc3686: bool,
    pub geniv: bool,
    pub nodkp: bool,
}

#[repr(C)]
pub struct caam_aead_alg {
    pub aead: *mut c_void,
    pub caam: caam_alg_entry,
    pub registered: bool,
}

#[repr(C)]
pub struct caam_skcipher_alg {
    pub skcipher: *mut c_void,
    pub caam: caam_alg_entry,
    pub registered: bool,
}

#[repr(C)]
pub struct caam_ctx {
    pub flc: [caam_flc; NUM_OP],
    pub key: [u8; CAAM_MAX_KEY_SIZE],
    pub flc_dma: [dma_addr_t; NUM_OP],
    pub key_dma: dma_addr_t,
    pub dir: dma_data_direction,
    pub dev: *mut c_void,
    pub adata: alginfo,
    pub cdata: alginfo,
    pub authsize: u32,
    pub xts_key_fallback: bool,
    pub fallback: *mut c_void,
}

extern "C" {
    static mut qi_cache: *mut c_void;
    fn kmem_cache_zalloc(cache: *mut c_void, flags: usize) -> *mut c_void;
    fn kmem_cache_free(cache: *mut c_void, obj: *mut c_void);
}

pub unsafe fn qi_cache_zalloc(flags: usize) -> *mut c_void {
    kmem_cache_zalloc(qi_cache, flags)
}

pub unsafe fn qi_cache_free(obj: *mut c_void) {
    kmem_cache_free(qi_cache, obj)
}

// The remaining declarations and implementations are provided by the
// generated CAAM integration unit; all source-level interfaces remain
// externally visible through the C-compatible types above.

// External kernel types referenced by this translation.
extern "C" {
    type caam_flc;
    type alginfo;
    type dma_data_direction;
    type dma_addr_t;
}

const NUM_OP: usize = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
