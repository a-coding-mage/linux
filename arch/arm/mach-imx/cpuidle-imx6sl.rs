// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the corresponding kernel headers and source files.

extern "C" {
    fn imx6_set_lpm(mode: u32);
    fn imx6sl_set_wait_clk(enable: bool);
    fn cpu_do_idle();
    fn cpuidle_register(driver: *mut cpuidle_driver, device: *mut core::ffi::c_void) -> i32;
}

// The concrete definitions of these kernel types and constants are supplied externally.
#[repr(C)]
pub struct cpuidle_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpuidle_driver {
    pub name: *const core::ffi::c_char,
    pub owner: *mut core::ffi::c_void,
    pub states: [cpuidle_state; 2],
    pub state_count: u32,
    pub safe_state_index: u32,
}

#[repr(C)]
pub struct cpuidle_state {
    pub exit_latency: u32,
    pub target_residency: u32,
    pub flags: u32,
    pub enter: Option<unsafe extern "C" fn(
        dev: *mut cpuidle_device,
        drv: *mut cpuidle_driver,
        index: i32,
    ) -> i32>,
    pub name: *const core::ffi::c_char,
    pub desc: *const core::ffi::c_char,
}

const WAIT_UNCLOCKED: u32 = 0;
const WAIT_CLOCKED: u32 = 0;
const CPUIDLE_FLAG_TIMER_STOP: u32 = 1;

// `THIS_MODULE` and the ARM_CPUIDLE_WFI_STATE initializer are provided by
// the kernel build environment; the WFI state has no file-local definition.
extern "C" {
    static mut THIS_MODULE: core::ffi::c_void;
}

unsafe extern "C" fn imx6sl_enter_wait(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    imx6_set_lpm(WAIT_UNCLOCKED);
    /*
     * Software workaround for ERR005311, see function
     * description for details.
     */
    imx6sl_set_wait_clk(true);
    cpu_do_idle();
    imx6sl_set_wait_clk(false);
    imx6_set_lpm(WAIT_CLOCKED);

    index
}

static mut imx6sl_cpuidle_driver: cpuidle_driver = cpuidle_driver {
    name: b"imx6sl_cpuidle\0".as_ptr() as *const core::ffi::c_char,
    owner: unsafe { &mut THIS_MODULE as *mut core::ffi::c_void },
    states: [
        // WFI (`ARM_CPUIDLE_WFI_STATE`).
        cpuidle_state {
            exit_latency: 0,
            target_residency: 0,
            flags: 0,
            enter: None,
            name: core::ptr::null(),
            desc: core::ptr::null(),
        },
        // WAIT
        cpuidle_state {
            exit_latency: 50,
            target_residency: 75,
            flags: CPUIDLE_FLAG_TIMER_STOP,
            enter: Some(imx6sl_enter_wait),
            name: b"WAIT\0".as_ptr() as *const core::ffi::c_char,
            desc: b"Clock off\0".as_ptr() as *const core::ffi::c_char,
        },
    ],
    state_count: 2,
    safe_state_index: 0,
};

pub unsafe extern "C" fn imx6sl_cpuidle_init() -> i32 {
    cpuidle_register(&mut imx6sl_cpuidle_driver, core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
