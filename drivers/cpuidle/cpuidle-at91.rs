// SPDX-License-Identifier: GPL-2.0-only
/*
 * based on arch/arm/mach-kirkwood/cpuidle.c
 *
 * CPU idle support for AT91 SoC
 *
 * The cpu idle uses wait-for-interrupt and RAM self refresh in order
 * to implement two idle states -
 * #1 wait-for-interrupt
 * #2 wait-for-interrupt and RAM self refresh
 */

// Dependencies supplied by the kernel headers and other translation units.

const AT91_MAX_STATES: usize = 2;

static mut at91_standby: Option<unsafe extern "C" fn()> = None;

/* Actual code that puts the SoC in different idle states */
unsafe extern "C" fn at91_enter_idle(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    if let Some(standby) = at91_standby {
        standby();
    }
    index
}

static mut at91_idle_driver: cpuidle_driver = cpuidle_driver {
    name: "at91_idle\0".as_ptr() as *const i8,
    owner: THIS_MODULE,
    states: [
        ARM_CPUIDLE_WFI_STATE,
        cpuidle_state {
            enter: Some(at91_enter_idle),
            exit_latency: 10,
            target_residency: 10000,
            name: "RAM_SR\0".as_ptr() as *const i8,
            desc: "WFI and DDR Self Refresh\0".as_ptr() as *const i8,
        },
    ],
    state_count: AT91_MAX_STATES as u32,
};

/* Initialize CPU idle by registering the idle states */
unsafe extern "C" fn at91_cpuidle_probe(dev: *mut platform_device) -> i32 {
    at91_standby = Some(core::mem::transmute((*dev).dev.platform_data));

    cpuidle_register(&mut at91_idle_driver, core::ptr::null_mut())
}

static mut at91_cpuidle_driver: platform_driver = platform_driver {
    driver: driver {
        name: "cpuidle-at91\0".as_ptr() as *const i8,
    },
    probe: Some(at91_cpuidle_probe),
};

// builtin_platform_driver(at91_cpuidle_driver);
// The platform-driver registration is performed by the kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
