// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"
// #include "bpf_kfuncs.h"
// #include "crypto_common.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

const EINVAL: c_int = 22;

#[repr(C)]
pub struct bpf_crypto_params {
    pub type_: *const c_char,
    pub algo: *const c_char,
    pub key_len: u32,
}

#[repr(C)]
pub struct bpf_crypto_ctx {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_crypto_ctx_create(
        params: *const bpf_crypto_params,
        params__sz: u32,
        err: *mut c_int,
    ) -> *mut bpf_crypto_ctx;
    fn bpf_crypto_ctx_release(ctx: *mut bpf_crypto_ctx);
    fn bpf_crypto_ctx_acquire(ctx: *mut bpf_crypto_ctx) -> *mut bpf_crypto_ctx;
}

#[unsafe(no_mangle)]
pub static mut status: c_int = 0;

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_release(ctx: *mut c_void) -> c_int {
    let _ = ctx;

    let params = bpf_crypto_params {
        type_: c"skcipher".as_ptr(),
        algo: c"ecb(aes)".as_ptr(),
        key_len: 16,
    };

    let mut cctx: *mut bpf_crypto_ctx;
    let mut err: c_int = 0;

    status = 0;

    cctx = bpf_crypto_ctx_create(&params, size_of_val(&params) as u32, &mut err);

    if cctx.is_null() {
        status = err;
        return 0;
    }

    bpf_crypto_ctx_release(cctx);

    0
}

// __failure __msg("Unreleased reference")
#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_acquire(ctx: *mut c_void) -> c_int {
    let _ = ctx;

    let params = bpf_crypto_params {
        type_: c"skcipher".as_ptr(),
        algo: c"ecb(aes)".as_ptr(),
        key_len: 16,
    };
    let mut cctx: *mut bpf_crypto_ctx;
    let mut err: c_int = 0;

    status = 0;

    cctx = bpf_crypto_ctx_create(&params, size_of_val(&params) as u32, &mut err);

    if cctx.is_null() {
        status = err;
        return 0;
    }

    cctx = bpf_crypto_ctx_acquire(cctx);
    if cctx.is_null() {
        return -EINVAL;
    }

    bpf_crypto_ctx_release(cctx);

    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static __license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
