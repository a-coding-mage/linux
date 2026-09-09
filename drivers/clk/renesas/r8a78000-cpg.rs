// SPDX-License-Identifier: GPL-2.0
/*
 * R-Car X5H Clock Pulse Generator
 *
 * Copyright (C) 2026 Glider bv
 */

// Linux kernel dependencies and DT binding declarations are supplied externally.

#[repr(C)]
pub struct clk_map {
    pub dt_id: i32, // DT binding clock ID or -1 sentinel
    pub fw_id: u32, // FIXED_CLK() ID
}

#[repr(u32)]
enum fixed_clk {
    FIXED_CLK_66M,
    FIXED_CLK_266M,
    NUM_FIXED_CLKS,
}

static fixed_clk_rates: [usize; NUM_FIXED_CLKS as usize] = [66666000, 266660000];

// Supplied by <dt-bindings/clock/renesas,r8a78000-cpg.h>.
extern "C" {
    static R8A78000_CPG_SGASYNCD4_PERW_BUS: i32;
    static R8A78000_CPG_SGASYNCD16_PERW_BUS: i32;
}

#[repr(C)]
pub struct r8a78000_cpg_priv {
    pub dev: *mut device,
    pub map: *const clk_map,
    pub fixed_hws: [*mut clk_hw; NUM_FIXED_CLKS as usize],
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct clk_hw;
#[repr(C)]
pub struct of_phandle_args {
    pub args_count: u32,
    pub args: *const u32,
}

unsafe extern "C" {
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const u8;
    fn clk_hw_get_rate(hw: *mut clk_hw) -> usize;
    fn of_device_get_match_data(dev: *mut device) -> *const clk_map;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_kasprintf(dev: *mut device, flags: u32, fmt: *const u8, ...) -> *const u8;
    fn devm_clk_hw_register_fixed_rate(
        dev: *mut device, name: *const u8, parent: *const u8, flags: u32, rate: usize,
    ) -> *mut clk_hw;
    fn devm_of_clk_add_hw_provider(
        dev: *mut device,
        get: unsafe extern "C" fn(*mut of_phandle_args, *mut core::ffi::c_void) -> *mut clk_hw,
        data: *mut core::ffi::c_void,
    ) -> i32;
}

unsafe fn clk_map_find(mut map: *const clk_map, id: u32) -> *const clk_map {
    if map.is_null() {
        return core::ptr::null();
    }
    while (*map).dt_id >= 0 {
        if (*map).dt_id as u32 == id {
            return map;
        }
        map = map.add(1);
    }
    core::ptr::null()
}

unsafe extern "C" fn r8a78000_clk_get(
    spec: *mut of_phandle_args,
    data: *mut core::ffi::c_void,
) -> *mut clk_hw {
    let priv_ = &mut *(data as *mut r8a78000_cpg_priv);
    let dev = priv_.dev;
    if (*spec).args_count != 1 {
        return (-22isize) as *mut clk_hw;
    }
    let id = *(*spec).args;
    let map = clk_map_find(priv_.map, id);
    if map.is_null() {
        dev_err(dev, c"Unknown clock %u\n".as_ptr().cast(), id);
        return (-2isize) as *mut clk_hw;
    }
    dev_dbg(dev, c"Mapping DT clock %u to fixed clock %u\n".as_ptr().cast(), id, (*map).fw_id);
    let hw = priv_.fixed_hws[(*map).fw_id as usize];
    dev_dbg(dev, c"clock %u is %s at %lu Hz\n".as_ptr().cast(), id, clk_hw_get_name(hw), clk_hw_get_rate(hw));
    hw
}

unsafe fn register_fixed_clks(priv_: *mut r8a78000_cpg_priv) -> i32 {
    for i in 0..fixed_clk_rates.len() {
        let rate = fixed_clk_rates[i];
        let hw = devm_clk_hw_register_fixed_rate((*priv_).dev, core::ptr::null(), core::ptr::null(), 0, rate);
        if hw.is_null() { return -12; }
        (*priv_).fixed_hws[i] = hw;
    }
    0
}

unsafe extern "C" fn r8a78000_cpg_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let map = of_device_get_match_data(dev);
    if map.is_null() { return -19; }
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<r8a78000_cpg_priv>(), 0) as *mut r8a78000_cpg_priv;
    if priv_.is_null() { return -12; }
    (*priv_).dev = dev;
    (*priv_).map = map;
    let ret = register_fixed_clks(priv_);
    if ret != 0 { return ret; }
    devm_of_clk_add_hw_provider(dev, r8a78000_clk_get, priv_.cast())
}

#[repr(C)]
pub struct platform_device { pub dev: *mut device }

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
    pub data: *const core::ffi::c_void,
}

static r8a78000_cpg_default: [clk_map; 3] = [
    clk_map { dt_id: 0, fw_id: FIXED_CLK_266M as u32 },
    clk_map { dt_id: 1, fw_id: FIXED_CLK_66M as u32 },
    clk_map { dt_id: -1, fw_id: 0 },
];

static r8a78000_cpg_match: [of_device_id; 2] = [
    of_device_id { compatible: c"renesas,r8a78000-cpg".as_ptr().cast(), data: r8a78000_cpg_default.as_ptr().cast() },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[repr(C)]
struct platform_driver {
    probe: unsafe extern "C" fn(*mut platform_device) -> i32,
    name: *const u8,
    of_match_table: *const of_device_id,
    suppress_bind_attrs: bool,
}

static mut r8a78000_cpg_driver: platform_driver = platform_driver {
    probe: r8a78000_cpg_probe,
    name: c"r8a78000-cpg".as_ptr().cast(),
    of_match_table: r8a78000_cpg_match.as_ptr(),
    suppress_bind_attrs: true,
};

// builtin_platform_driver(r8a78000_cpg_driver)
// MODULE_DESCRIPTION("R-Car X5H CPG Driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
