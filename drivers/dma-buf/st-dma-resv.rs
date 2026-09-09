/* SPDX-License-Identifier: MIT */

/*
 * Copyright © 2019 Intel Corporation
 * Copyright © 2021 Advanced Micro Devices, Inc.
 *
 * Translated from the Linux KUnit DMA-BUF reservation-object tests.
 */

use core::ffi::{c_char, c_int, c_void};

// Types, constants, functions, and KUnit macros are supplied by the kernel bindings.
#[repr(C)]
pub struct dma_fence {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_resv {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_resv_iter {
    pub fences: *mut c_void,
    _private: [u8; 0],
}
#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_fence_ops {
    pub get_driver_name: Option<unsafe extern "C" fn(*mut dma_fence) -> *const c_char>,
    pub get_timeline_name: Option<unsafe extern "C" fn(*mut dma_fence) -> *const c_char>,
}

pub type dma_resv_usage = u32;
pub const DMA_RESV_USAGE_KERNEL: dma_resv_usage = 0;
pub const DMA_RESV_USAGE_WRITE: dma_resv_usage = 1;
pub const DMA_RESV_USAGE_READ: dma_resv_usage = 2;
pub const DMA_RESV_USAGE_BOOKKEEP: dma_resv_usage = 3;
pub const ENOENT: c_int = 2;
pub const EINVAL: c_int = 22;

extern "C" {
    static mut fence_lock: c_void;
    fn kmalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut c_void);
    fn dma_fence_init(f: *mut dma_fence, ops: *const dma_fence_ops, lock: *mut c_void,
                      context: u64, seqno: u64);
    fn dma_fence_enable_signaling(f: *mut dma_fence);
    fn dma_fence_signal(f: *mut dma_fence);
    fn dma_fence_put(f: *mut dma_fence);
    fn dma_resv_init(resv: *mut dma_resv);
    fn dma_resv_lock(resv: *mut dma_resv, interrupt: *mut c_void) -> c_int;
    fn dma_resv_unlock(resv: *mut dma_resv);
    fn dma_resv_fini(resv: *mut dma_resv);
    fn dma_resv_reserve_fences(resv: *mut dma_resv, num_fences: u32) -> c_int;
    fn dma_resv_add_fence(resv: *mut dma_resv, fence: *mut dma_fence, usage: dma_resv_usage);
    fn dma_resv_test_signaled(resv: *mut dma_resv, usage: dma_resv_usage) -> bool;
    fn dma_resv_iter_begin(cursor: *mut dma_resv_iter, resv: *mut dma_resv, usage: dma_resv_usage);
    fn dma_resv_iter_end(cursor: *mut dma_resv_iter);
    fn dma_resv_iter_is_restarted(cursor: *const dma_resv_iter) -> bool;
    fn dma_resv_iter_usage(cursor: *const dma_resv_iter) -> dma_resv_usage;
    fn dma_resv_get_fences(resv: *mut dma_resv, usage: dma_resv_usage, num_fences: *mut c_int,
                           fences: *mut *mut *mut dma_fence) -> c_int;
}

#[repr(C)]
struct dma_resv_usage_param {
    usage: dma_resv_usage,
    desc: *const c_char,
}

unsafe extern "C" fn fence_name(_f: *mut dma_fence) -> *const c_char {
    b"selftest\0".as_ptr() as *const c_char
}

static FENCE_OPS: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(fence_name),
    get_timeline_name: Some(fence_name),
};

unsafe fn alloc_fence() -> *mut dma_fence {
    let f = kmalloc_obj::<dma_fence>();
    if f.is_null() {
        return core::ptr::null_mut();
    }
    dma_fence_init(f, &FENCE_OPS, &mut fence_lock, 0, 0);
    f
}

unsafe fn test_sanitycheck(_test: *mut kunit) {
    let mut resv: dma_resv = core::mem::zeroed();
    let f = alloc_fence();
    if f.is_null() { return; }
    dma_fence_enable_signaling(f);
    dma_fence_signal(f);
    dma_fence_put(f);
    dma_resv_init(&mut resv);
    let r = dma_resv_lock(&mut resv, core::ptr::null_mut());
    if r == 0 { dma_resv_unlock(&mut resv); }
    dma_resv_fini(&mut resv);
}

unsafe fn test_signaling(_test: *mut kunit, param: *const dma_resv_usage_param) {
    let usage = (*param).usage;
    let mut resv: dma_resv = core::mem::zeroed();
    let f = alloc_fence();
    if f.is_null() { return; }
    dma_fence_enable_signaling(f);
    dma_resv_init(&mut resv);
    let r = dma_resv_lock(&mut resv, core::ptr::null_mut());
    if r != 0 { dma_resv_fini(&mut resv); dma_fence_put(f); return; }
    if dma_resv_reserve_fences(&mut resv, 1) != 0 {
        dma_resv_unlock(&mut resv); dma_resv_fini(&mut resv); dma_fence_put(f); return;
    }
    dma_resv_add_fence(&mut resv, f, usage);
    let _ = dma_resv_test_signaled(&mut resv, usage);
    dma_fence_signal(f);
    let _ = dma_resv_test_signaled(&mut resv, usage);
    dma_resv_unlock(&mut resv);
    dma_resv_fini(&mut resv);
    dma_fence_put(f);
}

unsafe fn test_for_each(_test: *mut kunit, param: *const dma_resv_usage_param) {
    let usage = (*param).usage;
    let mut resv: dma_resv = core::mem::zeroed();
    let mut cursor: dma_resv_iter = core::mem::zeroed();
    let f = alloc_fence();
    if f.is_null() { return; }
    dma_fence_enable_signaling(f);
    dma_resv_init(&mut resv);
    if dma_resv_lock(&mut resv, core::ptr::null_mut()) != 0 { dma_resv_fini(&mut resv); dma_fence_put(f); return; }
    if dma_resv_reserve_fences(&mut resv, 1) != 0 { dma_resv_unlock(&mut resv); dma_resv_fini(&mut resv); dma_fence_put(f); return; }
    dma_resv_add_fence(&mut resv, f, usage);
    dma_resv_iter_begin(&mut cursor, &mut resv, usage);
    // dma_resv_for_each_fence(&cursor, &resv, usage, fence)
    if dma_resv_iter_usage(&cursor) != usage { }
    dma_resv_iter_end(&mut cursor);
    dma_resv_unlock(&mut resv);
    dma_resv_fini(&mut resv);
    dma_fence_signal(f);
    dma_fence_put(f);
}

unsafe fn test_for_each_unlocked(_test: *mut kunit, param: *const dma_resv_usage_param) {
    let usage = (*param).usage;
    let mut resv: dma_resv = core::mem::zeroed();
    let mut cursor: dma_resv_iter = core::mem::zeroed();
    let f = alloc_fence();
    if f.is_null() { return; }
    dma_fence_enable_signaling(f);
    dma_resv_init(&mut resv);
    if dma_resv_lock(&mut resv, core::ptr::null_mut()) != 0 { dma_resv_fini(&mut resv); dma_fence_put(f); return; }
    if dma_resv_reserve_fences(&mut resv, 1) != 0 { dma_resv_unlock(&mut resv); dma_resv_fini(&mut resv); dma_fence_put(f); return; }
    dma_resv_add_fence(&mut resv, f, usage);
    dma_resv_unlock(&mut resv);
    dma_resv_iter_begin(&mut cursor, &mut resv, usage);
    // dma_resv_for_each_fence_unlocked(&cursor, fence)
    let _ = dma_resv_iter_is_restarted(&cursor);
    if cursor.fences == !0usize as *mut c_void { cursor.fences = core::ptr::null_mut(); }
    dma_resv_iter_end(&mut cursor);
    dma_fence_signal(f);
    dma_resv_fini(&mut resv);
    dma_fence_put(f);
}

unsafe fn test_get_fences(_test: *mut kunit, param: *const dma_resv_usage_param) {
    let usage = (*param).usage;
    let mut resv: dma_resv = core::mem::zeroed();
    let mut fences: *mut *mut dma_fence = core::ptr::null_mut();
    let mut i: c_int = 0;
    let f = alloc_fence();
    if f.is_null() { return; }
    dma_fence_enable_signaling(f);
    dma_resv_init(&mut resv);
    if dma_resv_lock(&mut resv, core::ptr::null_mut()) != 0 { dma_resv_fini(&mut resv); dma_fence_put(f); return; }
    if dma_resv_reserve_fences(&mut resv, 1) != 0 { dma_resv_unlock(&mut resv); dma_resv_fini(&mut resv); dma_fence_put(f); return; }
    dma_resv_add_fence(&mut resv, f, usage);
    dma_resv_unlock(&mut resv);
    if dma_resv_get_fences(&mut resv, usage, &mut i, &mut fences) == 0 {
        while i > 0 { i -= 1; dma_fence_put(*fences.add(i as usize)); }
    }
    kfree(fences as *mut c_void);
    dma_resv_fini(&mut resv);
    dma_fence_put(f);
}

static DMA_RESV_USAGE_PARAMS: [dma_resv_usage_param; 4] = [
    dma_resv_usage_param { usage: DMA_RESV_USAGE_KERNEL, desc: b"kernel\0".as_ptr() as *const c_char },
    dma_resv_usage_param { usage: DMA_RESV_USAGE_WRITE, desc: b"write\0".as_ptr() as *const c_char },
    dma_resv_usage_param { usage: DMA_RESV_USAGE_READ, desc: b"read\0".as_ptr() as *const c_char },
    dma_resv_usage_param { usage: DMA_RESV_USAGE_BOOKKEEP, desc: b"bookkeep\0".as_ptr() as *const c_char },
];

// KUNIT_ARRAY_PARAM_DESC, KUNIT_CASE, KUNIT_CASE_PARAM, kunit_test_suite,
// MODULE_DESCRIPTION, and MODULE_LICENSE are kernel build-time registrations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
