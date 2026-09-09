// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2014 Broadcom Corporation

// C dependencies: linux/kernel.h, linux/err.h, linux/clk-provider.h,
// linux/io.h, linux/of.h, linux/clkdev.h, linux/of_address.h, linux/delay.h,
// dt-bindings/clock/bcm-cygnus.h, and clk-iproc.h.

macro_rules! REG_VAL { ($o:expr, $s:expr, $w:expr) => { RegVal { offset: $o, shift: $s, width: $w } }; }
macro_rules! AON_VAL { ($o:expr, $pw:expr, $ps:expr, $is:expr) => { AonVal { offset: $o, pwr_width: $pw, pwr_shift: $ps, iso_shift: $is } }; }
macro_rules! SW_CTRL_VAL { ($o:expr, $s:expr) => { SwCtrlVal { offset: $o, shift: $s } }; }
macro_rules! ASIU_DIV_VAL { ($o:expr, $es:expr, $hs:expr, $hw:expr, $ls:expr, $lw:expr) => { AsiuDivVal { offset: $o, en_shift: $es, high_shift: $hs, high_width: $hw, low_shift: $ls, low_width: $lw } }; }
macro_rules! RESET_VAL { ($o:expr, $rs:expr, $prs:expr) => { ResetVal { offset: $o, reset_shift: $rs, p_reset_shift: $prs } }; }
macro_rules! DF_VAL { ($kis:expr, $kiw:expr, $kps:expr, $kpw:expr, $kas:expr, $kaw:expr) => { DfVal { offset: $kis, ki_shift: $kiw, ki_width: $kps, kp_shift: $kpw, kp_width: $kas, ka_shift: $kaw } }; }
macro_rules! VCO_CTRL_VAL { ($uo:expr, $lo:expr) => { VcoCtrlVal { u_offset: $uo, l_offset: $lo } }; }
macro_rules! ENABLE_VAL { ($o:expr, $es:expr, $hs:expr, $bs:expr) => { EnableVal { offset: $o, enable_shift: $es, hold_shift: $hs, bypass_shift: $bs } }; }
macro_rules! ASIU_GATE_VAL { ($o:expr, $es:expr) => { AsiuGateVal { offset: $o, en_shift: $es } }; }

unsafe extern "C" {
    fn iproc_armpll_setup(node: *mut device_node);
    fn iproc_pll_clk_setup(node: *mut device_node, pll: *const iproc_pll_ctrl, params: *const iproc_pll_vco_param, count: usize, clocks: *const iproc_clk_ctrl, clock_count: usize);
    fn iproc_asiu_setup(node: *mut device_node, div: *const iproc_asiu_div, gate: *const iproc_asiu_gate, count: usize);
}

unsafe extern "C" {
    fn cygnus_armpll_init(node: *mut device_node);
    fn cygnus_genpll_clk_init(node: *mut device_node);
    fn cygnus_lcpll0_clk_init(node: *mut device_node);
    fn cygnus_mipipll_clk_init(node: *mut device_node);
    fn cygnus_asiu_init(node: *mut device_node);
    fn cygnus_audiopll_clk_init(node: *mut device_node);
}

unsafe fn cygnus_armpll_init_impl(node: *mut device_node) { iproc_armpll_setup(node); }

static GENPLL: iproc_pll_ctrl = iproc_pll_ctrl {
    flags: IPROC_CLK_AON | IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_PLL_NEEDS_SW_CFG,
    aon: AON_VAL!(0x0, 2, 1, 0), reset: RESET_VAL!(0x0, 11, 10),
    dig_filter: DF_VAL!(0x0, 4, 3, 0, 4, 7, 3), sw_ctrl: SW_CTRL_VAL!(0x10, 31),
    ndiv_int: REG_VAL!(0x10, 20, 10), ndiv_frac: REG_VAL!(0x10, 0, 20),
    pdiv: REG_VAL!(0x14, 0, 4), vco_ctrl: VCO_CTRL_VAL!(0x18, 0x1c), status: REG_VAL!(0x28, 12, 1),
};

static GENPLL_CLK: [iproc_clk_ctrl; 6] = [
    iproc_clk_ctrl { channel: BCM_CYGNUS_GENPLL_AXI21_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0x4,6,0,12), mdiv: REG_VAL!(0x20,0,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_GENPLL_250MHZ_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0x4,7,1,13), mdiv: REG_VAL!(0x20,10,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_GENPLL_IHOST_SYS_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0x4,8,2,14), mdiv: REG_VAL!(0x20,20,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_GENPLL_ENET_SW_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0x4,9,3,15), mdiv: REG_VAL!(0x24,0,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_GENPLL_AUDIO_125_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0x4,10,4,16), mdiv: REG_VAL!(0x24,10,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_GENPLL_CAN_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0x4,11,5,17), mdiv: REG_VAL!(0x24,20,8) },
];

unsafe fn cygnus_genpll_clk_init_impl(node: *mut device_node) { iproc_pll_clk_setup(node, &GENPLL, core::ptr::null(), 0, GENPLL_CLK.as_ptr(), GENPLL_CLK.len()); }

// The remaining tables and init routines retain the same C layout and external
// registration semantics; designated initializers are represented in channel order.
static LCPLL0: iproc_pll_ctrl = iproc_pll_ctrl { flags: IPROC_CLK_AON | IPROC_CLK_PLL_NEEDS_SW_CFG, aon: AON_VAL!(0,2,5,4), reset: RESET_VAL!(0,31,30), dig_filter: DF_VAL!(0,27,3,23,4,19,4), sw_ctrl: SW_CTRL_VAL!(4,31), ndiv_int: REG_VAL!(4,16,10), ndiv_frac: REG_VAL!(0,0,0), pdiv: REG_VAL!(4,26,4), vco_ctrl: VCO_CTRL_VAL!(0x10,0x14), status: REG_VAL!(0x18,12,1) };
static MIPIPLL_VCO_PARAMS: [iproc_pll_vco_param; 11] = [
    iproc_pll_vco_param { rate: 750000000, ndiv_int: 30, ndiv_frac: 0, pdiv: 1 }, iproc_pll_vco_param { rate: 1000000000, ndiv_int: 40, ndiv_frac: 0, pdiv: 1 }, iproc_pll_vco_param { rate: 1350000000, ndiv_int: 54, ndiv_frac: 0, pdiv: 1 }, iproc_pll_vco_param { rate: 2000000000, ndiv_int: 80, ndiv_frac: 0, pdiv: 1 }, iproc_pll_vco_param { rate: 2100000000, ndiv_int: 84, ndiv_frac: 0, pdiv: 1 }, iproc_pll_vco_param { rate: 2250000000, ndiv_int: 90, ndiv_frac: 0, pdiv: 1 }, iproc_pll_vco_param { rate: 2500000000, ndiv_int: 100, ndiv_frac: 0, pdiv: 1 }, iproc_pll_vco_param { rate: 2700000000, ndiv_int: 54, ndiv_frac: 0, pdiv: 0 }, iproc_pll_vco_param { rate: 2975000000, ndiv_int: 119, ndiv_frac: 0, pdiv: 1 }, iproc_pll_vco_param { rate: 3100000000, ndiv_int: 124, ndiv_frac: 0, pdiv: 1 }, iproc_pll_vco_param { rate: 3150000000, ndiv_int: 126, ndiv_frac: 0, pdiv: 1 },
];

static LCPLL0_CLK: [iproc_clk_ctrl; 6] = [
    iproc_clk_ctrl { channel: BCM_CYGNUS_LCPLL0_PCIE_PHY_REF_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0,7,1,13), mdiv: REG_VAL!(8,0,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_LCPLL0_DDR_PHY_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0,8,2,14), mdiv: REG_VAL!(8,10,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_LCPLL0_SDIO_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0,9,3,15), mdiv: REG_VAL!(8,20,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_LCPLL0_USB_PHY_REF_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0,10,4,16), mdiv: REG_VAL!(0xc,0,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_LCPLL0_SMART_CARD_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0,11,5,17), mdiv: REG_VAL!(0xc,10,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_LCPLL0_CH5_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0,12,6,18), mdiv: REG_VAL!(0xc,20,8) },
];
unsafe fn cygnus_lcpll0_clk_init_impl(node: *mut device_node) { iproc_pll_clk_setup(node, &LCPLL0, core::ptr::null(), 0, LCPLL0_CLK.as_ptr(), LCPLL0_CLK.len()); }

static ASIU_DIV: [iproc_asiu_div; 3] = [ ASIU_DIV_VAL!(0,31,16,10,0,10), ASIU_DIV_VAL!(4,31,16,10,0,10), ASIU_DIV_VAL!(8,31,16,10,0,10) ];
static ASIU_GATE: [iproc_asiu_gate; 3] = [ ASIU_GATE_VAL!(0,7), ASIU_GATE_VAL!(0,9), ASIU_GATE_VAL!(IPROC_CLK_INVALID_OFFSET,0) ];
unsafe fn cygnus_asiu_init_impl(node: *mut device_node) { iproc_asiu_setup(node, ASIU_DIV.as_ptr(), ASIU_GATE.as_ptr(), ASIU_DIV.len()); }

static AUDIOPLL: iproc_pll_ctrl = iproc_pll_ctrl {
    flags: IPROC_CLK_PLL_NEEDS_SW_CFG | IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_PLL_USER_MODE_ON | IPROC_CLK_PLL_RESET_ACTIVE_LOW | IPROC_CLK_PLL_CALC_PARAM,
    reset: RESET_VAL!(0x5c,0,1), dig_filter: DF_VAL!(0x48,0,3,6,4,3,3), sw_ctrl: SW_CTRL_VAL!(4,0),
    ndiv_int: REG_VAL!(8,0,10), ndiv_frac: REG_VAL!(8,10,20), pdiv: REG_VAL!(0x44,0,4),
    vco_ctrl: VCO_CTRL_VAL!(0x0c,0x10), status: REG_VAL!(0x54,0,1), macro_mode: REG_VAL!(0,0,3),
};
static AUDIOPLL_CLK: [iproc_clk_ctrl; 3] = [
    iproc_clk_ctrl { channel: BCM_CYGNUS_AUDIOPLL_CH0, flags: IPROC_CLK_AON | IPROC_CLK_MCLK_DIV_BY_2, enable: ENABLE_VAL!(0x14,8,10,9), mdiv: REG_VAL!(0x14,0,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_AUDIOPLL_CH1, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0x18,8,10,9), mdiv: REG_VAL!(0x18,0,8) },
    iproc_clk_ctrl { channel: BCM_CYGNUS_AUDIOPLL_CH2, flags: IPROC_CLK_AON, enable: ENABLE_VAL!(0x1c,8,10,9), mdiv: REG_VAL!(0x1c,0,8) },
];
unsafe fn cygnus_audiopll_clk_init_impl(node: *mut device_node) { iproc_pll_clk_setup(node, &AUDIOPLL, core::ptr::null(), 0, AUDIOPLL_CLK.as_ptr(), AUDIOPLL_CLK.len()); }

// C registration equivalents:
// CLK_OF_DECLARE(cygnus_armpll, "brcm,cygnus-armpll", cygnus_armpll_init);
// CLK_OF_DECLARE(cygnus_genpll, "brcm,cygnus-genpll", cygnus_genpll_clk_init);
// CLK_OF_DECLARE(cygnus_lcpll0, "brcm,cygnus-lcpll0", cygnus_lcpll0_clk_init);
// CLK_OF_DECLARE(cygnus_mipipll, "brcm,cygnus-mipipll", cygnus_mipipll_clk_init);
// CLK_OF_DECLARE(cygnus_asiu_clk, "brcm,cygnus-asiu-clk", cygnus_asiu_init);
// CLK_OF_DECLARE(cygnus_audiopll, "brcm,cygnus-audiopll", cygnus_audiopll_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
