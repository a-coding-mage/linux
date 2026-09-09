/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Queue read/write lock
 *
 * These use generic atomic and locking routines, but depend on a fair spinlock
 * implementation in order to be fair themselves.  The implementation in
 * asm-generic/spinlock.h meets these requirements.
 *
 * (C) Copyright 2013-2014 Hewlett-Packard Development Company, L.P.
 *
 * Authors: Waiman Long <waiman.long@hp.com>
 */

/* Dependencies: linux/atomic.h, asm/barrier.h, asm/processor.h,
 * and asm-generic/qrwlock_types.h. */

/* Must be included from asm/spinlock.h after defining arch_spin_is_locked. */

/* Writer states & reader shift and bias. */
pub const _QW_WAITING: u32 = 0x100; // A writer is waiting
pub const _QW_LOCKED: u32 = 0x0ff; // A writer holds the lock
pub const _QW_WMASK: u32 = 0x1ff; // Writer mask
pub const _QR_SHIFT: u32 = 9; // Reader count shift
pub const _QR_BIAS: u32 = 1u32 << _QR_SHIFT;

/* External function declarations. */
extern "C" {
    pub fn queued_read_lock_slowpath(lock: *mut qrwlock);
    pub fn queued_write_lock_slowpath(lock: *mut qrwlock);
}

/**
 * queued_read_trylock - try to acquire read lock of a queued rwlock
 * @lock : Pointer to queued rwlock structure
 * Return: 1 if lock acquired, 0 if failed
 */
#[inline]
pub unsafe fn queued_read_trylock(lock: *mut qrwlock) -> i32 {
    let mut cnts: i32;

    cnts = atomic_read(&(*lock).cnts);
    if likely((cnts as u32 & _QW_WMASK) == 0) {
        cnts = atomic_add_return_acquire(_QR_BIAS as i32, &(*lock).cnts) as u32 as i32;
        if likely((cnts as u32 & _QW_WMASK) == 0) {
            return 1;
        }
        atomic_sub(_QR_BIAS as i32, &(*lock).cnts);
    }
    0
}

/**
 * queued_write_trylock - try to acquire write lock of a queued rwlock
 * @lock : Pointer to queued rwlock structure
 * Return: 1 if lock acquired, 0 if failed
 */
#[inline]
pub unsafe fn queued_write_trylock(lock: *mut qrwlock) -> i32 {
    let mut cnts = atomic_read(&(*lock).cnts);

    if unlikely(cnts != 0) {
        return 0;
    }

    likely(atomic_try_cmpxchg_acquire(&(*lock).cnts, &mut cnts, _QW_LOCKED as i32))
}

/**
 * queued_read_lock - acquire read lock of a queued rwlock
 * @lock: Pointer to queued rwlock structure
 */
#[inline]
pub unsafe fn queued_read_lock(lock: *mut qrwlock) {
    let cnts = atomic_add_return_acquire(_QR_BIAS as i32, &(*lock).cnts);
    if likely((cnts as u32 & _QW_WMASK) == 0) {
        return;
    }

    /* The slowpath will decrement the reader count, if necessary. */
    queued_read_lock_slowpath(lock);
}

/**
 * queued_write_lock - acquire write lock of a queued rwlock
 * @lock : Pointer to queued rwlock structure
 */
#[inline]
pub unsafe fn queued_write_lock(lock: *mut qrwlock) {
    let mut cnts = 0i32;
    /* Optimize for the unfair lock case where the fair flag is 0. */
    if likely(atomic_try_cmpxchg_acquire(&(*lock).cnts, &mut cnts, _QW_LOCKED as i32)) {
        return;
    }

    queued_write_lock_slowpath(lock);
}

/**
 * queued_read_unlock - release read lock of a queued rwlock
 * @lock : Pointer to queued rwlock structure
 */
#[inline]
pub unsafe fn queued_read_unlock(lock: *mut qrwlock) {
    /* Atomically decrement the reader count. */
    let _ = atomic_sub_return_release(_QR_BIAS as i32, &(*lock).cnts);
}

/**
 * queued_write_unlock - release write lock of a queued rwlock
 * @lock : Pointer to queued rwlock structure
 */
#[inline]
pub unsafe fn queued_write_unlock(lock: *mut qrwlock) {
    smp_store_release(&mut (*lock).wlocked, 0);
}

/**
 * queued_rwlock_is_contended - check if the lock is contended
 * @lock : Pointer to queued rwlock structure
 * Return: 1 if lock contended, 0 otherwise
 */
#[inline]
pub unsafe fn queued_rwlock_is_contended(lock: *mut qrwlock) -> i32 {
    arch_spin_is_locked(&(*lock).wait_lock)
}

/* Remapping rwlock architecture specific functions to the corresponding
 * queued rwlock functions. */
pub fn arch_read_lock(l: *mut qrwlock) { unsafe { queued_read_lock(l) } }
pub fn arch_write_lock(l: *mut qrwlock) { unsafe { queued_write_lock(l) } }
pub fn arch_read_trylock(l: *mut qrwlock) -> i32 { unsafe { queued_read_trylock(l) } }
pub fn arch_write_trylock(l: *mut qrwlock) -> i32 { unsafe { queued_write_trylock(l) } }
pub fn arch_read_unlock(l: *mut qrwlock) { unsafe { queued_read_unlock(l) } }
pub fn arch_write_unlock(l: *mut qrwlock) { unsafe { queued_write_unlock(l) } }
pub fn arch_rwlock_is_contended(l: *mut qrwlock) -> i32 { unsafe { queued_rwlock_is_contended(l) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
