// SPDX-License-Identifier: GPL-2.0
/*
 * bitops.c: atomic operations which got too long to be inlined all over
 *      the place.
 *
 * Copyright 1999 Philipp Rumpf (prumpf@tux.org)
 * Copyright 2000 Grant Grundler (grundler@cup.hp.com)
 */

// Dependency declarations from the original Linux headers are supplied by
// other translation units.

#[cfg(CONFIG_SMP)]
static mut __atomic_hash: [arch_spinlock_t; ATOMIC_HASH_SIZE] =
    [__ARCH_SPIN_LOCK_UNLOCKED; ATOMIC_HASH_SIZE];

#[cfg(CONFIG_64BIT)]
pub unsafe fn __xchg64(x: c_ulong, ptr: *mut c_ulong) -> c_ulong {
    let mut flags: c_ulong = 0;
    let temp: c_ulong;

    _atomic_spin_lock_irqsave(ptr, &mut flags);
    temp = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, x);
    _atomic_spin_unlock_irqrestore(ptr, flags);
    temp
}

pub unsafe fn __xchg32(x: c_int, ptr: *mut c_int) -> c_ulong {
    let mut flags: c_ulong = 0;
    let temp: c_long;

    _atomic_spin_lock_irqsave(ptr, &mut flags);
    temp = core::ptr::read_volatile(ptr) as c_long; // XXX - sign extension wanted?
    core::ptr::write_volatile(ptr, x);
    _atomic_spin_unlock_irqrestore(ptr, flags);
    temp as c_ulong
}

pub unsafe fn __xchg8(x: c_char, ptr: *mut c_char) -> c_ulong {
    let mut flags: c_ulong = 0;
    let temp: c_long;

    _atomic_spin_lock_irqsave(ptr, &mut flags);
    temp = core::ptr::read_volatile(ptr) as c_long; // XXX - sign extension wanted?
    core::ptr::write_volatile(ptr, x);
    _atomic_spin_unlock_irqrestore(ptr, flags);
    temp as c_ulong
}

pub unsafe fn __cmpxchg_u64(ptr: *mut u64, old: u64, new: u64) -> u64 {
    let mut flags: c_ulong = 0;
    let prev: u64;

    _atomic_spin_lock_irqsave(ptr, &mut flags);
    prev = core::ptr::read_volatile(ptr);
    if prev == old {
        core::ptr::write_volatile(ptr, new);
    }
    _atomic_spin_unlock_irqrestore(ptr, flags);
    prev
}

pub unsafe fn __cmpxchg_u32(ptr: *mut u32, old: u32, new: u32) -> u32 {
    let mut flags: c_ulong = 0;
    let prev: u32;

    _atomic_spin_lock_irqsave(ptr, &mut flags);
    prev = core::ptr::read_volatile(ptr);
    if prev == old {
        core::ptr::write_volatile(ptr, new);
    }
    _atomic_spin_unlock_irqrestore(ptr, flags);
    prev
}

pub unsafe fn __cmpxchg_u16(ptr: *mut u16, old: u16, new: u16) -> u16 {
    let mut flags: c_ulong = 0;
    let prev: u16;

    _atomic_spin_lock_irqsave(ptr, &mut flags);
    prev = core::ptr::read_volatile(ptr);
    if prev == old {
        core::ptr::write_volatile(ptr, new);
    }
    _atomic_spin_unlock_irqrestore(ptr, flags);
    prev
}

pub unsafe fn __cmpxchg_u8(ptr: *mut u8, old: u8, new: u8) -> u8 {
    let mut flags: c_ulong = 0;
    let prev: u8;

    _atomic_spin_lock_irqsave(ptr, &mut flags);
    prev = core::ptr::read_volatile(ptr);
    if prev == old {
        core::ptr::write_volatile(ptr, new);
    }
    _atomic_spin_unlock_irqrestore(ptr, flags);
    prev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
