// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  CLPS711X CPU idle driver
 *
 *  Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const CLPS711X_CPUIDLE_NAME: *const core::ffi::c_char =
    b"clps711x-cpuidle\0".as_ptr() as *const core::ffi::c_char;

static mut clps711x_halt: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe extern "C" {
    fn writel(value: u32, address: *mut core::ffi::c_void);
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn IS_ERR(pointer: *mut core::ffi::c_void) -> bool;
    fn PTR_ERR(pointer: *mut core::ffi::c_void) -> i32;
    fn cpuidle_register(
        driver: *mut cpuidle_driver,
        device: *mut core::ffi::c_void,
    ) -> i32;
}

#[repr(C)]
pub struct cpuidle_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct cpuidle_state {
    pub name: *const core::ffi::c_char,
    pub desc: *const core::ffi::c_char,
    pub enter: Option<
        unsafe extern "C" fn(
            dev: *mut cpuidle_device,
            drv: *mut cpuidle_driver,
            index: i32,
        ) -> i32,
    >,
    pub exit_latency: u32,
}

#[repr(C)]
pub struct cpuidle_driver {
    pub name: *const core::ffi::c_char,
    pub owner: *mut core::ffi::c_void,
    pub states: [cpuidle_state; 1],
    pub state_count: u32,
}

#[repr(C)]
pub struct platform_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_base,
}

#[repr(C)]
pub struct platform_driver_base {
    pub name: *const core::ffi::c_char,
}

unsafe extern "C" fn clps711x_cpuidle_halt(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    writel(0xaa, clps711x_halt);

    index
}

static mut clps711x_idle_driver: cpuidle_driver = cpuidle_driver {
    name: CLPS711X_CPUIDLE_NAME,
    owner: core::ptr::null_mut(), // THIS_MODULE
    states: [cpuidle_state {
        name: b"HALT\0".as_ptr() as *const core::ffi::c_char,
        desc: b"CLPS711X HALT\0".as_ptr() as *const core::ffi::c_char,
        enter: Some(clps711x_cpuidle_halt),
        exit_latency: 1,
    }],
    state_count: 1,
};

unsafe extern "C" fn clps711x_cpuidle_probe(pdev: *mut platform_device) -> i32 {
    clps711x_halt = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(clps711x_halt) {
        return PTR_ERR(clps711x_halt);
    }

    cpuidle_register(&raw mut clps711x_idle_driver, core::ptr::null_mut())
}

static mut clps711x_cpuidle_driver: platform_driver = platform_driver {
    driver: platform_driver_base {
        name: CLPS711X_CPUIDLE_NAME,
    },
};

// Equivalent to builtin_platform_driver_probe(clps711x_cpuidle_driver,
// clps711x_cpuidle_probe).
unsafe fn builtin_platform_driver_probe() {
    let _ = (&raw mut clps711x_cpuidle_driver, clps711x_cpuidle_probe);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
