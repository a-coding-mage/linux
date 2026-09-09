// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel sources:
// <linux/cpuidle.h>, <linux/module.h>, <asm/system_misc.h>, and "cpuidle.h".

use core::ffi::c_void;

#[repr(C)]
pub struct cpuidle_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpuidle_driver {
    pub name: *const i8,
    pub owner: *mut c_void,
    pub states: [cpuidle_state; 1],
    pub state_count: u32,
}

#[repr(C)]
pub struct cpuidle_state {
    pub enter: Option<unsafe extern "C" fn(
        dev: *mut cpuidle_device,
        drv: *mut cpuidle_driver,
        index: i32,
    ) -> i32>,
    pub exit_latency: u32,
    pub target_residency: u32,
    pub name: *const i8,
    pub desc: *const i8,
}

unsafe extern "C" {
    fn arm_pm_idle();
    fn cpuidle_register(
        driver: *mut cpuidle_driver,
        device: *mut c_void,
    ) -> i32;
    static mut THIS_MODULE: c_void;
}

unsafe extern "C" fn imx5_cpuidle_enter(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    unsafe {
        arm_pm_idle();
    }
    index
}

static mut imx5_cpuidle_driver: cpuidle_driver = cpuidle_driver {
    name: b"imx5_cpuidle\0".as_ptr() as *const i8,
    owner: core::ptr::addr_of_mut!(THIS_MODULE),
    states: [cpuidle_state {
        enter: Some(imx5_cpuidle_enter),
        exit_latency: 2,
        target_residency: 1,
        name: b"IMX5 SRPG\0".as_ptr() as *const i8,
        desc: b"CPU state retained,powered off\0".as_ptr() as *const i8,
    }],
    state_count: 1,
};

pub unsafe extern "C" fn imx5_cpuidle_init() -> i32 {
    unsafe { cpuidle_register(core::ptr::addr_of_mut!(imx5_cpuidle_driver), core::ptr::null_mut()) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
