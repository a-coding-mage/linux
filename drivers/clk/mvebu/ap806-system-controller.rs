// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Armada AP806 System Controller
 *
 * Copyright (C) 2016 Marvell
 *
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation unit.

const AP806_SAR_REG: u32 = 0x400;
const AP806_SAR_CLKFREQ_MODE_MASK: u32 = 0x1f;
const AP806_CLK_NUM: usize = 6;

extern "C" {
    static mut ap806_clks: [*mut clk; AP806_CLK_NUM];
}

#[repr(C)]
struct clk_onecell_data {
    clks: *mut *mut clk,
    clk_num: u32,
}

extern "C" {
    type clk;
    type device;
    type device_node;
    type platform_device;
    type regmap;

    fn syscon_node_to_regmap(node: *mut device_node) -> *mut regmap;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn of_device_is_compatible(node: *mut device_node, compatible: *const u8) -> bool;
    fn ap_cp_unique_name(dev: *mut device, node: *mut device_node, name: *const u8) -> *const u8;
    fn clk_register_fixed_rate(dev: *mut device, name: *const u8, parent: *const u8,
                               flags: u32, rate: u32) -> *mut clk;
    fn clk_register_fixed_factor(dev: *mut device, name: *const u8, parent: *const u8,
                                 flags: u32, mult: u32, div: u32) -> *mut clk;
    fn clk_unregister_fixed_factor(clk: *mut clk);
    fn clk_unregister_fixed_rate(clk: *mut clk);
    fn of_clk_add_provider(node: *mut device_node, get: unsafe extern "C" fn(), data: *mut clk_onecell_data) -> i32;
    fn of_clk_src_onecell_get();
    fn dev_err(dev: *mut device, fmt: *const u8);
    fn dev_warn(dev: *mut device, fmt: *const u8);
}

static mut AP806_CLKS: [*mut clk; AP806_CLK_NUM] = [core::ptr::null_mut(); AP806_CLK_NUM];
static mut AP806_CLK_DATA: clk_onecell_data = clk_onecell_data {
    clks: unsafe { AP806_CLKS.as_mut_ptr() },
    clk_num: AP806_CLK_NUM as u32,
};

unsafe fn ap806_get_sar_clocks(freq_mode: u32, cpuclk_freq: *mut u32, dclk_freq: *mut u32) -> i32 {
    match freq_mode {
        0x0 => { *cpuclk_freq = 2000; *dclk_freq = 600; }
        0x1 => { *cpuclk_freq = 2000; *dclk_freq = 525; }
        0x6 => { *cpuclk_freq = 1800; *dclk_freq = 600; }
        0x7 => { *cpuclk_freq = 1800; *dclk_freq = 525; }
        0x4 => { *cpuclk_freq = 1600; *dclk_freq = 400; }
        0xB => { *cpuclk_freq = 1600; *dclk_freq = 450; }
        0xD => { *cpuclk_freq = 1600; *dclk_freq = 525; }
        0x1a => { *cpuclk_freq = 1400; *dclk_freq = 400; }
        0x14 => { *cpuclk_freq = 1300; *dclk_freq = 400; }
        0x17 => { *cpuclk_freq = 1300; *dclk_freq = 325; }
        0x19 => { *cpuclk_freq = 1200; *dclk_freq = 400; }
        0x13 => { *cpuclk_freq = 1000; *dclk_freq = 325; }
        0x1d => { *cpuclk_freq = 1000; *dclk_freq = 400; }
        0x1c => { *cpuclk_freq = 800; *dclk_freq = 400; }
        0x1b => { *cpuclk_freq = 600; *dclk_freq = 400; }
        _ => return -22,
    }
    0
}

unsafe fn ap807_get_sar_clocks(freq_mode: u32, cpuclk_freq: *mut u32, dclk_freq: *mut u32) -> i32 {
    match freq_mode {
        0x0 => { *cpuclk_freq = 2000; *dclk_freq = 1200; }
        0x6 => { *cpuclk_freq = 2200; *dclk_freq = 1200; }
        0xD => { *cpuclk_freq = 1600; *dclk_freq = 1200; }
        _ => return -22,
    }
    0
}

unsafe fn ap806_syscon_common_probe(pdev: *mut platform_device, syscon_node: *mut device_node) -> i32 {
    let dev: *mut device = core::ptr::null_mut();
    let np: *mut device_node = core::ptr::null_mut();
    let regmap = syscon_node_to_regmap(syscon_node);
    if regmap.is_null() { dev_err(dev, b"cannot get regmap\0".as_ptr()); return -22; }
    let mut reg = 0u32;
    let mut ret = regmap_read(regmap, AP806_SAR_REG, &mut reg);
    if ret != 0 { dev_err(dev, b"cannot read from regmap\0".as_ptr()); return ret; }
    let freq_mode = reg & AP806_SAR_CLKFREQ_MODE_MASK;
    let mut cpuclk_freq = 0u32;
    let mut dclk_freq = 0u32;
    ret = if of_device_is_compatible(np, b"marvell,ap806-clock\0".as_ptr()) {
        ap806_get_sar_clocks(freq_mode, &mut cpuclk_freq, &mut dclk_freq)
    } else if of_device_is_compatible(np, b"marvell,ap807-clock\0".as_ptr()) {
        ap807_get_sar_clocks(freq_mode, &mut cpuclk_freq, &mut dclk_freq)
    } else { dev_err(dev, b"compatible not supported\0".as_ptr()); return -22; };
    if ret != 0 { dev_err(dev, b"invalid Sample at Reset value\0".as_ptr()); return ret; }
    cpuclk_freq *= 1000 * 1000; dclk_freq *= 1000 * 1000;
    let name = ap_cp_unique_name(dev, syscon_node, b"pll-cluster-0\0".as_ptr());
    AP806_CLKS[0] = clk_register_fixed_rate(dev, name, core::ptr::null(), 0, cpuclk_freq);
    if AP806_CLKS[0].is_null() { return -22; }
    let name = ap_cp_unique_name(dev, syscon_node, b"pll-cluster-1\0".as_ptr());
    AP806_CLKS[1] = clk_register_fixed_rate(dev, name, core::ptr::null(), 0, cpuclk_freq);
    if AP806_CLKS[1].is_null() { clk_unregister_fixed_rate(AP806_CLKS[0]); return -22; }
    let fixedclk_name = ap_cp_unique_name(dev, syscon_node, b"fixed\0".as_ptr());
    AP806_CLKS[2] = clk_register_fixed_rate(dev, fixedclk_name, core::ptr::null(), 0, 1200 * 1000 * 1000);
    if AP806_CLKS[2].is_null() { clk_unregister_fixed_rate(AP806_CLKS[1]); clk_unregister_fixed_rate(AP806_CLKS[0]); return -22; }
    let name = ap_cp_unique_name(dev, syscon_node, b"mss\0".as_ptr());
    AP806_CLKS[3] = clk_register_fixed_factor(core::ptr::null_mut(), name, fixedclk_name, 0, 1, 6);
    let name = ap_cp_unique_name(dev, syscon_node, b"sdio\0".as_ptr());
    AP806_CLKS[4] = clk_register_fixed_factor(core::ptr::null_mut(), name, fixedclk_name, 0, 1, 3);
    let name = ap_cp_unique_name(dev, syscon_node, b"ap-dclk\0".as_ptr());
    AP806_CLKS[5] = clk_register_fixed_rate(dev, name, core::ptr::null(), 0, dclk_freq);
    ret = of_clk_add_provider(np, of_clk_src_onecell_get, &mut AP806_CLK_DATA);
    if ret != 0 { clk_unregister_fixed_factor(AP806_CLKS[5]); clk_unregister_fixed_factor(AP806_CLKS[4]); clk_unregister_fixed_factor(AP806_CLKS[3]); clk_unregister_fixed_rate(AP806_CLKS[2]); clk_unregister_fixed_rate(AP806_CLKS[1]); clk_unregister_fixed_rate(AP806_CLKS[0]); }
    ret
}

// Legacy and platform-driver registration declarations are supplied by the kernel integration.
unsafe fn ap806_syscon_legacy_probe(pdev: *mut platform_device) -> i32 { ap806_syscon_common_probe(pdev, core::ptr::null_mut()) }
unsafe fn ap806_clock_probe(pdev: *mut platform_device) -> i32 { ap806_syscon_common_probe(pdev, core::ptr::null_mut()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
