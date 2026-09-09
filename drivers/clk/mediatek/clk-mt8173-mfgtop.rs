// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024 Google LLC
 * Author: Chen-Yu Tsai <wenst@chromium.org>
 *
 * Based on driver in downstream ChromeOS v5.15 kernel.
 *
 * Copyright (c) 2014 MediaTek Inc.
 * Author: Chiawen Lee <chiawen.lee@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel clock, device-tree, and
// power-management code are intentionally left as external declarations.

use core::ffi::{c_char, c_int, c_void};

const CLK_MFG_AXI: usize = 0;
const CLK_MFG_MEM: usize = 1;
const CLK_MFG_G3D: usize = 2;
const CLK_MFG_26M: usize = 3;
const CLK_SET_RATE_PARENT: u32 = 1 << 0;

#[repr(C)]
struct MtkGateRegs { sta_ofs: u32, clr_ofs: u32, set_ofs: u32 }
#[repr(C)]
struct MtkGate { _private: [u8; 0] }
#[repr(C)]
struct ClkHwOnecellData { hws: *mut *mut ClkHw }
#[repr(C)] struct Regmap { _private: [u8; 0] }
#[repr(C)] struct GenericPmDomain { name: *const c_char, power_on: Option<unsafe extern "C" fn(*mut GenericPmDomain) -> c_int>, power_off: Option<unsafe extern "C" fn(*mut GenericPmDomain) -> c_int> }
#[repr(C)] struct OfPhandleArgs { np: *mut DeviceNode, args_count: u32 }
#[repr(C)] struct Clk { _private: [u8; 0] }
#[repr(C)] struct ClkHw { _private: [u8; 0] }
#[repr(C)] struct DeviceNode { _private: [u8; 0] }
#[repr(C)] struct Device { of_node: *mut DeviceNode }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct PlatformDriver { _private: [u8; 0] }

extern "C" {
    static mtk_clk_gate_ops_setclr: c_void;
    fn mtk_devm_alloc_clk_data(dev: *mut Device, n: usize) -> *mut ClkHwOnecellData;
    fn device_node_to_regmap(node: *mut DeviceNode) -> *mut Regmap;
    fn of_parse_phandle_with_args(node: *mut DeviceNode, name: *const c_char, cells: *const c_char, index: c_int, args: *mut OfPhandleArgs) -> c_int;
    fn devm_pm_runtime_enable(dev: *mut Device);
    fn pm_runtime_resume_and_get(dev: *mut Device) -> c_int;
    fn mtk_clk_register_gates(dev: *mut Device, node: *mut DeviceNode, gates: *const MtkGate, n: usize, data: *mut ClkHwOnecellData) -> c_int;
    fn clk_hw_get_clk(hw: *mut ClkHw, name: *const c_char) -> *mut Clk;
    fn of_clk_add_hw_provider(node: *mut DeviceNode, get: *const c_void, data: *mut ClkHwOnecellData) -> c_int;
    fn pm_genpd_init(pd: *mut GenericPmDomain, x: *mut c_void, is_off: bool) -> c_int;
    fn of_genpd_add_provider_simple(node: *mut DeviceNode, pd: *mut GenericPmDomain) -> c_int;
    fn of_genpd_add_subdomain(parent: *mut OfPhandleArgs, child: *mut OfPhandleArgs) -> c_int;
    fn pm_runtime_put(dev: *mut Device);
    fn of_genpd_del_provider(node: *mut DeviceNode);
    fn pm_genpd_remove(pd: *mut GenericPmDomain);
    fn of_clk_del_provider(node: *mut DeviceNode);
    fn clk_put(clk: *mut Clk);
    fn mtk_clk_unregister_gates(gates: *const MtkGate, n: usize, data: *mut ClkHwOnecellData);
    fn pm_runtime_put_sync(dev: *mut Device);
    fn of_node_put(node: *mut DeviceNode);
    fn of_genpd_remove_subdomain(parent: *mut OfPhandleArgs, child: *mut OfPhandleArgs);
    fn platform_get_drvdata(pdev: *mut PlatformDevice) -> *mut Mt8173MfgtopData;
    fn regmap_write(map: *mut Regmap, reg: u32, val: u32) -> c_int;
    fn clk_prepare_enable(clk: *mut Clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut Clk);
    fn dev_err_probe(dev: *mut Device, err: c_int, fmt: *const c_char) -> c_int;
}

static MFG_CG_REGS: MtkGateRegs = MtkGateRegs { sta_ofs: 0x0000, clr_ofs: 0x0008, set_ofs: 0x0004 };
// TODO: The block actually has dividers for the core and mem clocks.
static MFG_CLKS: [MtkGate; 4] = [MtkGate { _private: [] }, MtkGate { _private: [] }, MtkGate { _private: [] }, MtkGate { _private: [] }];

#[repr(C)]
struct Mt8173MfgtopData {
    clk_data: *mut ClkHwOnecellData,
    regmap: *mut Regmap,
    genpd: GenericPmDomain,
    parent_pd: OfPhandleArgs,
    child_pd: OfPhandleArgs,
    clk_26m: *mut Clk,
}

const MFG_ACTIVE_POWER_CON0: u32 = 0x24;
const RST_B_DELAY_CNT: u32 = 0xff;
const CLK_EN_DELAY_CNT: u32 = 0xff00;
const CLK_DIS_DELAY_CNT: u32 = 0xff0000;
const FORCE_ABORT: u32 = 1 << 30;
const ACTIVE_PWRCTL_EN: u32 = 1 << 31;
const MFG_ACTIVE_POWER_CON1: u32 = 0x28;
const PWR_ON_S_DELAY_CNT: u32 = 0xff;
const ISO_DELAY_CNT: u32 = 0xff00;
const ISOOFF_DELAY_CNT: u32 = 0xff0000;
const RST_DELAY_CNT: u32 = 0xff000000;

unsafe extern "C" fn clk_mt8173_mfgtop_power_on(domain: *mut GenericPmDomain) -> c_int {
    let data = (domain as *mut u8).sub(core::mem::offset_of!(Mt8173MfgtopData, genpd)) as *mut Mt8173MfgtopData;
    let ret = clk_prepare_enable((*data).clk_26m);
    if ret != 0 { return ret; }
    regmap_write((*data).regmap, MFG_ACTIVE_POWER_CON0, (77) | (61 << 8) | (60 << 16));
    regmap_write((*data).regmap, MFG_ACTIVE_POWER_CON1, (11) | (68 << 8) | (69 << 16) | (77 << 24));
    regmap_write((*data).regmap, 0xe0, 0x7a710184);
    regmap_write((*data).regmap, 0xe4, 0x835f6856);
    regmap_write((*data).regmap, 0xe8, 0x002b0234);
    regmap_write((*data).regmap, 0xec, 0x80000000);
    regmap_write((*data).regmap, 0xa0, 0x08000000);
    0
}

unsafe extern "C" fn clk_mt8173_mfgtop_power_off(domain: *mut GenericPmDomain) -> c_int {
    let data = (domain as *mut u8).sub(core::mem::offset_of!(Mt8173MfgtopData, genpd)) as *mut Mt8173MfgtopData;
    regmap_write((*data).regmap, 0xec, 0);
    clk_disable_unprepare((*data).clk_26m);
    0
}

unsafe extern "C" fn clk_mt8173_mfgtop_probe(pdev: *mut PlatformDevice) -> c_int {
    let dev = &mut (*pdev).dev as *mut Device;
    let node = (*dev).of_node;
    let data = devm_kzalloc(dev, core::mem::size_of::<Mt8173MfgtopData>()) as *mut Mt8173MfgtopData;
    if data.is_null() { return -12; }
    platform_set_drvdata(pdev, data);
    (*data).clk_data = mtk_devm_alloc_clk_data(dev, MFG_CLKS.len());
    if (*data).clk_data.is_null() { return -12; }
    (*data).regmap = device_node_to_regmap(node);
    if (*data).regmap.is_null() { return dev_err_probe(dev, -1, b"Failed to get regmap\0".as_ptr() as *const c_char); }
    (*data).child_pd.np = node;
    (*data).child_pd.args_count = 0;
    let ret = of_parse_phandle_with_args(node, b"power-domains\0".as_ptr() as *const c_char, b"#power-domain-cells\0".as_ptr() as *const c_char, 0, &mut (*data).parent_pd);
    if ret != 0 { return dev_err_probe(dev, ret, b"Failed to parse power domain\0".as_ptr() as *const c_char); }
    devm_pm_runtime_enable(dev);
    let mut ret = pm_runtime_resume_and_get(dev);
    if ret != 0 { dev_err_probe(dev, ret, b"Failed to runtime resume device\0".as_ptr() as *const c_char); of_node_put((*data).parent_pd.np); return ret; }
    ret = mtk_clk_register_gates(dev, node, MFG_CLKS.as_ptr(), MFG_CLKS.len(), (*data).clk_data);
    if ret != 0 { dev_err_probe(dev, ret, b"Failed to register clock gates\0".as_ptr() as *const c_char); pm_runtime_put_sync(dev); of_node_put((*data).parent_pd.np); return ret; }
    let hw = *(*data).clk_data.add(CLK_MFG_26M);
    (*data).clk_26m = clk_hw_get_clk(hw, b"26m\0".as_ptr() as *const c_char);
    if (*data).clk_26m.is_null() { ret = -1; mtk_clk_unregister_gates(MFG_CLKS.as_ptr(), MFG_CLKS.len(), (*data).clk_data); pm_runtime_put_sync(dev); of_node_put((*data).parent_pd.np); return ret; }
    ret = of_clk_add_hw_provider(node, core::ptr::null(), (*data).clk_data);
    if ret != 0 { clk_put((*data).clk_26m); mtk_clk_unregister_gates(MFG_CLKS.as_ptr(), MFG_CLKS.len(), (*data).clk_data); pm_runtime_put_sync(dev); of_node_put((*data).parent_pd.np); return ret; }
    (*data).genpd.name = b"mfg-top\0".as_ptr() as *const c_char;
    (*data).genpd.power_on = Some(clk_mt8173_mfgtop_power_on);
    (*data).genpd.power_off = Some(clk_mt8173_mfgtop_power_off);
    ret = pm_genpd_init(&mut (*data).genpd, core::ptr::null_mut(), true);
    if ret != 0 { of_clk_del_provider(node); clk_put((*data).clk_26m); mtk_clk_unregister_gates(MFG_CLKS.as_ptr(), MFG_CLKS.len(), (*data).clk_data); pm_runtime_put_sync(dev); of_node_put((*data).parent_pd.np); return ret; }
    ret = of_genpd_add_provider_simple(node, &mut (*data).genpd);
    if ret != 0 { pm_genpd_remove(&mut (*data).genpd); of_clk_del_provider(node); clk_put((*data).clk_26m); mtk_clk_unregister_gates(MFG_CLKS.as_ptr(), MFG_CLKS.len(), (*data).clk_data); pm_runtime_put_sync(dev); of_node_put((*data).parent_pd.np); return ret; }
    ret = of_genpd_add_subdomain(&mut (*data).parent_pd, &mut (*data).child_pd);
    if ret != 0 { of_genpd_del_provider(node); pm_genpd_remove(&mut (*data).genpd); of_clk_del_provider(node); clk_put((*data).clk_26m); mtk_clk_unregister_gates(MFG_CLKS.as_ptr(), MFG_CLKS.len(), (*data).clk_data); pm_runtime_put_sync(dev); of_node_put((*data).parent_pd.np); return ret; }
    pm_runtime_put(dev);
    0
}

unsafe extern "C" fn clk_mt8173_mfgtop_remove(pdev: *mut PlatformDevice) {
    let data = platform_get_drvdata(pdev);
    let node = (*pdev).dev.of_node;
    of_genpd_remove_subdomain(&mut (*data).parent_pd, &mut (*data).child_pd);
    of_genpd_del_provider(node);
    pm_genpd_remove(&mut (*data).genpd);
    of_clk_del_provider(node);
    clk_put((*data).clk_26m);
    mtk_clk_unregister_gates(MFG_CLKS.as_ptr(), MFG_CLKS.len(), (*data).clk_data);
    of_node_put((*data).parent_pd.np);
}

extern "C" {
    fn devm_kzalloc(dev: *mut Device, size: usize) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut Mt8173MfgtopData);
}

// Device-tree match table and module_platform_driver(clk_mt8173_mfgtop_drv).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
