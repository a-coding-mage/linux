/*
 * Header file for reservations for dma-buf and ttm
 *
 * Copyright(C) 2011 Linaro Limited. All rights reserved.
 * Copyright (C) 2012-2013 Canonical Ltd
 * Copyright (C) 2012 Texas Instruments
 *
 * Translated from the C header. Included Linux dependencies are supplied by
 * other translation units.
 */

use core::ffi::c_int;

extern "C" {
    pub static mut reservation_ww_class: ww_class;

    pub fn dma_fence_put(fence: *mut dma_fence);
    pub fn dma_resv_iter_first_unlocked(cursor: *mut dma_resv_iter) -> *mut dma_fence;
    pub fn dma_resv_iter_next_unlocked(cursor: *mut dma_resv) -> *mut dma_fence;
    pub fn dma_resv_iter_first(cursor: *mut dma_resv_iter) -> *mut dma_fence;
    pub fn dma_resv_iter_next(cursor: *mut dma_resv_iter) -> *mut dma_fence;
    pub fn ww_mutex_lock(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) -> c_int;
    pub fn ww_mutex_lock_interruptible(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) -> c_int;
    pub fn ww_mutex_lock_slow(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx);
    pub fn ww_mutex_lock_slow_interruptible(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) -> c_int;
    pub fn ww_mutex_trylock(lock: *mut ww_mutex, ctx: *mut ww_acquire_ctx) -> bool;
    pub fn ww_mutex_is_locked(lock: *mut ww_mutex) -> bool;
    pub fn ww_mutex_unlock(lock: *mut ww_mutex);
    pub fn lockdep_is_held(lock: *const c_void) -> c_int;
    pub fn lockdep_assert_held(lock: *const c_void);
    pub fn dma_resv_init(obj: *mut dma_resv);
    pub fn dma_resv_fini(obj: *mut dma_resv);
    pub fn dma_resv_reserve_fences(obj: *mut dma_resv, num_fences: u32) -> c_int;
    pub fn dma_resv_add_fence(obj: *mut dma_resv, fence: *mut dma_fence, usage: dma_resv_usage);
    pub fn dma_resv_replace_fences(obj: *mut dma_resv, context: u64, fence: *mut dma_fence, usage: dma_resv_usage);
    pub fn dma_resv_get_fences(obj: *mut dma_resv, usage: dma_resv_usage, num_fences: *mut u32, fences: *mut *mut *mut dma_fence) -> c_int;
    pub fn dma_resv_get_singleton(obj: *mut dma_resv, usage: dma_resv_usage, fence: *mut *mut dma_fence) -> c_int;
    pub fn dma_resv_copy_fences(dst: *mut dma_resv, src: *mut dma_resv) -> c_int;
    pub fn dma_resv_wait_timeout(obj: *mut dma_resv, usage: dma_resv_usage, intr: bool, timeout: c_ulong) -> c_long;
    pub fn dma_resv_set_deadline(obj: *mut dma_resv, usage: dma_resv_usage, deadline: ktime_t);
    pub fn dma_resv_test_signaled(obj: *mut dma_resv, usage: dma_resv_usage) -> bool;
    pub fn dma_resv_describe(obj: *mut dma_resv, seq: *mut seq_file);
    #[cfg(feature = "CONFIG_DEBUG_MUTEXES")]
    pub fn dma_resv_reset_max_fences(obj: *mut dma_resv);
}

pub type c_void = core::ffi::c_void;
pub type c_ulong = usize;
pub type c_long = isize;

#[repr(C)] pub struct ww_class { _private: [u8; 0] }
#[repr(C)] pub struct ww_mutex { pub base: mutex, pub ctx: *mut ww_acquire_ctx }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct ww_acquire_ctx { _private: [u8; 0] }
#[repr(C)] pub struct dma_resv_list { _private: [u8; 0] }
#[repr(C)] pub struct dma_fence { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
pub type ktime_t = i64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_resv_usage {
    DMA_RESV_USAGE_KERNEL,
    DMA_RESV_USAGE_WRITE,
    DMA_RESV_USAGE_READ,
    DMA_RESV_USAGE_BOOKKEEP,
}

#[inline]
pub unsafe fn dma_resv_usage_rw(write: bool) -> dma_resv_usage {
    if write { dma_resv_usage::DMA_RESV_USAGE_READ } else { dma_resv_usage::DMA_RESV_USAGE_WRITE }
}

#[repr(C)]
pub struct dma_resv {
    pub lock: ww_mutex,
    pub fences: *mut dma_resv_list,
}

#[repr(C)]
pub struct dma_resv_iter {
    pub obj: *mut dma_resv,
    pub usage: dma_resv_usage,
    pub fence: *mut dma_fence,
    pub fence_usage: dma_resv_usage,
    pub index: u32,
    pub fences: *mut dma_resv_list,
    pub num_fences: u32,
    pub is_restarted: bool,
}

#[inline]
pub unsafe fn dma_resv_iter_begin(cursor: *mut dma_resv_iter, obj: *mut dma_resv, usage: dma_resv_usage) {
    (*cursor).obj = obj;
    (*cursor).usage = usage;
    (*cursor).fence = core::ptr::null_mut();
}

#[inline]
pub unsafe fn dma_resv_iter_end(cursor: *mut dma_resv_iter) { dma_fence_put((*cursor).fence); }
#[inline]
pub unsafe fn dma_resv_iter_usage(cursor: *mut dma_resv_iter) -> dma_resv_usage { (*cursor).fence_usage }
#[inline]
pub unsafe fn dma_resv_iter_is_restarted(cursor: *mut dma_resv_iter) -> bool { (*cursor).is_restarted }

#[macro_export]
macro_rules! dma_resv_for_each_fence_unlocked {
    ($cursor:expr, $fence:ident) => {
        let mut $fence = unsafe { $crate::dma_resv_iter_first_unlocked($cursor) };
        while !$fence.is_null() {
            $fence = unsafe { $crate::dma_resv_iter_next_unlocked($cursor) };
        }
    };
}

#[macro_export]
macro_rules! dma_resv_for_each_fence {
    ($cursor:expr, $obj:expr, $usage:expr, $fence:ident) => {
        unsafe { $crate::dma_resv_iter_begin($cursor, $obj, $usage); }
        let mut $fence = unsafe { $crate::dma_resv_iter_first($cursor) };
        while !$fence.is_null() {
            $fence = unsafe { $crate::dma_resv_iter_next($cursor) };
        }
    };
}

#[inline] pub unsafe fn dma_resv_held(obj: *mut dma_resv) -> bool { lockdep_is_held(&mut (*obj).lock.base as *mut mutex as *const c_void) != 0 }
#[inline] pub unsafe fn dma_resv_assert_held(obj: *mut dma_resv) { lockdep_assert_held(&mut (*obj).lock.base as *mut mutex as *const c_void); }

#[inline] pub unsafe fn dma_resv_lock(obj: *mut dma_resv, ctx: *mut ww_acquire_ctx) -> c_int { ww_mutex_lock(&mut (*obj).lock, ctx) }
#[inline] pub unsafe fn dma_resv_lock_interruptible(obj: *mut dma_resv, ctx: *mut ww_acquire_ctx) -> c_int { ww_mutex_lock_interruptible(&mut (*obj).lock, ctx) }
#[inline] pub unsafe fn dma_resv_lock_slow(obj: *mut dma_resv, ctx: *mut ww_acquire_ctx) { ww_mutex_lock_slow(&mut (*obj).lock, ctx); }
#[inline] pub unsafe fn dma_resv_lock_slow_interruptible(obj: *mut dma_resv, ctx: *mut ww_acquire_ctx) -> c_int { ww_mutex_lock_slow_interruptible(&mut (*obj).lock, ctx) }
#[inline] pub unsafe fn dma_resv_trylock(obj: *mut dma_resv) -> bool { ww_mutex_trylock(&mut (*obj).lock, core::ptr::null_mut()) }
#[inline] pub unsafe fn dma_resv_is_locked(obj: *mut dma_resv) -> bool { ww_mutex_is_locked(&mut (*obj).lock) }
#[inline] pub unsafe fn dma_resv_locking_ctx(obj: *mut dma_resv) -> *mut ww_acquire_ctx { core::ptr::read_volatile(&(*obj).lock.ctx) }
#[inline] pub unsafe fn dma_resv_unlock(obj: *mut dma_resv) {
    #[cfg(feature = "CONFIG_DEBUG_MUTEXES")]
    dma_resv_reset_max_fences(obj);
    ww_mutex_unlock(&mut (*obj).lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
