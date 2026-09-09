// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2011 Wolfson Microelectronics, plc
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
//		http://www.samsung.com

// Dependencies corresponding to the C headers:
// linux/kernel.h, linux/init.h, linux/cpuidle.h, linux/io.h,
// linux/export.h, linux/time.h, asm/cpuidle.h, cpu.h, map.h,
// regs-sys-s3c64xx.h, and regs-syscon-power-s3c64xx.h.

static unsafe extern "C" {
    fn __raw_readl(addr: usize) -> usize;
    fn __raw_writel(value: usize, addr: usize);
    fn cpu_do_idle();
    fn soc_is_s3c64xx() -> bool;
    fn cpuidle_register(
        driver: *mut cpuidle_driver,
        device: *mut core::ffi::c_void,
    ) -> i32;
}

// External types and constants are supplied by the corresponding kernel
// dependencies.
#[allow(non_camel_case_types)]
type cpuidle_enter_t = unsafe extern "C" fn(
    dev: *mut cpuidle_device,
    drv: *mut cpuidle_driver,
    index: i32,
) -> i32;

#[repr(C)]
struct cpuidle_device;

#[repr(C)]
struct cpuidle_state {
    enter: Option<cpuidle_enter_t>,
    exit_latency: u32,
    target_residency: u32,
    name: *const u8,
    desc: *const u8,
}

#[repr(C)]
struct cpuidle_driver {
    name: *const u8,
    owner: *const core::ffi::c_void,
    states: [cpuidle_state; 1],
    state_count: u32,
}

extern "C" {
    static THIS_MODULE: core::ffi::c_void;
    static S3C64XX_PWR_CFG: usize;
    static S3C64XX_PWRCFG_CFG_WFI_MASK: usize;
    static S3C64XX_PWRCFG_CFG_WFI_IDLE: usize;
}

unsafe extern "C" fn s3c64xx_enter_idle(
    _dev: *mut cpuidle_device,
    _drv: *mut cpuidle_driver,
    index: i32,
) -> i32 {
    let mut tmp: usize;

    /* Setup PWRCFG to enter idle mode */
    tmp = __raw_readl(S3C64XX_PWR_CFG);
    tmp &= !S3C64XX_PWRCFG_CFG_WFI_MASK;
    tmp |= S3C64XX_PWRCFG_CFG_WFI_IDLE;
    __raw_writel(tmp, S3C64XX_PWR_CFG);

    cpu_do_idle();

    index
}

static mut s3c64xx_cpuidle_driver: cpuidle_driver = cpuidle_driver {
    name: b"s3c64xx_cpuidle\0".as_ptr(),
    owner: unsafe { &THIS_MODULE as *const _ as *const core::ffi::c_void },
    states: [cpuidle_state {
        enter: Some(s3c64xx_enter_idle),
        exit_latency: 1,
        target_residency: 1,
        name: b"IDLE\0".as_ptr(),
        desc: b"System active, ARM gated\0".as_ptr(),
    }],
    state_count: 1,
};

unsafe extern "C" fn s3c64xx_init_cpuidle() -> i32 {
    if soc_is_s3c64xx() {
        return cpuidle_register(&raw mut s3c64xx_cpuidle_driver, core::ptr::null_mut());
    }
    0
}

// device_initcall(s3c64xx_init_cpuidle);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
