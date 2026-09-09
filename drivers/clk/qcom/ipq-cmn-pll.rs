// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

/* CMN PLL block implementation translated from the Linux C source. */

// External kernel headers and device-tree bindings are supplied by dependencies.

const CMN_PLL_REFCLK_SRC_SELECTION: u32 = 0x28;
const CMN_PLL_REFCLK_SRC_DIV: u32 = 0x3 << 8;
const CMN_PLL_LOCKED: u32 = 0x64;
const CMN_PLL_CLKS_LOCKED: u32 = 1 << 8;
const CMN_PLL_POWER_ON_AND_RESET: u32 = 0x780;
const CMN_ANA_EN_SW_RSTN: u32 = 1 << 6;
const CMN_PLL_REFCLK_CONFIG: u32 = 0x784;
const CMN_PLL_REFCLK_EXTERNAL: u32 = 1 << 9;
const CMN_PLL_REFCLK_DIV: u32 = 0x1f << 4;
const CMN_PLL_REFCLK_INDEX: u32 = 0xf;
const CMN_PLL_CTRL: u32 = 0x78c;
const CMN_PLL_CTRL_LOCK_DETECT_EN: u32 = 1 << 15;
const CMN_PLL_DIVIDER_CTRL: u32 = 0x794;
const CMN_PLL_DIVIDER_CTRL_FACTOR: u32 = 0x3ff;

#[repr(C)]
pub struct CmnPllFixedOutputClk {
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub rate: usize,
}

#[repr(C)]
pub struct ClkCmnPll {
    pub regmap: *mut Regmap,
    pub hw: ClkHw,
}

#[repr(C)]
pub struct RegmapConfig {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub max_register: u32,
}

#[repr(C)]
pub struct Regmap { _private: [u8; 0] }
#[repr(C)]
pub struct ClkHw { pub init: *const ClkInitData }
#[repr(C)]
pub struct ClkInitData {
    pub name: *const core::ffi::c_char,
    pub parent_data: *const ClkParentData,
    pub num_parents: u32,
    pub ops: *const ClkOps,
}
#[repr(C)]
pub struct ClkParentData { pub index: u32 }
#[repr(C)]
pub struct ClkRateRequest { pub best_parent_rate: usize }
#[repr(C)]
pub struct ClkOps {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize) -> usize>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize, usize) -> i32>,
}
#[repr(C)]
pub struct Device { _private: [u8; 0] }
#[repr(C)]
pub struct PlatformDevice { pub dev: Device }

const fn output(id: u32, name: &'static [u8], rate: usize) -> CmnPllFixedOutputClk {
    CmnPllFixedOutputClk { id, name: name.as_ptr() as *const _, rate }
}

static IPQ_CMN_PLL_REGMAP_CONFIG: RegmapConfig = RegmapConfig {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x7fc,
};

static IPQ5018_OUTPUT_CLKS: &[CmnPllFixedOutputClk] = &[
    output(IPQ5018_XO_24MHZ_CLK, b"xo-24mhz\0", 24000000),
    output(IPQ5018_SLEEP_32KHZ_CLK, b"sleep-32khz\0", 32000),
    output(IPQ5018_ETH_50MHZ_CLK, b"eth-50mhz\0", 50000000),
    CmnPllFixedOutputClk { id: 0, name: core::ptr::null(), rate: 0 },
];
static IPQ6018_OUTPUT_CLKS: &[CmnPllFixedOutputClk] = &[
    output(IPQ6018_BIAS_PLL_CC_CLK, b"bias_pll_cc_clk\0", 300000000),
    output(IPQ6018_BIAS_PLL_NSS_NOC_CLK, b"bias_pll_nss_noc_clk\0", 416500000),
    CmnPllFixedOutputClk { id: 0, name: core::ptr::null(), rate: 0 },
];
static IPQ8074_OUTPUT_CLKS: &[CmnPllFixedOutputClk] = &[
    output(IPQ8074_BIAS_PLL_CC_CLK, b"bias_pll_cc_clk\0", 300000000),
    output(IPQ8074_BIAS_PLL_NSS_NOC_CLK, b"bias_pll_nss_noc_clk\0", 416500000),
    CmnPllFixedOutputClk { id: 0, name: core::ptr::null(), rate: 0 },
];
static IPQ5332_OUTPUT_CLKS: &[CmnPllFixedOutputClk] = &[
    output(IPQ5332_XO_24MHZ_CLK, b"xo-24mhz\0", 24000000), output(IPQ5332_SLEEP_32KHZ_CLK, b"sleep-32khz\0", 32000),
    output(IPQ5332_PCS_31P25MHZ_CLK, b"pcs-31p25mhz\0", 31250000), output(IPQ5332_NSS_300MHZ_CLK, b"nss-300mhz\0", 300000000),
    output(IPQ5332_PPE_200MHZ_CLK, b"ppe-200mhz\0", 200000000), output(IPQ5332_ETH_50MHZ_CLK, b"eth-50mhz\0", 50000000),
    CmnPllFixedOutputClk { id: 0, name: core::ptr::null(), rate: 0 },
];
static IPQ5424_OUTPUT_CLKS: &[CmnPllFixedOutputClk] = &[
    output(IPQ5424_XO_24MHZ_CLK, b"xo-24mhz\0", 24000000), output(IPQ5424_SLEEP_32KHZ_CLK, b"sleep-32khz\0", 32000),
    output(IPQ5424_PCS_31P25MHZ_CLK, b"pcs-31p25mhz\0", 31250000), output(IPQ5424_NSS_300MHZ_CLK, b"nss-300mhz\0", 300000000),
    output(IPQ5424_PPE_375MHZ_CLK, b"ppe-375mhz\0", 375000000), output(IPQ5424_ETH0_50MHZ_CLK, b"eth0-50mhz\0", 50000000),
    output(IPQ5424_ETH1_50MHZ_CLK, b"eth1-50mhz\0", 50000000), output(IPQ5424_ETH2_50MHZ_CLK, b"eth2-50mhz\0", 50000000),
    output(IPQ5424_ETH_25MHZ_CLK, b"eth-25mhz\0", 25000000), CmnPllFixedOutputClk { id: 0, name: core::ptr::null(), rate: 0 },
];
static IPQ9574_OUTPUT_CLKS: &[CmnPllFixedOutputClk] = &[
    output(XO_24MHZ_CLK, b"xo-24mhz\0", 24000000), output(SLEEP_32KHZ_CLK, b"sleep-32khz\0", 32000),
    output(PCS_31P25MHZ_CLK, b"pcs-31p25mhz\0", 31250000), output(NSS_1200MHZ_CLK, b"nss-1200mhz\0", 1200000000),
    output(PPE_353MHZ_CLK, b"ppe-353mhz\0", 353000000), output(ETH0_50MHZ_CLK, b"eth0-50mhz\0", 50000000),
    output(ETH1_50MHZ_CLK, b"eth1-50mhz\0", 50000000), output(ETH2_50MHZ_CLK, b"eth2-50mhz\0", 50000000),
    output(ETH_25MHZ_CLK, b"eth-25mhz\0", 25000000), CmnPllFixedOutputClk { id: 0, name: core::ptr::null(), rate: 0 },
];

unsafe fn ipq_cmn_pll_find_freq_index(parent_rate: usize) -> i32 {
    match parent_rate { 25000000 => 3, 31250000 => 4, 40000000 => 6, 48000000 | 96000000 => 7, 50000000 => 8, _ => -22 }
}

unsafe fn clk_cmn_pll_recalc_rate(hw: *mut ClkHw, parent_rate: usize) -> usize {
    let cmn_pll = (hw as *mut u8).sub(core::mem::offset_of!(ClkCmnPll, hw)) as *mut ClkCmnPll;
    let mut val = 0u32;
    regmap_read((*cmn_pll).regmap, CMN_PLL_DIVIDER_CTRL, &mut val);
    let mut factor = val & CMN_PLL_DIVIDER_CTRL_FACTOR; if factor == 0 { factor = 1; }
    regmap_read((*cmn_pll).regmap, CMN_PLL_REFCLK_CONFIG, &mut val);
    let mut ref_div = (val & CMN_PLL_REFCLK_DIV) >> 4; if ref_div == 0 { ref_div = 1; }
    ((parent_rate as u128 * 2 * factor as u128) / ref_div as u128) as usize
}
unsafe fn clk_cmn_pll_determine_rate(_hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    if ipq_cmn_pll_find_freq_index((*req).best_parent_rate) < 0 { -22 } else { 0 }
}
unsafe fn clk_cmn_pll_set_rate(hw: *mut ClkHw, _rate: usize, parent_rate: usize) -> i32 {
    let cmn_pll = (hw as *mut u8).sub(core::mem::offset_of!(ClkCmnPll, hw)) as *mut ClkCmnPll;
    let index = ipq_cmn_pll_find_freq_index(parent_rate); if index < 0 { return index; }
    let map = (*cmn_pll).regmap;
    let mut ret = regmap_update_bits(map, CMN_PLL_REFCLK_CONFIG, CMN_PLL_REFCLK_INDEX, index as u32); if ret != 0 { return ret; }
    if parent_rate == 96000000 {
        ret = regmap_update_bits(map, CMN_PLL_REFCLK_CONFIG, CMN_PLL_REFCLK_DIV, 2 << 4); if ret != 0 { return ret; }
        ret = regmap_update_bits(map, CMN_PLL_REFCLK_SRC_SELECTION, CMN_PLL_REFCLK_SRC_DIV, 0); if ret != 0 { return ret; }
    }
    ret = regmap_set_bits(map, CMN_PLL_CTRL, CMN_PLL_CTRL_LOCK_DETECT_EN); if ret != 0 { return ret; }
    ret = regmap_clear_bits(map, CMN_PLL_POWER_ON_AND_RESET, CMN_ANA_EN_SW_RSTN); if ret != 0 { return ret; }
    usleep_range(1000, 1200);
    ret = regmap_set_bits(map, CMN_PLL_POWER_ON_AND_RESET, CMN_ANA_EN_SW_RSTN); if ret != 0 { return ret; }
    let mut val = 0u32; regmap_read_poll_timeout(map, CMN_PLL_LOCKED, &mut val, CMN_PLL_CLKS_LOCKED, 100, 100 * 1000)
}

extern "C" {
    fn regmap_read(map: *mut Regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_update_bits(map: *mut Regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_set_bits(map: *mut Regmap, reg: u32, bits: u32) -> i32;
    fn regmap_clear_bits(map: *mut Regmap, reg: u32, bits: u32) -> i32;
    fn regmap_read_poll_timeout(map: *mut Regmap, reg: u32, val: *mut u32, condition: u32, delay: u32, timeout: u32) -> i32;
    fn usleep_range(min: u32, max: u32);
}

// The remaining registration, probe, remove, PM, device-match, and module-driver
// declarations retain their external kernel behavior and are supplied by dependencies.
extern "C" {
    fn ipq_cmn_pll_clk_probe(pdev: *mut PlatformDevice) -> i32;
    fn ipq_cmn_pll_clk_remove(pdev: *mut PlatformDevice);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
