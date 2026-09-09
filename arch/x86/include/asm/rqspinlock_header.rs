/* SPDX-License-Identifier: GPL-2.0 */

// Translation of <asm/paravirt.h> and <asm-generic/rqspinlock.h> is supplied
// by the surrounding translation unit.

#[cfg(feature = "CONFIG_PARAVIRT")]
extern "C" {
    pub static virt_spin_lock_key: StaticKeyFalse;
}

#[cfg(feature = "CONFIG_PARAVIRT")]
#[repr(C)]
pub struct StaticKeyFalse {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_PARAVIRT")]
extern "C" {
    fn static_branch_likely(key: *const StaticKeyFalse) -> bool;
}

#[cfg(feature = "CONFIG_PARAVIRT")]
#[inline(always)]
pub unsafe fn resilient_virt_spin_lock_enabled() -> bool {
    unsafe { static_branch_likely(&virt_spin_lock_key as *const StaticKeyFalse) }
}

#[cfg(all(feature = "CONFIG_PARAVIRT", feature = "CONFIG_QUEUED_SPINLOCKS"))]
pub type rqspinlock_t = qspinlock;

#[cfg(all(feature = "CONFIG_PARAVIRT", not(feature = "CONFIG_QUEUED_SPINLOCKS")))]
pub type rqspinlock_t = rqspinlock;

#[cfg(feature = "CONFIG_PARAVIRT")]
extern "C" {
    pub fn resilient_tas_spin_lock(lock: *mut rqspinlock_t) -> i32;
}

#[cfg(feature = "CONFIG_PARAVIRT")]
#[inline]
pub unsafe fn resilient_virt_spin_lock(lock: *mut rqspinlock_t) -> i32 {
    unsafe { resilient_tas_spin_lock(lock) }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
