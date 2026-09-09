// SPDX-License-Identifier: GPL-2.0+
/*
 * Amlogic Meson-AXG Clock Controller Driver
 *
 * Copyright (c) 2016 BayLibre, SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 *
 * Copyright (c) 2018 Amlogic, inc.
 * Author: Qiufang Dai <qiufang.dai@amlogic.com>
 * Author: Yixun Lan <yixun.lan@amlogic.com>
 */

// External Linux kernel declarations supplied by the corresponding headers:
// linux/platform_device.h, linux/reset-controller.h, linux/mfd/syscon.h,
// linux/of.h, linux/module.h, linux/slab.h, meson-aoclk.h, and clk-regmap.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_controller_dev {
    pub ops: *const reset_control_ops,
    pub nr_resets: u32,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct reset_control_ops {
    pub reset: Option<unsafe extern "C" fn(*mut reset_controller_dev, c_ulong) -> c_int>,
}

#[repr(C)]
pub struct meson_aoclk_reset_controller {
    pub reset: reset_controller_dev,
    pub regmap: *mut regmap,
    pub data: *const meson_aoclk_data,
}

#[repr(C)]
pub struct meson_clkc_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct meson_aoclk_data {
    pub clkc_data: meson_clkc_data,
    pub reset_reg: u32,
    pub reset: *const u32,
    pub num_reset: u32,
}

extern "C" {
    fn of_device_get_match_data(dev: *mut device) -> *const meson_clkc_data;
    fn meson_clkc_syscon_probe(pdev: *mut platform_device) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_ulong) -> *mut c_void;
    fn of_get_parent(node: *mut device_node) -> *mut device_node;
    fn syscon_node_to_regmap(node: *mut device_node) -> *mut regmap;
    fn of_node_put(node: *mut device_node);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
    fn devm_reset_controller_register(
        dev: *mut device,
        rcdev: *mut reset_controller_dev,
    ) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_ulong = 0;

#[inline]
unsafe fn meson_aoclk_do_reset(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int {
    let rstc = rcdev as *mut meson_aoclk_reset_controller;
    let data = (*rstc).data;
    let reset_bit = *(*data).reset.add(id as usize);
    regmap_write((*rstc).regmap, (*data).reset_reg, 1u32.wrapping_shl(reset_bit))
}

static meson_aoclk_reset_ops: reset_control_ops = reset_control_ops {
    reset: Some(meson_aoclk_do_reset),
};

pub unsafe extern "C" fn meson_aoclkc_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let clkc_data: *const meson_clkc_data;
    let data: *const meson_aoclk_data;
    let mut rstc: *mut meson_aoclk_reset_controller;
    let mut np: *mut device_node;
    let mut regmap: *mut regmap;
    let mut ret: c_int;

    clkc_data = of_device_get_match_data(dev);
    if clkc_data.is_null() {
        return -EINVAL;
    }

    ret = meson_clkc_syscon_probe(pdev);
    if ret != 0 {
        return ret;
    }

    data = clkc_data as *const meson_aoclk_data;

    rstc = devm_kzalloc(dev, core::mem::size_of::<meson_aoclk_reset_controller>(), GFP_KERNEL)
        as *mut meson_aoclk_reset_controller;
    if rstc.is_null() {
        return -ENOMEM;
    }

    np = of_get_parent((*dev).of_node);
    regmap = syscon_node_to_regmap(np);
    of_node_put(np);
    if regmap.is_null() {
        dev_err(dev, b"failed to get regmap\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    /* Reset Controller */
    (*rstc).data = data;
    (*rstc).regmap = regmap;
    (*rstc).reset.ops = &meson_aoclk_reset_ops;
    (*rstc).reset.nr_resets = (*data).num_reset;
    (*rstc).reset.of_node = (*dev).of_node;
    ret = devm_reset_controller_register(dev, &mut (*rstc).reset);
    if ret != 0 {
        dev_err(dev, b"failed to register reset controller\0".as_ptr() as *const c_char);
        return ret;
    }

    0
}

// EXPORT_SYMBOL_NS_GPL(meson_aoclkc_probe, "CLK_MESON");
// MODULE_DESCRIPTION("Amlogic Always-ON Clock Controller helpers");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
