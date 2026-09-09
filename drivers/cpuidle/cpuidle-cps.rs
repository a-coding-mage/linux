// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2014 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone)]
enum CpsIdleState {
    StateWait = 0,
    StateNcWait,
    StateClockGated,
    StatePowerGated,
    StateCount,
}

unsafe fn cps_nc_enter(
    dev: *mut CpuidleDevice,
    _drv: *mut CpuidleDriver,
    mut index: i32,
) -> i32 {
    let mut pm_state: CpsPmState;
    let err: i32;

    /* At least one core must remain powered up & clocked. */
    if unsafe { cpus_are_siblings(0, (*dev).cpu) } && index > CpsIdleState::StateNcWait as i32 {
        index = CpsIdleState::StateNcWait as i32;
    }

    pm_state = match index {
        x if x == CpsIdleState::StateNcWait as i32 => CpsPmState::CpsPmNcWait,
        x if x == CpsIdleState::StateClockGated as i32 => CpsPmState::CpsPmClockGated,
        x if x == CpsIdleState::StatePowerGated as i32 => CpsPmState::CpsPmPowerGated,
        _ => {
            unsafe { bug() };
            return -22; // -EINVAL
        }
    };

    if pm_state == CpsPmState::CpsPmPowerGated && unsafe { cpu_pm_enter() } != 0 {
        return -4; // -EINTR
    }

    err = unsafe { cps_pm_enter_state(pm_state) };

    if pm_state == CpsPmState::CpsPmPowerGated {
        unsafe { cpu_pm_exit() };
    }

    if err != 0 { err } else { index }
}

// The following declarations correspond to kernel-provided types and symbols.
// Their definitions are supplied by the surrounding translation unit.
#[allow(non_camel_case_types)]
type CpsPmState = kernel_CpsPmState;
extern "C" {
    static mut cps_driver: CpuidleDriver;
    static mut cpuidle_dev: CpuidleDevice;
    static coupled_coherence: bool;
    fn cpus_are_siblings(cpu: i32, sibling: i32) -> bool;
    fn cpu_pm_enter() -> i32;
    fn cpu_pm_exit();
    fn cps_pm_enter_state(state: CpsPmState) -> i32;
    fn cps_pm_support_state(state: CpsPmState) -> bool;
    fn cpuidle_register_driver(driver: *mut CpuidleDriver) -> i32;
    fn cpuidle_unregister_driver(driver: *mut CpuidleDriver);
    fn cpuidle_register_device(device: *mut CpuidleDevice) -> i32;
    fn cpuidle_unregister_device(device: *mut CpuidleDevice);
    fn bug();
}

#[repr(C)]
struct CpuidleDevice { cpu: i32 }
#[repr(C)]
struct CpuidleDriver {
    name: *const u8,
    owner: *const core::ffi::c_void,
    states: [CpuidleState; 4],
    state_count: i32,
    safe_state_index: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct CpuidleState {
    enter: Option<unsafe fn(*mut CpuidleDevice, *mut CpuidleDriver, i32) -> i32>,
    exit_latency: i32,
    target_residency: i32,
    flags: u32,
    name: *const u8,
    desc: *const u8,
}

unsafe fn cps_cpuidle_unregister() {
    let mut cpu: i32;
    let mut device: *mut CpuidleDevice;
    for_each_possible_cpu!(cpu) {
        device = &mut cpuidle_dev;
        cpuidle_unregister_device(device);
    }
    cpuidle_unregister_driver(&mut cps_driver);
}

unsafe fn cps_cpuidle_init() -> i32 {
    let mut err: i32;
    let mut cpu: i32;
    let mut i: i32;
    let mut device: *mut CpuidleDevice;

    if !cps_pm_support_state(CpsPmState::CpsPmPowerGated) { cps_driver.state_count = CpsIdleState::StateClockGated as i32 + 1; }
    if !cps_pm_support_state(CpsPmState::CpsPmClockGated) { cps_driver.state_count = CpsIdleState::StateNcWait as i32 + 1; }
    if !cps_pm_support_state(CpsPmState::CpsPmNcWait) { cps_driver.state_count = CpsIdleState::StateWait as i32 + 1; }

    if cps_driver.state_count < CpsIdleState::StateCount as i32 {
        match cps_driver.state_count - 1 {
            0 => pr_cont!("coherent wait\n"),
            1 => pr_cont!("non-coherent wait\n"),
            2 => pr_cont!("clock gating\n"),
            _ => {}
        }
    }

    if coupled_coherence {
        i = CpsIdleState::StateNcWait as i32;
        while i < cps_driver.state_count { cps_driver.states[i as usize].flags |= CPUIDLE_FLAG_COUPLED; i += 1; }
    }

    err = cpuidle_register_driver(&mut cps_driver);
    if err != 0 { return err; }
    for_each_possible_cpu!(cpu) {
        device = &mut cpuidle_dev;
        (*device).cpu = cpu;
        err = cpuidle_register_device(device);
        if err != 0 { cps_cpuidle_unregister(); return err; }
    }
    0
}

// Build-time kernel registration: device_initcall(cps_cpuidle_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
