// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for NVIDIA Generic Memory Interface
 *
 * Copyright (C) 2016 Host Mobility AB. All rights reserved.
 */

// Kernel headers and symbols used by this implementation are supplied by the
// surrounding platform bindings.

const TEGRA_GMI_CONFIG: u32 = 0x00;
const TEGRA_GMI_CONFIG_GO: u32 = 1 << 31;
const TEGRA_GMI_BUS_WIDTH_32BIT: u32 = 1 << 30;
const TEGRA_GMI_MUX_MODE: u32 = 1 << 28;
const TEGRA_GMI_RDY_BEFORE_DATA: u32 = 1 << 24;
const TEGRA_GMI_RDY_ACTIVE_HIGH: u32 = 1 << 23;
const TEGRA_GMI_ADV_ACTIVE_HIGH: u32 = 1 << 22;
const TEGRA_GMI_OE_ACTIVE_HIGH: u32 = 1 << 21;
const TEGRA_GMI_CS_ACTIVE_HIGH: u32 = 1 << 20;

const TEGRA_GMI_TIMING0: u32 = 0x10;
const TEGRA_GMI_TIMING1: u32 = 0x14;
const TEGRA_GMI_MAX_CHIP_SELECT: u32 = 8;

#[inline]
const fn tegra_gmi_cs_select(x: u32) -> u32 { (x & 0x7) << 4 }
#[inline]
const fn tegra_gmi_muxed_width(x: u32) -> u32 { (x & 0xf) << 12 }
#[inline]
const fn tegra_gmi_hold_width(x: u32) -> u32 { (x & 0xf) << 8 }
#[inline]
const fn tegra_gmi_adv_width(x: u32) -> u32 { (x & 0xf) << 4 }
#[inline]
const fn tegra_gmi_ce_width(x: u32) -> u32 { x & 0xf }
#[inline]
const fn tegra_gmi_we_width(x: u32) -> u32 { (x & 0xff) << 16 }
#[inline]
const fn tegra_gmi_oe_width(x: u32) -> u32 { (x & 0xff) << 8 }
#[inline]
const fn tegra_gmi_wait_width(x: u32) -> u32 { x & 0xff }

#[repr(C)]
struct tegra_gmi {
    dev: *mut device,
    base: *mut core::ffi::c_void,
    clk: *mut clk,
    rst: *mut reset_control,
    snor_config: u32,
    snor_timing0: u32,
    snor_timing1: u32,
}

#[allow(non_camel_case_types)]
type device = core::ffi::c_void;
#[allow(non_camel_case_types)]
type clk = core::ffi::c_void;
#[allow(non_camel_case_types)]
type reset_control = core::ffi::c_void;
#[allow(non_camel_case_types)]
type platform_device = core::ffi::c_void;
#[allow(non_camel_case_types)]
type device_node = core::ffi::c_void;

extern "C" {
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_resume_and_get(dev: *mut device) -> i32;
    fn pm_runtime_disable(dev: *mut device);
    fn reset_control_assert(rst: *mut reset_control);
    fn reset_control_deassert(rst: *mut reset_control);
    fn usleep_range(min: u32, max: u32);
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn pm_runtime_put_sync_suspend(dev: *mut device) -> i32;
    fn pm_runtime_force_suspend(dev: *mut device) -> i32;
    fn of_get_next_available_child(node: *mut device_node, prev: *mut device_node) -> *mut device_node;
    fn of_get_child_count(node: *mut device_node) -> u32;
    fn of_property_read_bool(node: *mut device_node, name: *const u8) -> bool;
    fn of_property_read_u32_array(node: *mut device_node, name: *const u8, values: *mut u32, count: usize) -> i32;
    fn of_property_read_u32(node: *mut device_node, name: *const u8, value: *mut u32) -> i32;
    fn of_node_put(node: *mut device_node);
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
}

unsafe fn tegra_gmi_enable(gmi: *mut tegra_gmi) -> i32 {
    pm_runtime_enable((*gmi).dev);
    let err = pm_runtime_resume_and_get((*gmi).dev);
    if err != 0 {
        pm_runtime_disable((*gmi).dev);
        return err;
    }
    reset_control_assert((*gmi).rst);
    usleep_range(2000, 4000);
    reset_control_deassert((*gmi).rst);
    writel((*gmi).snor_timing0, (*gmi).base.add(TEGRA_GMI_TIMING0 as usize));
    writel((*gmi).snor_timing1, (*gmi).base.add(TEGRA_GMI_TIMING1 as usize));
    (*gmi).snor_config |= TEGRA_GMI_CONFIG_GO;
    writel((*gmi).snor_config, (*gmi).base.add(TEGRA_GMI_CONFIG as usize));
    0
}

unsafe fn tegra_gmi_disable(gmi: *mut tegra_gmi) {
    let mut config = readl((*gmi).base.add(TEGRA_GMI_CONFIG as usize));
    config &= !TEGRA_GMI_CONFIG_GO;
    writel(config, (*gmi).base.add(TEGRA_GMI_CONFIG as usize));
    reset_control_assert((*gmi).rst);
    pm_runtime_put_sync_suspend((*gmi).dev);
    pm_runtime_force_suspend((*gmi).dev);
}

unsafe fn tegra_gmi_parse_dt(gmi: *mut tegra_gmi) -> i32 {
    let child = of_get_next_available_child((*gmi).dev as *mut device_node, core::ptr::null_mut());
    if child.is_null() { return -19; }
    let mut property: u32 = 0;
    let mut ranges = [0u32; 4];
    if of_get_child_count((*gmi).dev as *mut device_node) > 1 { /* only one child device is supported. */ }
    let names = [
        (b"nvidia,snor-data-width-32bit\0".as_ptr(), TEGRA_GMI_BUS_WIDTH_32BIT),
        (b"nvidia,snor-mux-mode\0".as_ptr(), TEGRA_GMI_MUX_MODE),
        (b"nvidia,snor-rdy-active-before-data\0".as_ptr(), TEGRA_GMI_RDY_BEFORE_DATA),
        (b"nvidia,snor-rdy-active-high\0".as_ptr(), TEGRA_GMI_RDY_ACTIVE_HIGH),
        (b"nvidia,snor-adv-active-high\0".as_ptr(), TEGRA_GMI_ADV_ACTIVE_HIGH),
        (b"nvidia,snor-oe-active-high\0".as_ptr(), TEGRA_GMI_OE_ACTIVE_HIGH),
        (b"nvidia,snor-cs-active-high\0".as_ptr(), TEGRA_GMI_CS_ACTIVE_HIGH),
    ];
    for (name, bit) in names { if of_property_read_bool(child, name) { (*gmi).snor_config |= bit; } }
    let mut err = of_property_read_u32_array(child, b"ranges\0".as_ptr(), ranges.as_mut_ptr(), 4);
    if err < 0 {
        if err == -75 { of_node_put(child); return err; }
        err = of_property_read_u32(child, b"reg\0".as_ptr(), &mut property);
        if err < 0 { of_node_put(child); return err; }
    } else { property = ranges[1]; }
    if property >= TEGRA_GMI_MAX_CHIP_SELECT { of_node_put(child); return -22; }
    (*gmi).snor_config |= tegra_gmi_cs_select(property);
    let timing0 = [(b"nvidia,snor-muxed-width\0".as_ptr(), tegra_gmi_muxed_width as fn(u32)->u32, 1), (b"nvidia,snor-hold-width\0".as_ptr(), tegra_gmi_hold_width as fn(u32)->u32, 1), (b"nvidia,snor-adv-width\0".as_ptr(), tegra_gmi_adv_width as fn(u32)->u32, 1), (b"nvidia,snor-ce-width\0".as_ptr(), tegra_gmi_ce_width as fn(u32)->u32, 4)];
    for (name, f, default) in timing0 { if of_property_read_u32(child, name, &mut property) < 0 { property = default; } (*gmi).snor_timing0 |= f(property); }
    let timing1 = [(b"nvidia,snor-we-width\0".as_ptr(), tegra_gmi_we_width as fn(u32)->u32, 1), (b"nvidia,snor-oe-width\0".as_ptr(), tegra_gmi_oe_width as fn(u32)->u32, 1), (b"nvidia,snor-wait-width\0".as_ptr(), tegra_gmi_wait_width as fn(u32)->u32, 3)];
    for (name, f, default) in timing1 { if of_property_read_u32(child, name, &mut property) < 0 { property = default; } (*gmi).snor_timing1 |= f(property); }
    of_node_put(child);
    err
}

unsafe fn tegra_gmi_probe(pdev: *mut platform_device) -> i32 {
    let dev = pdev as *mut device;
    let gmi = 0 as *mut tegra_gmi; // devm_kzalloc(dev, sizeof(*gmi), GFP_KERNEL)
    if gmi.is_null() { return -12; }
    // platform_set_drvdata(pdev, gmi); (*gmi).dev = dev;
    // (*gmi).base = devm_platform_ioremap_resource(pdev, 0);
    // (*gmi).clk = devm_clk_get(dev, "gmi");
    // (*gmi).rst = devm_reset_control_get(dev, "gmi");
    let mut err = 0;
    // err = devm_tegra_core_dev_init_opp_table_common(dev);
    if err != 0 { return err; }
    err = tegra_gmi_parse_dt(gmi);
    if err != 0 { return err; }
    err = tegra_gmi_enable(gmi);
    if err < 0 { return err; }
    // err = of_platform_default_populate((*dev).of_node, NULL, dev);
    if err < 0 { tegra_gmi_disable(gmi); return err; }
    0
}

unsafe fn tegra_gmi_remove(pdev: *mut platform_device) {
    let gmi = 0 as *mut tegra_gmi; // platform_get_drvdata(pdev)
    // of_platform_depopulate((*gmi).dev);
    tegra_gmi_disable(gmi);
}

unsafe fn tegra_gmi_runtime_resume(dev: *mut device) -> i32 {
    let gmi = 0 as *mut tegra_gmi; // dev_get_drvdata(dev)
    let err = clk_prepare_enable((*gmi).clk);
    if err < 0 { return err; }
    0
}

unsafe fn tegra_gmi_runtime_suspend(dev: *mut device) -> i32 {
    let gmi = 0 as *mut tegra_gmi; // dev_get_drvdata(dev)
    clk_disable_unprepare((*gmi).clk);
    0
}

#[repr(C)]
struct of_device_id { compatible: *const u8 }
static TEGRA_GMI_ID_TABLE: &[of_device_id] = &[
    of_device_id { compatible: b"nvidia,tegra20-gmi\0".as_ptr() },
    of_device_id { compatible: b"nvidia,tegra30-gmi\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

// Equivalent of MODULE_DEVICE_TABLE(of, tegra_gmi_id_table) and
// module_platform_driver(tegra_gmi_driver), using the surrounding kernel
// registration bindings.
const TEGRA_GMI_DRIVER_NAME: &[u8] = b"tegra-gmi\0";
const MODULE_AUTHOR: &[u8] = b"Mirza Krak <mirza.krak@gmail.com\0";
const MODULE_DESCRIPTION: &[u8] = b"NVIDIA Tegra GMI Bus Driver\0";
const MODULE_LICENSE: &[u8] = b"GPL v2\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
