/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *    Copyright IBM Corp. 1999
 *    Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 *
 *  Derived from "include/asm-i386/spinlock.h"
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline(always)]
pub unsafe fn spinlock_lockval() -> u32 {
    let mut _lc_lockval: usize;
    let mut lockval: u32;

    // BUILD_BUG_ON(sizeof_field(struct lowcore, spinlock_lockval) != sizeof(lockval));
    _lc_lockval = core::mem::offset_of!(lowcore, spinlock_lockval);
    // The C implementation uses ALTERNATIVE assembly to load the lock value
    // from lowcore, selecting the address according to MFEATURE_LOWCORE.
    // This architecture-specific alternative has no file-local Rust mapping.
    core::hint::unreachable_unchecked()
}

extern "C" {
    pub static mut spin_retry: i32;

    pub fn arch_vcpu_is_preempted(cpu: i32) -> bool;

    pub fn arch_spin_relax(lock: *mut arch_spinlock_t);
    pub fn arch_spin_lock_wait(lock: *mut arch_spinlock_t);
    pub fn arch_spin_trylock_retry(lock: *mut arch_spinlock_t) -> i32;
    pub fn arch_spin_lock_setup(cpu: i32);

    pub fn arch_read_lock_wait(lp: *mut arch_rwlock_t);
    pub fn arch_write_lock_wait(lp: *mut arch_rwlock_t);
}

#[inline(always)]
pub unsafe fn arch_spin_lockval(cpu: i32) -> u32 {
    (cpu + 1) as u32
}

#[inline(always)]
pub unsafe fn arch_spin_value_unlocked(lock: arch_spinlock_t) -> i32 {
    (lock.lock == 0) as i32
}

#[inline(always)]
pub unsafe fn arch_spin_is_locked(lp: *mut arch_spinlock_t) -> i32 {
    (core::ptr::read_volatile(core::ptr::addr_of!((*lp).lock)) != 0) as i32
}

#[inline(always)]
pub unsafe fn arch_spin_trylock_once(lp: *mut arch_spinlock_t) -> i32 {
    let mut old: i32 = 0;
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    (arch_try_cmpxchg(
        core::ptr::addr_of_mut!((*lp).lock),
        &mut old,
        spinlock_lockval(),
    ) != 0) as i32
}

#[inline(always)]
pub unsafe fn arch_spin_lock(lp: *mut arch_spinlock_t) {
    if arch_spin_trylock_once(lp) == 0 {
        arch_spin_lock_wait(lp);
    }
}

#[inline(always)]
pub unsafe fn arch_spin_trylock(lp: *mut arch_spinlock_t) -> i32 {
    if arch_spin_trylock_once(lp) == 0 {
        arch_spin_trylock_retry(lp)
    } else {
        1
    }
}

#[inline(always)]
pub unsafe fn arch_spin_unlock(lp: *mut arch_spinlock_t) {
    // typecheck(int, lp->lock); kcsan_release();
    // ALTERNATIVE(...); mvhhi %[lock],0 (with a memory clobber).
    core::ptr::write_volatile(
        (core::ptr::addr_of_mut!((*lp).lock) as *mut u16).add(1),
        0,
    );
}

#[inline(always)]
pub unsafe fn arch_read_relax(_rw: *mut arch_rwlock_t) {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn arch_write_relax(_rw: *mut arch_rwlock_t) {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn arch_read_lock(rw: *mut arch_rwlock_t) {
    let old = __atomic_add(1, core::ptr::addr_of_mut!((*rw).cnts));
    if (old & 0xffff0000) != 0 {
        arch_read_lock_wait(rw);
    }
}

#[inline(always)]
pub unsafe fn arch_read_unlock(rw: *mut arch_rwlock_t) {
    __atomic_add_const_barrier(-1, core::ptr::addr_of_mut!((*rw).cnts));
}

#[inline(always)]
pub unsafe fn arch_write_lock(rw: *mut arch_rwlock_t) {
    let mut old = 0;
    if arch_try_cmpxchg(core::ptr::addr_of_mut!((*rw).cnts), &mut old, 0x30000) == 0 {
        arch_write_lock_wait(rw);
    }
}

#[inline(always)]
pub unsafe fn arch_write_unlock(rw: *mut arch_rwlock_t) {
    __atomic_add_barrier(-0x30000, core::ptr::addr_of_mut!((*rw).cnts));
}

#[inline(always)]
pub unsafe fn arch_read_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut old = core::ptr::read_volatile(core::ptr::addr_of!((*rw).cnts));
    ((!((old & 0xffff0000) != 0)
        && arch_try_cmpxchg(core::ptr::addr_of_mut!((*rw).cnts), &mut old, old + 1) != 0)) as i32
}

#[inline(always)]
pub unsafe fn arch_write_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut old = core::ptr::read_volatile(core::ptr::addr_of!((*rw).cnts));
    ((old == 0 && arch_try_cmpxchg(core::ptr::addr_of_mut!((*rw).cnts), &mut old, 0x30000) != 0)) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
