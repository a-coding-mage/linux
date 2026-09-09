// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP4 SMP cpu-hotplug support
 *
 * Copyright (C) 2010 Texas Instruments, Inc.
 * Author:
 *      Santosh Shilimkar <santosh.shilimkar@ti.com>
 *
 * Platform file needed for the OMAP4 SMP. This file is based on arm
 * realview smp platform.
 * Copyright (c) 2002 ARM Limited.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/errno.h, linux/smp.h, linux/io.h,
// omap-wakeupgen.h, common.h, powerdomain.h

unsafe extern "C" {
    fn omap_get_wakeupgen_base() -> *mut core::ffi::c_void;
    fn omap_secure_apis_support() -> bool;
    fn omap_modify_auxcoreboot0(value: u32, mask: u32) -> u32;
    fn omap4_hotplug_cpu(cpu: u32, power_state: u32);
    fn omap_read_auxcoreboot0() -> u32;
    fn smp_processor_id() -> u32;
    fn writel_relaxed(value: u32, address: *mut core::ffi::c_void);
    fn readl_relaxed(address: *mut core::ffi::c_void) -> u32;
    fn pr_err(format: *const core::ffi::c_char, ...);
    fn pr_debug(format: *const core::ffi::c_char, ...);
}

// OMAP_AUX_CORE_BOOT_0 and PWRDM_POWER_OFF are supplied by the platform
// headers; their values are intentionally left as external build-time
// constants here.
unsafe extern "C" {
    static OMAP_AUX_CORE_BOOT_0: usize;
    static PWRDM_POWER_OFF: u32;
}

/*
 * platform-specific code to shutdown a CPU
 * Called with IRQs disabled
 */
pub unsafe fn omap4_cpu_die(cpu: u32) {
    let mut boot_cpu: u32 = 0;
    let base = omap_get_wakeupgen_base();

    /*
     * we're ready for shutdown now, so do it
     */
    if omap_secure_apis_support() {
        if omap_modify_auxcoreboot0(0x0, 0x200) != 0x0 {
            pr_err(b"Secure clear status failed\0".as_ptr() as *const core::ffi::c_char);
        }
    } else {
        writel_relaxed(0, base.add(OMAP_AUX_CORE_BOOT_0));
    }

    loop {
        /*
         * Enter into low power state
         */
        omap4_hotplug_cpu(cpu, PWRDM_POWER_OFF);

        if omap_secure_apis_support() {
            boot_cpu = omap_read_auxcoreboot0() >> 9;
        } else {
            boot_cpu = readl_relaxed(base.add(OMAP_AUX_CORE_BOOT_0)) >> 5;
        }

        if boot_cpu == smp_processor_id() {
            /*
             * OK, proper wakeup, we're done
             */
            break;
        }
        pr_debug(b"CPU%u: spurious wakeup call\n\0".as_ptr() as *const core::ffi::c_char, cpu);
    }
}

/* Needed by kexec and platform_can_cpu_hotplug() */
pub unsafe fn omap4_cpu_kill(_cpu: u32) -> i32 {
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
