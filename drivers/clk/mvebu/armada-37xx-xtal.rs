// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Armada 37xx SoC xtal clocks
 *
 * Copyright (C) 2016 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 *
 */

// Translated dependencies from:
// <linux/clk-provider.h>, <linux/mfd/syscon.h>, <linux/platform_device.h>,
// and <linux/regmap.h>.

const NB_GPIO1_LATCH: u32 = 0x8;
const XTAL_MODE: u32 = 1u32 << 9;

#[repr(C)]
pub struct device_node {
    pub parent: *mut device_node,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
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
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn syscon_node_to_regmap(node: *mut device_node) -> *mut regmap;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn of_property_read_string_index(
        np: *mut device_node,
        propname: *const core::ffi::c_char,
        index: usize,
        output: *mut *const core::ffi::c_char,
    ) -> i32;
    fn clk_hw_register_fixed_rate(
        dev: *mut device,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        flags: u32,
        rate: u32,
    ) -> *mut clk_hw;
    fn of_clk_add_hw_provider(
        np: *mut device_node,
        get: Option<unsafe extern "C" fn(*mut device_node, *const u32) -> *mut clk_hw>,
        hw: *mut clk_hw,
    ) -> i32;
    fn of_clk_hw_simple_get(np: *mut device_node, args: *const u32) -> *mut clk_hw;
    fn of_clk_del_provider(np: *mut device_node);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn is_err<T>(ptr: *mut T) -> bool;
}

const GFP_KERNEL: u32 = 0;

#[allow(non_snake_case)]
unsafe extern "C" fn armada_3700_xtal_clock_probe(pdev: *mut platform_device) -> i32 {
    let np = (*pdev).dev.of_node;
    let mut xtal_name = b"xtal\0".as_ptr() as *const core::ffi::c_char;
    let parent: *mut device_node;
    let regmap: *mut regmap;
    let mut xtal_hw: *mut clk_hw;
    let rate: u32;
    let mut reg: u32 = 0;
    let ret: i32;

    xtal_hw = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<clk_hw>(),
        GFP_KERNEL,
    ) as *mut clk_hw;
    if xtal_hw.is_null() {
        return -12;
    }

    platform_set_drvdata(pdev, xtal_hw as *mut core::ffi::c_void);

    parent = (*np).parent;
    if parent.is_null() {
        dev_err(&mut (*pdev).dev, b"no parent\n\0".as_ptr() as *const _);
        return -19;
    }

    regmap = syscon_node_to_regmap(parent);
    if is_err(regmap) {
        dev_err(&mut (*pdev).dev, b"cannot get regmap\n\0".as_ptr() as *const _);
        return ptr_err(regmap as *mut core::ffi::c_void);
    }

    ret = regmap_read(regmap, NB_GPIO1_LATCH, &mut reg);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"cannot read from regmap\n\0".as_ptr() as *const _);
        return ret;
    }

    if reg & XTAL_MODE != 0 {
        rate = 40000000;
    } else {
        rate = 25000000;
    }

    of_property_read_string_index(
        np,
        b"clock-output-names\0".as_ptr() as *const _,
        0,
        &mut xtal_name,
    );
    xtal_hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), xtal_name, core::ptr::null(), 0, rate);
    if is_err(xtal_hw) {
        return ptr_err(xtal_hw as *mut core::ffi::c_void);
    }
    ret = of_clk_add_hw_provider(Some(np), Some(of_clk_hw_simple_get), xtal_hw);

    ret
}

unsafe extern "C" fn armada_3700_xtal_clock_remove(pdev: *mut platform_device) {
    of_clk_del_provider((*pdev).dev.of_node);
}

static armada_3700_xtal_clock_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"marvell,armada-3700-xtal-clock\0".as_ptr() as *const _,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

static mut armada_3700_xtal_clock_driver: platform_driver = platform_driver {
    probe: Some(armada_3700_xtal_clock_probe),
    remove: Some(armada_3700_xtal_clock_remove),
    driver: driver {
        name: b"marvell-armada-3700-xtal-clock\0".as_ptr() as *const _,
        of_match_table: armada_3700_xtal_clock_of_match.as_ptr(),
    },
};

// builtin_platform_driver(armada_3700_xtal_clock_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
