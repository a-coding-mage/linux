// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 Freescale Semiconductor, Inc.
 * Copyright 2011 Linaro Ltd.
 */

// External declarations supplied by the Linux kernel and platform headers.
use core::ffi::c_ulong;

extern "C" {
    static mut jiffies: c_ulong;
    static louis: u32;

    fn v7_exit_coherency_flush(louis: u32);
    fn imx_set_cpu_arg(cpu: u32, arg: c_ulong);
    fn cpu_do_idle();
    fn msecs_to_jiffies(milliseconds: c_ulong) -> c_ulong;
    fn imx_get_cpu_arg(cpu: u32) -> c_ulong;
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
    fn imx_enable_cpu(cpu: u32, enable: bool);
    fn cpu_is_imx7d() -> bool;
    fn imx_gpcv2_set_core1_pdn_pup_by_software(enable: bool);
}

/*
 * platform-specific code to shutdown a CPU
 *
 * Called with IRQs disabled
 */
pub unsafe fn imx_cpu_die(cpu: u32) {
    v7_exit_coherency_flush(louis);
    /*
     * We use the cpu jumping argument register to sync with
     * imx_cpu_kill() which is running on cpu0 and waiting for
     * the register being cleared to kill the cpu.
     */
    imx_set_cpu_arg(cpu, !0);

    loop {
        cpu_do_idle();
    }
}

pub unsafe fn imx_cpu_kill(cpu: u32) -> i32 {
    let timeout: c_ulong = jiffies.wrapping_add(msecs_to_jiffies(50));

    while imx_get_cpu_arg(cpu) == 0 {
        if time_after(jiffies, timeout) {
            return 0;
        }
    }
    imx_enable_cpu(cpu, false);
    imx_set_cpu_arg(cpu, 0);
    if cpu_is_imx7d() {
        imx_gpcv2_set_core1_pdn_pup_by_software(true);
    }
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
