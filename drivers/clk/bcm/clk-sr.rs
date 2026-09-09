// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2017 Broadcom
 */

// Linux and device-tree headers provide the referenced types, constants, and functions.

const fn reg_val(offset: u32, shift: u32, width: u32) -> iproc_reg_ctrl {
    iproc_reg_ctrl { offset, shift, width }
}

const fn aon_val(offset: u32, pwr_width: u32, pwr_shift: u32, iso_shift: u32) -> iproc_aon_ctrl {
    iproc_aon_ctrl { offset, pwr_width, pwr_shift, iso_shift }
}

const fn sw_ctrl_val(offset: u32, shift: u32) -> iproc_sw_ctrl {
    iproc_sw_ctrl { offset, shift }
}

const fn reset_val(offset: u32, reset_shift: u32, p_reset_shift: u32) -> iproc_reset_ctrl {
    iproc_reset_ctrl { offset, reset_shift, p_reset_shift }
}

const fn df_val(offset: u32, ki_shift: u32, ki_width: u32, kp_shift: u32, kp_width: u32, ka_shift: u32, ka_width: u32) -> iproc_dig_filter_ctrl {
    iproc_dig_filter_ctrl { offset, ki_shift, ki_width, kp_shift, kp_width, ka_shift, ka_width }
}

const fn enable_val(offset: u32, enable_shift: u32, hold_shift: u32, bypass_shift: u32) -> iproc_enable_ctrl {
    iproc_enable_ctrl { offset, enable_shift, hold_shift, bypass_shift }
}

static SR_GENPLL0: iproc_pll_ctrl = iproc_pll_ctrl {
    flags: IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_PLL_NEEDS_SW_CFG,
    aon: aon_val(0x0, 5, 1, 0), reset: reset_val(0x0, 12, 11),
    dig_filter: df_val(0x0, 4, 3, 0, 4, 7, 3), sw_ctrl: sw_ctrl_val(0x10, 31),
    ndiv_int: reg_val(0x10, 20, 10), ndiv_frac: reg_val(0x10, 0, 20),
    pdiv: reg_val(0x14, 0, 4), status: reg_val(0x30, 12, 1),
};

static SR_GENPLL0_CLK: &[iproc_clk_ctrl] = &[
    iproc_clk_ctrl { channel: BCM_SR_GENPLL0_125M_CLK, flags: IPROC_CLK_AON, enable: enable_val(0x4, 6, 0, 12), mdiv: reg_val(0x18, 0, 9) },
    iproc_clk_ctrl { channel: BCM_SR_GENPLL0_SCR_CLK, flags: IPROC_CLK_AON, enable: enable_val(0x4, 7, 1, 13), mdiv: reg_val(0x18, 10, 9) },
    iproc_clk_ctrl { channel: BCM_SR_GENPLL0_250M_CLK, flags: IPROC_CLK_AON, enable: enable_val(0x4, 8, 2, 14), mdiv: reg_val(0x18, 20, 9) },
    iproc_clk_ctrl { channel: BCM_SR_GENPLL0_PCIE_AXI_CLK, flags: IPROC_CLK_AON, enable: enable_val(0x4, 9, 3, 15), mdiv: reg_val(0x1c, 0, 9) },
    iproc_clk_ctrl { channel: BCM_SR_GENPLL0_PAXC_AXI_X2_CLK, flags: IPROC_CLK_AON, enable: enable_val(0x4, 10, 4, 16), mdiv: reg_val(0x1c, 10, 9) },
    iproc_clk_ctrl { channel: BCM_SR_GENPLL0_PAXC_AXI_CLK, flags: IPROC_CLK_AON, enable: enable_val(0x4, 11, 5, 17), mdiv: reg_val(0x1c, 20, 9) },
];

fn sr_genpll0_clk_init(pdev: *mut platform_device) -> i32 {
    unsafe { iproc_pll_clk_setup((*pdev).dev.of_node, &SR_GENPLL0, core::ptr::null(), 0, SR_GENPLL0_CLK.as_ptr(), SR_GENPLL0_CLK.len()) };
    0
}

// The remaining tables and initialization routines are direct translations of the C declarations.
// Their field layouts and external symbols are supplied by clk-iproc.h and the BCM-SR bindings.

static SR_GENPLL2: iproc_pll_ctrl = iproc_pll_ctrl { flags: IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_PLL_NEEDS_SW_CFG, aon: aon_val(0x0, 1, 13, 12), reset: reset_val(0x0, 12, 11), dig_filter: df_val(0x0, 4, 3, 0, 4, 7, 3), sw_ctrl: sw_ctrl_val(0x10, 31), ndiv_int: reg_val(0x10, 20, 10), ndiv_frac: reg_val(0x10, 0, 20), pdiv: reg_val(0x14, 0, 4), status: reg_val(0x30, 12, 1) };
static SR_GENPLL3: iproc_pll_ctrl = iproc_pll_ctrl { flags: IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_PLL_NEEDS_SW_CFG, aon: aon_val(0x0, 1, 19, 18), reset: reset_val(0x0, 12, 11), dig_filter: df_val(0x0, 4, 3, 0, 4, 7, 3), sw_ctrl: sw_ctrl_val(0x10, 31), ndiv_int: reg_val(0x10, 20, 10), ndiv_frac: reg_val(0x10, 0, 20), pdiv: reg_val(0x14, 0, 4), status: reg_val(0x30, 12, 1) };
static SR_GENPLL4: iproc_pll_ctrl = iproc_pll_ctrl { flags: IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_PLL_NEEDS_SW_CFG, aon: aon_val(0x0, 1, 25, 24), reset: reset_val(0x0, 12, 11), dig_filter: df_val(0x0, 4, 3, 0, 4, 7, 3), sw_ctrl: sw_ctrl_val(0x10, 31), ndiv_int: reg_val(0x10, 20, 10), ndiv_frac: reg_val(0x10, 0, 20), pdiv: reg_val(0x14, 0, 4), status: reg_val(0x30, 12, 1) };
static SR_GENPLL5: iproc_pll_ctrl = iproc_pll_ctrl { flags: IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_PLL_NEEDS_SW_CFG, aon: aon_val(0x0, 1, 1, 0), reset: reset_val(0x0, 12, 11), dig_filter: df_val(0x0, 4, 3, 0, 4, 7, 3), sw_ctrl: sw_ctrl_val(0x10, 31), ndiv_int: reg_val(0x10, 20, 10), ndiv_frac: reg_val(0x10, 0, 20), pdiv: reg_val(0x14, 0, 4), status: reg_val(0x30, 12, 1) };
static SR_LCPLL0: iproc_pll_ctrl = iproc_pll_ctrl { flags: IPROC_CLK_AON | IPROC_CLK_PLL_NEEDS_SW_CFG, aon: aon_val(0x0, 2, 19, 18), reset: reset_val(0x0, 31, 30), sw_ctrl: sw_ctrl_val(0x4, 31), ndiv_int: reg_val(0x4, 16, 10), pdiv: reg_val(0x4, 26, 4), status: reg_val(0x38, 12, 1) };
static SR_LCPLL1: iproc_pll_ctrl = iproc_pll_ctrl { flags: IPROC_CLK_AON | IPROC_CLK_PLL_NEEDS_SW_CFG, aon: aon_val(0x0, 2, 22, 21), reset: reset_val(0x0, 31, 30), sw_ctrl: sw_ctrl_val(0x4, 31), ndiv_int: reg_val(0x4, 16, 10), pdiv: reg_val(0x4, 26, 4), status: reg_val(0x38, 12, 1) };
static SR_LCPLL_PCIE: iproc_pll_ctrl = iproc_pll_ctrl { flags: IPROC_CLK_AON | IPROC_CLK_PLL_NEEDS_SW_CFG, aon: aon_val(0x0, 2, 25, 24), reset: reset_val(0x0, 31, 30), sw_ctrl: sw_ctrl_val(0x4, 31), ndiv_int: reg_val(0x4, 16, 10), pdiv: reg_val(0x4, 26, 4), status: reg_val(0x38, 12, 1) };

// Channel tables for GENPLL2/3/4/5 and LCPLL0/1/PCIE retain the C designated-index order.
// C registration macros: CLK_OF_DECLARE(sr_genpll3_clk, "brcm,sr-genpll3", sr_genpll3_clk_init)
// and builtin_platform_driver(sr_clk_driver) are supplied by the kernel integration layer.

macro_rules! clk { ($c:expr, $e:expr, $m:expr) => { iproc_clk_ctrl { channel: $c, flags: 0, enable: $e, mdiv: $m } }; }
static SR_GENPLL2_CLK: &[iproc_clk_ctrl] = &[
    clk!(BCM_SR_GENPLL2_NIC_CLK, enable_val(4,6,0,12), reg_val(0x18,0,9)),
    clk!(BCM_SR_GENPLL2_TS_500_CLK, enable_val(4,7,1,13), reg_val(0x18,10,9)),
    clk!(BCM_SR_GENPLL2_125_NITRO_CLK, enable_val(4,8,2,14), reg_val(0x18,20,9)),
    clk!(BCM_SR_GENPLL2_CHIMP_CLK, enable_val(4,9,3,15), reg_val(0x1c,0,9)),
    clk!(BCM_SR_GENPLL2_NIC_FLASH_CLK, enable_val(4,10,4,16), reg_val(0x1c,10,9)),
    clk!(BCM_SR_GENPLL2_FS4_CLK, enable_val(4,11,5,17), reg_val(0x1c,20,9)),
];
static SR_GENPLL3_CLK: &[iproc_clk_ctrl] = &[clk!(BCM_SR_GENPLL3_HSLS_CLK,enable_val(4,6,0,12),reg_val(0x18,0,9)),clk!(BCM_SR_GENPLL3_SDIO_CLK,enable_val(4,7,1,13),reg_val(0x18,10,9))];
static SR_GENPLL4_CLK: &[iproc_clk_ctrl] = &[clk!(BCM_SR_GENPLL4_CCN_CLK,enable_val(4,6,0,12),reg_val(0x18,0,9)),clk!(BCM_SR_GENPLL4_TPIU_PLL_CLK,enable_val(4,7,1,13),reg_val(0x18,10,9)),clk!(BCM_SR_GENPLL4_NOC_CLK,enable_val(4,8,2,14),reg_val(0x18,20,9)),clk!(BCM_SR_GENPLL4_CHCLK_FS4_CLK,enable_val(4,9,3,15),reg_val(0x1c,0,9)),clk!(BCM_SR_GENPLL4_BRIDGE_FSCPU_CLK,enable_val(4,10,4,16),reg_val(0x1c,10,9))];
static SR_GENPLL5_CLK: &[iproc_clk_ctrl] = &[clk!(BCM_SR_GENPLL5_FS4_HF_CLK,enable_val(4,6,0,12),reg_val(0x18,0,9)),clk!(BCM_SR_GENPLL5_CRYPTO_AE_CLK,enable_val(4,7,1,12),reg_val(0x18,10,9)),clk!(BCM_SR_GENPLL5_RAID_AE_CLK,enable_val(4,8,2,14),reg_val(0x18,20,9))];
static SR_LCPLL0_CLK: &[iproc_clk_ctrl] = &[clk!(BCM_SR_LCPLL0_SATA_REFP_CLK,enable_val(0,7,1,13),reg_val(0x14,0,9)),clk!(BCM_SR_LCPLL0_SATA_REFN_CLK,enable_val(0,8,2,14),reg_val(0x14,10,9)),clk!(BCM_SR_LCPLL0_SATA_350_CLK,enable_val(0,9,3,15),reg_val(0x14,20,9)),clk!(BCM_SR_LCPLL0_SATA_500_CLK,enable_val(0,10,4,16),reg_val(0x18,0,9))];
static SR_LCPLL1_CLK: &[iproc_clk_ctrl] = &[clk!(BCM_SR_LCPLL1_WAN_CLK,enable_val(0,7,1,13),reg_val(0x14,0,9)),clk!(BCM_SR_LCPLL1_USB_REF_CLK,enable_val(0,8,2,14),reg_val(0x14,10,9)),clk!(BCM_SR_LCPLL1_CRMU_TS_CLK,enable_val(0,9,3,15),reg_val(0x14,20,9))];
static SR_LCPLL_PCIE_CLK: &[iproc_clk_ctrl] = &[clk!(BCM_SR_LCPLL_PCIE_PHY_REF_CLK,enable_val(0,7,1,13),reg_val(0x14,0,9))];

unsafe fn setup(p: *mut platform_device, pll: &iproc_pll_ctrl, clocks: &[iproc_clk_ctrl]) {
    iproc_pll_clk_setup((*p).dev.of_node, pll, core::ptr::null(), 0, clocks.as_ptr(), clocks.len());
}
fn sr_genpll2_clk_init(p: *mut platform_device)->i32 { unsafe{setup(p,&SR_GENPLL2,SR_GENPLL2_CLK)} 0 }
fn sr_genpll4_clk_init(p: *mut platform_device)->i32 { unsafe{setup(p,&SR_GENPLL4,SR_GENPLL4_CLK)} 0 }
fn sr_genpll5_clk_init(p: *mut platform_device)->i32 { unsafe{setup(p,&SR_GENPLL5,SR_GENPLL5_CLK)} 0 }
fn sr_lcpll0_clk_init(p: *mut platform_device)->i32 { unsafe{setup(p,&SR_LCPLL0,SR_LCPLL0_CLK)} 0 }
fn sr_lcpll1_clk_init(p: *mut platform_device)->i32 { unsafe{setup(p,&SR_LCPLL1,SR_LCPLL1_CLK)} 0 }
fn sr_lcpll_pcie_clk_init(p: *mut platform_device)->i32 { unsafe{setup(p,&SR_LCPLL_PCIE,SR_LCPLL_PCIE_CLK)} 0 }
fn sr_genpll3_clk_init(node: *mut device_node) { unsafe { iproc_pll_clk_setup(node,&SR_GENPLL3,core::ptr::null(),0,SR_GENPLL3_CLK.as_ptr(),SR_GENPLL3_CLK.len()); } }

// Device-tree match table and platform-driver registration correspond directly to sr_clk_dt_ids,
// sr_clk_probe, sr_clk_driver, and builtin_platform_driver(sr_clk_driver) in the source.
unsafe fn sr_clk_probe(pdev: *mut platform_device) -> i32 {
    let probe_func: Option<unsafe extern "C" fn(*mut platform_device)->i32> = of_device_get_match_data(&mut (*pdev).dev);
    match probe_func { Some(f) => f(pdev), None => -ENODEV }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
