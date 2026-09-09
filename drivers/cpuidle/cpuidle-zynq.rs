// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2013 Xilinx
 *
 * CPU idle support for Xilinx Zynq
 *
 * based on arch/arm/mach-at91/cpuidle.c
 *
 * The cpu idle uses wait-for-interrupt and RAM self refresh in order
 * to implement two idle states -
 * #1 wait-for-interrupt
 * #2 wait-for-interrupt and RAM self refresh
 *
 * Maintainer: Michal Simek <michal.simek@amd.com>
 */

// The following names are supplied by the Linux kernel dependencies.
use core::ffi::c_void;

const ZYNQ_MAX_STATES: u32 = 2;

#[repr(C)]
pub struct cpuidle_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpuidle_state {
    pub enter: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut cpuidle_driver, i32) -> i32>,
    pub exit_latency: u32,
    pub target_residency: u32,
    pub name: *const u8,
    pub desc: *const u8,
}

#[repr(C)]
pub struct cpuidle_driver {
    pub name: *const u8,
    pub owner: *mut c_void,
    pub states: [cpuidle_state; ZYNQ_MAX_STATES as usize],
    pub safe_state_index: u32,
    pub state_count: u32,
}

extern "C" {
    fn cpu_do_idle();
    fn cpuidle_register(driver: *mut cpuidle_driver, device: *mut c_void) -> i32;
    static THIS_MODULE: c_void;
}

// Actual code that puts the SoC in different idle states
unsafe extern "C" fn zynq_enter_idle(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    // Add code for DDR self refresh start
    cpu_do_idle();

    index
}

// ARM_CPUIDLE_WFI_STATE is supplied by asm/cpuidle.h.
static mut zynq_idle_driver: cpuidle_driver = cpuidle_driver {
    name: b"zynq_idle\0".as_ptr(),
    owner: unsafe { &THIS_MODULE as *const c_void as *mut c_void },
    states: [
        cpuidle_state {
            enter: None,
            exit_latency: 0,
            target_residency: 0,
            name: b"WFI\0".as_ptr(),
            desc: b"Wait For Interrupt\0".as_ptr(),
        },
        cpuidle_state {
            enter: Some(zynq_enter_idle),
            exit_latency: 10,
            target_residency: 10000,
            name: b"RAM_SR\0".as_ptr(),
            desc: b"WFI and RAM Self Refresh\0".as_ptr(),
        },
    ],
    safe_state_index: 0,
    state_count: ZYNQ_MAX_STATES,
};

// Initialize CPU idle by registering the idle states
unsafe extern "C" fn zynq_cpuidle_probe(_pdev: *mut platform_device) -> i32 {
    // pr_info("Xilinx Zynq CpuIdle Driver started\n");

    cpuidle_register(&raw mut zynq_idle_driver, core::ptr::null_mut())
}

#[repr(C)]
struct platform_driver {
    name: *const u8,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

static mut zynq_cpuidle_driver: platform_driver = platform_driver {
    name: b"cpuidle-zynq\0".as_ptr(),
    probe: Some(zynq_cpuidle_probe),
};

// builtin_platform_driver(zynq_cpuidle_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
