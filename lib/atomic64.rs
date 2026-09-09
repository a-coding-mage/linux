// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Generic implementation of 64-bit atomics using spinlocks,
 * useful on processors that don't have 64-bit atomic instructions.
 *
 * Copyright © 2009 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
 */

// linux/types.h, linux/cache.h, linux/spinlock.h, linux/init.h,
// linux/export.h, and linux/atomic.h provide the following names.

const NR_LOCKS: usize = 16;

#[repr(C)]
union Atomic64Lock {
    lock: arch_spinlock_t,
    pad: [u8; L1_CACHE_BYTES],
}

// __cacheline_aligned_in_smp
static mut atomic64_lock: [Atomic64Lock; NR_LOCKS] = [
    Atomic64Lock { lock: __ARCH_SPIN_LOCK_UNLOCKED };
    NR_LOCKS
];

#[inline]
unsafe fn lock_addr(v: *const atomic64_t) -> *mut arch_spinlock_t {
    let mut addr = v as usize;
    addr >>= L1_CACHE_SHIFT;
    addr ^= (addr >> 8) ^ (addr >> 16);
    &mut atomic64_lock[addr & (NR_LOCKS - 1)].lock
}

pub unsafe fn generic_atomic64_read(v: *const atomic64_t) -> s64 {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    let val = (*v).counter;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    val
}

pub unsafe fn generic_atomic64_set(v: *mut atomic64_t, i: s64) {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    (*v).counter = i;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
}

pub unsafe fn generic_atomic64_add(a: s64, v: *mut atomic64_t) {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    (*v).counter = (*v).counter.wrapping_add(a);
    arch_spin_unlock(lock);
    local_irq_restore(flags);
}

pub unsafe fn generic_atomic64_add_return(a: s64, v: *mut atomic64_t) -> s64 {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    let val = (*v).counter.wrapping_add(a);
    (*v).counter = val;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    val
}

pub unsafe fn generic_atomic64_fetch_add(a: s64, v: *mut atomic64_t) -> s64 {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    let val = (*v).counter;
    (*v).counter = (*v).counter.wrapping_add(a);
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    val
}

pub unsafe fn generic_atomic64_sub(a: s64, v: *mut atomic64_t) {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    (*v).counter = (*v).counter.wrapping_sub(a);
    arch_spin_unlock(lock);
    local_irq_restore(flags);
}

pub unsafe fn generic_atomic64_sub_return(a: s64, v: *mut atomic64_t) -> s64 {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    let val = (*v).counter.wrapping_sub(a);
    (*v).counter = val;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    val
}

pub unsafe fn generic_atomic64_fetch_sub(a: s64, v: *mut atomic64_t) -> s64 {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    let val = (*v).counter;
    (*v).counter = (*v).counter.wrapping_sub(a);
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    val
}

unsafe fn atomic64_fetch_bit_op(v: *mut atomic64_t, a: s64, op: u8) -> s64 {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    let val = (*v).counter;
    (*v).counter = match op {
        0 => (*v).counter & a,
        1 => (*v).counter | a,
        _ => (*v).counter ^ a,
    };
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    val
}

pub unsafe fn generic_atomic64_and(a: s64, v: *mut atomic64_t) { atomic64_fetch_bit_op(v, a, 0); }
pub unsafe fn generic_atomic64_or(a: s64, v: *mut atomic64_t) { atomic64_fetch_bit_op(v, a, 1); }
pub unsafe fn generic_atomic64_xor(a: s64, v: *mut atomic64_t) { atomic64_fetch_bit_op(v, a, 2); }
pub unsafe fn generic_atomic64_fetch_and(a: s64, v: *mut atomic64_t) -> s64 { atomic64_fetch_bit_op(v, a, 0) }
pub unsafe fn generic_atomic64_fetch_or(a: s64, v: *mut atomic64_t) -> s64 { atomic64_fetch_bit_op(v, a, 1) }
pub unsafe fn generic_atomic64_fetch_xor(a: s64, v: *mut atomic64_t) -> s64 { atomic64_fetch_bit_op(v, a, 2) }

pub unsafe fn generic_atomic64_dec_if_positive(v: *mut atomic64_t) -> s64 {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    let val = (*v).counter.wrapping_sub(1);
    if val >= 0 { (*v).counter = val; }
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    val
}

pub unsafe fn generic_atomic64_cmpxchg(v: *mut atomic64_t, o: s64, n: s64) -> s64 {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    let val = (*v).counter;
    if val == o { (*v).counter = n; }
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    val
}

pub unsafe fn generic_atomic64_xchg(v: *mut atomic64_t, new: s64) -> s64 {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    let val = (*v).counter;
    (*v).counter = new;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    val
}

pub unsafe fn generic_atomic64_fetch_add_unless(v: *mut atomic64_t, a: s64, u: s64) -> s64 {
    let mut flags: unsigned_long = 0;
    let lock = lock_addr(v);
    local_irq_save(&mut flags);
    arch_spin_lock(lock);
    let val = (*v).counter;
    if val != u { (*v).counter = (*v).counter.wrapping_add(a); }
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    val
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
