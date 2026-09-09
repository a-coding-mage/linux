// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2002 ARM Ltd.
 *  All Rights Reserved
 *
 * This hotplug implementation is _specific_ to the situation found on
 * ARM development platforms where there is _no_ possibility of actually
 * taking a CPU offline, resetting it, or otherwise.  Real platforms must
 * NOT copy this code.
 */

use core::ffi::{c_char, c_int, c_uint};

// Supplied by the platform headers.
extern "C" {
    static mut versatile_cpu_release: c_uint;
    fn cpu_logical_map(cpu: c_uint) -> c_uint;
    fn pr_warn(format: *const c_char, ...);
}

// CR_C is supplied by asm/cp15.h.
extern "C" {
    static CR_C: c_uint;
}

#[inline]
unsafe fn versatile_immitation_enter_lowpower(actrl_mask: c_uint) {
    let mut v: c_uint;

    core::arch::asm!(
        "mcr p15, 0, {zero}, c7, c5, 0",
        "mcr p15, 0, {zero}, c7, c10, 4",
        // Turn off coherency
        "mrc p15, 0, {v}, c1, c0, 1",
        "bic {v}, {v}, {actrl}",
        "mcr p15, 0, {v}, c1, c0, 1",
        "mrc p15, 0, {v}, c1, c0, 0",
        "bic {v}, {v}, {cr_c}",
        "mcr p15, 0, {v}, c1, c0, 0",
        zero = in(reg) 0u32,
        v = out(reg) v,
        cr_c = in(reg) CR_C,
        actrl = in(reg) actrl_mask,
        options(nostack)
    );
}

#[inline]
unsafe fn versatile_immitation_leave_lowpower(actrl_mask: c_uint) {
    let mut v: c_uint;

    core::arch::asm!(
        "mrc p15, 0, {v}, c1, c0, 0",
        "orr {v}, {v}, {cr_c}",
        "mcr p15, 0, {v}, c1, c0, 0",
        "mrc p15, 0, {v}, c1, c0, 1",
        "orr {v}, {v}, {actrl}",
        "mcr p15, 0, {v}, c1, c0, 1",
        v = out(reg) v,
        cr_c = in(reg) CR_C,
        actrl = in(reg) actrl_mask,
        options(nostack)
    );
}

#[inline]
unsafe fn versatile_immitation_do_lowpower(cpu: c_uint, spurious: *mut c_int) {
    /*
     * there is no power-control hardware on this platform, so all
     * we can do is put the core into WFI; this is safe as the calling
     * code will have already disabled interrupts.
     *
     * This code should not be used outside Versatile platforms.
     */
    loop {
        core::arch::asm!("wfi", options(nomem, nostack));

        if versatile_cpu_release == cpu_logical_map(cpu) {
            /* OK, proper wakeup, we're done */
            break;
        }

        /*
         * Getting here, means that we have come out of WFI without
         * having been woken up - this shouldn't happen
         *
         * Just note it happening - when we're woken, we can report
         * its occurrence.
         */
        *spurious += 1;
    }
}

/*
 * platform-specific code to shutdown a CPU.
 * This code supports immitation-style CPU hotplug for Versatile/Realview/
 * Versatile Express platforms that are unable to do real CPU hotplug.
 */
#[no_mangle]
pub unsafe extern "C" fn versatile_immitation_cpu_die(cpu: c_uint, actrl_mask: c_uint) {
    let mut spurious: c_int = 0;

    versatile_immitation_enter_lowpower(actrl_mask);
    versatile_immitation_do_lowpower(cpu, &mut spurious);
    versatile_immitation_leave_lowpower(actrl_mask);

    if spurious != 0 {
        // The kernel's pr_warn format string is preserved as a C variadic call.
        pr_warn(b"CPU%u: %u spurious wakeup calls\0".as_ptr() as *const c_char,
            cpu, spurious as c_uint);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
