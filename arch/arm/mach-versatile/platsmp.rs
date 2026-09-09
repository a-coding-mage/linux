// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2002 ARM Ltd.
 *  All Rights Reserved
 *
 * This code is specific to the hardware found on ARM Realview and
 * Versatile Express platforms where the CPUs are unable to be individually
 * woken, and where there is no way to hot-unplug CPUs.  Real platforms
 * should not copy this code.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_int, c_ulong};

extern "C" {
    fn smp_wmb();
    fn sync_cache_w(addr: *const c_int);
    fn raw_spin_lock(lock: *mut raw_spinlock_t);
    fn raw_spin_unlock(lock: *mut raw_spinlock_t);
    fn arch_send_wakeup_ipi_mask(mask: *const cpumask_t);
    fn cpumask_of(cpu: c_uint) -> *const cpumask_t;
    fn cpu_logical_map(cpu: c_uint) -> c_uint;
    fn udelay(usecs: c_ulong);
    fn time_before(a: c_ulong, b: c_ulong) -> bool;
}

type c_uint = u32;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct raw_spinlock_t {
    _private: [u8; 0],
}

extern "C" {
    static mut jiffies: c_ulong;
    static HZ: c_ulong;
}

/*
 * versatile_cpu_release controls the release of CPUs from the holding
 * pen in headsmp.S, which exists because we are not always able to
 * control the release of individual CPUs from the board firmware.
 * Production platforms do not need this.
 */
#[no_mangle]
pub static mut versatile_cpu_release: c_int = -1;

/*
 * Write versatile_cpu_release in a way that is guaranteed to be visible to
 * all observers, irrespective of whether they're taking part in coherency
 * or not.  This is necessary for the hotplug code to work reliably.
 */
unsafe fn versatile_write_cpu_release(val: c_int) {
    core::ptr::write_volatile(&raw mut versatile_cpu_release, val);
    smp_wmb();
    sync_cache_w(&raw const versatile_cpu_release);
}

/*
 * versatile_lock exists to avoid running the loops_per_jiffy delay loop
 * calibrations on the secondary CPU while the requesting CPU is using
 * the limited-bandwidth bus - which affects the calibration value.
 * Production platforms do not need this.
 */
static mut versatile_lock: raw_spinlock_t = raw_spinlock_t { _private: [] };

#[no_mangle]
pub unsafe fn versatile_secondary_init(cpu: c_uint) {
    /*
     * let the primary processor know we're out of the
     * pen, then head off into the C entry point
     */
    versatile_write_cpu_release(-1);

    /*
     * Synchronise with the boot thread.
     */
    raw_spin_lock(&raw mut versatile_lock);
    raw_spin_unlock(&raw mut versatile_lock);
}

#[no_mangle]
pub unsafe fn versatile_boot_secondary(cpu: c_uint, idle: *mut task_struct) -> c_int {
    let mut timeout: c_ulong;

    /*
     * Set synchronisation state between this boot processor
     * and the secondary one
     */
    raw_spin_lock(&raw mut versatile_lock);

    /*
     * This is really belt and braces; we hold unintended secondary
     * CPUs in the holding pen until we're ready for them.  However,
     * since we haven't sent them a soft interrupt, they shouldn't
     * be there.
     */
    versatile_write_cpu_release(cpu_logical_map(cpu) as c_int);

    /*
     * Send the secondary CPU a soft interrupt, thereby causing
     * the boot monitor to read the system wide flags register,
     * and branch to the address found there.
     */
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));

    timeout = jiffies.wrapping_add(1u64.wrapping_mul(HZ));
    while time_before(jiffies, timeout) {
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        if core::ptr::read_volatile(&raw const versatile_cpu_release) == -1 {
            break;
        }

        udelay(10);
    }

    /*
     * now the secondary core is starting up let it run its
     * calibrations, then wait for it to finish
     */
    raw_spin_unlock(&raw mut versatile_lock);

    if core::ptr::read_volatile(&raw const versatile_cpu_release) != -1 {
        -38
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
