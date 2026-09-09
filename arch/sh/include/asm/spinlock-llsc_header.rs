/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/spinlock-llsc.h
 *
 * Copyright (C) 2002, 2003 Paul Mundt
 * Copyright (C) 2006, 2007 Akio Idehara
 */

// Dependencies supplied by the surrounding translation unit:
// asm/barrier.h, asm/processor.h

/* Your basic SMP spinlocks, allowing only a single CPU anywhere */

#[inline]
pub unsafe fn arch_spin_is_locked(x: *const arch_spinlock_t) -> bool {
    (*x).lock <= 0
}

/* Simple spin lock operations. There are two variants, one clears IRQ's
 * on the local processor, one does not.
 *
 * We make no fairness assumptions. They have a cost.
 */
#[inline]
pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    let mut tmp: usize;
    let mut oldval: usize;
    core::arch::asm!(
        "1:",
        "movli.l @2, {tmp}",
        "mov {tmp}, {oldval}",
        "mov #0, {tmp}",
        "movco.l {tmp}, @2",
        "bf 1b",
        "cmp/pl {oldval}",
        "bf 1b",
        tmp = lateout(reg) tmp, oldval = lateout(reg) oldval,
        in("r2") &mut (*lock).lock,
        options(nostack)
    );
}

#[inline]
pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    let mut tmp: usize;
    mmiowb();
    core::arch::asm!(
        "mov #1, {tmp}",
        "mov.l {tmp}, @1",
        tmp = lateout(reg) tmp,
        in("r1") &mut (*lock).lock,
        options(nostack)
    );
}

#[inline]
pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> i32 {
    let mut tmp: usize;
    let mut oldval: usize;
    core::arch::asm!(
        "1:",
        "movli.l @2, {tmp}",
        "mov {tmp}, {oldval}",
        "mov #0, {tmp}",
        "movco.l {tmp}, @2",
        "bf 1b",
        "synco",
        tmp = lateout(reg) tmp, oldval = lateout(reg) oldval,
        in("r2") &mut (*lock).lock,
        options(nostack)
    );
    oldval as i32
}

/* Read-write spinlocks, allowing multiple readers but only one writer. */

#[inline]
pub unsafe fn arch_read_lock(rw: *mut arch_rwlock_t) {
    let mut tmp: usize;
    core::arch::asm!(
        "1:", "movli.l @1, {tmp}", "cmp/pl {tmp}", "bf 1b",
        "add #-1, {tmp}", "movco.l {tmp}, @1", "bf 1b",
        tmp = lateout(reg) tmp, in("r1") &mut (*rw).lock, options(nostack)
    );
}

#[inline]
pub unsafe fn arch_read_unlock(rw: *mut arch_rwlock_t) {
    let mut tmp: usize;
    core::arch::asm!(
        "1:", "movli.l @1, {tmp}", "add #1, {tmp}",
        "movco.l {tmp}, @1", "bf 1b",
        tmp = lateout(reg) tmp, in("r1") &mut (*rw).lock, options(nostack)
    );
}

#[inline]
pub unsafe fn arch_write_lock(rw: *mut arch_rwlock_t) {
    let mut tmp: usize;
    core::arch::asm!(
        "1:", "movli.l @1, {tmp}", "cmp/hs {bias}, {tmp}", "bf 1b",
        "sub {bias}, {tmp}", "movco.l {tmp}, @1", "bf 1b",
        tmp = lateout(reg) tmp, bias = in(reg) RW_LOCK_BIAS,
        in("r1") &mut (*rw).lock, options(nostack)
    );
}

#[inline]
pub unsafe fn arch_write_unlock(rw: *mut arch_rwlock_t) {
    core::arch::asm!(
        "mov.l {bias}, @0",
        in("r0") &mut (*rw).lock, bias = in(reg) RW_LOCK_BIAS,
        options(nostack)
    );
}

#[inline]
pub unsafe fn arch_read_trylock(rw: *mut arch_rwlock_t) -> bool {
    let mut tmp: usize;
    let mut oldval: usize;
    core::arch::asm!(
        "1:", "movli.l @2, {tmp}", "mov {tmp}, {oldval}",
        "cmp/pl {tmp}", "bf 2f", "add #-1, {tmp}",
        "movco.l {tmp}, @2", "bf 1b", "2:", "synco",
        tmp = lateout(reg) tmp, oldval = lateout(reg) oldval,
        in("r2") &mut (*rw).lock, options(nostack)
    );
    oldval > 0
}

#[inline]
pub unsafe fn arch_write_trylock(rw: *mut arch_rwlock_t) -> bool {
    let mut tmp: usize;
    let mut oldval: usize;
    core::arch::asm!(
        "1:", "movli.l @2, {tmp}", "mov {tmp}, {oldval}",
        "cmp/hs {bias}, {tmp}", "bf 2f", "sub {bias}, {tmp}",
        "2:", "movco.l {tmp}, @2", "bf 1b", "synco",
        tmp = lateout(reg) tmp, oldval = lateout(reg) oldval,
        bias = in(reg) RW_LOCK_BIAS, in("r2") &mut (*rw).lock,
        options(nostack)
    );
    oldval > (RW_LOCK_BIAS - 1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
