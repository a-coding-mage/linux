// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012 Linaro : Daniel Lezcano <daniel.lezcano@linaro.org> (IBM)
 *
 * Based on the work of Rickard Andersson <rickard.andersson@stericsson.com>
 * and Jonas Aaberg <jonas.aberg@stericsson.com>.
 */

// Linux kernel headers and symbols are supplied by other translation units.

use core::ffi::c_void;

#[repr(C)]
pub struct cpuidle_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct cpuidle_driver {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _opaque: [u8; 0],
}

#[repr(C)]
struct atomic_t {
    counter: core::sync::atomic::AtomicI32,
}

extern "C" {
    fn smp_processor_id() -> i32;
    fn num_online_cpus() -> i32;
    fn cpu_do_idle();
    fn prcmu_gic_decouple() -> i32;
    fn prcmu_is_cpu_in_wfi(cpu: i32) -> bool;
    fn prcmu_copy_gic_settings() -> i32;
    fn prcmu_gic_pending_irq() -> i32;
    fn prcmu_pending_irq() -> i32;
    fn db8500_prcmu_set_power_state(state: i32, arg1: bool, arg2: bool) -> i32;
    fn prcmu_gic_recouple();
    fn db8500_prcmu_enable_wakeups(reasons: u32);
    fn cpuidle_register(driver: *mut cpuidle_driver, device: *mut c_void) -> i32;
    fn spin_trylock(lock: *mut spinlock_t) -> bool;
    fn spin_unlock(lock: *mut spinlock_t);
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
}

const PRCMU_AP_IDLE: i32 = 0;
const PRCMU_WAKEUP_ARM: u32 = 1;
const PRCMU_WAKEUP_RTC: u32 = 2;
const PRCMU_WAKEUP_ABB: u32 = 4;

static MASTER: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(0);
#[repr(C)]
struct spinlock_t {
    _opaque: [u8; 0],
}

static mut MASTER_LOCK: spinlock_t = spinlock_t { _opaque: [] };

#[inline]
unsafe fn ux500_enter_idle(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    let this_cpu = smp_processor_id();
    let mut recouple = false;

    if MASTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst) + 1 == num_online_cpus() {
        // With this lock, prevent the other cpu from exiting and entering this
        // function again and becoming the master.
        if !spin_trylock(core::ptr::addr_of_mut!(MASTER_LOCK)) {
            cpu_do_idle();
            MASTER.fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
            return index;
        }

        // Decouple the gic from the A9 cores.
        if prcmu_gic_decouple() != 0 {
            spin_unlock(core::ptr::addr_of_mut!(MASTER_LOCK));
            MASTER.fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
            return index;
        }

        // If an error occurs, manually recouple the gic.
        recouple = true;

        // As the gic is decoupled, safely go to retention if the other cpu is
        // in WFI.
        if !prcmu_is_cpu_in_wfi(if this_cpu != 0 { 0 } else { 1 }) {
            spin_unlock(core::ptr::addr_of_mut!(MASTER_LOCK));
            MASTER.fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
            return index;
        }

        // The prcmu watches interrupts and wakes the cpus.
        if prcmu_copy_gic_settings() != 0 {
            spin_unlock(core::ptr::addr_of_mut!(MASTER_LOCK));
            MASTER.fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
            return index;
        }
        if prcmu_gic_pending_irq() != 0 {
            spin_unlock(core::ptr::addr_of_mut!(MASTER_LOCK));
            MASTER.fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
            return index;
        }
        if prcmu_pending_irq() != 0 {
            spin_unlock(core::ptr::addr_of_mut!(MASTER_LOCK));
            MASTER.fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
            return index;
        }
        if db8500_prcmu_set_power_state(PRCMU_AP_IDLE, true, true) != 0 {
            spin_unlock(core::ptr::addr_of_mut!(MASTER_LOCK));
            MASTER.fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
            return index;
        }

        // The prcmu recouples the gic automatically on retention.
        recouple = false;
        spin_unlock(core::ptr::addr_of_mut!(MASTER_LOCK));
    }

    cpu_do_idle();
    MASTER.fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
    if recouple {
        prcmu_gic_recouple();
        spin_unlock(core::ptr::addr_of_mut!(MASTER_LOCK));
    }
    index
}

#[no_mangle]
pub unsafe extern "C" fn dbx500_cpuidle_probe(_pdev: *mut platform_device) -> i32 {
    // Configure wake up reasons.
    db8500_prcmu_enable_wakeups(PRCMU_WAKEUP_ARM | PRCMU_WAKEUP_RTC | PRCMU_WAKEUP_ABB);
    cpuidle_register(core::ptr::addr_of_mut!(UX500_IDLE_DRIVER), core::ptr::null_mut())
}

#[repr(C)]
struct cpuidle_state {
    enter: Option<unsafe fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32>,
    exit_latency: u32,
    target_residency: u32,
    flags: u32,
    name: *const u8,
    desc: *const u8,
}

static mut UX500_IDLE_DRIVER: cpuidle_driver = cpuidle_driver { _opaque: [] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
