// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct raw_spinlock_t {
    _private: [u8; 0],
}

extern "C" {
    fn atomic_add_unless(atomic: *mut atomic_t, add: i32, unless: i32) -> i32;
    fn spin_lock(lock: *mut spinlock_t);
    fn atomic_dec_and_test(atomic: *mut atomic_t) -> i32;
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: ::core::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: ::core::ffi::c_ulong);
    fn raw_spin_lock(lock: *mut raw_spinlock_t);
    fn raw_spin_unlock(lock: *mut raw_spinlock_t);
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: ::core::ffi::c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: ::core::ffi::c_ulong);
}

// This is an implementation of the notion of "decrement a
// reference count, and return locked if it decremented to zero".
//
// NOTE NOTE NOTE! This is _not_ equivalent to
//
//     if (atomic_dec_and_test(&atomic)) {
//         spin_lock(&lock);
//         return 1;
//     }
//     return 0;
//
// because the spin-lock and the decrement must be
// "atomic".
#[no_mangle]
pub unsafe extern "C" fn atomic_dec_and_lock(
    atomic: *mut atomic_t,
    lock: *mut spinlock_t,
) -> i32 {
    // Subtract 1 from counter unless that drops it to 0 (ie. it was 1)
    if atomic_add_unless(atomic, -1, 1) != 0 {
        return 0;
    }

    // Otherwise do it the slow way
    spin_lock(lock);
    if atomic_dec_and_test(atomic) != 0 {
        return 1;
    }
    spin_unlock(lock);
    0
}

// EXPORT_SYMBOL(atomic_dec_and_lock);

#[no_mangle]
pub unsafe extern "C" fn _atomic_dec_and_lock_irqsave(
    atomic: *mut atomic_t,
    lock: *mut spinlock_t,
    flags: *mut ::core::ffi::c_ulong,
) -> i32 {
    // Subtract 1 from counter unless that drops it to 0 (ie. it was 1)
    if atomic_add_unless(atomic, -1, 1) != 0 {
        return 0;
    }

    // Otherwise do it the slow way
    spin_lock_irqsave(lock, *flags);
    if atomic_dec_and_test(atomic) != 0 {
        return 1;
    }
    spin_unlock_irqrestore(lock, *flags);
    0
}

// EXPORT_SYMBOL(_atomic_dec_and_lock_irqsave);

#[no_mangle]
pub unsafe extern "C" fn atomic_dec_and_raw_lock(
    atomic: *mut atomic_t,
    lock: *mut raw_spinlock_t,
) -> i32 {
    // Subtract 1 from counter unless that drops it to 0 (ie. it was 1)
    if atomic_add_unless(atomic, -1, 1) != 0 {
        return 0;
    }

    // Otherwise do it the slow way
    raw_spin_lock(lock);
    if atomic_dec_and_test(atomic) != 0 {
        return 1;
    }
    raw_spin_unlock(lock);
    0
}

// EXPORT_SYMBOL(atomic_dec_and_raw_lock);

#[no_mangle]
pub unsafe extern "C" fn _atomic_dec_and_raw_lock_irqsave(
    atomic: *mut atomic_t,
    lock: *mut raw_spinlock_t,
    flags: *mut ::core::ffi::c_ulong,
) -> i32 {
    // Subtract 1 from counter unless that drops it to 0 (ie. it was 1)
    if atomic_add_unless(atomic, -1, 1) != 0 {
        return 0;
    }

    // Otherwise do it the slow way
    raw_spin_lock_irqsave(lock, *flags);
    if atomic_dec_and_test(atomic) != 0 {
        return 1;
    }
    raw_spin_unlock_irqrestore(lock, *flags);
    0
}

// EXPORT_SYMBOL(_atomic_dec_and_raw_lock_irqsave);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
