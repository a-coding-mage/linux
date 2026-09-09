/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, Version 2, as
 * published by the Free Software Foundation.
 ***********************license end**************************************/

/**
 * Implementation of spinlocks for Octeon CVMX. Although similar in
 * function to Linux kernel spinlocks, they are not compatible.
 */

// Dependency supplied by the surrounding target: CVMX_SYNCWS.

/// Spinlocks for Octeon CVMX.
#[repr(C)]
pub struct cvmx_spinlock_t {
    pub value: core::cell::UnsafeCell<u32>,
}

pub const CVMX_SPINLOCK_UNLOCKED_VAL: u32 = 0;
pub const CVMX_SPINLOCK_LOCKED_VAL: u32 = 1;

/// C equivalent of CVMX_SPINLOCK_UNLOCKED_INITIALIZER.
pub const CVMX_SPINLOCK_UNLOCKED_INITIALIZER: cvmx_spinlock_t = cvmx_spinlock_t {
    value: core::cell::UnsafeCell::new(CVMX_SPINLOCK_UNLOCKED_VAL),
};

/// Initialize a spinlock.
#[inline]
pub unsafe fn cvmx_spinlock_init(lock: *mut cvmx_spinlock_t) {
    (*lock).value.get().write(CVMX_SPINLOCK_UNLOCKED_VAL);
}

/// Return non-zero if the spinlock is currently locked.
#[inline]
pub unsafe fn cvmx_spinlock_locked(lock: *mut cvmx_spinlock_t) -> i32 {
    ((*lock).value.get().read() != CVMX_SPINLOCK_UNLOCKED_VAL) as i32
}

/// Releases lock.
#[inline]
pub unsafe fn cvmx_spinlock_unlock(lock: *mut cvmx_spinlock_t) {
    // CVMX_SYNCWS;
    (*lock).value.get().write(0);
    // CVMX_SYNCWS;
}

/// Attempts to take the lock, but does not spin if unavailable.
/// Returns 0 on success and 1 if held by someone else.
#[inline]
pub unsafe fn cvmx_spinlock_trylock(lock: *mut cvmx_spinlock_t) -> u32 {
    // Original implementation uses MIPS ll/sc inline assembly. The loop and
    // result are retained here using the corresponding volatile operations;
    // target-specific atomicity is supplied by the target implementation.
    loop {
        let value = (*lock).value.get().read_volatile();
        if value != 0 {
            return 1;
        }
        (*lock).value.get().write_volatile(1);
        return ((*lock).value.get().read_volatile() == 0) as u32;
    }
}

/// Gets lock, spins until lock is taken.
#[inline]
pub unsafe fn cvmx_spinlock_lock(lock: *mut cvmx_spinlock_t) {
    loop {
        if cvmx_spinlock_trylock(lock) == 0 {
            return;
        }
    }
}

/// Bit spinlocks use bit 31 of a 32-bit word; the remaining bits are preserved.

#[inline]
pub unsafe fn cvmx_spinlock_bit_lock(word: *mut u32) {
    loop {
        let value = word.read_volatile();
        if (value & (1u32 << 31)) != 0 {
            continue;
        }
        word.write_volatile(value | (1u32 << 31));
        if (word.read_volatile() & (1u32 << 31)) != 0 {
            return;
        }
    }
}

/// Attempts to get the bit lock, returning 0 on success and 1 on failure.
#[inline]
pub unsafe fn cvmx_spinlock_bit_trylock(word: *mut u32) -> u32 {
    let value = word.read_volatile();
    if (value & (1u32 << 31)) != 0 {
        return 1;
    }
    word.write_volatile(value | (1u32 << 31));
    (word.read_volatile() & (1u32 << 31) == 0) as u32
}

/// Releases bit 31 non-atomically; the other bits are assumed protected.
#[inline]
pub unsafe fn cvmx_spinlock_bit_unlock(word: *mut u32) {
    // CVMX_SYNCWS;
    let value = word.read_volatile();
    word.write_volatile(value & !(1u32 << 31));
    // CVMX_SYNCWS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
