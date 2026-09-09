/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Resilient Queued Spin Lock */

// Dependencies supplied by the surrounding kernel translation.
use core::sync::atomic::AtomicI32;

#[repr(C)]
pub union Rqspinlock {
    pub val: AtomicI32,
    pub locked: u32,
}

/* Distinct BTF type, despite having the same layout as Rqspinlock. */
#[repr(C, align(4))]
pub struct BpfResSpinLock {
    pub val: u32,
}

#[repr(C)]
pub struct Qspinlock {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_QUEUED_SPINLOCKS")]
pub type RqspinlockT = Qspinlock;
#[cfg(not(feature = "CONFIG_QUEUED_SPINLOCKS"))]
pub type RqspinlockT = Rqspinlock;

extern "C" {
    pub fn resilient_tas_spin_lock(lock: *mut RqspinlockT) -> i32;
    #[cfg(feature = "CONFIG_QUEUED_SPINLOCKS")]
    pub fn resilient_queued_spin_lock_slowpath(lock: *mut RqspinlockT, val: u32) -> i32;
}

#[inline(always)]
pub fn resilient_virt_spin_lock_enabled() -> bool { false }

#[inline(always)]
pub unsafe fn resilient_virt_spin_lock(_lock: *mut RqspinlockT) -> i32 { 0 }

/* Default timeout for waiting loops is 0.25 seconds. */
pub const RES_DEF_TIMEOUT: u64 = NSEC_PER_SEC / 4;
pub const RES_NR_HELD: usize = 31;

#[repr(C)]
pub struct RqspinlockHeld {
    pub cnt: i32,
    pub locks: [*mut core::ffi::c_void; RES_NR_HELD],
}

extern "C" {
    pub static mut rqspinlock_held_locks: RqspinlockHeld;
    pub fn this_cpu_inc_return(cnt: *mut i32) -> i32;
    pub fn this_cpu_write(slot: *mut *mut core::ffi::c_void, value: *mut core::ffi::c_void);
    pub fn this_cpu_ptr(value: *mut RqspinlockHeld) -> *mut RqspinlockHeld;
    pub fn this_cpu_dec(cnt: *mut i32);
    pub fn smp_wmb();
    pub fn smp_store_release(ptr: *mut u32, value: u32);
    pub fn preempt_disable();
    pub fn preempt_enable();
}

#[inline(always)]
pub unsafe fn grab_held_lock_entry(lock: *mut core::ffi::c_void) {
    let cnt = this_cpu_inc_return(core::ptr::addr_of_mut!(rqspinlock_held_locks.cnt));
    if cnt > RES_NR_HELD as i32 { return; }
    this_cpu_write(
        core::ptr::addr_of_mut!(rqspinlock_held_locks.locks[(cnt - 1) as usize]),
        lock,
    );
}

#[inline(always)]
pub unsafe fn release_held_lock_entry() {
    let rqh = this_cpu_ptr(core::ptr::addr_of_mut!(rqspinlock_held_locks));
    if (*rqh).cnt <= RES_NR_HELD as i32 {
        core::ptr::write_volatile(&mut (*rqh).locks[((*rqh).cnt - 1) as usize], core::ptr::null_mut());
    }
    smp_wmb();
    this_cpu_dec(core::ptr::addr_of_mut!(rqspinlock_held_locks.cnt));
}

#[cfg(feature = "CONFIG_QUEUED_SPINLOCKS")]
#[inline(always)]
pub unsafe fn res_spin_lock(lock: *mut RqspinlockT) -> i32 {
    let mut val: i32 = 0;
    grab_held_lock_entry(lock.cast());
    // Equivalent to atomic_try_cmpxchg_acquire(&lock->val, &val, _Q_LOCKED_VAL).
    if atomic_try_cmpxchg_acquire(lock, &mut val, 0x1) { 0 }
    else { resilient_queued_spin_lock_slowpath(lock, val as u32) }
}

#[cfg(not(feature = "CONFIG_QUEUED_SPINLOCKS"))]
#[inline(always)]
pub unsafe fn res_spin_lock(lock: *mut RqspinlockT) -> i32 {
    grab_held_lock_entry(lock.cast());
    resilient_tas_spin_lock(lock)
}

extern "C" {
    fn atomic_try_cmpxchg_acquire(lock: *mut RqspinlockT, old: *mut i32, new: i32) -> bool;
}

#[inline(always)]
pub unsafe fn res_spin_unlock(lock: *mut RqspinlockT) {
    let rqh = this_cpu_ptr(core::ptr::addr_of_mut!(rqspinlock_held_locks));
    smp_store_release(lock.cast::<u32>(), 0);
    if (*rqh).cnt <= RES_NR_HELD as i32 {
        core::ptr::write_volatile(&mut (*rqh).locks[((*rqh).cnt - 1) as usize], core::ptr::null_mut());
    }
    this_cpu_dec(core::ptr::addr_of_mut!(rqspinlock_held_locks.cnt));
}

#[cfg(feature = "CONFIG_QUEUED_SPINLOCKS")]
pub unsafe fn raw_res_spin_lock_init(lock: *mut RqspinlockT) {
    *lock = core::mem::zeroed(); // __ARCH_SPIN_LOCK_UNLOCKED
}

#[cfg(not(feature = "CONFIG_QUEUED_SPINLOCKS"))]
pub unsafe fn raw_res_spin_lock_init(lock: *mut RqspinlockT) {
    *lock = core::mem::zeroed();
}

#[inline(always)]
pub unsafe fn raw_res_spin_lock(lock: *mut RqspinlockT) -> i32 {
    preempt_disable();
    let ret = res_spin_lock(lock);
    if ret != 0 { preempt_enable(); }
    ret
}

#[inline(always)]
pub unsafe fn raw_res_spin_unlock(lock: *mut RqspinlockT) {
    res_spin_unlock(lock);
    preempt_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
