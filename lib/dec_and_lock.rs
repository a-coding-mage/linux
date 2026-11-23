// SPDX-License-Identifier: GPL-2.0

// Dependencies: linux/spinlock.h, linux/atomic.h.

use kernel::bindings::{atomic_t, raw_spinlock_t, spinlock_t};
use kernel::ffi::{c_int, c_ulong};

extern "C" {
    fn lupos_dec_and_lock_add_unless_one(atomic: *mut atomic_t) -> bool;
    fn lupos_dec_and_lock_dec_and_test(atomic: *mut atomic_t) -> bool;
    fn lupos_dec_and_lock_spin_lock(lock: *mut spinlock_t);
    fn lupos_dec_and_lock_spin_unlock(lock: *mut spinlock_t);
    fn lupos_dec_and_lock_spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn lupos_dec_and_lock_spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn lupos_dec_and_lock_raw_spin_lock(lock: *mut raw_spinlock_t);
    fn lupos_dec_and_lock_raw_spin_unlock(lock: *mut raw_spinlock_t);
    fn lupos_dec_and_lock_raw_spin_lock_irqsave(lock: *mut raw_spinlock_t) -> c_ulong;
    fn lupos_dec_and_lock_raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
}

/*
 * This is an implementation of the notion of "decrement a
 * reference count, and return locked if it decremented to zero".
 *
 * NOTE NOTE NOTE! This is _not_ equivalent to
 *
 *	if (atomic_dec_and_test(&atomic)) {
 *		spin_lock(&lock);
 *		return 1;
 *	}
 *	return 0;
 *
 * because the spin-lock and the decrement must be
 * "atomic".
 */
/// Decrements `atomic` and returns 1 with `lock` held if it reached zero.
///
/// # Safety
/// `atomic` and `lock` are valid; the lock is not already held.
#[no_mangle]
pub unsafe extern "C" fn atomic_dec_and_lock(atomic: *mut atomic_t, lock: *mut spinlock_t) -> c_int {
    // SAFETY: native atomic and lock primitives on caller-owned objects.
    unsafe {
        /* Subtract 1 from counter unless that drops it to 0 (ie. it was 1) */
        if lupos_dec_and_lock_add_unless_one(atomic) {
            return 0;
        }

        /* Otherwise do it the slow way */
        lupos_dec_and_lock_spin_lock(lock);
        if lupos_dec_and_lock_dec_and_test(atomic) {
            return 1;
        }
        lupos_dec_and_lock_spin_unlock(lock);
    }
    0
}
// EXPORT_SYMBOL(atomic_dec_and_lock);

/// Like atomic_dec_and_lock(), saving the IRQ state in `flags`.
///
/// # Safety
/// `atomic`, `lock` and `flags` are valid; the lock is not already held.
#[no_mangle]
pub unsafe extern "C" fn _atomic_dec_and_lock_irqsave(
    atomic: *mut atomic_t,
    lock: *mut spinlock_t,
    flags: *mut c_ulong,
) -> c_int {
    // SAFETY: native atomic and lock primitives on caller-owned objects.
    unsafe {
        /* Subtract 1 from counter unless that drops it to 0 (ie. it was 1) */
        if lupos_dec_and_lock_add_unless_one(atomic) {
            return 0;
        }

        /* Otherwise do it the slow way */
        *flags = lupos_dec_and_lock_spin_lock_irqsave(lock);
        if lupos_dec_and_lock_dec_and_test(atomic) {
            return 1;
        }
        lupos_dec_and_lock_spin_unlock_irqrestore(lock, *flags);
    }
    0
}
// EXPORT_SYMBOL(_atomic_dec_and_lock_irqsave);

/// Like atomic_dec_and_lock() for a raw spinlock.
///
/// # Safety
/// `atomic` and `lock` are valid; the lock is not already held.
#[no_mangle]
pub unsafe extern "C" fn atomic_dec_and_raw_lock(atomic: *mut atomic_t, lock: *mut raw_spinlock_t) -> c_int {
    // SAFETY: native atomic and lock primitives on caller-owned objects.
    unsafe {
        /* Subtract 1 from counter unless that drops it to 0 (ie. it was 1) */
        if lupos_dec_and_lock_add_unless_one(atomic) {
            return 0;
        }

        /* Otherwise do it the slow way */
        lupos_dec_and_lock_raw_spin_lock(lock);
        if lupos_dec_and_lock_dec_and_test(atomic) {
            return 1;
        }
        lupos_dec_and_lock_raw_spin_unlock(lock);
    }
    0
}
// EXPORT_SYMBOL(atomic_dec_and_raw_lock);

/// Like atomic_dec_and_raw_lock(), saving the IRQ state in `flags`.
///
/// # Safety
/// `atomic`, `lock` and `flags` are valid; the lock is not already held.
#[no_mangle]
pub unsafe extern "C" fn _atomic_dec_and_raw_lock_irqsave(
    atomic: *mut atomic_t,
    lock: *mut raw_spinlock_t,
    flags: *mut c_ulong,
) -> c_int {
    // SAFETY: native atomic and lock primitives on caller-owned objects.
    unsafe {
        /* Subtract 1 from counter unless that drops it to 0 (ie. it was 1) */
        if lupos_dec_and_lock_add_unless_one(atomic) {
            return 0;
        }

        /* Otherwise do it the slow way */
        *flags = lupos_dec_and_lock_raw_spin_lock_irqsave(lock);
        if lupos_dec_and_lock_dec_and_test(atomic) {
            return 1;
        }
        lupos_dec_and_lock_raw_spin_unlock_irqrestore(lock, *flags);
    }
    0
}
// EXPORT_SYMBOL(_atomic_dec_and_raw_lock_irqsave);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
