// SPDX-License-Identifier: GPL-2.0
//
// Translated from ptr_ring.c. C-only includes:
// - "main.h"
// - <stdlib.h>, <stdio.h>, <string.h>, <pthread.h>, <malloc.h>,
//   <assert.h>, <errno.h>, <limits.h>
// - "../../../include/linux/ptr_ring.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

const SMP_CACHE_BYTES: usize = 64;

#[inline]
const fn cache_line_size() -> usize {
    SMP_CACHE_BYTES
}

// C macros:
// - ____cacheline_aligned_in_smp: __attribute__((aligned(SMP_CACHE_BYTES)))
// - unlikely(x): __builtin_expect(!!(x), 0)
// - likely(x): __builtin_expect(!!(x), 1)
#[inline]
const fn ALIGN(x: usize, a: usize) -> usize {
    ((x + a - 1) / a) * a
}

const SIZE_MAX: usize = usize::MAX;
const KMALLOC_MAX_SIZE: usize = SIZE_MAX;

type pthread_spinlock_t = c_int;
type spinlock_t = pthread_spinlock_t;

type gfp_t = c_int;
const __GFP_ZERO: gfp_t = 0x1;

#[repr(C)]
pub struct ptr_ring {
    pub producer: spinlock_t,
    pub consumer_head: spinlock_t,
    pub consumer_tail: spinlock_t,
    pub head: c_int,
    pub tail: c_int,
    pub size: c_int,
    pub batch: c_int,
    pub queue: *mut *mut c_void,
}

#[repr(align(64))]
struct cacheline_aligned_ptr_ring(ptr_ring);

unsafe extern "C" {
    fn memalign(alignment: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn pthread_spin_init(lock: *mut pthread_spinlock_t, pshared: c_int) -> c_int;
    fn pthread_spin_lock(lock: *mut pthread_spinlock_t) -> c_int;
    fn pthread_spin_unlock(lock: *mut pthread_spinlock_t) -> c_int;

    static mut ring_size: c_int;
    static mut param: c_int;

    fn ptr_ring_init(r: *mut ptr_ring, size: c_int, gfp: gfp_t) -> c_int;
    fn __ptr_ring_produce(r: *mut ptr_ring, ptr: *mut c_void) -> c_int;
    fn __ptr_ring_full(r: *mut ptr_ring) -> bool;
    fn __ptr_ring_empty(r: *mut ptr_ring) -> bool;
    fn __ptr_ring_consume(r: *mut ptr_ring) -> *mut c_void;
}

unsafe fn kmalloc(size: c_uint, gfp: gfp_t) -> *mut c_void {
    let p = unsafe { memalign(64, size as usize) };
    if p.is_null() {
        return p;
    }

    if (gfp & __GFP_ZERO) != 0 {
        unsafe {
            memset(p, 0, size as usize);
        }
    }
    p
}

#[inline]
unsafe fn kzalloc(size: c_uint, flags: gfp_t) -> *mut c_void {
    unsafe { kmalloc(size, flags | __GFP_ZERO) }
}

#[inline]
unsafe fn kmalloc_array(n: usize, size: usize, flags: gfp_t) -> *mut c_void {
    if size != 0 && n > SIZE_MAX / size {
        return ptr::null_mut();
    }
    unsafe { kmalloc((n * size) as c_uint, flags) }
}

#[inline]
unsafe fn kcalloc(n: usize, size: usize, flags: gfp_t) -> *mut c_void {
    unsafe { kmalloc_array(n, size, flags | __GFP_ZERO) }
}

unsafe fn kfree(p: *mut c_void) {
    if !p.is_null() {
        unsafe {
            free(p);
        }
    }
}

// C aliases:
// #define kvmalloc_array kmalloc_array
// #define kvfree kfree

unsafe fn spin_lock_init(lock: *mut spinlock_t) {
    let r = unsafe { pthread_spin_init(lock, 0) };
    assert!(r == 0);
}

unsafe fn spin_lock(lock: *mut spinlock_t) {
    let ret = unsafe { pthread_spin_lock(lock) };
    assert!(ret == 0);
}

unsafe fn spin_unlock(lock: *mut spinlock_t) {
    let ret = unsafe { pthread_spin_unlock(lock) };
    assert!(ret == 0);
}

unsafe fn spin_lock_bh(lock: *mut spinlock_t) {
    unsafe {
        spin_lock(lock);
    }
}

unsafe fn spin_unlock_bh(lock: *mut spinlock_t) {
    unsafe {
        spin_unlock(lock);
    }
}

unsafe fn spin_lock_irq(lock: *mut spinlock_t) {
    unsafe {
        spin_lock(lock);
    }
}

unsafe fn spin_unlock_irq(lock: *mut spinlock_t) {
    unsafe {
        spin_unlock(lock);
    }
}

unsafe fn spin_lock_irqsave(lock: *mut spinlock_t, _f: c_ulong) {
    unsafe {
        spin_lock(lock);
    }
}

unsafe fn spin_unlock_irqrestore(lock: *mut spinlock_t, _f: c_ulong) {
    unsafe {
        spin_unlock(lock);
    }
}

static mut headcnt: u64 = 0;
static mut tailcnt: u64 = 0;
static mut array: cacheline_aligned_ptr_ring = cacheline_aligned_ptr_ring(unsafe {
    MaybeUninit::<ptr_ring>::zeroed().assume_init()
});

/* implemented by ring */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc_ring() {
    let ret = unsafe { ptr_ring_init(&raw mut array.0, ring_size, 0) };
    assert!(ret == 0);
    /* Hacky way to poke at ring internals. Useful for testing though. */
    if unsafe { param } != 0 {
        unsafe {
            array.0.batch = param;
        }
    }
}

/* guest side */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_inbuf(_len: c_uint, buf: *mut c_void, _datap: *mut c_void) -> c_int {
    let mut ret: c_int;

    ret = unsafe { __ptr_ring_produce(&raw mut array.0, buf) };
    if ret >= 0 {
        ret = 0;
        unsafe {
            headcnt = headcnt.wrapping_add(1);
        }
    }

    ret
}

/*
 * ptr_ring API provides no way for producer to find out whether a given
 * buffer was consumed.  Our tests merely require that a successful get_buf
 * implies that add_inbuf succeed in the past, and that add_inbuf will succeed,
 * fake it accordingly.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_buf(_lenp: *mut c_uint, _bufp: *mut *mut c_void) -> *mut c_void {
    let datap: *mut c_void;

    if unsafe { tailcnt == headcnt } || unsafe { __ptr_ring_full(&raw mut array.0) } {
        datap = ptr::null_mut();
    } else {
        datap = c"Buffer\n".as_ptr() as *mut c_void;
        unsafe {
            tailcnt = tailcnt.wrapping_add(1);
        }
    }

    datap
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn used_empty() -> bool {
    unsafe { tailcnt == headcnt } || unsafe { __ptr_ring_full(&raw mut array.0) }
}

#[unsafe(no_mangle)]
pub extern "C" fn disable_call() {
    assert!(false);
}

#[unsafe(no_mangle)]
pub extern "C" fn enable_call() -> bool {
    assert!(false);
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn kick_available() {
    assert!(false);
}

/* host side */
#[unsafe(no_mangle)]
pub extern "C" fn disable_kick() {
    assert!(false);
}

#[unsafe(no_mangle)]
pub extern "C" fn enable_kick() -> bool {
    assert!(false);
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avail_empty() -> bool {
    unsafe { __ptr_ring_empty(&raw mut array.0) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_buf(_lenp: *mut c_uint, _bufp: *mut *mut c_void) -> bool {
    let ptr: *mut c_void;

    ptr = unsafe { __ptr_ring_consume(&raw mut array.0) };

    !ptr.is_null()
}

#[unsafe(no_mangle)]
pub extern "C" fn call_used() {
    assert!(false);
}
