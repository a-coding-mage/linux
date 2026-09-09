/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Queued spinlock
 *
 * Rust translation of asm-generic/qspinlock.h.  The atomic, tracing, and
 * qspinlock type definitions are supplied by the corresponding dependencies.
 */

extern "C" {
    pub fn queued_spin_lock_slowpath(lock: *mut qspinlock, val: u32);
    pub fn queued_spin_release_traced(lock: *mut qspinlock);
    pub fn tracepoint_enabled(point: contended_release_t) -> bool;
}

// External types and operations supplied by the included kernel headers.
#[allow(non_camel_case_types)]
pub type contended_release_t = u8;

// The following functions/macros correspond to the atomic operations used by
// the C header and are intentionally left as external dependencies.
extern "C" {
    pub fn atomic_read(v: *const atomic_t) -> i32;
    pub fn atomic_try_cmpxchg_acquire(v: *mut atomic_t, old: *mut i32, new: i32) -> bool;
    pub fn smp_store_release(v: *mut u8, value: u8);
}

#[inline(always)]
pub unsafe fn queued_spin_is_locked(lock: *mut qspinlock) -> i32 {
    /* Any non-zero state indicates that the lock is locked. */
    atomic_read(core::ptr::addr_of!((*lock).val))
}

#[inline(always)]
pub unsafe fn queued_spin_value_unlocked(lock: qspinlock) -> i32 {
    if lock.val.counter == 0 { 1 } else { 0 }
}

#[inline(always)]
pub unsafe fn queued_spin_is_contended(lock: *mut qspinlock) -> i32 {
    atomic_read(core::ptr::addr_of!((*lock).val)) & !_Q_LOCKED_MASK
}

#[inline(always)]
pub unsafe fn queued_spin_trylock(lock: *mut qspinlock) -> i32 {
    let mut val = atomic_read(core::ptr::addr_of!((*lock).val));
    if val != 0 {
        return 0;
    }
    if atomic_try_cmpxchg_acquire(core::ptr::addr_of_mut!((*lock).val), &mut val, _Q_LOCKED_VAL) {
        1
    } else {
        0
    }
}

#[inline(always)]
pub unsafe fn queued_spin_lock(lock: *mut qspinlock) {
    let mut val: i32 = 0;
    if atomic_try_cmpxchg_acquire(core::ptr::addr_of_mut!((*lock).val), &mut val, _Q_LOCKED_VAL) {
        return;
    }
    queued_spin_lock_slowpath(lock, val as u32);
}

#[inline(always)]
pub unsafe fn queued_spin_release(lock: *mut qspinlock) {
    /* unlock() needs release semantics. */
    smp_store_release(core::ptr::addr_of_mut!((*lock).locked), 0);
}

// DECLARE_TRACEPOINT(contended_release)
pub static mut contended_release: contended_release_t = 0;

#[inline(always)]
pub unsafe fn queued_spin_unlock(lock: *mut qspinlock) {
    /* CONFIG_QUEUED_SPINLOCKS_TRACE_CONTENDED_RELEASE is a build-time option. */
    if tracepoint_enabled(contended_release) {
        queued_spin_release_traced(lock);
        return;
    }
    queued_spin_release(lock);
}

#[inline(always)]
pub unsafe fn virt_spin_lock(_lock: *mut qspinlock) -> bool {
    false
}

// C architecture remapping macros:
// arch_spin_is_locked       -> queued_spin_is_locked
// arch_spin_is_contended    -> queued_spin_is_contended
// arch_spin_value_unlocked  -> queued_spin_value_unlocked
// arch_spin_lock             -> queued_spin_lock
// arch_spin_trylock          -> queued_spin_trylock
// arch_spin_unlock           -> queued_spin_unlock

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
