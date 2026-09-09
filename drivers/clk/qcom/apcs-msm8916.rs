// SPDX-License-Identifier: GPL-2.0
/*
 * Qualcomm APCS clock controller driver
 *
 * Copyright (c) 2017, Linaro Limited
 * Author: Georgi Djakov <georgi.djakov@linaro.org>
 */

// Linux kernel and local clock/regmap definitions are supplied by other files.

extern "C" {
    fn mux_div_set_src_div(md: *mut clk_regmap_mux_div, src: u32, div: u32) -> i32;
    fn notifier_from_errno(err: i32) -> i32;
    fn dev_get_regmap(dev: *mut device, name: *const i8) -> *mut regmap;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_kasprintf(dev: *mut device, flags: u32, fmt: *const i8, ...) -> *mut i8;
    fn strchrnul(s: *const i8, c: i32) -> *const i8;
    fn devm_clk_get(dev: *mut device, name: *const i8) -> *mut clk;
    fn clk_notifier_register(clk: *mut clk, nb: *mut notifier_block) -> i32;
    fn clk_notifier_unregister(clk: *mut clk, nb: *mut notifier_block);
    fn devm_clk_register_regmap(dev: *mut device, clkr: *mut clk_regmap) -> i32;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: *const core::ffi::c_void,
                                   hw: *mut clk_hw) -> i32;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut core::ffi::c_void;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
}

#[repr(C)]
struct clk_parent_data {
    fw_name: *const i8,
    name: *const i8,
}

#[repr(C)]
struct a53cc_init_data {
    name: *mut i8,
    parent_data: *const clk_parent_data,
    num_parents: usize,
    ops: *const core::ffi::c_void,
    flags: u32,
}

#[repr(C)]
struct clk_regmap_mux_div {
    clkr: clk_regmap,
    reg_offset: u32,
    hid_width: u32,
    hid_shift: u32,
    src_width: u32,
    src_shift: u32,
    parent_map: *const u32,
    pclk: *mut clk,
    clk_nb: notifier_block,
}

#[repr(C)]
struct clk_regmap { hw: clk_hw, regmap: *mut regmap, init: *mut a53cc_init_data }
#[repr(C)] struct clk_hw;
#[repr(C)] struct clk;
#[repr(C)] struct regmap;
#[repr(C)] struct device_node { full_name: *const i8 }
#[repr(C)] struct regmap_placeholder;
#[repr(C)] struct device { parent: *mut device, of_node: *mut device_node }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct notifier_block { notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut core::ffi::c_void) -> i32> }

const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const EPROBE_DEFER: i32 = 517;
const PRE_RATE_CHANGE: usize = 0x1;
const CLK_IS_CRITICAL: u32 = 1 << 11;
const CLK_SET_RATE_PARENT: u32 = 1 << 5;
const GFP_KERNEL: u32 = 0xCC0;

static gpll0_a53cc_map: [u32; 2] = [4, 5];
static pdata: [clk_parent_data; 2] = [
    clk_parent_data { fw_name: b"aux\0".as_ptr() as *const i8, name: b"gpll0_vote\0".as_ptr() as *const i8 },
    clk_parent_data { fw_name: b"pll\0".as_ptr() as *const i8, name: b"a53pll\0".as_ptr() as *const i8 },
];

/* We use the notifier function for switching to a temporary safe configuration
 * (mux and divider), while the A53 PLL is reconfigured.
 */
unsafe extern "C" fn a53cc_notifier_cb(nb: *mut notifier_block, event: usize,
                                        _data: *mut core::ffi::c_void) -> i32 {
    let mut ret = 0;
    let md = (nb as *mut u8).sub(core::mem::offset_of!(clk_regmap_mux_div, clk_nb))
        as *mut clk_regmap_mux_div;
    if event == PRE_RATE_CHANGE {
        // set the mux and divider to safe frequency (400mhz)
        ret = mux_div_set_src_div(md, 4, 3);
    }
    notifier_from_errno(ret)
}

unsafe extern "C" fn qcom_apcs_msm8916_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let parent = (*dev).parent;
    let np = (*parent).of_node;
    let mut a53cc: *mut clk_regmap_mux_div;
    let regmap: *mut regmap;
    let mut init = core::mem::zeroed::<a53cc_init_data>();
    let mut ret = -ENODEV;

    regmap = dev_get_regmap(parent, core::ptr::null());
    if regmap.is_null() { dev_err(dev, b"failed to get regmap: %d\n\0".as_ptr() as *const i8, ret); return ret; }
    a53cc = devm_kzalloc(dev, core::mem::size_of::<clk_regmap_mux_div>(), GFP_KERNEL) as *mut clk_regmap_mux_div;
    if a53cc.is_null() { return -ENOMEM; }
    init.name = devm_kasprintf(dev, GFP_KERNEL, b"a53mux%s\0".as_ptr() as *const i8, strchrnul((*np).full_name, b'@' as i32));
    if init.name.is_null() { return -ENOMEM; }
    init.parent_data = pdata.as_ptr(); init.num_parents = pdata.len();
    init.ops = core::ptr::null(); init.flags = CLK_IS_CRITICAL | CLK_SET_RATE_PARENT;
    (*a53cc).clkr.init = &mut init; (*a53cc).clkr.regmap = regmap;
    (*a53cc).reg_offset = 0x50; (*a53cc).hid_width = 5; (*a53cc).hid_shift = 0;
    (*a53cc).src_width = 3; (*a53cc).src_shift = 8; (*a53cc).parent_map = gpll0_a53cc_map.as_ptr();
    (*a53cc).pclk = devm_clk_get(parent, core::ptr::null());
    if (*a53cc).pclk.is_null() { return ret; }
    (*a53cc).clk_nb.notifier_call = Some(a53cc_notifier_cb);
    ret = clk_notifier_register((*a53cc).pclk, &mut (*a53cc).clk_nb);
    if ret != 0 { return ret; }
    ret = devm_clk_register_regmap(dev, &mut (*a53cc).clkr);
    if ret != 0 { clk_notifier_unregister((*a53cc).pclk, &mut (*a53cc).clk_nb); return ret; }
    ret = devm_of_clk_add_hw_provider(dev, core::ptr::null(), &mut (*a53cc).clkr.hw);
    if ret != 0 { clk_notifier_unregister((*a53cc).pclk, &mut (*a53cc).clk_nb); return ret; }
    platform_set_drvdata(pdev, a53cc as *mut core::ffi::c_void);
    0
}

unsafe extern "C" fn qcom_apcs_msm8916_clk_remove(pdev: *mut platform_device) {
    let a53cc = platform_get_drvdata(pdev) as *mut clk_regmap_mux_div;
    clk_notifier_unregister((*a53cc).pclk, &mut (*a53cc).clk_nb);
}

// module_platform_driver(qcom_apcs_msm8916_clk_driver);
// MODULE_AUTHOR("Georgi Djakov <georgi.djakov@linaro.org>");
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("Qualcomm MSM8916 APCS clock driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
