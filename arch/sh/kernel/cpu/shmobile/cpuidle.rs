// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/shmobile/cpuidle.c
 *
 * Cpuidle support code for SuperH Mobile
 *
 *  Copyright (C) 2009 Magnus Damm
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/kernel.h, linux/io.h, linux/suspend.h,
// linux/cpuidle.h, linux/export.h, asm/suspend.h, linux/uaccess.h

static mut cpuidle_mode: [c_ulong; 3] = [
    SUSP_SH_SLEEP,                         // regular sleep mode
    SUSP_SH_SLEEP | SUSP_SH_SF,            // sleep mode + self refresh
    SUSP_SH_STANDBY | SUSP_SH_SF,          // software standby mode + self refresh
];

unsafe fn cpuidle_sleep_enter(
    dev: *mut cpuidle_device,
    drv: *mut cpuidle_driver,
    index: c_int,
) -> c_int {
    let allowed_mode: c_ulong = SUSP_SH_SLEEP;
    let requested_state: c_int = index;
    let allowed_state: c_int;
    let mut k: c_int;

    // convert allowed mode to allowed state
    k = (core::mem::size_of_val(&cpuidle_mode) / core::mem::size_of::<c_ulong>()) as c_int - 1;
    while k > 0 {
        if cpuidle_mode[k as usize] == allowed_mode {
            break;
        }
        k -= 1;
    }

    allowed_state = k;

    /* take the following into account for sleep mode selection:
     * - allowed_state: best mode allowed by hardware (clock deps)
     * - requested_state: best mode allowed by software (latencies)
     */
    k = if allowed_state < requested_state {
        allowed_state
    } else {
        requested_state
    };

    sh_mobile_call_standby(cpuidle_mode[k as usize]);

    k
}

static mut cpuidle_driver: cpuidle_driver = cpuidle_driver {
    name: "sh_idle",
    owner: THIS_MODULE,
    states: [
        cpuidle_state {
            exit_latency: 1,
            target_residency: 1 * 2,
            power_usage: 3,
            enter: Some(cpuidle_sleep_enter),
            name: "C1",
            desc: "SuperH Sleep Mode",
            ..cpuidle_state::ZERO
        },
        cpuidle_state {
            exit_latency: 100,
            target_residency: 1 * 2,
            power_usage: 1,
            enter: Some(cpuidle_sleep_enter),
            name: "C2",
            desc: "SuperH Sleep Mode [SF]",
            flags: CPUIDLE_FLAG_UNUSABLE,
            ..cpuidle_state::ZERO
        },
        cpuidle_state {
            exit_latency: 2300,
            target_residency: 1 * 2,
            power_usage: 1,
            enter: Some(cpuidle_sleep_enter),
            name: "C3",
            desc: "SuperH Mobile Standby Mode [SF]",
            flags: CPUIDLE_FLAG_UNUSABLE,
            ..cpuidle_state::ZERO
        },
    ],
    safe_state_index: 0,
    state_count: 3,
    ..cpuidle_driver::ZERO
};

unsafe fn sh_mobile_setup_cpuidle() -> c_int {
    if sh_mobile_sleep_supported & SUSP_SH_SF != 0 {
        cpuidle_driver.states[1].flags = CPUIDLE_FLAG_NONE;
    }

    if sh_mobile_sleep_supported & SUSP_SH_STANDBY != 0 {
        cpuidle_driver.states[2].flags = CPUIDLE_FLAG_NONE;
    }

    cpuidle_register(&mut cpuidle_driver, core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
