/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018-2019 SiFive, Inc.
 * Wesley Terpstra
 * Paul Walmsley
 * Zong Li
 */

// Dependency types supplied by the Linux clock, reset, platform, and WRPLL bindings.

pub const EXPECTED_CLK_PARENT_COUNT: u32 = 2;

pub const PRCI_COREPLLCFG0_OFFSET: u32 = 0x4;
pub const PRCI_COREPLLCFG0_DIVR_SHIFT: u32 = 0;
pub const PRCI_COREPLLCFG0_DIVR_MASK: u32 = 0x3f << PRCI_COREPLLCFG0_DIVR_SHIFT;
pub const PRCI_COREPLLCFG0_DIVF_SHIFT: u32 = 6;
pub const PRCI_COREPLLCFG0_DIVF_MASK: u32 = 0x1ff << PRCI_COREPLLCFG0_DIVF_SHIFT;
pub const PRCI_COREPLLCFG0_DIVQ_SHIFT: u32 = 15;
pub const PRCI_COREPLLCFG0_DIVQ_MASK: u32 = 0x7 << PRCI_COREPLLCFG0_DIVQ_SHIFT;
pub const PRCI_COREPLLCFG0_RANGE_SHIFT: u32 = 18;
pub const PRCI_COREPLLCFG0_RANGE_MASK: u32 = 0x7 << PRCI_COREPLLCFG0_RANGE_SHIFT;
pub const PRCI_COREPLLCFG0_BYPASS_SHIFT: u32 = 24;
pub const PRCI_COREPLLCFG0_BYPASS_MASK: u32 = 0x1 << PRCI_COREPLLCFG0_BYPASS_SHIFT;
pub const PRCI_COREPLLCFG0_FSE_SHIFT: u32 = 25;
pub const PRCI_COREPLLCFG0_FSE_MASK: u32 = 0x1 << PRCI_COREPLLCFG0_FSE_SHIFT;
pub const PRCI_COREPLLCFG0_LOCK_SHIFT: u32 = 31;
pub const PRCI_COREPLLCFG0_LOCK_MASK: u32 = 0x1 << PRCI_COREPLLCFG0_LOCK_SHIFT;

pub const PRCI_COREPLLCFG1_OFFSET: u32 = 0x8;
pub const PRCI_COREPLLCFG1_CKE_SHIFT: u32 = 31;
pub const PRCI_COREPLLCFG1_CKE_MASK: u32 = 0x1 << PRCI_COREPLLCFG1_CKE_SHIFT;

pub const PRCI_DDRPLLCFG0_OFFSET: u32 = 0xc;
pub const PRCI_DDRPLLCFG0_DIVR_SHIFT: u32 = 0;
pub const PRCI_DDRPLLCFG0_DIVR_MASK: u32 = 0x3f << PRCI_DDRPLLCFG0_DIVR_SHIFT;
pub const PRCI_DDRPLLCFG0_DIVF_SHIFT: u32 = 6;
pub const PRCI_DDRPLLCFG0_DIVF_MASK: u32 = 0x1ff << PRCI_DDRPLLCFG0_DIVF_SHIFT;
pub const PRCI_DDRPLLCFG0_DIVQ_SHIFT: u32 = 15;
pub const PRCI_DDRPLLCFG0_DIVQ_MASK: u32 = 0x7 << PRCI_DDRPLLCFG0_DIVQ_SHIFT;
pub const PRCI_DDRPLLCFG0_RANGE_SHIFT: u32 = 18;
pub const PRCI_DDRPLLCFG0_RANGE_MASK: u32 = 0x7 << PRCI_DDRPLLCFG0_RANGE_SHIFT;
pub const PRCI_DDRPLLCFG0_BYPASS_SHIFT: u32 = 24;
pub const PRCI_DDRPLLCFG0_BYPASS_MASK: u32 = 0x1 << PRCI_DDRPLLCFG0_BYPASS_SHIFT;
pub const PRCI_DDRPLLCFG0_FSE_SHIFT: u32 = 25;
pub const PRCI_DDRPLLCFG0_FSE_MASK: u32 = 0x1 << PRCI_DDRPLLCFG0_FSE_SHIFT;
pub const PRCI_DDRPLLCFG0_LOCK_SHIFT: u32 = 31;
pub const PRCI_DDRPLLCFG0_LOCK_MASK: u32 = 0x1 << PRCI_DDRPLLCFG0_LOCK_SHIFT;
pub const PRCI_DDRPLLCFG1_OFFSET: u32 = 0x10;
pub const PRCI_DDRPLLCFG1_CKE_SHIFT: u32 = 31;
pub const PRCI_DDRPLLCFG1_CKE_MASK: u32 = 0x1 << PRCI_DDRPLLCFG1_CKE_SHIFT;

pub const PRCI_PCIE_AUX_OFFSET: u32 = 0x14;
pub const PRCI_PCIE_AUX_EN_SHIFT: u32 = 0;
pub const PRCI_PCIE_AUX_EN_MASK: u32 = 0x1 << PRCI_PCIE_AUX_EN_SHIFT;
pub const PRCI_GEMGXLPLLCFG0_OFFSET: u32 = 0x1c;
pub const PRCI_GEMGXLPLLCFG0_DIVR_SHIFT: u32 = 0;
pub const PRCI_GEMGXLPLLCFG0_DIVR_MASK: u32 = 0x3f << PRCI_GEMGXLPLLCFG0_DIVR_SHIFT;
pub const PRCI_GEMGXLPLLCFG0_DIVF_SHIFT: u32 = 6;
pub const PRCI_GEMGXLPLLCFG0_DIVF_MASK: u32 = 0x1ff << PRCI_GEMGXLPLLCFG0_DIVF_SHIFT;
pub const PRCI_GEMGXLPLLCFG0_DIVQ_SHIFT: u32 = 15;
pub const PRCI_GEMGXLPLLCFG0_DIVQ_MASK: u32 = 0x7 << PRCI_GEMGXLPLLCFG0_DIVQ_SHIFT;
pub const PRCI_GEMGXLPLLCFG0_RANGE_SHIFT: u32 = 18;
pub const PRCI_GEMGXLPLLCFG0_RANGE_MASK: u32 = 0x7 << PRCI_GEMGXLPLLCFG0_RANGE_SHIFT;
pub const PRCI_GEMGXLPLLCFG0_BYPASS_SHIFT: u32 = 24;
pub const PRCI_GEMGXLPLLCFG0_BYPASS_MASK: u32 = 0x1 << PRCI_GEMGXLPLLCFG0_BYPASS_SHIFT;
pub const PRCI_GEMGXLPLLCFG0_FSE_SHIFT: u32 = 25;
pub const PRCI_GEMGXLPLLCFG0_FSE_MASK: u32 = 0x1 << PRCI_GEMGXLPLLCFG0_FSE_SHIFT;
pub const PRCI_GEMGXLPLLCFG0_LOCK_SHIFT: u32 = 31;
pub const PRCI_GEMGXLPLLCFG0_LOCK_MASK: u32 = 0x1 << PRCI_GEMGXLPLLCFG0_LOCK_SHIFT;
pub const PRCI_GEMGXLPLLCFG1_OFFSET: u32 = 0x20;
pub const PRCI_GEMGXLPLLCFG1_CKE_SHIFT: u32 = 31;
pub const PRCI_GEMGXLPLLCFG1_CKE_MASK: u32 = 0x1 << PRCI_GEMGXLPLLCFG1_CKE_SHIFT;

pub const PRCI_CORECLKSEL_OFFSET: u32 = 0x24;
pub const PRCI_CORECLKSEL_CORECLKSEL_SHIFT: u32 = 0;
pub const PRCI_CORECLKSEL_CORECLKSEL_MASK: u32 = 0x1 << PRCI_CORECLKSEL_CORECLKSEL_SHIFT;
pub const PRCI_DEVICESRESETREG_OFFSET: u32 = 0x28;
pub const PRCI_DEVICESRESETREG_DDR_CTRL_RST_N_SHIFT: u32 = 0;
pub const PRCI_DEVICESRESETREG_DDR_CTRL_RST_N_MASK: u32 = 0x1 << PRCI_DEVICESRESETREG_DDR_CTRL_RST_N_SHIFT;
pub const PRCI_DEVICESRESETREG_DDR_AXI_RST_N_SHIFT: u32 = 1;
pub const PRCI_DEVICESRESETREG_DDR_AXI_RST_N_MASK: u32 = 0x1 << PRCI_DEVICESRESETREG_DDR_AXI_RST_N_SHIFT;
pub const PRCI_DEVICESRESETREG_DDR_AHB_RST_N_SHIFT: u32 = 2;
pub const PRCI_DEVICESRESETREG_DDR_AHB_RST_N_MASK: u32 = 0x1 << PRCI_DEVICESRESETREG_DDR_AHB_RST_N_SHIFT;
pub const PRCI_DEVICESRESETREG_DDR_PHY_RST_N_SHIFT: u32 = 3;
pub const PRCI_DEVICESRESETREG_DDR_PHY_RST_N_MASK: u32 = 0x1 << PRCI_DEVICESRESETREG_DDR_PHY_RST_N_SHIFT;
pub const PRCI_DEVICESRESETREG_GEMGXL_RST_N_SHIFT: u32 = 5;
pub const PRCI_DEVICESRESETREG_GEMGXL_RST_N_MASK: u32 = 0x1 << PRCI_DEVICESRESETREG_GEMGXL_RST_N_SHIFT;
pub const PRCI_DEVICESRESETREG_CHIPLINK_RST_N_SHIFT: u32 = 6;
pub const PRCI_DEVICESRESETREG_CHIPLINK_RST_N_MASK: u32 = 0x1 << PRCI_DEVICESRESETREG_CHIPLINK_RST_N_SHIFT;
pub const PRCI_RST_NR: u32 = 7;
pub const PRCI_CLKMUXSTATUSREG_OFFSET: u32 = 0x2c;
pub const PRCI_CLKMUXSTATUSREG_TLCLKSEL_STATUS_SHIFT: u32 = 1;
pub const PRCI_CLKMUXSTATUSREG_TLCLKSEL_STATUS_MASK: u32 = 0x1 << PRCI_CLKMUXSTATUSREG_TLCLKSEL_STATUS_SHIFT;

pub const PRCI_CLTXPLLCFG0_OFFSET: u32 = 0x30;
pub const PRCI_CLTXPLLCFG0_DIVR_SHIFT: u32 = 0;
pub const PRCI_CLTXPLLCFG0_DIVR_MASK: u32 = 0x3f << PRCI_CLTXPLLCFG0_DIVR_SHIFT;
pub const PRCI_CLTXPLLCFG0_DIVF_SHIFT: u32 = 6;
pub const PRCI_CLTXPLLCFG0_DIVF_MASK: u32 = 0x1ff << PRCI_CLTXPLLCFG0_DIVF_SHIFT;
pub const PRCI_CLTXPLLCFG0_DIVQ_SHIFT: u32 = 15;
pub const PRCI_CLTXPLLCFG0_DIVQ_MASK: u32 = 0x7 << PRCI_CLTXPLLCFG0_DIVQ_SHIFT;
pub const PRCI_CLTXPLLCFG0_RANGE_SHIFT: u32 = 18;
pub const PRCI_CLTXPLLCFG0_RANGE_MASK: u32 = 0x7 << PRCI_CLTXPLLCFG0_RANGE_SHIFT;
pub const PRCI_CLTXPLLCFG0_BYPASS_SHIFT: u32 = 24;
pub const PRCI_CLTXPLLCFG0_BYPASS_MASK: u32 = 0x1 << PRCI_CLTXPLLCFG0_BYPASS_SHIFT;
pub const PRCI_CLTXPLLCFG0_FSE_SHIFT: u32 = 25;
pub const PRCI_CLTXPLLCFG0_FSE_MASK: u32 = 0x1 << PRCI_CLTXPLLCFG0_FSE_SHIFT;
pub const PRCI_CLTXPLLCFG0_LOCK_SHIFT: u32 = 31;
pub const PRCI_CLTXPLLCFG0_LOCK_MASK: u32 = 0x1 << PRCI_CLTXPLLCFG0_LOCK_SHIFT;
pub const PRCI_CLTXPLLCFG1_OFFSET: u32 = 0x34;
pub const PRCI_CLTXPLLCFG1_CKE_SHIFT: u32 = 31;
pub const PRCI_CLTXPLLCFG1_CKE_MASK: u32 = 0x1 << PRCI_CLTXPLLCFG1_CKE_SHIFT;
pub const PRCI_DVFSCOREPLLCFG0_OFFSET: u32 = 0x38;
pub const PRCI_DVFSCOREPLLCFG1_OFFSET: u32 = 0x3c;
pub const PRCI_DVFSCOREPLLCFG1_CKE_SHIFT: u32 = 31;
pub const PRCI_DVFSCOREPLLCFG1_CKE_MASK: u32 = 0x1 << PRCI_DVFSCOREPLLCFG1_CKE_SHIFT;
pub const PRCI_COREPLLSEL_OFFSET: u32 = 0x40;
pub const PRCI_COREPLLSEL_COREPLLSEL_SHIFT: u32 = 0;
pub const PRCI_COREPLLSEL_COREPLLSEL_MASK: u32 = 0x1 << PRCI_COREPLLSEL_COREPLLSEL_SHIFT;
pub const PRCI_HFPCLKPLLCFG0_OFFSET: u32 = 0x50;
pub const PRCI_HFPCLKPLL_CFG0_DIVR_SHIFT: u32 = 0;
pub const PRCI_HFPCLKPLL_CFG0_DIVR_MASK: u32 = 0x3f << PRCI_HFPCLKPLL_CFG0_DIVR_SHIFT;
pub const PRCI_HFPCLKPLL_CFG0_DIVF_SHIFT: u32 = 6;
pub const PRCI_HFPCLKPLL_CFG0_DIVF_MASK: u32 = 0x1ff << PRCI_HFPCLKPLL_CFG0_DIVF_SHIFT;
pub const PRCI_HFPCLKPLL_CFG0_DIVQ_SHIFT: u32 = 15;
pub const PRCI_HFPCLKPLL_CFG0_DIVQ_MASK: u32 = 0x7 << PRCI_HFPCLKPLL_CFG0_DIVQ_SHIFT;
pub const PRCI_HFPCLKPLL_CFG0_RANGE_SHIFT: u32 = 18;
pub const PRCI_HFPCLKPLL_CFG0_RANGE_MASK: u32 = 0x7 << PRCI_HFPCLKPLL_CFG0_RANGE_SHIFT;
pub const PRCI_HFPCLKPLL_CFG0_BYPASS_SHIFT: u32 = 24;
pub const PRCI_HFPCLKPLL_CFG0_BYPASS_MASK: u32 = 0x1 << PRCI_HFPCLKPLL_CFG0_BYPASS_SHIFT;
pub const PRCI_HFPCLKPLL_CFG0_FSE_SHIFT: u32 = 25;
pub const PRCI_HFPCLKPLL_CFG0_FSE_MASK: u32 = 0x1 << PRCI_HFPCLKPLL_CFG0_FSE_SHIFT;
pub const PRCI_HFPCLKPLL_CFG0_LOCK_SHIFT: u32 = 31;
pub const PRCI_HFPCLKPLL_CFG0_LOCK_MASK: u32 = 0x1 << PRCI_HFPCLKPLL_CFG0_LOCK_SHIFT;
pub const PRCI_HFPCLKPLLCFG1_OFFSET: u32 = 0x54;
pub const PRCI_HFPCLKPLL_CFG1_CKE_SHIFT: u32 = 31;
pub const PRCI_HFPCLKPLL_CFG1_CKE_MASK: u32 = 0x1 << PRCI_HFPCLKPLL_CFG1_CKE_SHIFT;
pub const PRCI_HFPCLKPLLSEL_OFFSET: u32 = 0x58;
pub const PRCI_HFPCLKPLLSEL_HFPCLKPLLSEL_SHIFT: u32 = 0;
pub const PRCI_HFPCLKPLLSEL_HFPCLKPLLSEL_MASK: u32 = 0x1 << PRCI_HFPCLKPLLSEL_HFPCLKPLLSEL_SHIFT;
pub const PRCI_HFPCLKPLLDIV_OFFSET: u32 = 0x5c;
pub const PRCI_PRCIPLL_OFFSET: u32 = 0xe0;
pub const PRCI_PROCMONCFG_OFFSET: u32 = 0xf0;

#[repr(C)]
pub struct __prci_data {
    pub va: *mut core::ffi::c_void,
    pub reset: reset_simple_data,
    pub hw_clks: clk_hw_onecell_data,
}

#[repr(C)]
pub struct __prci_wrpll_data {
    pub c: wrpll_cfg,
    pub enable_bypass: Option<unsafe extern "C" fn(*mut __prci_data)>,
    pub disable_bypass: Option<unsafe extern "C" fn(*mut __prci_data)>,
    pub cfg0_offs: u8,
    pub cfg1_offs: u8,
}

#[repr(C)]
pub struct __prci_clock {
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub hw: clk_hw,
    pub pwd: *mut __prci_wrpll_data,
    pub pd: *mut __prci_data,
}

#[inline]
pub unsafe fn clk_hw_to_prci_clock(pwd: *mut clk_hw) -> *mut __prci_clock {
    (pwd as *mut u8).sub(core::mem::offset_of!(__prci_clock, hw)) as *mut __prci_clock
}

#[repr(C)]
pub struct prci_clk_desc {
    pub clks: *mut __prci_clock,
    pub num_clks: usize,
}

extern "C" {
    pub fn sifive_prci_coreclksel_use_hfclk(pd: *mut __prci_data);
    pub fn sifive_prci_coreclksel_use_corepll(pd: *mut __prci_data);
    pub fn sifive_prci_coreclksel_use_final_corepll(pd: *mut __prci_data);
    pub fn sifive_prci_corepllsel_use_dvfscorepll(pd: *mut __prci_data);
    pub fn sifive_prci_corepllsel_use_corepll(pd: *mut __prci_data);
    pub fn sifive_prci_hfpclkpllsel_use_hfclk(pd: *mut __prci_data);
    pub fn sifive_prci_hfpclkpllsel_use_hfpclkpll(pd: *mut __prci_data);

    pub fn sifive_prci_wrpll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32;
    pub fn sifive_prci_wrpll_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32;
    pub fn sifive_clk_is_enabled(hw: *mut clk_hw) -> i32;
    pub fn sifive_prci_clock_enable(hw: *mut clk_hw) -> i32;
    pub fn sifive_prci_clock_disable(hw: *mut clk_hw);
    pub fn sifive_prci_wrpll_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64;
    pub fn sifive_prci_tlclksel_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64;
    pub fn sifive_prci_hfpclkplldiv_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64;
    pub fn sifive_prci_pcie_aux_clock_is_enabled(hw: *mut clk_hw) -> i32;
    pub fn sifive_prci_pcie_aux_clock_enable(hw: *mut clk_hw) -> i32;
    pub fn sifive_prci_pcie_aux_clock_disable(hw: *mut clk_hw);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
