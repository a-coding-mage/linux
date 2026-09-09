/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Translated from asm/spinlock.h. Dependencies are supplied by other headers.

#[inline]
pub unsafe fn arch_spin_is_locked(x: *const arch_spinlock_t) -> bool {
    (*x).slock != __ARCH_SPIN_LOCK_UNLOCKED__
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[inline]
pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    let mut val: u32;
    core::arch::asm!(
        "1: llock {val}, [{slock}]",
        "breq {val}, {locked}, 1b",
        "scond {locked}, [{slock}]",
        "bnz 1b",
        val = lateout(reg) val,
        slock = in(reg) &mut (*lock).slock,
        locked = in(reg) __ARCH_SPIN_LOCK_LOCKED__,
        options(nostack)
    );
    smp_mb();
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[inline]
pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> i32 {
    let mut val: u32;
    let mut got_it: u32 = 0;
    core::arch::asm!(
        "1: llock {val}, [{slock}]",
        "breq {val}, {locked}, 4f",
        "scond {locked}, [{slock}]",
        "bnz 1b",
        "mov {got_it}, 1",
        "4:",
        val = lateout(reg) val,
        got_it = inout(reg) got_it,
        slock = in(reg) &mut (*lock).slock,
        locked = in(reg) __ARCH_SPIN_LOCK_LOCKED__,
        options(nostack)
    );
    smp_mb();
    got_it as i32
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[inline]
pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    smp_mb();
    core::ptr::write_volatile(&mut (*lock).slock, __ARCH_SPIN_LOCK_UNLOCKED__);
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[inline]
pub unsafe fn arch_read_lock(rw: *mut arch_rwlock_t) {
    let mut val: u32;
    core::arch::asm!(
        "1: llock {val}, [{rwlock}]",
        "brls {val}, 0, 1b",
        "sub {val}, {val}, 1",
        "scond {val}, [{rwlock}]",
        "bnz 1b",
        val = lateout(reg) val,
        rwlock = in(reg) &mut (*rw).counter,
        options(nostack)
    );
    smp_mb();
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[inline]
pub unsafe fn arch_read_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut val: u32;
    let mut got_it: u32 = 0;
    core::arch::asm!(
        "1: llock {val}, [{rwlock}]",
        "brls {val}, 0, 4f",
        "sub {val}, {val}, 1",
        "scond {val}, [{rwlock}]",
        "bnz 1b",
        "mov {got_it}, 1",
        "4:",
        val = lateout(reg) val,
        got_it = inout(reg) got_it,
        rwlock = in(reg) &mut (*rw).counter,
        options(nostack)
    );
    smp_mb();
    got_it as i32
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[inline]
pub unsafe fn arch_write_lock(rw: *mut arch_rwlock_t) {
    let mut val: u32;
    core::arch::asm!(
        "1: llock {val}, [{rwlock}]",
        "brne {val}, {unlocked}, 1b",
        "mov {val}, 0",
        "scond {val}, [{rwlock}]",
        "bnz 1b",
        val = lateout(reg) val,
        rwlock = in(reg) &mut (*rw).counter,
        unlocked = in(reg) __ARCH_RW_LOCK_UNLOCKED__,
        options(nostack)
    );
    smp_mb();
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[inline]
pub unsafe fn arch_write_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut val: u32;
    let mut got_it: u32 = 0;
    core::arch::asm!(
        "1: llock {val}, [{rwlock}]",
        "brne {val}, {unlocked}, 4f",
        "mov {val}, 0",
        "scond {val}, [{rwlock}]",
        "bnz 1b",
        "mov {got_it}, 1",
        "4:",
        val = lateout(reg) val,
        got_it = inout(reg) got_it,
        rwlock = in(reg) &mut (*rw).counter,
        unlocked = in(reg) __ARCH_RW_LOCK_UNLOCKED__,
        options(nostack)
    );
    smp_mb();
    got_it as i32
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[inline]
pub unsafe fn arch_read_unlock(rw: *mut arch_rwlock_t) {
    let mut val: u32;
    smp_mb();
    core::arch::asm!(
        "1: llock {val}, [{rwlock}]",
        "add {val}, {val}, 1",
        "scond {val}, [{rwlock}]",
        "bnz 1b",
        val = lateout(reg) val,
        rwlock = in(reg) &mut (*rw).counter,
        options(nostack)
    );
}

#[cfg(CONFIG_ARC_HAS_LLSC)]
#[inline]
pub unsafe fn arch_write_unlock(rw: *mut arch_rwlock_t) {
    smp_mb();
    core::ptr::write_volatile(&mut (*rw).counter, __ARCH_RW_LOCK_UNLOCKED__);
}

#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[inline]
pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    let mut val = __ARCH_SPIN_LOCK_LOCKED__;
    smp_mb();
    core::arch::asm!("1: ex {0}, [{1}]", "breq {0}, {2}, 1b", inout(reg) val, in(reg) &mut (*lock).slock, in(reg) __ARCH_SPIN_LOCK_LOCKED__, options(nostack));
    smp_mb();
}

#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[inline]
pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> i32 {
    let mut val = __ARCH_SPIN_LOCK_LOCKED__;
    smp_mb();
    core::arch::asm!("1: ex {0}, [{1}]", inout(reg) val, in(reg) &mut (*lock).slock, options(nostack));
    smp_mb();
    (val == __ARCH_SPIN_LOCK_UNLOCKED__) as i32
}

#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[inline]
pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    let mut val = __ARCH_SPIN_LOCK_UNLOCKED__;
    smp_mb();
    core::arch::asm!("ex {0}, [{1}]", inout(reg) val, in(reg) &mut (*lock).slock, options(nostack));
    smp_mb();
}

#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[inline]
pub unsafe fn arch_read_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut ret = 0;
    let mut flags: c_ulong;
    local_irq_save(&mut flags);
    arch_spin_lock(&mut (*rw).lock_mutex);
    if (*rw).counter > 0 { (*rw).counter -= 1; ret = 1; }
    arch_spin_unlock(&mut (*rw).lock_mutex);
    local_irq_restore(flags);
    ret
}

#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[inline]
pub unsafe fn arch_write_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut ret = 0;
    let mut flags: c_ulong;
    local_irq_save(&mut flags);
    arch_spin_lock(&mut (*rw).lock_mutex);
    if (*rw).counter == __ARCH_RW_LOCK_UNLOCKED__ { (*rw).counter = 0; ret = 1; }
    arch_spin_unlock(&mut (*rw).lock_mutex);
    local_irq_restore(flags);
    ret
}

#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[inline]
pub unsafe fn arch_read_lock(rw: *mut arch_rwlock_t) { while arch_read_trylock(rw) == 0 { cpu_relax(); } }

#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[inline]
pub unsafe fn arch_write_lock(rw: *mut arch_rwlock_t) { while arch_write_trylock(rw) == 0 { cpu_relax(); } }

#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[inline]
pub unsafe fn arch_read_unlock(rw: *mut arch_rwlock_t) {
    let mut flags: c_ulong;
    local_irq_save(&mut flags);
    arch_spin_lock(&mut (*rw).lock_mutex);
    (*rw).counter += 1;
    arch_spin_unlock(&mut (*rw).lock_mutex);
    local_irq_restore(flags);
}

#[cfg(not(CONFIG_ARC_HAS_LLSC))]
#[inline]
pub unsafe fn arch_write_unlock(rw: *mut arch_rwlock_t) {
    let mut flags: c_ulong;
    local_irq_save(&mut flags);
    arch_spin_lock(&mut (*rw).lock_mutex);
    (*rw).counter = __ARCH_RW_LOCK_UNLOCKED__;
    arch_spin_unlock(&mut (*rw).lock_mutex);
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
