// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Orion SoC clocks
 *
 * Copyright (C) 2014 Thomas Petazzoni
 *
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

use core::ffi::c_void;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct coreclk_ratio {
    pub id: i32,
    pub name: *const u8,
}

#[repr(C)]
pub struct coreclk_soc_desc {
    pub get_tclk_freq: unsafe extern "C" fn(*mut c_void) -> u32,
    pub get_cpu_freq: unsafe extern "C" fn(*mut c_void) -> u32,
    pub get_clk_ratio: unsafe extern "C" fn(*mut c_void, i32, *mut i32, *mut i32),
    pub ratios: *const coreclk_ratio,
    pub num_ratios: usize,
}

unsafe extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn mvebu_coreclk_setup(np: *mut device_node, desc: *const coreclk_soc_desc);
}

static ORION_CORECLK_RATIOS: [coreclk_ratio; 1] = [coreclk_ratio {
    id: 0,
    name: b"ddrclk\0".as_ptr(),
}];

const SAR_MV88F5181_TCLK_FREQ: u32 = 8;
const SAR_MV88F5181_TCLK_FREQ_MASK: u32 = 0x3;

unsafe extern "C" fn mv88f5181_get_tclk_freq(sar: *mut c_void) -> u32 {
    let opt = (readl(sar) >> SAR_MV88F5181_TCLK_FREQ) & SAR_MV88F5181_TCLK_FREQ_MASK;
    if opt == 0 { 133333333 } else if opt == 1 { 150000000 } else if opt == 2 { 166666667 } else { 0 }
}

const SAR_MV88F5181_CPU_FREQ: u32 = 4;
const SAR_MV88F5181_CPU_FREQ_MASK: u32 = 0xf;

unsafe extern "C" fn mv88f5181_get_cpu_freq(sar: *mut c_void) -> u32 {
    let opt = (readl(sar) >> SAR_MV88F5181_CPU_FREQ) & SAR_MV88F5181_CPU_FREQ_MASK;
    if opt == 0 { 333333333 } else if opt == 1 || opt == 2 { 400000000 } else if opt == 3 { 500000000 } else { 0 }
}

unsafe extern "C" fn mv88f5181_get_clk_ratio(sar: *mut c_void, _id: i32, mult: *mut i32, div: *mut i32) {
    let opt = (readl(sar) >> SAR_MV88F5181_CPU_FREQ) & SAR_MV88F5181_CPU_FREQ_MASK;
    if opt == 0 || opt == 1 { *mult = 1; *div = 2; } else if opt == 2 || opt == 3 { *mult = 1; *div = 3; } else { *mult = 0; *div = 1; }
}

static MV88F5181_CORECLKS: coreclk_soc_desc = coreclk_soc_desc { get_tclk_freq: mv88f5181_get_tclk_freq, get_cpu_freq: mv88f5181_get_cpu_freq, get_clk_ratio: mv88f5181_get_clk_ratio, ratios: ORION_CORECLK_RATIOS.as_ptr(), num_ratios: ORION_CORECLK_RATIOS.len() };

unsafe extern "C" fn mv88f5181_clk_init(np: *mut device_node) { mvebu_coreclk_setup(np, &MV88F5181_CORECLKS); }
// CLK_OF_DECLARE(mv88f5181_clk, "marvell,mv88f5181-core-clock", mv88f5181_clk_init);

const SAR_MV88F5182_TCLK_FREQ: u32 = 8;
const SAR_MV88F5182_TCLK_FREQ_MASK: u32 = 0x3;
unsafe extern "C" fn mv88f5182_get_tclk_freq(sar: *mut c_void) -> u32 { let opt = (readl(sar) >> SAR_MV88F5182_TCLK_FREQ) & SAR_MV88F5182_TCLK_FREQ_MASK; if opt == 1 { 150000000 } else if opt == 2 { 166666667 } else { 0 } }
const SAR_MV88F5182_CPU_FREQ: u32 = 4;
const SAR_MV88F5182_CPU_FREQ_MASK: u32 = 0xf;
unsafe extern "C" fn mv88f5182_get_cpu_freq(sar: *mut c_void) -> u32 { let opt = (readl(sar) >> SAR_MV88F5182_CPU_FREQ) & SAR_MV88F5182_CPU_FREQ_MASK; if opt == 0 { 333333333 } else if opt == 1 || opt == 2 { 400000000 } else if opt == 3 { 500000000 } else { 0 } }
unsafe extern "C" fn mv88f5182_get_clk_ratio(sar: *mut c_void, _id: i32, mult: *mut i32, div: *mut i32) { let opt = (readl(sar) >> SAR_MV88F5182_CPU_FREQ) & SAR_MV88F5182_CPU_FREQ_MASK; if opt == 0 || opt == 1 { *mult = 1; *div = 2; } else if opt == 2 || opt == 3 { *mult = 1; *div = 3; } else { *mult = 0; *div = 1; } }
static MV88F5182_CORECLKS: coreclk_soc_desc = coreclk_soc_desc { get_tclk_freq: mv88f5182_get_tclk_freq, get_cpu_freq: mv88f5182_get_cpu_freq, get_clk_ratio: mv88f5182_get_clk_ratio, ratios: ORION_CORECLK_RATIOS.as_ptr(), num_ratios: ORION_CORECLK_RATIOS.len() };
unsafe extern "C" fn mv88f5182_clk_init(np: *mut device_node) { mvebu_coreclk_setup(np, &MV88F5182_CORECLKS); }
// CLK_OF_DECLARE(mv88f5182_clk, "marvell,mv88f5182-core-clock", mv88f5182_clk_init);

unsafe extern "C" fn mv88f5281_get_tclk_freq(_sar: *mut c_void) -> u32 { 166666667 }
const SAR_MV88F5281_CPU_FREQ: u32 = 4;
const SAR_MV88F5281_CPU_FREQ_MASK: u32 = 0xf;
unsafe extern "C" fn mv88f5281_get_cpu_freq(sar: *mut c_void) -> u32 { let opt = (readl(sar) >> SAR_MV88F5281_CPU_FREQ) & SAR_MV88F5281_CPU_FREQ_MASK; if opt == 1 || opt == 2 { 400000000 } else if opt == 3 { 500000000 } else { 0 } }
unsafe extern "C" fn mv88f5281_get_clk_ratio(sar: *mut c_void, _id: i32, mult: *mut i32, div: *mut i32) { let opt = (readl(sar) >> SAR_MV88F5281_CPU_FREQ) & SAR_MV88F5281_CPU_FREQ_MASK; if opt == 1 { *mult = 1; *div = 2; } else if opt == 2 || opt == 3 { *mult = 1; *div = 3; } else { *mult = 0; *div = 1; } }
static MV88F5281_CORECLKS: coreclk_soc_desc = coreclk_soc_desc { get_tclk_freq: mv88f5281_get_tclk_freq, get_cpu_freq: mv88f5281_get_cpu_freq, get_clk_ratio: mv88f5281_get_clk_ratio, ratios: ORION_CORECLK_RATIOS.as_ptr(), num_ratios: ORION_CORECLK_RATIOS.len() };
unsafe extern "C" fn mv88f5281_clk_init(np: *mut device_node) { mvebu_coreclk_setup(np, &MV88F5281_CORECLKS); }
// CLK_OF_DECLARE(mv88f5281_clk, "marvell,mv88f5281-core-clock", mv88f5281_clk_init);

const SAR_MV88F6183_TCLK_FREQ: u32 = 9;
const SAR_MV88F6183_TCLK_FREQ_MASK: u32 = 0x1;
unsafe extern "C" fn mv88f6183_get_tclk_freq(sar: *mut c_void) -> u32 { let opt = (readl(sar) >> SAR_MV88F6183_TCLK_FREQ) & SAR_MV88F6183_TCLK_FREQ_MASK; if opt == 0 { 133333333 } else if opt == 1 { 166666667 } else { 0 } }
const SAR_MV88F6183_CPU_FREQ: u32 = 1;
const SAR_MV88F6183_CPU_FREQ_MASK: u32 = 0x3f;
unsafe extern "C" fn mv88f6183_get_cpu_freq(sar: *mut c_void) -> u32 { let opt = (readl(sar) >> SAR_MV88F6183_CPU_FREQ) & SAR_MV88F6183_CPU_FREQ_MASK; if opt == 9 { 333333333 } else if opt == 17 { 400000000 } else { 0 } }
unsafe extern "C" fn mv88f6183_get_clk_ratio(sar: *mut c_void, _id: i32, mult: *mut i32, div: *mut i32) { let opt = (readl(sar) >> SAR_MV88F6183_CPU_FREQ) & SAR_MV88F6183_CPU_FREQ_MASK; if opt == 9 || opt == 17 { *mult = 1; *div = 2; } else { *mult = 0; *div = 1; } }
static MV88F6183_CORECLKS: coreclk_soc_desc = coreclk_soc_desc { get_tclk_freq: mv88f6183_get_tclk_freq, get_cpu_freq: mv88f6183_get_cpu_freq, get_clk_ratio: mv88f6183_get_clk_ratio, ratios: ORION_CORECLK_RATIOS.as_ptr(), num_ratios: ORION_CORECLK_RATIOS.len() };
unsafe extern "C" fn mv88f6183_clk_init(np: *mut device_node) { mvebu_coreclk_setup(np, &MV88F6183_CORECLKS); }
// CLK_OF_DECLARE(mv88f6183_clk, "marvell,mv88f6183-core-clock", mv88f6183_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
