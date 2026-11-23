// SPDX-License-Identifier: GPL-2.0

// Dependencies: linux/lockref.h.
//
// lib/Kconfig only offers RUST_LOCKREF where USE_CMPXCHG_LOCKREF holds (a
// four-byte spinlock_t), so the lock_count member exists;
// lib/lockref_helpers.c asserts it.

use core::mem::zeroed;
use core::ptr::{addr_of_mut, read_volatile};
use kernel::bindings::lockref;
use kernel::ffi::c_int;

extern "C" {
    fn lupos_lockref_value_unlocked(lock_count: u64) -> bool;
    fn lupos_lockref_try_cmpxchg64_relaxed(ptr: *mut u64, old: *mut u64, new: u64) -> bool;
    fn lupos_lockref_lock(lockref: *mut lockref);
    fn lupos_lockref_unlock(lockref: *mut lockref);
    fn lupos_lockref_assert_locked(lockref: *mut lockref);
}

/// __LOCKREF_DEAD_VAL; lib/lockref_helpers.c asserts it.
const LOCKREF_DEAD_VAL: c_int = -128;

/// The count half of a lock_count snapshot (`struct lockref` union view).
fn count_of(lock_count: u64) -> c_int {
    // SAFETY: both union members are plain data of the same 8 bytes.
    unsafe {
        let mut view: lockref = zeroed();
        view.__bindgen_anon_1.lock_count = lock_count;
        view.__bindgen_anon_1.__bindgen_anon_1.count
    }
}

/// A lock_count snapshot with its count half replaced.
fn with_count(lock_count: u64, count: c_int) -> u64 {
    // SAFETY: both union members are plain data of the same 8 bytes.
    unsafe {
        let mut view: lockref = zeroed();
        view.__bindgen_anon_1.lock_count = lock_count;
        view.__bindgen_anon_1.__bindgen_anon_1.count = count;
        view.__bindgen_anon_1.lock_count
    }
}

/// What the CODE part of CMPXCHG_LOOP() does with one iteration.
enum Code<R> {
    /// Try to install the new count.
    Swap(c_int),
    /// `return` from the calling function.
    Return(R),
    /// `break` out of the loop to the locked slow path.
    Break,
}

/*
 * Note that the "cmpxchg()" reloads the "old" value for the
 * failure case.
 */
/// CMPXCHG_LOOP(CODE, SUCCESS): `Some` when the caller returns from the
/// loop, `None` when it falls through to the locked path.
///
/// # Safety
/// `lockref` points to a valid lockref.
#[inline(always)]
unsafe fn cmpxchg_loop<R>(
    lockref: *mut lockref,
    code: impl Fn(c_int) -> Code<R>,
    success: impl Fn(c_int) -> R,
) -> Option<R> {
    let mut retry: c_int = 100;

    // SAFETY: lock_count is the configured 8-byte union member; it is only
    // updated with the native relaxed try_cmpxchg64().
    unsafe {
        let lock_count = addr_of_mut!((*lockref).__bindgen_anon_1.lock_count);
        let mut old: u64 = read_volatile(lock_count);
        while lupos_lockref_value_unlocked(old) {
            let new = match code(count_of(old)) {
                Code::Swap(count) => with_count(old, count),
                Code::Return(r) => return Some(r),
                Code::Break => break,
            };
            if lupos_lockref_try_cmpxchg64_relaxed(lock_count, &mut old, new) {
                return Some(success(count_of(new)));
            }
            retry -= 1;
            if retry == 0 {
                break;
            }
        }
    }
    None
}

/// The lockref count, accessed under its spinlock.
///
/// # Safety
/// `lockref` points to a valid lockref whose lock is held.
unsafe fn count_ptr(lockref: *mut lockref) -> *mut c_int {
    // SAFETY: forwarded caller contract.
    unsafe { addr_of_mut!((*lockref).__bindgen_anon_1.__bindgen_anon_1.count) }
}

/**
 * lockref_get - Increments reference count unconditionally
 * @lockref: pointer to lockref structure
 *
 * This operation is only valid if you already hold a reference
 * to the object, so you know the count cannot be zero.
 */
/// # Safety
/// `lockref` points to a valid lockref.
#[no_mangle]
pub unsafe extern "C" fn lockref_get(lockref: *mut lockref) {
    // SAFETY: the count is only changed by cmpxchg or under the lock.
    unsafe {
        if cmpxchg_loop(lockref, |old| Code::Swap(old.wrapping_add(1)), |_| ()).is_some() {
            return;
        }

        lupos_lockref_lock(lockref);
        *count_ptr(lockref) = (*count_ptr(lockref)).wrapping_add(1);
        lupos_lockref_unlock(lockref);
    }
}
// EXPORT_SYMBOL(lockref_get);

/**
 * lockref_get_not_zero - Increments count unless the count is 0 or dead
 * @lockref: pointer to lockref structure
 * Return: 1 if count updated successfully or 0 if count was zero
 */
/// # Safety
/// `lockref` points to a valid lockref.
#[no_mangle]
pub unsafe extern "C" fn lockref_get_not_zero(lockref: *mut lockref) -> bool {
    let mut retval = false;

    // SAFETY: the count is only changed by cmpxchg or under the lock.
    unsafe {
        if let Some(r) = cmpxchg_loop(
            lockref,
            |old| if old <= 0 { Code::Return(false) } else { Code::Swap(old.wrapping_add(1)) },
            |_| true,
        ) {
            return r;
        }

        lupos_lockref_lock(lockref);
        if *count_ptr(lockref) > 0 {
            *count_ptr(lockref) = (*count_ptr(lockref)).wrapping_add(1);
            retval = true;
        }
        lupos_lockref_unlock(lockref);
    }
    retval
}
// EXPORT_SYMBOL(lockref_get_not_zero);

/**
 * lockref_put_return - Decrement reference count if possible
 * @lockref: pointer to lockref structure
 *
 * Decrement the reference count and return the new value.
 * If the lockref was dead or locked, return -1.
 */
/// # Safety
/// `lockref` points to a valid lockref.
#[no_mangle]
pub unsafe extern "C" fn lockref_put_return(lockref: *mut lockref) -> c_int {
    // SAFETY: the count is only changed by cmpxchg.
    unsafe {
        cmpxchg_loop(
            lockref,
            |old| if old <= 0 { Code::Return(-1) } else { Code::Swap(old.wrapping_sub(1)) },
            |new| new,
        )
        .unwrap_or(-1)
    }
}
// EXPORT_SYMBOL(lockref_put_return);

/**
 * lockref_put_or_lock - decrements count unless count <= 1 before decrement
 * @lockref: pointer to lockref structure
 * Return: 1 if count updated successfully or 0 if count <= 1 and lock taken
 */
/// # Safety
/// `lockref` points to a valid lockref.
#[no_mangle]
pub unsafe extern "C" fn lockref_put_or_lock(lockref: *mut lockref) -> bool {
    // SAFETY: the count is only changed by cmpxchg or under the lock.
    unsafe {
        if let Some(r) = cmpxchg_loop(
            lockref,
            |old| if old <= 1 { Code::Break } else { Code::Swap(old.wrapping_sub(1)) },
            |_| true,
        ) {
            return r;
        }

        lupos_lockref_lock(lockref);
        if *count_ptr(lockref) <= 1 {
            return false;
        }
        *count_ptr(lockref) = (*count_ptr(lockref)).wrapping_sub(1);
        lupos_lockref_unlock(lockref);
    }
    true
}
// EXPORT_SYMBOL(lockref_put_or_lock);

/**
 * lockref_mark_dead - mark lockref dead
 * @lockref: pointer to lockref structure
 */
/// # Safety
/// `lockref` points to a valid lockref whose lock is held.
#[no_mangle]
pub unsafe extern "C" fn lockref_mark_dead(lockref: *mut lockref) {
    // SAFETY: the caller holds the lock, which the native assertion checks.
    unsafe {
        lupos_lockref_assert_locked(lockref);
        *count_ptr(lockref) = LOCKREF_DEAD_VAL;
    }
}
// EXPORT_SYMBOL(lockref_mark_dead);

/**
 * lockref_get_not_dead - Increments count unless the ref is dead
 * @lockref: pointer to lockref structure
 * Return: 1 if count updated successfully or 0 if lockref was dead
 */
/// # Safety
/// `lockref` points to a valid lockref.
#[no_mangle]
pub unsafe extern "C" fn lockref_get_not_dead(lockref: *mut lockref) -> bool {
    let mut retval = false;

    // SAFETY: the count is only changed by cmpxchg or under the lock.
    unsafe {
        if let Some(r) = cmpxchg_loop(
            lockref,
            |old| if old < 0 { Code::Return(false) } else { Code::Swap(old.wrapping_add(1)) },
            |_| true,
        ) {
            return r;
        }

        lupos_lockref_lock(lockref);
        if *count_ptr(lockref) >= 0 {
            *count_ptr(lockref) = (*count_ptr(lockref)).wrapping_add(1);
            retval = true;
        }
        lupos_lockref_unlock(lockref);
    }
    retval
}
// EXPORT_SYMBOL(lockref_get_not_dead);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
