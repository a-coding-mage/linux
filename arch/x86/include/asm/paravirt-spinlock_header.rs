/* SPDX-License-Identifier: GPL-2.0-only */

// C dependencies: asm/paravirt_types.h, linux/static_call_types.h,
// asm/spinlock_types.h (when CONFIG_SMP is enabled).

use core::ffi::c_void;

#[repr(C)]
pub struct qspinlock {
    pub val: u32,
    pub locked: u8,
}

#[repr(C)]
pub struct paravirt_callee_save {
    pub func: *const c_void,
}

#[repr(C)]
pub struct pv_lock_ops {
    pub wait: Option<unsafe extern "C" fn(ptr: *mut u8, val: u8)>,
    pub kick: Option<unsafe extern "C" fn(cpu: i32)>,
    pub vcpu_is_preempted: paravirt_callee_save,
}

unsafe extern "C" {
    pub static mut pv_ops_lock: pv_lock_ops;
    pub static mut nopvspin: bool;

    pub fn native_queued_spin_lock_slowpath(lock: *mut qspinlock, val: u32);
    pub fn __pv_init_lock_hash();
    pub fn __pv_queued_spin_lock_slowpath(lock: *mut qspinlock, val: u32);
    pub fn __raw_callee_save___native_queued_spin_unlock(lock: *mut qspinlock);
    pub fn __raw_callee_save___pv_queued_spin_unlock(lock: *mut qspinlock);
    pub fn __raw_callee_save___native_queued_spin_unlock(lock: *mut qspinlock);
    pub fn __raw_callee_save___native_vcpu_is_preempted(cpu: i64) -> bool;

    pub fn native_pv_lock_init();
    pub fn __native_queued_spin_unlock(lock: *mut qspinlock);
    pub fn native_queued_spin_unlock_traced(lock: *mut qspinlock);
    pub fn pv_queued_spin_unlock_traced(lock: *mut qspinlock);
    pub fn pv_is_native_spin_unlock() -> bool;
    pub fn __native_vcpu_is_preempted(cpu: i64) -> bool;
    pub fn pv_is_native_vcpu_is_preempted() -> bool;
    pub fn kcsan_release();
    pub fn static_branch_likely(key: *const static_key_false) -> bool;
    pub fn atomic_read(ptr: *const u32) -> i32;
    pub fn atomic_try_cmpxchg(ptr: *mut u32, old: *mut i32, new: u32) -> bool;
    pub fn cpu_relax();
}

#[repr(C)]
pub struct static_key_false {
    pub enabled: bool,
}

// DECLARE_STATIC_CALL(queued_spin_lock_slowpath, native_queued_spin_lock_slowpath)
// DECLARE_STATIC_CALL(queued_spin_unlock, __raw_callee_save___native_queued_spin_unlock)
// DECLARE_STATIC_KEY_FALSE(virt_spin_lock_key)
unsafe extern "C" {
    pub static mut virt_spin_lock_key: static_key_false;
}

#[inline(always)]
pub unsafe fn pv_queued_spin_lock_slowpath(lock: *mut qspinlock, val: u32) {
    native_queued_spin_lock_slowpath(lock, val);
}

#[inline(always)]
pub unsafe fn pv_queued_spin_unlock(lock: *mut qspinlock) {
    // The C implementation performs a static-call trampoline invocation with
    // PVOP_VCALLEE_CLOBBERS and ASM_CALL_CONSTRAINT; preserve that external call.
    __raw_callee_save___native_queued_spin_unlock(lock);
}

#[inline(always)]
pub unsafe fn pv_vcpu_is_preempted(cpu: i64) -> bool {
    // PVOP_ALT_CALLEE1 selects the paravirtual callee, or clears eax when the
    // alternate X86_FEATURE_VCPUPREEMPT path is selected.
    __raw_callee_save___native_vcpu_is_preempted(cpu)
}

// queued_spin_unlock - release a queued spinlock.
// A smp_store_release() on the least-significant byte.
#[inline]
pub unsafe fn native_queued_spin_unlock(lock: *mut qspinlock) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*lock).locked), 0);
}

#[inline]
pub unsafe fn queued_spin_lock_slowpath(lock: *mut qspinlock, val: u32) {
    pv_queued_spin_lock_slowpath(lock, val);
}

#[inline]
pub unsafe fn queued_spin_unlock(lock: *mut qspinlock) {
    kcsan_release();
    pv_queued_spin_unlock(lock);
}

#[inline]
pub unsafe fn vcpu_is_preempted(cpu: i64) -> bool {
    pv_vcpu_is_preempted(cpu)
}

#[inline(always)]
pub unsafe fn pv_wait(ptr: *mut u8, val: u8) {
    if let Some(wait) = pv_ops_lock.wait {
        wait(ptr, val);
    }
}

#[inline(always)]
pub unsafe fn pv_kick(cpu: i32) {
    if let Some(kick) = pv_ops_lock.kick {
        kick(cpu);
    }
}

#[inline]
pub unsafe fn virt_spin_lock(lock: *mut qspinlock) -> bool {
    if !static_branch_likely(core::ptr::addr_of!(virt_spin_lock_key)) {
        return false;
    }

    // On hypervisors without PARAVIRT_SPINLOCKS support, fall back to a
    // test-and-set spinlock because fair locks have holder-preemption issues.
    loop {
        let mut val = atomic_read(core::ptr::addr_of!((*lock).val));
        if val == 0 && atomic_try_cmpxchg(
            core::ptr::addr_of_mut!((*lock).val),
            &mut val,
            1,
        ) {
            return true;
        }
        cpu_relax();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
