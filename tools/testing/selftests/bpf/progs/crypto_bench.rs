// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include "vmlinux.h"
// #include "bpf_tracing_net.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"
// #include "bpf_kfuncs.h"
// #include "crypto_common.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;

const EINVAL: i32 = 22;
const EEXIST: i32 = 17;
const ENOENT: i32 = 2;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_crypto_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_crypto_params {
    pub type_: [core::ffi::c_char; 16],
    pub algo: [core::ffi::c_char; 128],
    pub key: [u8; 256],
    pub key_len: u32,
    pub authsize: u32,
}

#[repr(C)]
pub struct __crypto_ctx_value {
    pub ctx: *mut bpf_crypto_ctx,
}

extern "C" {
    fn bpf_crypto_ctx_create(
        params: *mut bpf_crypto_params,
        params__sz: u32,
        err: *mut core::ffi::c_int,
    ) -> *mut bpf_crypto_ctx;
    fn crypto_ctx_insert(cctx: *mut bpf_crypto_ctx) -> core::ffi::c_int;
    fn crypto_ctx_value_lookup() -> *mut __crypto_ctx_value;
    fn bpf_dynptr_from_skb(
        skb: *mut __sk_buff,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> core::ffi::c_long;
    fn bpf_dynptr_from_mem(
        data: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> core::ffi::c_long;
    fn bpf_crypto_encrypt(
        ctx: *mut bpf_crypto_ctx,
        src: *mut bpf_dynptr,
        dst: *mut bpf_dynptr,
        iv: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn bpf_crypto_decrypt(
        ctx: *mut bpf_crypto_ctx,
        src: *mut bpf_dynptr,
        dst: *mut bpf_dynptr,
        iv: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

#[no_mangle]
pub static len: u32 = 16;

/*
 * cipher[] and key[] are 8-byte aligned and 'params' is kept off the stack to
 * work around an LLVM code generation bug. clang lowers the memcpy() of these
 * byte-aligned globals into a per-byte load/store sequence staged on the stack,
 * and additionally materializes the on-stack 'struct bpf_crypto_params' twice.
 * Both blow the 512-byte BPF stack limit. Aligning the sources lets clang copy
 * word-wise, and a global 'params' removes the large object from the stack.
 */
#[repr(align(8))]
pub struct AlignedChar128(pub [core::ffi::c_char; 128]);

#[repr(align(8))]
pub struct AlignedU8_256(pub [u8; 256]);

#[no_mangle]
pub static mut cipher: AlignedChar128 = AlignedChar128([0; 128]);
#[no_mangle]
pub static mut key_len: u32 = 0;
#[no_mangle]
pub static mut authsize: u32 = 0;
#[no_mangle]
pub static mut dst: [core::ffi::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut key: AlignedU8_256 = AlignedU8_256([0; 256]);
static mut params: bpf_crypto_params = bpf_crypto_params {
    type_: [0; 16],
    algo: [0; 128],
    key: [0; 256],
    key_len: 0,
    authsize: 0,
};
#[no_mangle]
pub static mut hits: core::ffi::c_long = 0;
#[no_mangle]
pub static mut status: core::ffi::c_int = 0;

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn crypto_setup(args: *mut core::ffi::c_void) -> core::ffi::c_int {
    let mut cctx: *mut bpf_crypto_ctx;
    let mut err: core::ffi::c_int = 0;

    let _ = args;
    status = 0;

    if cipher.0[0] == 0 || key_len == 0 || key_len > 256 {
        status = -EINVAL;
        return 0;
    }

    core::ptr::copy_nonoverlapping(
        b"skcipher\0".as_ptr() as *const core::ffi::c_char,
        params.type_.as_mut_ptr(),
        core::mem::size_of_val(b"skcipher\0"),
    );
    params.key_len = key_len;
    params.authsize = authsize;
    core::ptr::copy_nonoverlapping(
        cipher.0.as_ptr(),
        params.algo.as_mut_ptr(),
        core::mem::size_of_val(&cipher.0),
    );
    core::ptr::copy_nonoverlapping(
        key.0.as_ptr(),
        params.key.as_mut_ptr(),
        core::mem::size_of_val(&key.0),
    );
    cctx = bpf_crypto_ctx_create(
        &mut params,
        core::mem::size_of::<bpf_crypto_params>() as u32,
        &mut err,
    );

    if cctx.is_null() {
        status = err;
        return 0;
    }

    err = crypto_ctx_insert(cctx);
    if err != 0 && err != -EEXIST {
        status = err;
    }

    return 0;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn crypto_encrypt(skb: *mut __sk_buff) -> core::ffi::c_int {
    let mut v: *mut __crypto_ctx_value;
    let mut ctx: *mut bpf_crypto_ctx;
    let mut psrc: bpf_dynptr = core::mem::zeroed();
    let mut pdst: bpf_dynptr = core::mem::zeroed();

    v = crypto_ctx_value_lookup();
    if v.is_null() {
        status = -ENOENT;
        return 0;
    }

    ctx = (*v).ctx;
    if ctx.is_null() {
        status = -ENOENT;
        return 0;
    }

    bpf_dynptr_from_skb(skb, 0, &mut psrc);
    bpf_dynptr_from_mem(dst.as_mut_ptr() as *mut core::ffi::c_void, len, 0, &mut pdst);

    status = bpf_crypto_encrypt(ctx, &mut psrc, &mut pdst, core::ptr::null_mut());
    let _ = core::intrinsics::atomic_xadd_seqcst(&raw mut hits, 1) + 1;

    return 0;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn crypto_decrypt(skb: *mut __sk_buff) -> core::ffi::c_int {
    let mut psrc: bpf_dynptr = core::mem::zeroed();
    let mut pdst: bpf_dynptr = core::mem::zeroed();
    let mut v: *mut __crypto_ctx_value;
    let mut ctx: *mut bpf_crypto_ctx;

    v = crypto_ctx_value_lookup();
    if v.is_null() {
        return -ENOENT;
    }

    ctx = (*v).ctx;
    if ctx.is_null() {
        return -ENOENT;
    }

    bpf_dynptr_from_skb(skb, 0, &mut psrc);
    bpf_dynptr_from_mem(dst.as_mut_ptr() as *mut core::ffi::c_void, len, 0, &mut pdst);

    status = bpf_crypto_decrypt(ctx, &mut psrc, &mut pdst, core::ptr::null_mut());
    let _ = core::intrinsics::atomic_xadd_seqcst(&raw mut hits, 1) + 1;

    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];
