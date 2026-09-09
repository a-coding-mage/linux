/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2000 Philipp Rumpf <prumpf@tux.org>
 * Copyright (C) 2006 Kyle McMartin <kyle@parisc-linux.org>
 */

/* Translated from asm/atomic.h.  The included kernel types and primitives are
 * supplied by other translation units. */

#[cfg(CONFIG_SMP)]
pub const ATOMIC_HASH_SIZE: usize = 4;

#[cfg(CONFIG_SMP)]
extern "C" {
    static mut __atomic_hash: [arch_spinlock_t; ATOMIC_HASH_SIZE];
}

#[cfg(CONFIG_SMP)]
unsafe fn atomic_spin_lock_irqsave<T>(l: *const T, flags: &mut usize) {
    let s = &mut __atomic_hash[(((l as usize) / L1_CACHE_BYTES) & (ATOMIC_HASH_SIZE - 1))];
    local_irq_save(flags);
    arch_spin_lock(s as *mut arch_spinlock_t);
}

#[cfg(not(CONFIG_SMP))]
unsafe fn atomic_spin_lock_irqsave<T>(_l: *const T, flags: &mut usize) {
    local_irq_save(flags);
}

#[cfg(CONFIG_SMP)]
unsafe fn atomic_spin_unlock_irqrestore<T>(_l: *const T, flags: usize) {
    let s = &mut __atomic_hash[((( _l as usize) / L1_CACHE_BYTES) & (ATOMIC_HASH_SIZE - 1))];
    arch_spin_unlock(s as *mut arch_spinlock_t);
    local_irq_restore(flags);
}

#[cfg(not(CONFIG_SMP))]
unsafe fn atomic_spin_unlock_irqrestore<T>(_l: *const T, flags: usize) {
    local_irq_restore(flags);
}

pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) {
    let mut flags: usize = 0;
    atomic_spin_lock_irqsave(v, &mut flags);
    (*v).counter = i;
    atomic_spin_unlock_irqrestore(v, flags);
}

pub unsafe fn arch_atomic_set_release(v: *mut atomic_t, i: i32) { arch_atomic_set(v, i); }

pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 {
    core::ptr::read_volatile(&(*v).counter)
}

pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    let mut flags = 0usize;
    atomic_spin_lock_irqsave(v, &mut flags);
    (*v).counter = (*v).counter.wrapping_add(i);
    atomic_spin_unlock_irqrestore(v, flags);
}
pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags = 0usize;
    atomic_spin_lock_irqsave(v, &mut flags);
    let ret = (*v).counter.wrapping_add(i); (*v).counter = ret;
    atomic_spin_unlock_irqrestore(v, flags); ret
}
pub unsafe fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags = 0usize;
    atomic_spin_lock_irqsave(v, &mut flags);
    let ret = (*v).counter; (*v).counter = ret.wrapping_add(i);
    atomic_spin_unlock_irqrestore(v, flags); ret
}
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) { arch_atomic_add(i.wrapping_neg(), v); }
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_add_return(i.wrapping_neg(), v) }
pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_fetch_add(i.wrapping_neg(), v) }

unsafe fn atomic_fetch_bit_op(i: i32, v: *mut atomic_t, op: u8) -> i32 {
    let mut flags = 0usize;
    atomic_spin_lock_irqsave(v, &mut flags);
    let ret = (*v).counter;
    (*v).counter = match op { 0 => ret & i, 1 => ret | i, _ => ret ^ i };
    atomic_spin_unlock_irqrestore(v, flags); ret
}
pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) { let _ = atomic_fetch_bit_op(i, v, 0); }
pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) { let _ = atomic_fetch_bit_op(i, v, 1); }
pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) { let _ = atomic_fetch_bit_op(i, v, 2); }
pub unsafe fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_bit_op(i, v, 0) }
pub unsafe fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_bit_op(i, v, 1) }
pub unsafe fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_bit_op(i, v, 2) }

#[cfg(CONFIG_64BIT)]
pub const fn ATOMIC64_INIT(i: i64) -> atomic64_t { atomic64_t { counter: i } }

#[cfg(CONFIG_64BIT)]
pub unsafe fn arch_atomic64_add(i: i64, v: *mut atomic64_t) { let mut f=0usize; atomic_spin_lock_irqsave(v,&mut f); (*v).counter=(*v).counter.wrapping_add(i); atomic_spin_unlock_irqrestore(v,f); }
#[cfg(CONFIG_64BIT)]
pub unsafe fn arch_atomic64_add_return(i: i64, v: *mut atomic64_t) -> i64 { let mut f=0usize; atomic_spin_lock_irqsave(v,&mut f); let r=(*v).counter.wrapping_add(i); (*v).counter=r; atomic_spin_unlock_irqrestore(v,f); r }
#[cfg(CONFIG_64BIT)]
pub unsafe fn arch_atomic64_fetch_add(i: i64, v: *mut atomic64_t) -> i64 { let mut f=0usize; atomic_spin_lock_irqsave(v,&mut f); let r=(*v).counter; (*v).counter=r.wrapping_add(i); atomic_spin_unlock_irqrestore(v,f); r }
#[cfg(CONFIG_64BIT)]
pub unsafe fn arch_atomic64_sub(i: i64, v: *mut atomic64_t) { arch_atomic64_add(i.wrapping_neg(),v); }
#[cfg(CONFIG_64BIT)]
pub unsafe fn arch_atomic64_sub_return(i: i64, v: *mut atomic64_t) -> i64 { arch_atomic64_add_return(i.wrapping_neg(),v) }
#[cfg(CONFIG_64BIT)]
pub unsafe fn arch_atomic64_fetch_sub(i: i64, v: *mut atomic64_t) -> i64 { arch_atomic64_fetch_add(i.wrapping_neg(),v) }
#[cfg(CONFIG_64BIT)]
pub unsafe fn arch_atomic64_set(v: *mut atomic64_t, i: i64) { let mut f=0usize; atomic_spin_lock_irqsave(v,&mut f); (*v).counter=i; atomic_spin_unlock_irqrestore(v,f); }
#[cfg(CONFIG_64BIT)]
pub unsafe fn arch_atomic64_set_release(v: *mut atomic64_t, i: i64) { arch_atomic64_set(v,i); }
#[cfg(CONFIG_64BIT)]
pub unsafe fn arch_atomic64_read(v: *const atomic64_t) -> i64 { core::ptr::read_volatile(&(*v).counter) }

/* External types and primitives referenced above are provided by the kernel. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
