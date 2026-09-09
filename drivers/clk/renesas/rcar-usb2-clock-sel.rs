// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas R-Car USB2.0 clock selector
 *
 * Copyright (C) 2017 Renesas Electronics Corp.
 *
 * Based on renesas-cpg-mssr.c
 *
 * Copyright (C) 2015 Glider bvba
 */

// Linux kernel dependencies supplied by other translation units.

const USB20_CLKSET0: usize = 0x00;
const CLKSET0_INTCLK_EN: u16 = 1 << 11;
const CLKSET0_PRIVATE: u16 = 1 << 0;
const CLKSET0_EXTAL_ONLY: u16 = CLKSET0_INTCLK_EN | CLKSET0_PRIVATE;

#[repr(C)]
pub struct ClkBulkData {
    pub id: *const core::ffi::c_char,
}

static RCAR_USB2_CLOCKS: [ClkBulkData; 2] = [
    ClkBulkData { id: c"ehci_ohci".as_ptr() },
    ClkBulkData { id: c"hs-usb-if".as_ptr() },
];

#[repr(C)]
pub struct ClkHw {
    pub init: *const ClkInitData,
}

#[repr(C)]
pub struct Usb2ClockSelPriv {
    pub base: *mut u8,
    pub hw: ClkHw,
    pub clks: [ClkBulkData; 2],
    pub rsts: *mut ResetControl,
    pub extal: bool,
    pub xtal: bool,
}

#[repr(C)]
pub struct ResetControl;
#[repr(C)]
pub struct Device;
#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
}
#[repr(C)]
pub struct DeviceNode;
#[repr(C)]
pub struct Clk;
#[repr(C)]
pub struct ClkOps {
    pub enable: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut ClkHw)>,
}
#[repr(C)]
pub struct ClkInitData {
    pub name: *const core::ffi::c_char,
    pub ops: *const ClkOps,
}
#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const core::ffi::c_char,
}
#[repr(C)]
pub struct DevPmOps {
    pub suspend: Option<unsafe extern "C" fn(*mut Device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut Device) -> i32>,
}
#[repr(C)]
pub struct Driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
    pub pm: *const DevPmOps,
}
#[repr(C)]
pub struct PlatformDriver {
    pub driver: Driver,
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut PlatformDevice)>,
}

extern "C" {
    fn readw(addr: *mut u8) -> u16;
    fn writew(value: u16, addr: *mut u8);
    fn reset_control_deassert(rst: *mut ResetControl) -> i32;
    fn reset_control_assert(rst: *mut ResetControl);
    fn clk_bulk_prepare_enable(n: usize, clks: *mut ClkBulkData) -> i32;
    fn clk_bulk_disable_unprepare(n: usize, clks: *mut ClkBulkData);
    fn dev_get_drvdata(dev: *mut Device) -> *mut Usb2ClockSelPriv;
    fn pm_runtime_put(dev: *mut Device) -> i32;
    fn pm_runtime_get_sync(dev: *mut Device) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut u8;
    fn devm_clk_bulk_get(dev: *mut Device, n: usize, clks: *mut ClkBulkData) -> i32;
    fn devm_reset_control_array_get_shared(dev: *mut Device) -> *mut ResetControl;
    fn devm_clk_get(dev: *mut Device, id: *const core::ffi::c_char) -> *mut Clk;
    fn clk_prepare_enable(clk: *mut Clk) -> i32;
    fn clk_get_rate(clk: *mut Clk) -> u64;
    fn clk_disable_unprepare(clk: *mut Clk);
    fn pm_runtime_enable(dev: *mut Device);
    fn pm_runtime_disable(dev: *mut Device);
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut Usb2ClockSelPriv);
    fn dev_set_drvdata(dev: *mut Device, data: *mut Usb2ClockSelPriv);
    fn devm_clk_hw_register(dev: *mut Device, hw: *mut ClkHw) -> i32;
    fn of_clk_add_hw_provider(np: *mut DeviceNode, get: *const core::ffi::c_void, data: *mut ClkHw) -> i32;
    fn of_clk_del_provider(np: *mut DeviceNode);
    fn device_node(dev: *mut Device) -> *mut DeviceNode;
}

unsafe fn usb2_clock_sel_enable_extal_only(priv_: *mut Usb2ClockSelPriv) {
    let val = readw((*priv_).base.add(USB20_CLKSET0));
    if (*priv_).extal && !(*priv_).xtal && val != CLKSET0_EXTAL_ONLY {
        writew(CLKSET0_EXTAL_ONLY, (*priv_).base.add(USB20_CLKSET0));
    }
}

unsafe fn usb2_clock_sel_disable_extal_only(priv_: *mut Usb2ClockSelPriv) {
    if (*priv_).extal && !(*priv_).xtal {
        writew(CLKSET0_PRIVATE, (*priv_).base.add(USB20_CLKSET0));
    }
}

unsafe extern "C" fn usb2_clock_sel_enable(hw: *mut ClkHw) -> i32 {
    let priv_ = (hw as *mut u8).sub(core::mem::offset_of!(Usb2ClockSelPriv, hw)) as *mut Usb2ClockSelPriv;
    let ret = reset_control_deassert((*priv_).rsts);
    if ret != 0 { return ret; }
    let ret = clk_bulk_prepare_enable(2, (*priv_).clks.as_mut_ptr());
    if ret != 0 {
        reset_control_assert((*priv_).rsts);
        return ret;
    }
    usb2_clock_sel_enable_extal_only(priv_);
    0
}

unsafe extern "C" fn usb2_clock_sel_disable(hw: *mut ClkHw) {
    let priv_ = (hw as *mut u8).sub(core::mem::offset_of!(Usb2ClockSelPriv, hw)) as *mut Usb2ClockSelPriv;
    usb2_clock_sel_disable_extal_only(priv_);
    clk_bulk_disable_unprepare(2, (*priv_).clks.as_mut_ptr());
    reset_control_assert((*priv_).rsts);
}

/* This module seems a mux, but this driver assumes a gate because
 * ehci/ohci platform drivers don't support clk_set_parent() for now.
 * If this driver acts as a gate, ehci/ohci-platform drivers don't need
 * any modification.
 */
static USB2_CLOCK_SEL_CLOCK_OPS: ClkOps = ClkOps {
    enable: Some(usb2_clock_sel_enable),
    disable: Some(usb2_clock_sel_disable),
};

static RCAR_USB2_CLOCK_SEL_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: c"renesas,rcar-gen3-usb2-clock-sel".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn rcar_usb2_clock_sel_suspend(dev: *mut Device) -> i32 {
    let priv_ = dev_get_drvdata(dev);
    usb2_clock_sel_disable_extal_only(priv_);
    pm_runtime_put(dev);
    0
}

unsafe extern "C" fn rcar_usb2_clock_sel_resume(dev: *mut Device) -> i32 {
    let priv_ = dev_get_drvdata(dev);
    pm_runtime_get_sync(dev);
    usb2_clock_sel_enable_extal_only(priv_);
    0
}

unsafe extern "C" fn rcar_usb2_clock_sel_remove(pdev: *mut PlatformDevice) {
    let dev = &mut (*pdev).dev as *mut Device;
    of_clk_del_provider(device_node(dev));
    pm_runtime_put(dev);
    pm_runtime_disable(dev);
}

unsafe extern "C" fn rcar_usb2_clock_sel_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let np = device_node(dev);
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<Usb2ClockSelPriv>(), 0) as *mut Usb2ClockSelPriv;
    if priv_.is_null() { return -12; }
    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if (*priv_).base.is_null() { return -1; }
    core::ptr::copy_nonoverlapping(RCAR_USB2_CLOCKS.as_ptr(), (*priv_).clks.as_mut_ptr(), 2);
    let mut ret = devm_clk_bulk_get(dev, 2, (*priv_).clks.as_mut_ptr());
    if ret < 0 { return ret; }
    (*priv_).rsts = devm_reset_control_array_get_shared(dev);
    if (*priv_).rsts.is_null() { return -1; }
    let clk = devm_clk_get(dev, c"usb_extal".as_ptr());
    if !clk.is_null() && clk_prepare_enable(clk) == 0 {
        (*priv_).extal = clk_get_rate(clk) != 0;
        clk_disable_unprepare(clk);
    }
    let clk = devm_clk_get(dev, c"usb_xtal".as_ptr());
    if !clk.is_null() && clk_prepare_enable(clk) == 0 {
        (*priv_).xtal = clk_get_rate(clk) != 0;
        clk_disable_unprepare(clk);
    }
    if !(*priv_).extal && !(*priv_).xtal { return -2; }
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    platform_set_drvdata(pdev, priv_);
    dev_set_drvdata(dev, priv_);
    let mut init = ClkInitData { name: c"rcar_usb2_clock_sel".as_ptr(), ops: &USB2_CLOCK_SEL_CLOCK_OPS };
    (*priv_).hw.init = &mut init;
    ret = devm_clk_hw_register(dev, &mut (*priv_).hw);
    if ret != 0 { pm_runtime_put(dev); pm_runtime_disable(dev); return ret; }
    ret = of_clk_add_hw_provider(np, core::ptr::null(), &mut (*priv_).hw);
    if ret != 0 { pm_runtime_put(dev); pm_runtime_disable(dev); return ret; }
    0
}

static RCAR_USB2_CLOCK_SEL_PM_OPS: DevPmOps = DevPmOps {
    suspend: Some(rcar_usb2_clock_sel_suspend),
    resume: Some(rcar_usb2_clock_sel_resume),
};

static mut RCAR_USB2_CLOCK_SEL_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver {
        name: c"rcar-usb2-clock-sel".as_ptr(),
        of_match_table: RCAR_USB2_CLOCK_SEL_MATCH.as_ptr(),
        pm: &RCAR_USB2_CLOCK_SEL_PM_OPS,
    },
    probe: Some(rcar_usb2_clock_sel_probe),
    remove: Some(rcar_usb2_clock_sel_remove),
};

// builtin_platform_driver(rcar_usb2_clock_sel_driver);
// MODULE_DESCRIPTION("Renesas R-Car USB2 clock selector Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
