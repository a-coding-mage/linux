// SPDX-License-Identifier: GPL-2.0
/*
 * atomic32.c: 32-bit atomic_t implementation
 *
 * Copyright (C) 2004 Keith M Wesolowski
 * Copyright (C) 2007 Kyle McMartin
 *
 * Based on asm-parisc/atomic.h Copyright (C) 2000 Philipp Rumpf
 */

// C dependencies: linux/atomic.h, linux/spinlock.h, and linux/module.h.
// Build-time CONFIG_SMP selects the hash-lock implementation.

#[cfg(CONFIG_SMP)]
const ATOMIC_HASH_SIZE: usize = 4;
#[cfg(not(CONFIG_SMP))]
const ATOMIC_HASH_SIZE: usize = 1;

#[cfg(CONFIG_SMP)]
extern "C" {
    static mut __atomic_hash: [crate::spinlock_t; ATOMIC_HASH_SIZE];
}
#[cfg(not(CONFIG_SMP))]
extern "C" {
    static mut dummy: crate::spinlock_t;
}

extern "C" {
    fn spin_lock_irqsave(lock: *mut crate::spinlock_t, flags: *mut libc::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut crate::spinlock_t, flags: libc::c_ulong);
}

#[inline]
unsafe fn atomic_hash<T>(a: *const T) -> *mut crate::spinlock_t {
    #[cfg(CONFIG_SMP)]
    {
        &mut __atomic_hash[(((a as usize) >> 8) & (ATOMIC_HASH_SIZE - 1))]
    }
    #[cfg(not(CONFIG_SMP))]
    {
        &mut dummy
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_atomic_add_return(i: libc::c_int, v: *mut crate::atomic_t) -> libc::c_int {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(v);
    spin_lock_irqsave(lock, &mut flags);
    (*v).counter += i;
    let ret = (*v).counter;
    spin_unlock_irqrestore(lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn arch_atomic_fetch_add(i: libc::c_int, v: *mut crate::atomic_t) -> libc::c_int {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(v);
    spin_lock_irqsave(lock, &mut flags);
    let ret = (*v).counter;
    (*v).counter += i;
    spin_unlock_irqrestore(lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn arch_atomic_fetch_and(i: libc::c_int, v: *mut crate::atomic_t) -> libc::c_int {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(v);
    spin_lock_irqsave(lock, &mut flags);
    let ret = (*v).counter;
    (*v).counter &= i;
    spin_unlock_irqrestore(lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn arch_atomic_fetch_or(i: libc::c_int, v: *mut crate::atomic_t) -> libc::c_int {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(v);
    spin_lock_irqsave(lock, &mut flags);
    let ret = (*v).counter;
    (*v).counter |= i;
    spin_unlock_irqrestore(lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn arch_atomic_fetch_xor(i: libc::c_int, v: *mut crate::atomic_t) -> libc::c_int {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(v);
    spin_lock_irqsave(lock, &mut flags);
    let ret = (*v).counter;
    (*v).counter ^= i;
    spin_unlock_irqrestore(lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn arch_atomic_xchg(v: *mut crate::atomic_t, new: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(v);
    spin_lock_irqsave(lock, &mut flags);
    let ret = (*v).counter;
    (*v).counter = new;
    spin_unlock_irqrestore(lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn arch_atomic_cmpxchg(v: *mut crate::atomic_t, old: libc::c_int, new: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(v);
    spin_lock_irqsave(lock, &mut flags);
    let ret = (*v).counter;
    if ret == old { (*v).counter = new; }
    spin_unlock_irqrestore(lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn arch_atomic_fetch_add_unless(v: *mut crate::atomic_t, a: libc::c_int, u: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(v);
    spin_lock_irqsave(lock, &mut flags);
    let ret = (*v).counter;
    if ret != u { (*v).counter += a; }
    spin_unlock_irqrestore(lock, flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn arch_atomic_set(v: *mut crate::atomic_t, i: libc::c_int) {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(v);
    spin_lock_irqsave(lock, &mut flags);
    (*v).counter = i;
    spin_unlock_irqrestore(lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn sp32___set_bit(addr: *mut libc::c_ulong, mask: libc::c_ulong) -> libc::c_ulong {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(addr);
    spin_lock_irqsave(lock, &mut flags);
    let old = *addr;
    *addr = old | mask;
    spin_unlock_irqrestore(lock, flags);
    old & mask
}

#[no_mangle]
pub unsafe extern "C" fn sp32___clear_bit(addr: *mut libc::c_ulong, mask: libc::c_ulong) -> libc::c_ulong {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(addr);
    spin_lock_irqsave(lock, &mut flags);
    let old = *addr;
    *addr = old & !mask;
    spin_unlock_irqrestore(lock, flags);
    old & mask
}

#[no_mangle]
pub unsafe extern "C" fn sp32___change_bit(addr: *mut libc::c_ulong, mask: libc::c_ulong) -> libc::c_ulong {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(addr);
    spin_lock_irqsave(lock, &mut flags);
    let old = *addr;
    *addr = old ^ mask;
    spin_unlock_irqrestore(lock, flags);
    old & mask
}

macro_rules! cmpxchg {
    ($name:ident, $ty:ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(ptr: *mut $ty, old: $ty, new: $ty) -> $ty {
            let mut flags: libc::c_ulong = 0;
            let lock = atomic_hash(ptr);
            spin_lock_irqsave(lock, &mut flags);
            let prev = *ptr;
            if prev == old { *ptr = new; }
            spin_unlock_irqrestore(lock, flags);
            prev
        }
    };
}

cmpxchg!(__cmpxchg_u8, u8);
cmpxchg!(__cmpxchg_u16, u16);
cmpxchg!(__cmpxchg_u32, u32);
cmpxchg!(__cmpxchg_u64, u64);

#[no_mangle]
pub unsafe extern "C" fn __xchg_u32(ptr: *mut u32, new: u32) -> libc::c_ulong {
    let mut flags: libc::c_ulong = 0;
    let lock = atomic_hash(ptr);
    spin_lock_irqsave(lock, &mut flags);
    let prev = *ptr;
    *ptr = new;
    spin_unlock_irqrestore(lock, flags);
    prev as libc::c_ulong
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
