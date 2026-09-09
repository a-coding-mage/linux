// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/context_tracking.h, linux/cpuidle.h, linux/module.h,
// asm/cpuidle.h, soc/imx/cpuidle.h, common.h, cpuidle.h, and hardware.h.

static mut num_idle_cpus: ::core::ffi::c_int = 0;

// Equivalent to DEFINE_RAW_SPINLOCK(cpuidle_lock).
extern "C" {
    static mut cpuidle_lock: raw_spinlock_t;
}

extern "C" {
    fn raw_spin_lock(lock: *mut raw_spinlock_t);
    fn raw_spin_unlock(lock: *mut raw_spinlock_t);
    fn num_online_cpus() -> ::core::ffi::c_int;
    fn imx6_set_lpm(mode: ::core::ffi::c_int);
    fn ct_cpuidle_enter();
    fn cpu_do_idle();
    fn ct_cpuidle_exit();
    fn cpuidle_driver_state_disabled(
        drv: *mut cpuidle_driver,
        index: ::core::ffi::c_uint,
        disabled: bool,
    );
    fn imx6_set_int_mem_clk_lpm(enable: bool);
    fn cpuidle_register(
        drv: *mut cpuidle_driver,
        gov: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

unsafe fn imx6q_enter_wait(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    raw_spin_lock(&raw mut cpuidle_lock);
    num_idle_cpus = num_idle_cpus.wrapping_add(1);
    if num_idle_cpus == num_online_cpus() {
        imx6_set_lpm(WAIT_UNCLOCKED);
    }
    raw_spin_unlock(&raw mut cpuidle_lock);

    ct_cpuidle_enter();
    cpu_do_idle();
    ct_cpuidle_exit();

    raw_spin_lock(&raw mut cpuidle_lock);
    if num_idle_cpus == num_online_cpus() {
        num_idle_cpus = num_idle_cpus.wrapping_sub(1);
        imx6_set_lpm(WAIT_CLOCKED);
    } else {
        num_idle_cpus = num_idle_cpus.wrapping_sub(1);
    }
    raw_spin_unlock(&raw mut cpuidle_lock);

    index
}

static mut imx6q_cpuidle_driver: cpuidle_driver = cpuidle_driver {
    name: "imx6q_cpuidle" as *const str,
    owner: THIS_MODULE,
    states: [
        // WFI: ARM_CPUIDLE_WFI_STATE
        cpuidle_state {
            exit_latency: 1,
            target_residency: 1,
            flags: 0,
            enter: ARM_CPUIDLE_WFI_STATE_ENTER,
            name: "WFI",
            desc: "WFI",
        },
        // WAIT
        cpuidle_state {
            exit_latency: 50,
            target_residency: 75,
            flags: CPUIDLE_FLAG_TIMER_STOP | CPUIDLE_FLAG_RCU_IDLE,
            enter: Some(imx6q_enter_wait),
            name: "WAIT",
            desc: "Clock off",
        },
    ],
    state_count: 2,
    safe_state_index: 0,
};

/*
 * i.MX6 Q/DL has an erratum (ERR006687) that prevents the FEC from waking the
 * CPUs when they are in wait(unclocked) state. As the hardware workaround isn't
 * applicable to all boards, disable the deeper idle state when the workaround
 * isn't present and the FEC is in use.
 */
#[no_mangle]
pub unsafe extern "C" fn imx6q_cpuidle_fec_irqs_used() {
    cpuidle_driver_state_disabled(&raw mut imx6q_cpuidle_driver, 1, true);
}

// EXPORT_SYMBOL_GPL(imx6q_cpuidle_fec_irqs_used);

#[no_mangle]
pub unsafe extern "C" fn imx6q_cpuidle_fec_irqs_unused() {
    cpuidle_driver_state_disabled(&raw mut imx6q_cpuidle_driver, 1, false);
}

// EXPORT_SYMBOL_GPL(imx6q_cpuidle_fec_irqs_unused);

pub unsafe extern "C" fn imx6q_cpuidle_init() -> ::core::ffi::c_int {
    // Set INT_MEM_CLK_LPM bit to get a reliable WAIT mode support
    imx6_set_int_mem_clk_lpm(true);

    cpuidle_register(&raw mut imx6q_cpuidle_driver, ::core::ptr::null())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
