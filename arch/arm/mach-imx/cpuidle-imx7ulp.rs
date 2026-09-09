// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017-2018 NXP
 *   Anson Huang <Anson.Huang@nxp.com>
 */

// Dependencies supplied by the Linux cpuidle, module, ARM cpuidle, common,
// and cpuidle interfaces are intentionally left external to this translation.

#[repr(C)]
pub struct CpuidleDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CpuidleDriver {
    _private: [u8; 0],
}

extern "C" {
    fn imx7ulp_set_lpm(mode: i32);
    fn cpu_do_idle();
    fn cpuidle_register(driver: *mut CpuidleDriver, device: *mut CpuidleDevice) -> i32;
}

// ULP_PM_WAIT, ULP_PM_STOP, and ULP_PM_RUN are supplied by common.h.
extern "C" {
    static ULP_PM_WAIT: i32;
    static ULP_PM_STOP: i32;
    static ULP_PM_RUN: i32;
}

// ARM_CPUIDLE_WFI_STATE is the architecture-provided WFI state initializer.
// The corresponding first state is retained in the driver state array below.

#[repr(C)]
pub struct CpuidleState {
    pub exit_latency: u32,
    pub target_residency: u32,
    pub enter: Option<unsafe extern "C" fn(*mut CpuidleDevice, *mut CpuidleDriver, i32) -> i32>,
    pub name: *const u8,
    pub desc: *const u8,
}

#[repr(C)]
pub struct Imx7ulpCpuidleDriver {
    pub name: *const u8,
    pub owner: *const (),
    pub states: [CpuidleState; 3],
    pub state_count: u32,
    pub safe_state_index: u32,
}

unsafe extern "C" fn imx7ulp_enter_wait(
    _dev: *mut CpuidleDevice,
    _drv: *mut CpuidleDriver,
    index: i32,
) -> i32 {
    if index == 1 {
        imx7ulp_set_lpm(ULP_PM_WAIT);
    } else {
        imx7ulp_set_lpm(ULP_PM_STOP);
    }

    cpu_do_idle();

    imx7ulp_set_lpm(ULP_PM_RUN);

    index
}

static mut imx7ulp_cpuidle_driver: Imx7ulpCpuidleDriver = Imx7ulpCpuidleDriver {
    name: b"imx7ulp_cpuidle\0".as_ptr(),
    owner: core::ptr::null(),
    states: [
        // WFI: ARM_CPUIDLE_WFI_STATE
        CpuidleState {
            exit_latency: 0,
            target_residency: 0,
            enter: None,
            name: b"WFI\0".as_ptr(),
            desc: b"WFI\0".as_ptr(),
        },
        // WAIT
        CpuidleState {
            exit_latency: 50,
            target_residency: 75,
            enter: Some(imx7ulp_enter_wait),
            name: b"WAIT\0".as_ptr(),
            desc: b"PSTOP2\0".as_ptr(),
        },
        // STOP
        CpuidleState {
            exit_latency: 100,
            target_residency: 150,
            enter: Some(imx7ulp_enter_wait),
            name: b"STOP\0".as_ptr(),
            desc: b"PSTOP1\0".as_ptr(),
        },
    ],
    state_count: 3,
    safe_state_index: 0,
};

pub unsafe extern "C" fn imx7ulp_cpuidle_init() -> i32 {
    cpuidle_register(
        &raw mut imx7ulp_cpuidle_driver as *mut Imx7ulpCpuidleDriver as *mut CpuidleDriver,
        core::ptr::null_mut(),
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
