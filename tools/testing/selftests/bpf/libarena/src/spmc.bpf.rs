// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/*
 * Copyright (c) 2025-2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2025-2026 Emil Tsalapatis <etsal@meta.com>
 */

/*
 * Rust translation of dependencies originally included from:
 * <bpf_atomic.h>
 * <libarena/common.h>
 * <libarena/asan.h>
 * <libarena/spmc.h>
 *
 * The concrete definitions for arena allocation, atomics, errno constants,
 * can_loop, and SPMC layout/constants are supplied by those dependencies.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

type u64 = u64;
type ssize_t = isize;

const EINVAL: i32 = 22;
const E2BIG: i32 = 7;
const ENOMEM: i32 = 12;
const ENOENT: i32 = 2;
const EAGAIN: i32 = 11;

extern "C" {
    static can_loop: bool;

    static SPMC_ARR_BASESZ: u64;
    static SPMC_ARR_ORDERS: i32;

    fn arena_malloc(size: usize) -> *mut core::ffi::c_void;
    fn arena_free(ptr: *mut core::ffi::c_void);

    fn smp_mb();
}

#[repr(C)]
pub struct spmc_arr {
    pub data: *mut u64,
    pub order: i32,
}

#[repr(C)]
pub struct spmc {
    pub bottom: u64,
    pub top: u64,
    pub cur: *mut spmc_arr,
    pub arr: [spmc_arr; 0],
}

#[inline(always)]
unsafe fn unlikely(v: bool) -> bool {
    v
}

#[inline(always)]
unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    core::ptr::read_volatile(p)
}

#[inline(always)]
unsafe fn WRITE_ONCE<T>(p: *mut T, v: T) {
    core::ptr::write_volatile(p, v);
}

#[inline(always)]
unsafe fn smp_load_acquire<T: Copy>(p: *const T) -> T {
    let v = core::ptr::read_volatile(p);
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Acquire);
    v
}

#[inline(always)]
unsafe fn smp_store_release<T>(p: *mut T, v: T) {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
    core::ptr::write_volatile(p, v);
}

#[inline(always)]
unsafe fn cmpxchg(p: *mut u64, old: u64, new: u64) -> u64 {
    let cur = core::ptr::read_volatile(p);
    if cur == old {
        core::ptr::write_volatile(p, new);
    }
    cur
}

#[inline(always)]
unsafe fn spmc_arr_size(spmc_arr: *mut spmc_arr) -> u64 {
    SPMC_ARR_BASESZ << (*spmc_arr).order
}

#[inline(always)]
unsafe fn spmc_arr_get(spmc_arr: *mut spmc_arr, ind: u64) -> u64 {
    let ret: u64 = READ_ONCE((*spmc_arr).data.add((ind % spmc_arr_size(spmc_arr)) as usize));

    ret
}

#[inline(always)]
unsafe fn spmc_arr_put(spmc_arr: *mut spmc_arr, ind: u64, value: u64) {
    WRITE_ONCE(
        (*spmc_arr).data.add((ind % spmc_arr_size(spmc_arr)) as usize),
        value,
    );
}

#[inline(always)]
unsafe fn spmc_arr_copy(dst: *mut spmc_arr, src: *mut spmc_arr, b: u64, t: u64) {
    let mut i: u64;

    i = t;
    while i < b && can_loop {
        spmc_arr_put(dst, i, spmc_arr_get(src, i));
        i = i.wrapping_add(1);
    }
}

#[inline(always)]
unsafe fn spmc_order_init(spmc: *mut spmc, order: i32) -> i32 {
    let arr: *mut spmc_arr = (*spmc).arr.as_mut_ptr().add(order as usize);

    if unlikely(spmc.is_null()) {
        return -EINVAL;
    }

    if order >= SPMC_ARR_ORDERS {
        return -E2BIG;
    }

    /* Already allocated? */
    if !(*arr).data.is_null() {
        return 0;
    }

    (*arr).data = arena_malloc(
        ((SPMC_ARR_BASESZ << order) as usize) * core::mem::size_of::<u64>(),
    ) as *mut u64;
    if (*arr).data.is_null() {
        return -ENOMEM;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn spmc_owned_add(spmc: *mut spmc, val: u64) -> i32 {
    let mut newarr: *mut spmc_arr;
    let mut arr: *mut spmc_arr;
    let sz: ssize_t;
    let b: u64;
    let t: u64;
    let ret: i32;

    if unlikely(spmc.is_null()) {
        return -EINVAL;
    }

    /*
     * Bottom must always be read first, also
     * see spmc_steal().
     */
    b = smp_load_acquire(core::ptr::addr_of!((*spmc).bottom));
    t = READ_ONCE(core::ptr::addr_of!((*spmc).top));
    arr = READ_ONCE(core::ptr::addr_of!((*spmc).cur));

    sz = b.wrapping_sub(t) as ssize_t;
    if sz >= spmc_arr_size(arr).wrapping_sub(1) as ssize_t {
        ret = spmc_order_init(spmc, (*arr).order + 1);
        if ret != 0 {
            return ret;
        }

        newarr = (*spmc).arr.as_mut_ptr().add(((*arr).order + 1) as usize);

        spmc_arr_copy(newarr, arr, b, t);
        smp_store_release(core::ptr::addr_of_mut!((*spmc).cur), newarr);
        arr = newarr;
    }

    spmc_arr_put(arr, b, val);
    smp_store_release(core::ptr::addr_of_mut!((*spmc).bottom), b.wrapping_add(1));

    0
}

#[no_mangle]
pub unsafe extern "C" fn spmc_owned_remove(spmc: *mut spmc, val: *mut u64) -> i32 {
    let arr: *mut spmc_arr;
    let mut ret: i32 = 0;
    let sz: ssize_t;
    let value: u64;
    let b: u64;
    let t: u64;

    if unlikely(spmc.is_null() || val.is_null()) {
        return -EINVAL;
    }

    b = READ_ONCE(core::ptr::addr_of!((*spmc).bottom)).wrapping_sub(1);
    WRITE_ONCE(core::ptr::addr_of_mut!((*spmc).bottom), b);
    smp_mb();

    t = READ_ONCE(core::ptr::addr_of!((*spmc).top));
    arr = READ_ONCE(core::ptr::addr_of!((*spmc).cur));

    sz = b.wrapping_sub(t) as ssize_t;
    if sz < 0 {
        WRITE_ONCE(core::ptr::addr_of_mut!((*spmc).bottom), t);
        return -ENOENT;
    }

    value = spmc_arr_get(arr, b);
    if sz > 0 {
        *val = value;
        return 0;
    }

    if cmpxchg(core::ptr::addr_of_mut!((*spmc).top), t, t.wrapping_add(1)) != t {
        ret = -EAGAIN;
    }

    WRITE_ONCE(core::ptr::addr_of_mut!((*spmc).bottom), t.wrapping_add(1));

    if ret != 0 {
        return ret;
    }

    *val = value;

    0
}

#[no_mangle]
pub unsafe extern "C" fn spmc_steal(spmc: *mut spmc, val: *mut u64) -> i32 {
    let arr: *mut spmc_arr;
    let sz: ssize_t;
    let value: u64;
    let b: u64;
    let t: u64;

    if unlikely(spmc.is_null() || val.is_null()) {
        return -EINVAL;
    }

    /*
     * It is important that t is read before b for
     * stealers to avoid racing with the owner.
     * Races between stealers are dealt with using
     * CAS to increment the top value below.
     */
    t = smp_load_acquire(core::ptr::addr_of!((*spmc).top));
    b = smp_load_acquire(core::ptr::addr_of!((*spmc).bottom));

    sz = b.wrapping_sub(t) as ssize_t;
    if sz <= 0 {
        return -ENOENT;
    }

    arr = smp_load_acquire(core::ptr::addr_of!((*spmc).cur));
    value = spmc_arr_get(arr, t);

    if cmpxchg(core::ptr::addr_of_mut!((*spmc).top), t, t.wrapping_add(1)) != t {
        return -EAGAIN;
    }

    *val = value;

    0
}

#[no_mangle]
pub unsafe extern "C" fn spmc_create() -> *mut spmc {
    /*
     * Marked as volatile because otherwise the array
     * reference in the internal loop gets demoted to
     * scalar and the program fails verification.
     */
    let mut spmc: *mut spmc;
    let ret: i32;
    let mut i: i32;

    spmc = arena_malloc(core::mem::size_of::<spmc>()) as *mut spmc;
    if spmc.is_null() {
        return core::ptr::null_mut();
    }

    (*spmc).bottom = 0;
    (*spmc).top = 0;

    i = 0;
    while i < SPMC_ARR_ORDERS && can_loop {
        (*(*spmc).arr.as_mut_ptr().add(i as usize)).data = core::ptr::null_mut();
        (*(*spmc).arr.as_mut_ptr().add(i as usize)).order = i;
        i += 1;
    }

    ret = spmc_order_init(spmc as *mut spmc, 0);
    if ret != 0 {
        arena_free(spmc as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }

    (*spmc).cur = (*spmc).arr.as_mut_ptr().add(0);

    spmc as *mut spmc
}

#[no_mangle]
pub unsafe extern "C" fn spmc_destroy(spmc: *mut spmc) -> i32 {
    let mut i: i32;

    if unlikely(spmc.is_null()) {
        return -EINVAL;
    }

    i = 0;
    while i < SPMC_ARR_ORDERS && can_loop {
        arena_free((*(*spmc).arr.as_mut_ptr().add(i as usize)).data as *mut core::ffi::c_void);
        i += 1;
    }

    arena_free(spmc as *mut core::ffi::c_void);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
