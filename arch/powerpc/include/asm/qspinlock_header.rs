/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[cfg(CONFIG_PPC64)]
pub const _Q_SPIN_EH_HINT: i32 = 1;
#[cfg(not(CONFIG_PPC64))]
pub const _Q_SPIN_EH_HINT: i32 = 0;

/*
 * The trylock itself may steal. This makes trylocks slightly stronger, and
 * makes locks slightly more efficient when stealing.
 *
 * This is compile-time, so if true then there may always be stealers, so the
 * nosteal paths become unused.
 */
pub const _Q_SPIN_TRY_LOCK_STEAL: i32 = 1;

/* Put a speculation barrier after testing the lock/node and finding it busy. */
pub const _Q_SPIN_SPEC_BARRIER: i32 = 0;

#[cfg(CONFIG_PPC64)]
pub const _Q_SPIN_MISO: i32 = 0;
#[cfg(not(CONFIG_PPC64))]
pub const _Q_SPIN_MISO: i32 = 0;

#[cfg(CONFIG_PPC64)]
pub const _Q_SPIN_MISO_UNLOCK: i32 = 0;
#[cfg(not(CONFIG_PPC64))]
pub const _Q_SPIN_MISO_UNLOCK: i32 = 0;

pub const _Q_SPIN_PREFETCH_NEXT: i32 = 0;

#[inline(always)]
pub unsafe fn queued_spin_is_locked(lock: *mut qspinlock) -> i32 {
    core::ptr::read_volatile(core::ptr::addr_of!((*lock).val)) as i32
}

#[inline(always)]
pub unsafe fn queued_spin_value_unlocked(lock: qspinlock) -> i32 {
    (!lock.val) as i32
}

#[inline(always)]
pub unsafe fn queued_spin_is_contended(lock: *mut qspinlock) -> i32 {
    (core::ptr::read_volatile(core::ptr::addr_of!((*lock).val)) & _Q_TAIL_CPU_MASK) != 0 as u32 as i32
}

#[inline(always)]
pub unsafe fn queued_spin_encode_locked_val() -> u32 {
    /* XXX: make this use lock value in paca like simple spinlocks? */
    _Q_LOCKED_VAL | (smp_processor_id() << _Q_OWNER_CPU_OFFSET)
}

#[inline(always)]
pub unsafe fn __queued_spin_trylock_nosteal(lock: *mut qspinlock) -> i32 {
    let new = queued_spin_encode_locked_val();
    let mut prev: u32;

    /* Trylock succeeds only when unlocked and no queued nodes */
    core::arch::asm!(
        "1: lwarx {prev}, 0, {ptr}, {hint}",
        "cmpwi 0, {prev}, 0",
        "bne- 2f",
        "stwcx. {new}, 0, {ptr}",
        "bne- 1b",
        "2:",
        prev = lateout(reg) prev,
        ptr = in(reg) core::ptr::addr_of_mut!((*lock).val),
        new = in(reg) new,
        hint = const _Q_SPIN_EH_HINT,
        options(nostack)
    );
    (prev == 0) as i32
}

#[inline(always)]
pub unsafe fn __queued_spin_trylock_steal(lock: *mut qspinlock) -> i32 {
    let new = queued_spin_encode_locked_val();
    let mut prev: u32;
    let mut tmp: u32;

    /* Trylock may get ahead of queued nodes if it finds unlocked */
    core::arch::asm!(
        "1: lwarx {prev}, 0, {ptr}, {hint}",
        "andc. {tmp}, {prev}, {mask}",
        "bne- 2f",
        "and {tmp}, {prev}, {mask}",
        "or {tmp}, {tmp}, {new}",
        "stwcx. {tmp}, 0, {ptr}",
        "bne- 1b",
        "2:",
        prev = lateout(reg) prev,
        tmp = lateout(reg) tmp,
        ptr = in(reg) core::ptr::addr_of_mut!((*lock).val),
        new = in(reg) new,
        mask = in(reg) _Q_TAIL_CPU_MASK,
        hint = const _Q_SPIN_EH_HINT,
        options(nostack)
    );
    (!(prev & !_Q_TAIL_CPU_MASK)) as i32
}

#[inline(always)]
pub unsafe fn queued_spin_trylock(lock: *mut qspinlock) -> i32 {
    if _Q_SPIN_TRY_LOCK_STEAL == 0 {
        __queued_spin_trylock_nosteal(lock)
    } else {
        __queued_spin_trylock_steal(lock)
    }
}

extern "C" {
    pub fn queued_spin_lock_slowpath(lock: *mut qspinlock);
}

#[inline(always)]
pub unsafe fn queued_spin_lock(lock: *mut qspinlock) {
    if queued_spin_trylock(lock) == 0 {
        queued_spin_lock_slowpath(lock);
    }
}

#[inline]
pub unsafe fn queued_spin_unlock(lock: *mut qspinlock) {
    smp_store_release(core::ptr::addr_of_mut!((*lock).locked), 0);
    if _Q_SPIN_MISO_UNLOCK != 0 {
        core::arch::asm!("miso", options(nostack));
    }
}

pub use queued_spin_is_locked as arch_spin_is_locked;
pub use queued_spin_is_contended as arch_spin_is_contended;
pub use queued_spin_value_unlocked as arch_spin_value_unlocked;
pub use queued_spin_lock as arch_spin_lock;
pub use queued_spin_trylock as arch_spin_trylock;
pub use queued_spin_unlock as arch_spin_unlock;

#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
extern "C" {
    pub fn pv_spinlocks_init();
}

#[cfg(not(CONFIG_PARAVIRT_SPINLOCKS))]
#[inline]
pub unsafe fn pv_spinlocks_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
