/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of `linux/spinlock.h`.
//!
//! The original header includes architecture-, configuration-, and kernel-
//! supplied declarations. Those dependencies remain external to this file.

// LOCK_SECTION_NAME = ".text..lock." KBUILD_BASENAME
// LOCK_SECTION_START/END and __lockfunc are compiler/assembler attributes in C.

#[cfg(CONFIG_DEBUG_SPINLOCK)]
extern "C" {
    pub fn __raw_spin_lock_init(
        lock: *mut raw_spinlock_t,
        name: *const core::ffi::c_char,
        key: *mut lock_class_key,
        inner: i16,
    );
    pub fn do_raw_spin_lock(lock: *mut raw_spinlock_t);
    pub fn do_raw_spin_trylock(lock: *mut raw_spinlock_t) -> i32;
    pub fn do_raw_spin_unlock(lock: *mut raw_spinlock_t);
}

// These types and functions are supplied by the translated dependency headers.
#[allow(non_camel_case_types)]
pub type raw_spinlock_t = crate::raw_spinlock_t;
#[allow(non_camel_case_types)]
pub type spinlock_t = crate::spinlock_t;
#[allow(non_camel_case_types)]
pub type rwlock_t = crate::rwlock_t;
#[allow(non_camel_case_types)]
pub type lock_class_key = crate::lock_class_key;

extern "C" {
    pub fn _raw_spin_trylock(lock: *mut raw_spinlock_t) -> i32;
    pub fn _raw_spin_lock(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_lock_irqsave(lock: *mut raw_spinlock_t) -> c_ulong;
    pub fn _raw_spin_lock_irqsave_nested(lock: *mut raw_spinlock_t, subclass: i32) -> c_ulong;
    pub fn _raw_spin_lock_irq(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_lock_irq_disable(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_lock_bh(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_unlock(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_unlock_irq(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_unlock_irq_enable(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
    pub fn _raw_spin_unlock_bh(lock: *mut raw_spinlock_t);
    pub fn _raw_spin_trylock_bh(lock: *mut raw_spinlock_t) -> i32;
    pub fn _raw_spin_trylock_irq(lock: *mut raw_spinlock_t) -> i32;
    pub fn _raw_spin_trylock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong) -> i32;
    pub fn _raw_spin_trylock_irq_disable(lock: *mut raw_spinlock_t) -> i32;
    pub fn raw_spin_is_locked(lock: *mut raw_spinlock_t) -> i32;
    pub fn raw_spin_is_contended(lock: *mut raw_spinlock_t) -> i32;
    pub fn raw_spin_lock_init(lock: *mut raw_spinlock_t);
    pub fn preempt_model_preemptible() -> bool;
    pub fn rwlock_is_contended(lock: *mut rwlock_t) -> i32;
    pub fn spin_is_contended(lock: *mut spinlock_t) -> i32;
}

pub type c_ulong = core::ffi::c_ulong;

/* smp_mb__after_spinlock provides a full memory barrier; supplied by kcsan. */
extern "C" {
    pub fn kcsan_mb();
}

#[inline(always)]
pub unsafe fn spinlock_check(lock: *mut spinlock_t) -> *mut raw_spinlock_t {
    // C: return &lock->rlock;
    lock as *mut raw_spinlock_t
}

#[inline(always)]
pub unsafe fn spin_lock(lock: *mut spinlock_t) {
    _raw_spin_lock(spinlock_check(lock));
}

#[inline(always)]
pub unsafe fn spin_lock_bh(lock: *mut spinlock_t) {
    _raw_spin_lock_bh(spinlock_check(lock));
}

#[inline(always)]
pub unsafe fn spin_trylock(lock: *mut spinlock_t) -> i32 {
    _raw_spin_trylock(spinlock_check(lock))
}

#[inline(always)]
pub unsafe fn spin_lock_irq(lock: *mut spinlock_t) {
    _raw_spin_lock_irq(spinlock_check(lock));
}

#[inline(always)]
pub unsafe fn spin_lock_irq_disable(lock: *mut spinlock_t) {
    _raw_spin_lock_irq_disable(spinlock_check(lock));
}

#[inline(always)]
pub unsafe fn spin_unlock(lock: *mut spinlock_t) {
    _raw_spin_unlock(spinlock_check(lock));
}

#[inline(always)]
pub unsafe fn spin_unlock_bh(lock: *mut spinlock_t) {
    _raw_spin_unlock_bh(spinlock_check(lock));
}

#[inline(always)]
pub unsafe fn spin_unlock_irq(lock: *mut spinlock_t) {
    _raw_spin_unlock_irq(spinlock_check(lock));
}

#[inline(always)]
pub unsafe fn spin_unlock_irq_enable(lock: *mut spinlock_t) {
    _raw_spin_unlock_irq_enable(spinlock_check(lock));
}

#[inline(always)]
pub unsafe fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong) {
    _raw_spin_unlock_irqrestore(spinlock_check(lock), flags);
}

#[inline(always)]
pub unsafe fn spin_trylock_bh(lock: *mut spinlock_t) -> i32 {
    _raw_spin_trylock_bh(spinlock_check(lock))
}

#[inline(always)]
pub unsafe fn spin_trylock_irq(lock: *mut spinlock_t) -> i32 {
    _raw_spin_trylock_irq(spinlock_check(lock))
}

#[inline(always)]
pub unsafe fn _spin_trylock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong) -> bool {
    _raw_spin_trylock_irqsave(spinlock_check(lock), flags) != 0
}

#[inline(always)]
pub unsafe fn spin_trylock_irq_disable(lock: *mut spinlock_t) -> i32 {
    _raw_spin_trylock_irq_disable(spinlock_check(lock))
}

/** Check whether a spinlock is locked; no memory-ordering guarantee. */
#[inline(always)]
pub unsafe fn spin_is_locked(lock: *mut spinlock_t) -> i32 {
    raw_spin_is_locked(spinlock_check(lock))
}

#[inline(always)]
pub unsafe fn spin_is_contended(lock: *mut spinlock_t) -> i32 {
    raw_spin_is_contended(spinlock_check(lock))
}

#[inline]
pub unsafe fn spin_needbreak(lock: *mut spinlock_t) -> i32 {
    if !preempt_model_preemptible() { 0 } else { spin_is_contended(lock) }
}

#[inline]
pub unsafe fn rwlock_needbreak(lock: *mut rwlock_t) -> i32 {
    if !preempt_model_preemptible() { 0 } else { rwlock_is_contended(lock) }
}

// The following C macros are represented as Rust declarative macros so their
// argument evaluation and call-site expansion remain explicit.
#[macro_export]
macro_rules! spin_lock_irqsave {
    ($lock:expr, $flags:expr) => {{ $flags = unsafe { $crate::_raw_spin_lock_irqsave(unsafe { $crate::spinlock_check($lock) }) }; }};
}
#[macro_export]
macro_rules! spin_trylock_irqsave {
    ($lock:expr, $flags:expr) => { unsafe { $crate::_spin_trylock_irqsave($lock, &mut $flags) } };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
