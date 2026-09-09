// SPDX-License-Identifier: GPL-2.0
/*
 * Qualcomm APCS clock controller driver
 *
 * Copyright (c) 2022, Linaro Limited
 * Author: Dmitry Baryshkov <dmitry.baryshkov@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_uint, c_void};

const APCS_AUX_OFFSET: c_uint = 0x50;
const APCS_AUX_DIV_MASK: c_uint = ((1u32 << (17 - 16 + 1)) - 1) << 16;
const APCS_AUX_DIV_2: c_uint = 0x1;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
}

extern "C" {
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn udelay(usecs: c_uint);
    fn devm_clk_hw_register_fixed_rate(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        rate: c_ulong,
    ) -> *mut clk_hw;
    fn IS_ERR(ptr: *mut c_void) -> bool;
    fn PTR_ERR(ptr: *mut c_void) -> c_int;
    fn devm_of_clk_add_hw_provider(
        dev: *mut device,
        get: unsafe extern "C" fn(),
        hw: *mut clk_hw,
    ) -> c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

type c_ulong = usize;

unsafe extern "C" fn of_clk_hw_simple_get() {}

unsafe extern "C" fn qcom_apcs_msm8996_clk_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let parent: *mut device = (*dev).parent;
    let mut regmap: *mut regmap;
    let mut hw: *mut clk_hw;
    let mut val: c_uint = 0;
    let ret: c_int = -19; // -ENODEV

    regmap = dev_get_regmap(parent, core::ptr::null());
    if regmap.is_null() {
        dev_err(dev, b"failed to get regmap: %d\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    regmap_read(regmap, APCS_AUX_OFFSET, &mut val);
    regmap_update_bits(
        regmap,
        APCS_AUX_OFFSET,
        APCS_AUX_DIV_MASK,
        (APCS_AUX_DIV_2 << 16) & APCS_AUX_DIV_MASK,
    );

    /*
     * This clock is used during CPU cluster setup while setting up CPU PLLs.
     * Add hardware mandated delay to make sure that the sys_apcs_aux clock
     * is stable (after setting the divider) before continuing
     * bootstrapping to keep CPUs from ending up in a weird state.
     */
    udelay(5);

    /*
     * As this clocks is a parent of the CPU cluster clocks and is actually
     * used as a parent during CPU clocks setup, we want for it to register
     * as early as possible, without letting fw_devlink to delay probing of
     * either of the drivers.
     *
     * The sys_apcs_aux is a child (divider) of gpll0, but we register it
     * as a fixed rate clock instead to ease bootstrapping procedure. By
     * doing this we make sure that CPU cluster clocks are able to be setup
     * early during the boot process (as it is recommended by Qualcomm).
     */
    hw = devm_clk_hw_register_fixed_rate(
        dev,
        b"sys_apcs_aux\0".as_ptr() as *const c_char,
        core::ptr::null(),
        0,
        300000000,
    );
    if IS_ERR(hw as *mut c_void) {
        return PTR_ERR(hw as *mut c_void);
    }

    devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, hw)
}

static mut qcom_apcs_msm8996_clk_driver: platform_driver = platform_driver {
    probe: Some(qcom_apcs_msm8996_clk_probe),
    driver: driver {
        name: b"qcom-apcs-msm8996-clk\0".as_ptr() as *const c_char,
    },
};

/* Register early enough to fix the clock to be used for other cores */
unsafe extern "C" fn qcom_apcs_msm8996_clk_init() -> c_int {
    platform_driver_register(&raw mut qcom_apcs_msm8996_clk_driver)
}

unsafe extern "C" fn qcom_apcs_msm8996_clk_exit() {
    platform_driver_unregister(&raw mut qcom_apcs_msm8996_clk_driver);
}

// postcore_initcall(qcom_apcs_msm8996_clk_init);
// module_exit(qcom_apcs_msm8996_clk_exit);

// MODULE_AUTHOR("Dmitry Baryshkov <dmitry.baryshkov@linaro.org>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Qualcomm MSM8996 APCS clock driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
