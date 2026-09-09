/* SPDX-License-Identifier: GPL-2.0 */
/* Variant of atomic_t specialized for reference counts. */
/*
 * The interface matches the atomic_t interface (to aid in porting) but only
 * provides the few functions one should use for reference counting.
 *
 * Saturation semantics and memory-ordering details are preserved from the
 * original header above the declarations; this implementation uses the same
 * unchecked arithmetic followed by explicit saturation warnings.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ptr;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct refcount_t {
    pub refs: atomic_t,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct atomic_t {
    pub counter: core::sync::atomic::AtomicI32,
}

#[allow(non_camel_case_types)]
pub enum spinlock_t {}
#[repr(C)]
pub struct mutex;

pub const REFCOUNT_MAX: i32 = i32::MAX;
pub const REFCOUNT_SATURATED: i32 = i32::MIN / 2;

#[macro_export]
macro_rules! REFCOUNT_INIT {
    ($n:expr) => { refcount_t { refs: atomic_init!($n) } };
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum refcount_saturation_type {
    REFCOUNT_ADD_NOT_ZERO_OVF,
    REFCOUNT_ADD_OVF,
    REFCOUNT_ADD_UAF,
    REFCOUNT_SUB_UAF,
    REFCOUNT_DEC_LEAK,
}

extern "C" {
    pub fn refcount_warn_saturate(r: *mut refcount_t, t: refcount_saturation_type);
    pub fn atomic_set(r: *mut atomic_t, n: i32);
    pub fn atomic_set_release(r: *mut atomic_t, n: i32);
    pub fn atomic_read(r: *const atomic_t) -> u32;
    pub fn atomic_try_cmpxchg_relaxed(r: *mut atomic_t, old: *mut i32, new: i32) -> bool;
    pub fn atomic_try_cmpxchg_acquire(r: *mut atomic_t, old: *mut i32, new: i32) -> bool;
    pub fn atomic_fetch_add_relaxed(n: i32, r: *mut atomic_t) -> i32;
    pub fn atomic_fetch_sub_release(n: i32, r: *mut atomic_t) -> i32;
    pub fn smp_acquire__after_ctrl_dep();
    pub fn refcount_dec_if_one(r: *mut refcount_t) -> bool;
    pub fn refcount_dec_not_one(r: *mut refcount_t) -> bool;
    pub fn refcount_dec_and_mutex_lock(r: *mut refcount_t, lock: *mut mutex) -> bool;
    pub fn refcount_dec_and_lock(r: *mut refcount_t, lock: *mut spinlock_t) -> bool;
    pub fn refcount_dec_and_lock_irqsave(r: *mut refcount_t, lock: *mut spinlock_t, flags: *mut usize) -> bool;
}

#[inline]
pub unsafe fn refcount_set(r: *mut refcount_t, n: i32) { atomic_set(&mut (*r).refs, n); }

#[inline]
pub unsafe fn refcount_set_release(r: *mut refcount_t, n: i32) { atomic_set_release(&mut (*r).refs, n); }

#[inline]
pub unsafe fn refcount_read(r: *const refcount_t) -> u32 { atomic_read(&(*r).refs) }

#[inline]
pub unsafe fn __refcount_add_not_zero(i: i32, r: *mut refcount_t, oldp: *mut i32) -> bool {
    let mut old = refcount_read(r) as i32;
    loop {
        if old == 0 { break; }
        if !atomic_try_cmpxchg_relaxed(&mut (*r).refs, &mut old, old.wrapping_add(i)) { continue; }
        break;
    }
    if !oldp.is_null() { *oldp = old; }
    if old < 0 || old.wrapping_add(i) < 0 { refcount_warn_saturate(r, refcount_saturation_type::REFCOUNT_ADD_NOT_ZERO_OVF); }
    old != 0
}

#[inline]
pub unsafe fn refcount_add_not_zero(i: i32, r: *mut refcount_t) -> bool { __refcount_add_not_zero(i, r, ptr::null_mut()) }

#[inline]
pub unsafe fn __refcount_add_not_zero_limited_acquire(i: i32, r: *mut refcount_t, oldp: *mut i32, limit: i32) -> bool {
    let mut old = refcount_read(r) as i32;
    loop {
        if old == 0 { break; }
        if i > limit.wrapping_sub(old) { if !oldp.is_null() { *oldp = old; } return false; }
        if !atomic_try_cmpxchg_acquire(&mut (*r).refs, &mut old, old.wrapping_add(i)) { continue; }
        break;
    }
    if !oldp.is_null() { *oldp = old; }
    if old < 0 || old.wrapping_add(i) < 0 { refcount_warn_saturate(r, refcount_saturation_type::REFCOUNT_ADD_NOT_ZERO_OVF); }
    old != 0
}

#[inline]
pub unsafe fn __refcount_inc_not_zero_limited_acquire(r: *mut refcount_t, oldp: *mut i32, limit: i32) -> bool { __refcount_add_not_zero_limited_acquire(1, r, oldp, limit) }
#[inline]
pub unsafe fn __refcount_add_not_zero_acquire(i: i32, r: *mut refcount_t, oldp: *mut i32) -> bool { __refcount_add_not_zero_limited_acquire(i, r, oldp, i32::MAX) }
#[inline]
pub unsafe fn refcount_add_not_zero_acquire(i: i32, r: *mut refcount_t) -> bool { __refcount_add_not_zero_acquire(i, r, ptr::null_mut()) }

#[inline]
pub unsafe fn __refcount_add(i: i32, r: *mut refcount_t, oldp: *mut i32) {
    let old = atomic_fetch_add_relaxed(i, &mut (*r).refs);
    if !oldp.is_null() { *oldp = old; }
    if old == 0 { refcount_warn_saturate(r, refcount_saturation_type::REFCOUNT_ADD_UAF); }
    else if old < 0 || old.wrapping_add(i) < 0 { refcount_warn_saturate(r, refcount_saturation_type::REFCOUNT_ADD_OVF); }
}
#[inline]
pub unsafe fn refcount_add(i: i32, r: *mut refcount_t) { __refcount_add(i, r, ptr::null_mut()); }
#[inline]
pub unsafe fn __refcount_inc_not_zero(r: *mut refcount_t, oldp: *mut i32) -> bool { __refcount_add_not_zero(1, r, oldp) }
#[inline]
pub unsafe fn refcount_inc_not_zero(r: *mut refcount_t) -> bool { __refcount_inc_not_zero(r, ptr::null_mut()) }
#[inline]
pub unsafe fn __refcount_inc_not_zero_acquire(r: *mut refcount_t, oldp: *mut i32) -> bool { __refcount_add_not_zero_acquire(1, r, oldp) }
#[inline]
pub unsafe fn refcount_inc_not_zero_acquire(r: *mut refcount_t) -> bool { __refcount_inc_not_zero_acquire(r, ptr::null_mut()) }
#[inline]
pub unsafe fn __refcount_inc(r: *mut refcount_t, oldp: *mut i32) { __refcount_add(1, r, oldp); }
#[inline]
pub unsafe fn refcount_inc(r: *mut refcount_t) { __refcount_inc(r, ptr::null_mut()); }

#[inline]
pub unsafe fn __refcount_sub_and_test(i: i32, r: *mut refcount_t, oldp: *mut i32) -> bool {
    let old = atomic_fetch_sub_release(i, &mut (*r).refs);
    if !oldp.is_null() { *oldp = old; }
    if old > 0 && old == i { smp_acquire__after_ctrl_dep(); return true; }
    if old <= 0 || old.wrapping_sub(i) < 0 { refcount_warn_saturate(r, refcount_saturation_type::REFCOUNT_SUB_UAF); }
    false
}
#[inline]
pub unsafe fn refcount_sub_and_test(i: i32, r: *mut refcount_t) -> bool { __refcount_sub_and_test(i, r, ptr::null_mut()) }
#[inline]
pub unsafe fn __refcount_dec_and_test(r: *mut refcount_t, oldp: *mut i32) -> bool { __refcount_sub_and_test(1, r, oldp) }
#[inline]
pub unsafe fn refcount_dec_and_test(r: *mut refcount_t) -> bool { __refcount_dec_and_test(r, ptr::null_mut()) }
#[inline]
pub unsafe fn __refcount_dec(r: *mut refcount_t, oldp: *mut i32) {
    let old = atomic_fetch_sub_release(1, &mut (*r).refs);
    if !oldp.is_null() { *oldp = old; }
    if old <= 1 { refcount_warn_saturate(r, refcount_saturation_type::REFCOUNT_DEC_LEAK); }
}
#[inline]
pub unsafe fn refcount_dec(r: *mut refcount_t) { __refcount_dec(r, ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
