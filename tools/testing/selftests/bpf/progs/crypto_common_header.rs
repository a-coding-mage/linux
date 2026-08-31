/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/*
 * Rust translation of crypto_common.h.
 *
 * C-only header guards and include directives are intentionally omitted.
 * The original BPF map declaration used libbpf macros:
 *   __uint(type, BPF_MAP_TYPE_ARRAY);
 *   __type(key, int);
 *   __type(value, struct __crypto_ctx_value);
 *   __uint(max_entries, 1);
 * and placed the object in SEC(".maps").
 */

#[repr(C)]
pub struct bpf_crypto_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_crypto_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __crypto_ctx_value {
    pub ctx: *mut bpf_crypto_ctx,
}

#[repr(C)]
pub struct array_map {
    /* __uint(type, BPF_MAP_TYPE_ARRAY); */
    /* __type(key, int); */
    /* __type(value, struct __crypto_ctx_value); */
    /* __uint(max_entries, 1); */
    _private: [u8; 0],
}

unsafe extern "C" {
    #[link_name = "__crypto_ctx_map"]
    pub static mut __crypto_ctx_map: array_map;

    pub fn bpf_crypto_ctx_create(
        params: *const bpf_crypto_params,
        params__sz: u32,
        err: *mut i32,
    ) -> *mut bpf_crypto_ctx;
    pub fn bpf_crypto_ctx_acquire(ctx: *mut bpf_crypto_ctx) -> *mut bpf_crypto_ctx;
    pub fn bpf_crypto_ctx_release(ctx: *mut bpf_crypto_ctx);
    pub fn bpf_crypto_encrypt(
        ctx: *mut bpf_crypto_ctx,
        src: *const bpf_dynptr,
        dst: *const bpf_dynptr,
        iv: *const bpf_dynptr,
    ) -> i32;
    pub fn bpf_crypto_decrypt(
        ctx: *mut bpf_crypto_ctx,
        src: *const bpf_dynptr,
        dst: *const bpf_dynptr,
        iv: *const bpf_dynptr,
    ) -> i32;

    pub fn bpf_map_lookup_elem(map: *mut array_map, key: *const u32) -> *mut __crypto_ctx_value;
    pub fn bpf_map_update_elem(
        map: *mut array_map,
        key: *const u32,
        value: *const __crypto_ctx_value,
        flags: u64,
    ) -> i32;
    pub fn bpf_kptr_xchg(
        kptr: *mut *mut bpf_crypto_ctx,
        ptr: *mut bpf_crypto_ctx,
    ) -> *mut bpf_crypto_ctx;
}

pub const ENOENT: i32 = 2;
pub const EEXIST: i32 = 17;

#[inline]
pub unsafe fn crypto_ctx_value_lookup() -> *mut __crypto_ctx_value {
    let key: u32 = 0;

    unsafe { bpf_map_lookup_elem(&raw mut __crypto_ctx_map, &key) }
}

#[inline]
pub unsafe fn crypto_ctx_insert(ctx: *mut bpf_crypto_ctx) -> i32 {
    let mut local: __crypto_ctx_value;
    let mut v: *mut __crypto_ctx_value;
    let mut old: *mut bpf_crypto_ctx;
    let key: u32 = 0;
    let err: i32;

    local = __crypto_ctx_value {
        ctx: core::ptr::null_mut(),
    };
    err = unsafe { bpf_map_update_elem(&raw mut __crypto_ctx_map, &key, &local, 0) };
    if err != 0 {
        unsafe { bpf_crypto_ctx_release(ctx) };
        return err;
    }

    v = unsafe { bpf_map_lookup_elem(&raw mut __crypto_ctx_map, &key) };
    if v.is_null() {
        unsafe { bpf_crypto_ctx_release(ctx) };
        return -ENOENT;
    }

    old = unsafe { bpf_kptr_xchg(&raw mut (*v).ctx, ctx) };
    if !old.is_null() {
        unsafe { bpf_crypto_ctx_release(old) };
        return -EEXIST;
    }

    0
}
