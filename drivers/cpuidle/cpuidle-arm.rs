// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARM/ARM64 generic CPU idle driver.
 *
 * Copyright (C) 2014 ARM Ltd.
 * Author: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
 */

// pr_fmt(fmt) = "CPUidle arm: " fmt
// Dependencies supplied by the Linux kernel and other translation units are
// intentionally referenced but not reimplemented here.

/*
 * arm_enter_idle_state - Programs CPU to enter the specified state
 *
 * dev: cpuidle device
 * drv: cpuidle driver
 * idx: state index
 *
 * Called from the CPUidle framework to program the device to the
 * specified target state selected by the governor.
 */
unsafe extern "C" fn arm_enter_idle_state(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    idx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    /*
     * Pass idle state index to arm_cpuidle_suspend which in turn
     * will call the CPU ops suspend protocol with idle index as a
     * parameter.
     */
    CPU_PM_CPU_IDLE_ENTER(arm_cpuidle_suspend, idx)
}

static mut arm_idle_driver: cpuidle_driver = cpuidle_driver {
    name: "arm_idle\0".as_ptr() as *const ::core::ffi::c_char,
    owner: THIS_MODULE,
    states: {
        let mut states = [cpuidle_state::ZERO; CPUIDLE_STATE_MAX];
        states[0] = cpuidle_state {
            enter: Some(arm_enter_idle_state),
            exit_latency: 1,
            target_residency: 1,
            power_usage: UINT_MAX,
            name: "WFI\0".as_ptr() as *const ::core::ffi::c_char,
            desc: "ARM WFI\0".as_ptr() as *const ::core::ffi::c_char,
            ..cpuidle_state::ZERO
        };
        states
    },
    ..cpuidle_driver::ZERO
};

static arm_idle_state_match: [of_device_id; 2] = [
    of_device_id {
        compatible: "arm,idle-state\0".as_ptr() as *const ::core::ffi::c_char,
        data: arm_enter_idle_state as *const (),
        ..of_device_id::ZERO
    },
    of_device_id::ZERO,
];

/*
 * arm_idle_init_cpu
 *
 * Registers the arm specific cpuidle driver with the cpuidle
 * framework. It relies on core code to parse the idle states
 * and initialize them using driver data structures accordingly.
 */
unsafe extern "C" fn arm_idle_init_cpu(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut drv: *mut cpuidle_driver;
    let mut ret: ::core::ffi::c_int;

    drv = kmemdup(
        &arm_idle_driver as *const cpuidle_driver as *const (),
        core::mem::size_of::<cpuidle_driver>(),
        GFP_KERNEL,
    ) as *mut cpuidle_driver;
    if drv.is_null() {
        return -ENOMEM;
    }

    (*drv).cpumask = cpumask_of(cpu) as *mut cpumask;

    /* Initialize idle states data, starting at index 1. */
    ret = dt_init_idle_driver(drv, arm_idle_state_match.as_ptr(), 1);
    if ret <= 0 {
        ret = if ret != 0 { ret } else { -ENODEV };
        kfree(drv as *mut ());
        return ret;
    }

    /* Initialize idle states suspend back-end specific data. */
    ret = arm_cpuidle_init(cpu);
    if ret != 0 {
        if ret != -EOPNOTSUPP {
            pr_err("CPUidle arm: CPU %d failed to init idle CPU ops\n", cpu);
        }
        ret = if ret == -ENXIO { 0 } else { ret };
        kfree(drv as *mut ());
        return ret;
    }

    ret = cpuidle_register(drv, core::ptr::null_mut());
    if ret != 0 {
        kfree(drv as *mut ());
        return ret;
    }

    cpuidle_cooling_register(drv);
    0
}

/*
 * arm_idle_init - Initializes arm cpuidle driver
 *
 * Initializes arm cpuidle driver for all present CPUs, if any
 * CPU fails to register cpuidle driver then rollback to cancel
 * all CPUs registration.
 */
unsafe extern "C" fn arm_idle_init() -> ::core::ffi::c_int {
    let mut cpu: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int;
    let mut drv: *mut cpuidle_driver;
    let mut dev: *mut cpuidle_device;

    for_each_present_cpu!(cpu, {
        ret = arm_idle_init_cpu(cpu);
        if ret != 0 {
            while cpu > 0 {
                cpu -= 1;
                dev = *per_cpu(cpuidle_devices, cpu);
                drv = cpuidle_get_cpu_driver(dev);
                cpuidle_unregister(drv);
                kfree(drv as *mut ());
            }
            return ret;
        }
    });

    0
}

device_initcall!(arm_idle_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
