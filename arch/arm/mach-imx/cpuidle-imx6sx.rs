// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the corresponding kernel headers and local headers:
// linux/cpuidle.h, linux/cpu_pm.h, linux/module.h, asm/cacheflush.h,
// asm/cpuidle.h, asm/suspend.h, common.h, cpuidle.h, hardware.h

unsafe fn imx6sx_idle_finish(val: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let _ = val;
    /*
     * for Cortex-A7 which has an internal L2
     * cache, need to flush it before powering
     * down ARM platform, since flushing L1 cache
     * here again has very small overhead, compared
     * to adding conditional code for L2 cache type,
     * just call flush_cache_all() is fine.
     */
    unsafe {
        flush_cache_all();
        cpu_do_idle();
    }

    0
}

unsafe extern "C" fn imx6sx_enter_wait(
    dev: *mut cpuidle_device,
    drv: *mut cpuidle_driver,
    index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let _ = (dev, drv);
    unsafe {
        imx6_set_lpm(WAIT_UNCLOCKED);

        match index {
            1 => {
                cpu_do_idle();
            }
            2 => {
                imx6_enable_rbc(true);
                imx_gpc_set_arm_power_in_lpm(true);
                imx_set_cpu_jump(0, v7_cpu_resume);
                /* Need to notify there is a cpu pm operation. */
                cpu_pm_enter();
                cpu_cluster_pm_enter();

                ct_cpuidle_enter();
                cpu_suspend(0, Some(imx6sx_idle_finish));
                ct_cpuidle_exit();

                cpu_cluster_pm_exit();
                cpu_pm_exit();
                imx_gpc_set_arm_power_in_lpm(false);
                imx6_enable_rbc(false);
            }
            _ => {}
        }

        imx6_set_lpm(WAIT_CLOCKED);
    }

    index
}

static mut imx6sx_cpuidle_driver: cpuidle_driver = cpuidle_driver {
    name: "imx6sx_cpuidle",
    owner: THIS_MODULE,
    states: [
        /* WFI */
        ARM_CPUIDLE_WFI_STATE,
        /* WAIT */
        cpuidle_state {
            exit_latency: 50,
            target_residency: 75,
            flags: CPUIDLE_FLAG_TIMER_STOP,
            enter: Some(imx6sx_enter_wait),
            name: "WAIT",
            desc: "Clock off",
        },
        /* WAIT + ARM power off  */
        cpuidle_state {
            /*
             * ARM gating 31us * 5 + RBC clear 65us
             * and some margin for SW execution, here set it
             * to 300us.
             */
            exit_latency: 300,
            target_residency: 500,
            flags: CPUIDLE_FLAG_TIMER_STOP | CPUIDLE_FLAG_RCU_IDLE,
            enter: Some(imx6sx_enter_wait),
            name: "LOW-POWER-IDLE",
            desc: "ARM power off",
        },
    ],
    state_count: 3,
    safe_state_index: 0,
};

unsafe fn imx6sx_cpuidle_init() -> ::core::ffi::c_int {
    unsafe {
        imx6_set_int_mem_clk_lpm(true);
        imx6_enable_rbc(false);
        imx_gpc_set_l2_mem_power_in_lpm(false);
        /*
         * set ARM power up/down timing to the fastest,
         * sw2iso and sw can be set to one 32K cycle = 31us
         * except for power up sw2iso which need to be
         * larger than LDO ramp up time.
         */
        imx_gpc_set_arm_power_up_timing(if cpu_is_imx6sx() { 0xf } else { 0x2 }, 1);
        imx_gpc_set_arm_power_down_timing(1, 1);

        cpuidle_register(&raw mut imx6sx_cpuidle_driver, ::core::ptr::null_mut())
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
