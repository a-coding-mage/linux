// SPDX-License-Identifier: GPL-2.0-only
/*
 * s390 generic CPU idle driver.
 *
 * Copyright IBM Corp. 2026
 */

// C dependency: pr_fmt(fmt) "CPUidle s390: " fmt
// C dependencies supplied by the kernel headers are intentionally external.

use core::ffi::c_void;

extern "C" {
    static mut cpu_present_mask: c_void;
    fn arch_cpu_idle();
    fn cpuidle_pause_and_lock();
    fn cpuidle_resume_and_unlock();
    fn cpuidle_enable_device(dev: *mut cpuidle_device) -> i32;
    fn cpuidle_disable_device(dev: *mut cpuidle_device);
    fn cpuidle_register_device(dev: *mut cpuidle_device) -> i32;
    fn cpuidle_poll_state_init(drv: *mut cpuidle_driver);
    fn cpuidle_register(drv: *mut cpuidle_driver, cpumask: *mut c_void) -> i32;
    fn cpuidle_unregister(drv: *mut cpuidle_driver);
    fn cpuhp_setup_state_nocalls(
        state: i32,
        name: *const u8,
        online: Option<unsafe extern "C" fn(u32) -> i32>,
        dead: Option<unsafe extern "C" fn(u32) -> i32>,
    ) -> i32;
    fn machine_is_lpar() -> bool;
    fn pr_err(fmt: *const u8, ...);
    fn per_cpu_cpuidle_dev(cpu: u32) -> *mut cpuidle_device;
}

#[repr(C)]
pub struct cpuidle_device {
    pub registered: bool,
    pub cpu: u32,
}

#[repr(C)]
pub struct cpuidle_state {
    pub enter: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32>,
    pub name: *const u8,
    pub desc: *const u8,
    pub target_residency: u64,
    pub exit_latency: u64,
}

#[repr(C)]
pub struct cpuidle_driver {
    pub cpumask: *mut c_void,
    pub name: *const u8,
    pub states: [cpuidle_state; 2],
    pub safe_state_index: u32,
    pub state_count: u32,
}

unsafe extern "C" fn s390_enter_idle(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    arch_cpu_idle();
    index
}

static mut s390_cpuidle_driver: cpuidle_driver = cpuidle_driver {
    cpumask: core::ptr::null_mut(),
    name: b"s390-idle\0".as_ptr(),
    states: [
        cpuidle_state {
            enter: None,
            name: core::ptr::null(),
            desc: core::ptr::null(),
            target_residency: 0,
            exit_latency: 0,
        },
        cpuidle_state {
            enter: Some(s390_enter_idle),
            name: b"IDLE\0".as_ptr(),
            desc: b"ENABLED WAIT\0".as_ptr(),
            target_residency: 0,
            exit_latency: 0,
        },
    ],
    safe_state_index: 0,
    state_count: 2,
};

unsafe extern "C" fn s390_cpuidle_cpu_online(cpu: u32) -> i32 {
    let dev = per_cpu_cpuidle_dev(cpu);
    let rc: i32;

    if (*dev).registered {
        cpuidle_pause_and_lock();
        rc = cpuidle_enable_device(dev);
        cpuidle_resume_and_unlock();
        if rc != 0 {
            pr_err(b"Failed to enable cpuidle device on cpu %u\n\0".as_ptr(), cpu);
        }
    } else {
        (*dev).cpu = cpu;
        rc = cpuidle_register_device(dev);
        if rc != 0 {
            pr_err(b"Failed to register cpuidle driver on cpu %u\n\0".as_ptr(), cpu);
        }
    }
    rc
}

unsafe extern "C" fn s390_cpuidle_cpu_dead(cpu: u32) -> i32 {
    let dev = per_cpu_cpuidle_dev(cpu);

    if !(*dev).registered {
        return 0;
    }
    cpuidle_pause_and_lock();
    cpuidle_disable_device(dev);
    cpuidle_resume_and_unlock();
    0
}

/*
 * The target_residency and exit_latency values are benchmark-derived estimates
 * that remain non-deterministic due to s390's virtualized architecture.
 *
 * Configuration strategy:
 * - Poll idle state: Values derived from the next enabled idle state (EW)
 * - Enabled Wait state: Values selected based on idle behavior and empirical
 *   measurement data
 *
 * Goal is to improve responsiveness for workloads with frequent sleep/wakeup
 * cycles while minimizing any side effects.
 */
unsafe fn s390_cpuidle_ew_tune() {
    let state = &mut s390_cpuidle_driver.states[1];

    if machine_is_lpar() {
        state.target_residency = 5;
        state.exit_latency = 5;
    } else {
        state.target_residency = 1;
        state.exit_latency = 1;
    }
}

unsafe extern "C" fn s390_cpuidle_init() -> i32 {
    let rc: i32;

    s390_cpuidle_ew_tune();
    cpuidle_poll_state_init(&mut s390_cpuidle_driver);
    rc = cpuidle_register(&mut s390_cpuidle_driver, core::ptr::null_mut());
    if rc != 0 {
        return rc;
    }
    rc = cpuhp_setup_state_nocalls(
        0, // CPUHP_AP_ONLINE_DYN
        b"cpuidle/s390:online\0".as_ptr(),
        Some(s390_cpuidle_cpu_online),
        Some(s390_cpuidle_cpu_dead),
    );
    if rc < 0 {
        cpuidle_unregister(&mut s390_cpuidle_driver);
        pr_err(b"Failed to allocate hotplug state: cpuidle/s390:online\n\0".as_ptr());
        return rc;
    }
    0
}

// C: device_initcall(s390_cpuidle_init)
#[used]
static S390_CPUIDLE_INITCALL: unsafe extern "C" fn() -> i32 = s390_cpuidle_init;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
