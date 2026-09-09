/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/spinlock-cas.h
 *
 * Copyright (C) 2015 SEI
 */

// Dependencies supplied by the surrounding translation unit:
// asm/barrier.h and asm/processor.h

/// Compare-and-swap primitive corresponding to the SH `cas.l` instruction.
#[inline]
pub unsafe fn __sl_cas(p: *mut ::core::ffi::c_uint, old: ::core::ffi::c_uint, new: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let mut value = new;
    // The SH-specific inline assembly is retained as a source-level operation.
    ::core::arch::asm!(
        "cas.l {old},{value},@r0",
        old = in(reg) old,
        value = inout(reg) value,
        in("r0") p,
        lateout("t") _,
        options(nostack)
    );
    value
}

/*
 * Your basic SMP spinlocks, allowing only a single CPU anywhere
 */

#[inline]
pub unsafe fn arch_spin_is_locked(x: *const arch_spinlock_t) -> bool {
    (*x).lock <= 0
}

#[inline]
pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    while __sl_cas(&mut (*lock).lock, 1, 0) == 0 {}
}

#[inline]
pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    __sl_cas(&mut (*lock).lock, 0, 1);
}

#[inline]
pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> ::core::ffi::c_int {
    __sl_cas(&mut (*lock).lock, 1, 0) as ::core::ffi::c_int
}

/*
 * Read-write spinlocks, allowing multiple readers but only one writer.
 *
 * NOTE! it is quite common to have readers in interrupts but no interrupt
 * writers. For those circumstances we can "mix" irq-safe locks - any writer
 * needs to get a irq-safe write-lock, but readers can get non-irqsafe
 * read-locks.
 */

#[inline]
pub unsafe fn arch_read_lock(rw: *mut arch_rwlock_t) {
    let mut old: ::core::ffi::c_uint;
    loop {
        old = (*rw).lock;
        if old != 0 && __sl_cas(&mut (*rw).lock, old, old.wrapping_sub(1)) == old {
            break;
        }
    }
}

#[inline]
pub unsafe fn arch_read_unlock(rw: *mut arch_rwlock_t) {
    let mut old: ::core::ffi::c_uint;
    loop {
        old = (*rw).lock;
        if __sl_cas(&mut (*rw).lock, old, old.wrapping_add(1)) == old {
            break;
        }
    }
}

#[inline]
pub unsafe fn arch_write_lock(rw: *mut arch_rwlock_t) {
    while __sl_cas(&mut (*rw).lock, RW_LOCK_BIAS, 0) != RW_LOCK_BIAS {}
}

#[inline]
pub unsafe fn arch_write_unlock(rw: *mut arch_rwlock_t) {
    __sl_cas(&mut (*rw).lock, 0, RW_LOCK_BIAS);
}

#[inline]
pub unsafe fn arch_read_trylock(rw: *mut arch_rwlock_t) -> ::core::ffi::c_int {
    let mut old: ::core::ffi::c_uint;
    loop {
        old = (*rw).lock;
        if old == 0 || __sl_cas(&mut (*rw).lock, old, old.wrapping_sub(1)) == old {
            break;
        }
    }
    (old != 0) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn arch_write_trylock(rw: *mut arch_rwlock_t) -> bool {
    __sl_cas(&mut (*rw).lock, RW_LOCK_BIAS, 0) == RW_LOCK_BIAS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
