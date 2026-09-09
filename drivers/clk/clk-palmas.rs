// SPDX-License-Identifier: GPL-2.0
/*
 * Clock driver for Palmas device.
 *
 * Copyright (c) 2013, NVIDIA Corporation.
 * Copyright (c) 2013-2014 Texas Instruments, Inc.
 *
 * Author: Laxman Dewangan <ldewangan@nvidia.com>
 *         Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

pub const PALMAS_CLOCK_DT_EXT_CONTROL_ENABLE1: u32 = 1;
pub const PALMAS_CLOCK_DT_EXT_CONTROL_ENABLE2: u32 = 2;
pub const PALMAS_CLOCK_DT_EXT_CONTROL_NSLEEP: u32 = 3;

#[repr(C)]
pub struct palmas_clk32k_desc {
    pub clk_name: *const core::ffi::c_char,
    pub control_reg: u32,
    pub enable_mask: u32,
    pub sleep_mask: u32,
    pub sleep_reqstr_id: u32,
    pub delay: i32,
}

#[repr(C)]
pub struct palmas_clock_info {
    pub dev: *mut device,
    pub hw: clk_hw,
    pub palmas: *mut palmas,
    pub clk_desc: *const palmas_clk32k_desc,
    pub ext_control_pin: i32,
}

#[inline]
unsafe fn to_palmas_clks_info(hw: *mut clk_hw) -> *mut palmas_clock_info {
    (hw as *mut u8).sub(core::mem::offset_of!(palmas_clock_info, hw)) as *mut palmas_clock_info
}

unsafe extern "C" {
    fn palmas_update_bits(palmas: *mut palmas, base: u32, reg: u32, mask: u32, val: u32) -> i32;
    fn palmas_read(palmas: *mut palmas, base: u32, reg: u32, val: *mut u32) -> i32;
    fn udelay(usecs: u32);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn clk_unprepare(clk: *mut clk);
    fn clk_prepare(clk: *mut clk) -> i32;
    fn palmas_ext_control_req_config(palmas: *mut palmas, id: u32, pin: i32, enable: bool) -> i32;
    fn of_property_read_u32(node: *mut device_node, name: *const core::ffi::c_char, prop: *mut u32) -> i32;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut core::ffi::c_void), data: *mut core::ffi::c_void) -> i32;
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const core::ffi::c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: *const core::ffi::c_void, data: *mut core::ffi::c_void) -> i32;
}

unsafe fn palmas_clks_recalc_rate(_hw: *mut clk_hw, _parent_rate: usize) -> usize { 32768 }

unsafe fn palmas_clks_prepare(hw: *mut clk_hw) -> i32 {
    let cinfo = &mut *to_palmas_clks_info(hw);
    let d = &*cinfo.clk_desc;
    let ret = palmas_update_bits(cinfo.palmas, PALMAS_RESOURCE_BASE, d.control_reg, d.enable_mask, d.enable_mask);
    if ret < 0 { dev_err(cinfo.dev, c"Reg 0x%02x update failed, %d\n".as_ptr(), d.control_reg, ret); }
    else if d.delay != 0 { udelay(d.delay as u32); }
    ret
}

unsafe fn palmas_clks_unprepare(hw: *mut clk_hw) {
    let cinfo = &mut *to_palmas_clks_info(hw);
    if cinfo.ext_control_pin != 0 { return; }
    let d = &*cinfo.clk_desc;
    let ret = palmas_update_bits(cinfo.palmas, PALMAS_RESOURCE_BASE, d.control_reg, d.enable_mask, 0);
    if ret < 0 { dev_err(cinfo.dev, c"Reg 0x%02x update failed, %d\n".as_ptr(), d.control_reg, ret); }
}

unsafe fn palmas_clks_is_prepared(hw: *mut clk_hw) -> i32 {
    let cinfo = &mut *to_palmas_clks_info(hw);
    if cinfo.ext_control_pin != 0 { return 1; }
    let d = &*cinfo.clk_desc;
    let mut val = 0u32;
    let ret = palmas_read(cinfo.palmas, PALMAS_RESOURCE_BASE, d.control_reg, &mut val);
    if ret < 0 { dev_err(cinfo.dev, c"Reg 0x%02x read failed, %d\n".as_ptr(), d.control_reg, ret); return ret; }
    if val & d.enable_mask != 0 { 1 } else { 0 }
}

#[repr(C)]
pub struct palmas_clks_of_match_data { pub init: clk_init_data, pub desc: palmas_clk32k_desc }

unsafe fn palmas_clks_get_clk_data(pdev: *mut platform_device, cinfo: *mut palmas_clock_info) {
    let node = (*pdev).dev.of_node;
    let mut prop = 0u32;
    if of_property_read_u32(node, c"ti,external-sleep-control".as_ptr(), &mut prop) != 0 { return; }
    prop = match prop { PALMAS_CLOCK_DT_EXT_CONTROL_ENABLE1 => PALMAS_EXT_CONTROL_ENABLE1, PALMAS_CLOCK_DT_EXT_CONTROL_ENABLE2 => PALMAS_EXT_CONTROL_ENABLE2, PALMAS_CLOCK_DT_EXT_CONTROL_NSLEEP => PALMAS_EXT_CONTROL_NSLEEP, _ => { dev_warn(&mut (*pdev).dev, c"%pOFn: Invalid ext control option: %u\n".as_ptr(), node, prop); 0 } };
    (*cinfo).ext_control_pin = prop as i32;
}

unsafe extern "C" fn palmas_clks_unprepare_ext_control(data: *mut core::ffi::c_void) { let cinfo = data as *mut palmas_clock_info; clk_unprepare((*cinfo).hw.clk); }

unsafe fn palmas_clks_init_configure(cinfo: *mut palmas_clock_info) -> i32 {
    let d = &*(*cinfo).clk_desc;
    let mut ret = palmas_update_bits((*cinfo).palmas, PALMAS_RESOURCE_BASE, d.control_reg, d.sleep_mask, 0);
    if ret < 0 { dev_err((*cinfo).dev, c"Reg 0x%02x update failed, %d\n".as_ptr(), d.control_reg, ret); return ret; }
    if (*cinfo).ext_control_pin != 0 {
        ret = clk_prepare((*cinfo).hw.clk); if ret < 0 { dev_err((*cinfo).dev, c"Clock prep failed, %d\n".as_ptr(), ret); return ret; }
        ret = devm_add_action_or_reset((*cinfo).dev, palmas_clks_unprepare_ext_control, cinfo as *mut core::ffi::c_void); if ret != 0 { return ret; }
        ret = palmas_ext_control_req_config((*cinfo).palmas, d.sleep_reqstr_id, (*cinfo).ext_control_pin, true);
        if ret < 0 { dev_err((*cinfo).dev, c"Ext config for %s failed, %d\n".as_ptr(), d.clk_name, ret); return ret; }
    }
    ret
}

pub unsafe fn palmas_clks_probe(pdev: *mut platform_device) -> i32 {
    let palmas = dev_get_drvdata((*pdev).dev.parent) as *mut palmas;
    let match_data = of_device_get_match_data(&mut (*pdev).dev) as *const palmas_clks_of_match_data;
    if match_data.is_null() { return 1; }
    let cinfo = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<palmas_clock_info>(), GFP_KERNEL) as *mut palmas_clock_info;
    if cinfo.is_null() { return -12; }
    palmas_clks_get_clk_data(pdev, cinfo);
    platform_set_drvdata(pdev, cinfo as *mut core::ffi::c_void);
    (*cinfo).dev = &mut (*pdev).dev;
    (*cinfo).palmas = palmas;
    (*cinfo).clk_desc = &(*match_data).desc;
    (*cinfo).hw.init = &(*match_data).init;
    let mut ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*cinfo).hw);
    if ret != 0 { dev_err(&mut (*pdev).dev, c"Fail to register clock %s, %d\n".as_ptr(), (*match_data).desc.clk_name, ret); return ret; }
    ret = palmas_clks_init_configure(cinfo);
    if ret < 0 { dev_err(&mut (*pdev).dev, c"Clock config failed, %d\n".as_ptr(), ret); return ret; }
    ret = devm_of_clk_add_hw_provider(&mut (*pdev).dev, of_clk_hw_simple_get, &mut (*cinfo).hw as *mut clk_hw as *mut core::ffi::c_void);
    if ret < 0 { dev_err(&mut (*pdev).dev, c"Fail to add clock driver, %d\n".as_ptr(), ret); }
    ret
}

// C module metadata and platform-driver registration are retained as external kernel integration declarations.
unsafe extern "C" {
    static mut palmas_clks_driver: platform_driver;
    fn of_clk_hw_simple_get();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
