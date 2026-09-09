// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 1996-2000 Russell King - Converted to ARM.
 *  Original Copyright (C) 1995  Linus Torvalds
 */

use core::ffi::c_char;

type PhysResetT = unsafe extern "C" fn(usize, bool);

/* Function pointers to optional machine specific functions */
#[no_mangle]
pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;
// EXPORT_SYMBOL(pm_power_off);

/*
 * A temporary stack to use for CPU reset. This is static so that we
 * don't clobber it with the identity mapping. When running with this
 * stack, any references to the current task *will not work* so you
 * should really do as little as possible before jumping to your reset
 * code.
 */
static mut soft_restart_stack: [u64; 16] = [0; 16];

extern "C" {
    fn setup_mm_for_reboot();
    fn flush_cache_all();
    fn cpu_proc_fin();
    fn virt_to_idmap(addr: unsafe extern "C" fn(usize, bool)) -> *mut core::ffi::c_void;
    fn cpu_reset(addr: usize, hyp_mode: bool);
    fn is_hyp_mode_available() -> bool;
    fn BUG() -> !;
    fn raw_local_irq_disable();
    fn local_fiq_disable();
    fn outer_disable();
    fn call_with_stack(
        func: unsafe extern "C" fn(*mut core::ffi::c_void),
        arg: *mut core::ffi::c_void,
        stack: *mut core::ffi::c_void,
    );
    fn num_online_cpus() -> i32;
    fn smp_shutdown_nonboot_cpus(cpu: i32);
    fn smp_send_stop();
    fn local_irq_disable();
    fn do_kernel_power_off();
    fn do_kernel_restart(cmd: *mut c_char);
    fn mdelay(ms: u32);
    fn printk(fmt: *const c_char, ...);
    static mut reboot_cpu: i32;
}

unsafe extern "C" fn __soft_restart(addr: *mut core::ffi::c_void) {
    let phys_reset: PhysResetT;

    /* Take out a flat memory mapping. */
    setup_mm_for_reboot();

    /* Clean and invalidate caches */
    flush_cache_all();

    /* Turn off caching */
    cpu_proc_fin();

    /* Push out any further dirty data, and ensure cache is empty */
    flush_cache_all();

    /* Switch to the identity mapping. */
    phys_reset = core::mem::transmute(virt_to_idmap(cpu_reset));

    /* original stub should be restored by kvm */
    phys_reset(addr as usize, is_hyp_mode_available());

    /* Should never get here. */
    BUG();
}

#[no_mangle]
pub unsafe extern "C" fn _soft_restart(addr: usize, disable_l2: bool) {
    let stack = soft_restart_stack.as_mut_ptr().add(soft_restart_stack.len());

    /* Disable interrupts first */
    raw_local_irq_disable();
    local_fiq_disable();

    /* Disable the L2 if we're the last man standing. */
    if disable_l2 {
        outer_disable();
    }

    /* Change to the new stack and continue with the reset. */
    call_with_stack(__soft_restart, addr as *mut core::ffi::c_void, stack as *mut core::ffi::c_void);

    /* Should never get here. */
    BUG();
}

#[no_mangle]
pub unsafe extern "C" fn soft_restart(addr: usize) {
    _soft_restart(addr, num_online_cpus() == 1);
}

/* Called by kexec, immediately prior to machine_kexec(). */
#[no_mangle]
pub unsafe extern "C" fn machine_shutdown() {
    smp_shutdown_nonboot_cpus(reboot_cpu);
}

/* Halting simply requires that the secondary CPUs stop performing any activity. */
#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    local_irq_disable();
    smp_send_stop();
    loop {}
}

/* Power-off simply requires that the secondary CPUs stop performing any activity. */
#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    local_irq_disable();
    smp_send_stop();
    do_kernel_power_off();
}

/* Restart requires that the secondary CPUs stop performing any activity. */
#[no_mangle]
pub unsafe extern "C" fn machine_restart(cmd: *mut c_char) {
    local_irq_disable();
    smp_send_stop();

    do_kernel_restart(cmd);

    /* Give a grace period for failure to restart of 1s */
    mdelay(1000);

    /* Whoops - the platform was unable to reboot. Tell the user! */
    printk(b"Reboot failed -- System halted\0".as_ptr() as *const c_char);
    loop {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
