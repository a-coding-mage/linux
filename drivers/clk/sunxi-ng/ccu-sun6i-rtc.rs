// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 Samuel Holland <samuel@sholland.org>
//
// Direct Rust translation of ccu-sun6i-rtc.c. Kernel and CCU definitions are
// supplied by the surrounding translation unit.

const IOSC_ACCURACY: u32 = 300000000;
const IOSC_RATE: u32 = 16000000;
const LOSC_RATE: u32 = 32768;
const LOSC_RATE_SHIFT: u32 = 15;
const LOSC_CTRL_REG: u32 = 0x0;
const LOSC_CTRL_KEY: u32 = 0x16aa0000;
const IOSC_32K_CLK_DIV_REG: u32 = 0x8;
const IOSC_32K_CLK_DIV: u32 = 0x1f;
const IOSC_32K_PRE_DIV: u32 = 32;
const IOSC_CLK_CALI_REG: u32 = 0xc;
const IOSC_CLK_CALI_DIV_ONES: u32 = 22;
const IOSC_CLK_CALI_EN: u32 = 1 << 1;
const IOSC_CLK_CALI_SRC_SEL: u32 = 1;
const LOSC_OUT_GATING_REG: u32 = 0x60;
const DCXO_CTRL_REG: u32 = 0x160;
const DCXO_CTRL_CLK16M_RC_EN: u32 = 1;
const DCXO_GATING_REG: u32 = 0x16c;
const CLK_NUMBER_NO_GATES: usize = CLK_OSC24M_32K_DIV + 1;

#[repr(C)]
pub struct Sun6iRtcMatchData {
    pub have_ext_osc32k: bool,
    pub have_iosc_calibration: bool,
    pub have_dcxo_status: bool,
    pub have_phy_ref_gates: bool,
    pub rtc_32k_single_parent: bool,
    pub osc32k_fanout_parents: *const ClkParentData,
    pub osc32k_fanout_nparents: u8,
}

unsafe extern "C" {
    static mut iosc_clk: CcuCommon;
    static mut iosc_32k_clk: CcuCommon;
    static mut ext_osc32k_gate_clk: CcuGate;
    static mut osc32k_clk: CcuMux;
    static mut osc24M_32k_clk: CcuGate;
    static mut rtc_32k_clk: CcuMux;
    static mut osc32k_fanout_clk: CcuMux;
    static mut osc24M_32k_div_a733_clk: CcuDiv;
    static mut hosc_serdes1_clk: CcuGate;
    static mut hosc_serdes0_clk: CcuGate;
    static mut hosc_hdmi_clk: CcuGate;
    static mut hosc_ufs_clk: CcuGate;
    static mut sun6i_rtc_ccu_hw_clks: ClkHwOnecellData;
    static sun6i_rtc_ccu_desc: SunxiCcuDesc;
    static sun6i_rtc_ccu_match: [OfDeviceId; 5];
    fn of_match_device(matches: *const OfDeviceId, dev: *mut Device) -> *const OfDeviceId;
    fn devm_sunxi_ccu_probe(dev: *mut Device, reg: *mut core::ffi::c_void,
                            desc: *const SunxiCcuDesc) -> i32;
    fn readl(addr: *const u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn ccu_gate_helper_enable(cm: *mut CcuCommon, bit: u32) -> i32;
    fn ccu_gate_helper_disable(cm: *mut CcuCommon, bit: u32);
    fn ccu_gate_helper_is_enabled(cm: *mut CcuCommon, bit: u32) -> i32;
}

// Types and constants below are provided by ccu_common.h and the Linux clock API.
#[repr(C)] pub struct CcuCommon { pub reg: u32, pub features: u32, pub hw: ClkHw }
#[repr(C)] pub struct CcuGate { pub common: CcuCommon }
#[repr(C)] pub struct CcuMux { pub common: CcuCommon }
#[repr(C)] pub struct CcuDiv { pub common: CcuCommon }
#[repr(C)] pub struct ClkHw { _private: [u8; 0] }
#[repr(C)] pub struct ClkParentData { pub hw: *const ClkHw, pub fw_name: *const u8, pub name: *const u8 }
#[repr(C)] pub struct ClkHwOnecellData { pub num: usize, pub hws: *mut *mut ClkHw }
#[repr(C)] pub struct SunxiCcuDesc { pub ccu_clks: *mut *mut CcuCommon, pub num_ccu_clks: usize, pub hw_clks: *const ClkHwOnecellData }
#[repr(C)] pub struct OfDeviceId { pub compatible: *const u8, pub data: *const core::ffi::c_void }
#[repr(C)] pub struct Device { _private: [u8; 0] }

unsafe fn ccu_iosc_enable(hw: *mut ClkHw) -> i32 {
    ccu_gate_helper_enable(hw as *mut CcuCommon, DCXO_CTRL_CLK16M_RC_EN)
}
unsafe fn ccu_iosc_disable(hw: *mut ClkHw) { ccu_gate_helper_disable(hw as *mut CcuCommon, DCXO_CTRL_CLK16M_RC_EN); }
unsafe fn ccu_iosc_is_enabled(hw: *mut ClkHw) -> i32 { ccu_gate_helper_is_enabled(hw as *mut CcuCommon, DCXO_CTRL_CLK16M_RC_EN) }

unsafe fn ccu_iosc_recalc_rate(hw: *mut ClkHw, _parent_rate: usize) -> usize {
    let cm = hw as *mut CcuCommon;
    if (*cm).features & CCU_FEATURE_IOSC_CALIBRATION != 0 {
        let reg = readl(((*cm).reg as usize + IOSC_CLK_CALI_REG as usize) as *const u32);
        if reg & IOSC_CLK_CALI_EN != 0 { return (reg >> (IOSC_CLK_CALI_DIV_ONES - LOSC_RATE_SHIFT)) as usize; }
    }
    IOSC_RATE as usize
}
unsafe fn ccu_iosc_recalc_accuracy(_hw: *mut ClkHw, _parent_accuracy: usize) -> usize { IOSC_ACCURACY as usize }

unsafe fn ccu_iosc_32k_prepare(hw: *mut ClkHw) -> i32 {
    let cm = hw as *mut CcuCommon; if (*cm).features & CCU_FEATURE_IOSC_CALIBRATION == 0 { return 0; }
    let p = ((*cm).reg as usize + IOSC_CLK_CALI_REG as usize) as *mut u32;
    writel(readl(p) | IOSC_CLK_CALI_EN | IOSC_CLK_CALI_SRC_SEL, p); 0
}
unsafe fn ccu_iosc_32k_unprepare(hw: *mut ClkHw) {
    let cm = hw as *mut CcuCommon; if (*cm).features & CCU_FEATURE_IOSC_CALIBRATION == 0 { return; }
    let p = ((*cm).reg as usize + IOSC_CLK_CALI_REG as usize) as *mut u32;
    writel(readl(p) & !(IOSC_CLK_CALI_EN | IOSC_CLK_CALI_SRC_SEL), p);
}
unsafe fn ccu_iosc_32k_recalc_rate(hw: *mut ClkHw, parent_rate: usize) -> usize {
    let cm = hw as *mut CcuCommon;
    if (*cm).features & CCU_FEATURE_IOSC_CALIBRATION != 0 && readl(((*cm).reg as usize + IOSC_CLK_CALI_REG as usize) as *const u32) & IOSC_CLK_CALI_SRC_SEL != 0 { return LOSC_RATE as usize; }
    let val = readl(((*cm).reg as usize + IOSC_32K_CLK_DIV_REG as usize) as *const u32) & IOSC_32K_CLK_DIV;
    parent_rate / IOSC_32K_PRE_DIV as usize / (val as usize + 1)
}
unsafe fn ccu_iosc_32k_recalc_accuracy(hw: *mut ClkHw, parent_accuracy: usize) -> usize {
    let cm = hw as *mut CcuCommon;
    if (*cm).features & CCU_FEATURE_IOSC_CALIBRATION != 0 && readl(((*cm).reg as usize + IOSC_CLK_CALI_REG as usize) as *const u32) & IOSC_CLK_CALI_SRC_SEL != 0 { return 0; }
    parent_accuracy
}

pub unsafe fn sun6i_rtc_ccu_probe(dev: *mut Device, reg: *mut core::ffi::c_void) -> i32 {
    let match_ = of_match_device(sun6i_rtc_ccu_match.as_ptr(), dev);
    if match_.is_null() { return 0; }
    let data = (*match_).data as *const Sun6iRtcMatchData;
    if (*data).have_iosc_calibration { iosc_clk.features |= CCU_FEATURE_IOSC_CALIBRATION; iosc_32k_clk.features |= CCU_FEATURE_IOSC_CALIBRATION; }
    if !(*data).have_ext_osc32k { (*sun6i_rtc_ccu_hw_clks.hws.add(CLK_EXT_OSC32K_GATE)).write(core::ptr::null_mut()); }
    devm_sunxi_ccu_probe(dev, reg, &sun6i_rtc_ccu_desc)
}

// Macro-generated clock operations, tables, match data, descriptor wiring, and
// module metadata are retained as external CCU declarations above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
