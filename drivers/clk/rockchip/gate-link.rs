// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2024 Collabora Ltd.
 * Author: Sebastian Reichel <sebastian.reichel@collabora.com>
 */

// Translated dependencies:
// linux/clk.h, linux/platform_device.h, linux/pm_clock.h,
// linux/pm_runtime.h, linux/property.h, and "clk.h".

extern "C" {
    fn clk_register_gate(
        dev: *mut device,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        flags: c_ulong,
        reg: *mut core::ffi::c_void,
        bit_idx: u8,
        clk_gate_flags: u32,
        lock: *mut core::ffi::c_void,
    ) -> *mut clk;
    fn rockchip_clk_set_lookup(ctx: *mut rockchip_clk_provider, clk: *mut clk, id: i32);
    fn rockchip_clk_get_lookup(ctx: *mut rockchip_clk_provider, id: i32) -> *mut clk;
    fn dev_get_platdata(dev: *mut device) -> *mut rockchip_gate_link_platdata;
    fn devm_pm_runtime_enable(dev: *mut device) -> i32;
    fn devm_pm_clk_create(dev: *mut device) -> i32;
    fn pm_clk_add_clk(dev: *mut device, clk: *mut clk) -> i32;
    fn pm_clk_remove_clk(dev: *mut device, clk: *mut clk);
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const core::ffi::c_char) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn pm_clk_suspend(dev: *mut device) -> i32;
    fn pm_clk_resume(dev: *mut device) -> i32;
}

// External types and constants supplied by the Linux/Rockchip headers.
use core::ffi::{c_char, c_ulong};

const CLK_SET_RATE_PARENT: c_ulong = 1 << 2;
const ENODEV: i32 = 19;

unsafe fn rk_clk_gate_link_register(
    dev: *mut device,
    ctx: *mut rockchip_clk_provider,
    clkbr: *mut rockchip_clk_branch,
) -> i32 {
    let flags = (*clkbr).flags | CLK_SET_RATE_PARENT;
    let clk: *mut clk;

    clk = clk_register_gate(
        dev,
        (*clkbr).name,
        *(*clkbr).parent_names,
        flags,
        (*ctx).reg_base.add((*clkbr).gate_offset as usize),
        (*clkbr).gate_shift,
        (*clkbr).gate_flags,
        &mut (*ctx).lock as *mut _ as *mut core::ffi::c_void,
    );

    if (clk as isize) < 0 {
        return clk as i32;
    }

    rockchip_clk_set_lookup(ctx, clk, (*clkbr).id);
    0
}

unsafe fn rk_clk_gate_link_probe(pdev: *mut platform_device) -> i32 {
    let pdata: *mut rockchip_gate_link_platdata;
    let dev: *mut device = &mut (*pdev).dev;
    let linked_clk: *mut clk;
    let mut ret: i32;

    pdata = dev_get_platdata(dev);
    if pdata.is_null() {
        return dev_err_probe(dev, -ENODEV, b"missing platform data\0".as_ptr() as *const c_char);
    }

    ret = devm_pm_runtime_enable(dev);
    if ret != 0 {
        return ret;
    }

    ret = devm_pm_clk_create(dev);
    if ret != 0 {
        return ret;
    }

    linked_clk = rockchip_clk_get_lookup((*pdata).ctx, (*(*pdata).clkbr).linked_clk_id);
    ret = pm_clk_add_clk(dev, linked_clk);
    if ret != 0 {
        return ret;
    }

    ret = rk_clk_gate_link_register(dev, (*pdata).ctx, (*pdata).clkbr);
    if ret != 0 {
        pm_clk_remove_clk(dev, linked_clk);
        return ret;
    }

    0
}

static mut rk_clk_gate_link_pm_ops: dev_pm_ops = dev_pm_ops {
    // SET_RUNTIME_PM_OPS(pm_clk_suspend, pm_clk_resume, NULL)
    runtime_suspend: Some(pm_clk_suspend),
    runtime_resume: Some(pm_clk_resume),
    runtime_idle: None,
};

static mut rk_clk_gate_link_driver: platform_driver = platform_driver {
    probe: Some(rk_clk_gate_link_probe),
    driver: device_driver {
        name: b"rockchip-gate-link-clk\0".as_ptr() as *const c_char,
        pm: &raw mut rk_clk_gate_link_pm_ops,
        suppress_bind_attrs: true,
        ..device_driver::default()
    },
    ..platform_driver::default()
};

unsafe fn rk_clk_gate_link_drv_register() -> i32 {
    platform_driver_register(&raw mut rk_clk_gate_link_driver)
}

// core_initcall(rk_clk_gate_link_drv_register);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
