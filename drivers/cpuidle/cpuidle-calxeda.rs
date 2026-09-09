// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2012 Calxeda, Inc.
 *
 * Based on arch/arm/plat-mxc/cpuidle.c: #v3.7
 * Copyright 2012 Freescale Semiconductor, Inc.
 * Copyright 2012 Linaro Ltd.
 *
 * Maintainer: Rob Herring <rob.herring@calxeda.com>
 */

// Dependencies supplied by the kernel environment:
// linux/cpuidle.h, linux/cpu_pm.h, linux/init.h, linux/mm.h,
// linux/platform_device.h, linux/psci.h, asm/cpuidle.h, asm/suspend.h,
// and uapi/linux/psci.h.

const CALXEDA_IDLE_PARAM: u32 =
    (0 << PSCI_0_2_POWER_STATE_ID_SHIFT)
    | (0 << PSCI_0_2_POWER_STATE_AFFL_SHIFT)
    | (PSCI_POWER_STATE_TYPE_POWER_DOWN << PSCI_0_2_POWER_STATE_TYPE_SHIFT);

unsafe extern "C" {
    static psci_ops: PsciOps;
    static cpu_resume: unsafe extern "C" fn();

    fn __pa(addr: unsafe extern "C" fn()) -> usize;
    fn cpu_pm_enter();
    fn cpu_suspend(arg: u32, fnptr: unsafe extern "C" fn(unsigned long) -> i32) -> i32;
    fn cpu_pm_exit();
    fn cpuidle_register(driver: *mut CpuidleDriver, device: *mut CpuidleDevice) -> i32;
    fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
}

#[repr(C)]
struct PsciOps {
    cpu_suspend: unsafe extern "C" fn(u32, usize) -> i32,
}

#[repr(C)]
struct CpuidleDevice {
    _private: [u8; 0],
}

#[repr(C)]
struct CpuidleDriver {
    name: *const u8,
    states: [CpuidleState; 2],
    state_count: u32,
}

#[repr(C)]
struct CpuidleState {
    name: *const u8,
    desc: *const u8,
    exit_latency: u32,
    power_usage: u32,
    target_residency: u32,
    enter: Option<unsafe extern "C" fn(*mut CpuidleDevice, *mut CpuidleDriver, i32) -> i32>,
}

#[repr(C)]
struct PlatformDriver {
    driver: Driver,
    probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
}

#[repr(C)]
struct Driver {
    name: *const u8,
}

#[repr(C)]
struct PlatformDevice {
    _private: [u8; 0],
}

// ARM_CPUIDLE_WFI_STATE is supplied by asm/cpuidle.h.
unsafe extern "C" {
    static ARM_CPUIDLE_WFI_STATE: CpuidleState;
}

unsafe extern "C" fn calxeda_idle_finish(_val: u64) -> i32 {
    ((*(&raw const psci_ops)).cpu_suspend)(CALXEDA_IDLE_PARAM, __pa(cpu_resume))
}

unsafe extern "C" fn calxeda_pwrdown_idle(
    _dev: *mut CpuidleDevice,
    _drv: *mut CpuidleDriver,
    index: i32,
) -> i32 {
    cpu_pm_enter();
    cpu_suspend(0, calxeda_idle_finish);
    cpu_pm_exit();

    index
}

static mut CALXEDA_IDLE_DRIVER: CpuidleDriver = CpuidleDriver {
    name: b"calxeda_idle\0".as_ptr(),
    states: [
        // The first state is ARM_CPUIDLE_WFI_STATE.
        CpuidleState {
            name: b"\0".as_ptr(),
            desc: b"\0".as_ptr(),
            exit_latency: 0,
            power_usage: 0,
            target_residency: 0,
            enter: None,
        },
        CpuidleState {
            name: b"PG\0".as_ptr(),
            desc: b"Power Gate\0".as_ptr(),
            exit_latency: 30,
            power_usage: 50,
            target_residency: 200,
            enter: Some(calxeda_pwrdown_idle),
        },
    ],
    state_count: 2,
};

unsafe extern "C" fn calxeda_cpuidle_probe(_pdev: *mut PlatformDevice) -> i32 {
    cpuidle_register(&raw mut CALXEDA_IDLE_DRIVER, core::ptr::null_mut())
}

static mut CALXEDA_CPUIDLE_PLAT_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver {
        name: b"cpuidle-calxeda\0".as_ptr(),
    },
    probe: Some(calxeda_cpuidle_probe),
};

// builtin_platform_driver(calxeda_cpuidle_plat_driver);
#[used]
static CALXEDA_CPUIDLE_PLAT_DRIVER_INIT: unsafe extern "C" fn() -> i32 = {
    unsafe extern "C" fn init() -> i32 {
        platform_driver_register(&raw mut CALXEDA_CPUIDLE_PLAT_DRIVER)
    }
    init
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
