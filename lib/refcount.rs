// SPDX-License-Identifier: GPL-2.0
/*
 * Out-of-line refcount functions.
 */

// Dependencies: linux/mutex.h, linux/refcount.h, linux/spinlock.h, linux/bug.h.

use core::ptr::{addr_of_mut, read_volatile, write_volatile};
use kernel::bindings::{atomic_t, mutex, mutex_unlock, refcount_t, spinlock_t};
use kernel::ffi::{c_int, c_uint, c_ulong};

extern "C" {
    fn lupos_refcount_try_cmpxchg_release(v: *mut atomic_t, old: *mut c_int, new: c_int) -> bool;
    fn lupos_refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn lupos_refcount_mutex_lock(lock: *mut mutex);
    fn lupos_refcount_spin_lock(lock: *mut spinlock_t);
    fn lupos_refcount_spin_unlock(lock: *mut spinlock_t);
    fn lupos_refcount_spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn lupos_refcount_spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);

    fn lupos_refcount_warn_add_not_zero_ovf();
    fn lupos_refcount_warn_add_ovf();
    fn lupos_refcount_warn_add_uaf();
    fn lupos_refcount_warn_sub_uaf();
    fn lupos_refcount_warn_dec_leak();
    fn lupos_refcount_warn_unknown();
    fn lupos_refcount_warn_dec_not_one_underflow();
}

/// REFCOUNT_SATURATED, `INT_MIN / 2`; lib/refcount_helpers.c asserts it.
const REFCOUNT_SATURATED: c_int = c_int::MIN / 2;

/// `enum refcount_saturation_type`; lib/refcount_helpers.c asserts it.
const REFCOUNT_ADD_NOT_ZERO_OVF: c_uint = 0;
const REFCOUNT_ADD_OVF: c_uint = 1;
const REFCOUNT_ADD_UAF: c_uint = 2;
const REFCOUNT_SUB_UAF: c_uint = 3;
const REFCOUNT_DEC_LEAK: c_uint = 4;

/// atomic_read(&r->refs).
///
/// # Safety
/// `r` points to a valid refcount.
unsafe fn refs_read(r: *mut refcount_t) -> c_int {
    // SAFETY: atomic_read() is a volatile load of the counter.
    unsafe { read_volatile(addr_of_mut!((*r).refs.counter)) }
}

/// refcount_set(r, n): atomic_set(&r->refs, n).
///
/// # Safety
/// `r` points to a valid refcount.
unsafe fn refcount_set(r: *mut refcount_t, n: c_int) {
    // SAFETY: atomic_set() is a volatile store of the counter.
    unsafe { write_volatile(addr_of_mut!((*r).refs.counter), n) };
}

/// Saturates `r` and reports the saturation event `t`.
///
/// # Safety
/// `r` points to a valid refcount.
#[no_mangle]
pub unsafe extern "C" fn refcount_warn_saturate(r: *mut refcount_t, t: c_uint) {
    // SAFETY: r is valid; each event has its own native WARN_ONCE() site.
    unsafe {
        refcount_set(r, REFCOUNT_SATURATED);

        match t {
            REFCOUNT_ADD_NOT_ZERO_OVF => lupos_refcount_warn_add_not_zero_ovf(),
            REFCOUNT_ADD_OVF => lupos_refcount_warn_add_ovf(),
            REFCOUNT_ADD_UAF => lupos_refcount_warn_add_uaf(),
            REFCOUNT_SUB_UAF => lupos_refcount_warn_sub_uaf(),
            REFCOUNT_DEC_LEAK => lupos_refcount_warn_dec_leak(),
            _ => lupos_refcount_warn_unknown(),
        }
    }
}
// EXPORT_SYMBOL(refcount_warn_saturate);

/**
 * refcount_dec_if_one - decrement a refcount if it is 1
 * @r: the refcount
 *
 * No atomic_t counterpart, it attempts a 1 -> 0 transition and returns the
 * success thereof.
 *
 * Like all decrement operations, it provides release memory order and provides
 * a control dependency.
 *
 * It can be used like a try-delete operator; this explicit case is provided
 * and not cmpxchg in generic, because that would allow implementing unsafe
 * operations.
 *
 * Return: true if the resulting refcount is 0, false otherwise
 */
/// # Safety
/// `r` points to a valid refcount.
#[no_mangle]
pub unsafe extern "C" fn refcount_dec_if_one(r: *mut refcount_t) -> bool {
    let mut val: c_int = 1;

    // SAFETY: native atomic_try_cmpxchg_release() on the counter.
    unsafe { lupos_refcount_try_cmpxchg_release(addr_of_mut!((*r).refs), &mut val, 0) }
}
// EXPORT_SYMBOL(refcount_dec_if_one);

/**
 * refcount_dec_not_one - decrement a refcount if it is not 1
 * @r: the refcount
 *
 * No atomic_t counterpart, it decrements unless the value is 1, in which case
 * it will return false.
 *
 * Was often done like: atomic_add_unless(&var, -1, 1)
 *
 * Return: true if the decrement operation was successful, false otherwise
 */
/// # Safety
/// `r` points to a valid refcount.
#[no_mangle]
pub unsafe extern "C" fn refcount_dec_not_one(r: *mut refcount_t) -> bool {
    // SAFETY: r is valid; the update uses the native release cmpxchg.
    unsafe {
        let mut val = refs_read(r) as c_uint;

        loop {
            if val == REFCOUNT_SATURATED as c_uint {
                return true;
            }

            if val == 1 {
                return false;
            }

            let new = val.wrapping_sub(1);
            if new > val {
                lupos_refcount_warn_dec_not_one_underflow();
                return true;
            }

            let mut old = val as c_int;
            let swapped = lupos_refcount_try_cmpxchg_release(addr_of_mut!((*r).refs), &mut old, new as c_int);
            val = old as c_uint;
            if swapped {
                break;
            }
        }
    }

    true
}
// EXPORT_SYMBOL(refcount_dec_not_one);

/**
 * refcount_dec_and_mutex_lock - return holding mutex if able to decrement
 *                               refcount to 0
 * @r: the refcount
 * @lock: the mutex to be locked
 *
 * Similar to atomic_dec_and_mutex_lock(), it will WARN on underflow and fail
 * to decrement when saturated at REFCOUNT_SATURATED.
 *
 * Provides release memory ordering, such that prior loads and stores are done
 * before, and provides a control dependency such that free() must come after.
 * See the comment on top.
 *
 * Return: true and hold mutex if able to decrement refcount to 0, false
 *         otherwise
 */
/// # Safety
/// `r` and `lock` are valid; the caller may sleep.
#[no_mangle]
pub unsafe extern "C" fn refcount_dec_and_mutex_lock(r: *mut refcount_t, lock: *mut mutex) -> bool {
    // SAFETY: native refcount and mutex primitives.
    unsafe {
        if refcount_dec_not_one(r) {
            return false;
        }

        lupos_refcount_mutex_lock(lock);
        if !lupos_refcount_dec_and_test(r) {
            mutex_unlock(lock);
            return false;
        }
    }

    true
}
// EXPORT_SYMBOL(refcount_dec_and_mutex_lock);

/**
 * refcount_dec_and_lock - return holding spinlock if able to decrement
 *                         refcount to 0
 * @r: the refcount
 * @lock: the spinlock to be locked
 *
 * Similar to atomic_dec_and_lock(), it will WARN on underflow and fail to
 * decrement when saturated at REFCOUNT_SATURATED.
 *
 * Provides release memory ordering, such that prior loads and stores are done
 * before, and provides a control dependency such that free() must come after.
 * See the comment on top.
 *
 * Return: true and hold spinlock if able to decrement refcount to 0, false
 *         otherwise
 */
/// # Safety
/// `r` and `lock` are valid.
#[no_mangle]
pub unsafe extern "C" fn refcount_dec_and_lock(r: *mut refcount_t, lock: *mut spinlock_t) -> bool {
    // SAFETY: native refcount and spinlock primitives.
    unsafe {
        if refcount_dec_not_one(r) {
            return false;
        }

        lupos_refcount_spin_lock(lock);
        if !lupos_refcount_dec_and_test(r) {
            lupos_refcount_spin_unlock(lock);
            return false;
        }
    }

    true
}
// EXPORT_SYMBOL(refcount_dec_and_lock);

/**
 * refcount_dec_and_lock_irqsave - return holding spinlock with disabled
 *                                 interrupts if able to decrement refcount to 0
 * @r: the refcount
 * @lock: the spinlock to be locked
 * @flags: saved IRQ-flags if the is acquired
 *
 * Same as refcount_dec_and_lock() above except that the spinlock is acquired
 * with disabled interrupts.
 *
 * Return: true and hold spinlock if able to decrement refcount to 0, false
 *         otherwise
 */
/// # Safety
/// `r`, `lock` and `flags` are valid.
#[no_mangle]
pub unsafe extern "C" fn refcount_dec_and_lock_irqsave(r: *mut refcount_t, lock: *mut spinlock_t, flags: *mut c_ulong) -> bool {
    // SAFETY: native refcount and spinlock primitives.
    unsafe {
        if refcount_dec_not_one(r) {
            return false;
        }

        *flags = lupos_refcount_spin_lock_irqsave(lock);
        if !lupos_refcount_dec_and_test(r) {
            lupos_refcount_spin_unlock_irqrestore(lock, *flags);
            return false;
        }
    }

    true
}
// EXPORT_SYMBOL(refcount_dec_and_lock_irqsave);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
