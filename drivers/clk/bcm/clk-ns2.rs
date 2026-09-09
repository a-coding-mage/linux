// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2015 Broadcom Corporation

// Kernel dependencies: linux/kernel.h, linux/err.h, linux/clk-provider.h,
// linux/io.h, linux/of.h, linux/of_address.h, dt-bindings/clock/bcm-ns2.h,
// and clk-iproc.h.

macro_rules! REG_VAL { ($o:expr, $s:expr, $w:expr) => { iproc_reg_ctrl { offset: $o, shift: $s, width: $w } }; }
macro_rules! AON_VAL { ($o:expr, $pw:expr, $ps:expr, $is:expr) => { iproc_aon_ctrl { offset: $o, pwr_width: $pw, pwr_shift: $ps, iso_shift: $is } }; }
macro_rules! RESET_VAL { ($o:expr, $rs:expr, $prs:expr) => { iproc_reset_ctrl { offset: $o, reset_shift: $rs, p_reset_shift: $prs } }; }
macro_rules! DF_VAL { ($o:expr, $kis:expr, $kiw:expr, $kps:expr, $kpw:expr, $kas:expr, $kaw:expr) => { iproc_dig_filter_ctrl { offset: $o, ki_shift: $kis, ki_width: $kiw, kp_shift: $kps, kp_width: $kpw, ka_shift: $kas, ka_width: $kaw } }; }
macro_rules! VCO_CTRL_VAL { ($uo:expr, $lo:expr) => { iproc_vco_ctrl { u_offset: $uo, l_offset: $lo } }; }
macro_rules! ENABLE_VAL { ($o:expr, $es:expr, $hs:expr, $bs:expr) => { iproc_enable_ctrl { offset: $o, enable_shift: $es, hold_shift: $hs, bypass_shift: $bs } }; }

static GENPLL_SCR: iproc_pll_ctrl = iproc_pll_ctrl {
    flags: IPROC_CLK_AON | IPROC_CLK_PLL_SPLIT_STAT_CTRL,
    aon: AON_VAL(0x0, 1, 15, 12),
    reset: RESET_VAL(0x4, 2, 1),
    dig_filter: DF_VAL(0x0, 9, 3, 5, 4, 2, 3),
    ndiv_int: REG_VAL(0x8, 4, 10),
    pdiv: REG_VAL(0x8, 0, 4),
    vco_ctrl: VCO_CTRL_VAL(0x10, 0xc),
    status: REG_VAL(0x0, 27, 1),
};

// bypass_shift is not defined in NS2; it does not appear to be used, so it is 0.
static GENPLL_SCR_CLK: [iproc_clk_ctrl; 6] = [
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SCR_SCR_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 18, 12, 0), mdiv: REG_VAL(0x18, 0, 8) },
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SCR_FS_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 19, 13, 0), mdiv: REG_VAL(0x18, 8, 8) },
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SCR_AUDIO_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 20, 14, 0), mdiv: REG_VAL(0x14, 0, 8) },
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SCR_CH3_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 21, 15, 0), mdiv: REG_VAL(0x14, 8, 8) },
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SCR_CH4_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 22, 16, 0), mdiv: REG_VAL(0x14, 16, 8) },
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SCR_CH5_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 23, 17, 0), mdiv: REG_VAL(0x14, 24, 8) },
];

unsafe fn ns2_genpll_scr_clk_init(node: *mut device_node) {
    iproc_pll_clk_setup(node, &GENPLL_SCR, core::ptr::null_mut(), 0, &GENPLL_SCR_CLK, GENPLL_SCR_CLK.len());
}
// CLK_OF_DECLARE(ns2_genpll_src_clk, "brcm,ns2-genpll-scr", ns2_genpll_scr_clk_init);

static GENPLL_SW: iproc_pll_ctrl = iproc_pll_ctrl {
    flags: IPROC_CLK_AON | IPROC_CLK_PLL_SPLIT_STAT_CTRL,
    aon: AON_VAL(0x0, 1, 11, 10), reset: RESET_VAL(0x4, 2, 1),
    dig_filter: DF_VAL(0x0, 9, 3, 5, 4, 2, 3), ndiv_int: REG_VAL(0x8, 4, 10),
    pdiv: REG_VAL(0x8, 0, 4), vco_ctrl: VCO_CTRL_VAL(0x10, 0xc), status: REG_VAL(0x0, 13, 1),
};

static GENPLL_SW_CLK: [iproc_clk_ctrl; 6] = [
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SW_RPE_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 18, 12, 0), mdiv: REG_VAL(0x18, 0, 8) },
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SW_250_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 19, 13, 0), mdiv: REG_VAL(0x18, 8, 8) },
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SW_NIC_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 20, 14, 0), mdiv: REG_VAL(0x14, 0, 8) },
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SW_CHIMP_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 21, 15, 0), mdiv: REG_VAL(0x14, 8, 8) },
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SW_PORT_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 22, 16, 0), mdiv: REG_VAL(0x14, 16, 8) },
    iproc_clk_ctrl { channel: BCM_NS2_GENPLL_SW_SDIO_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0, 23, 17, 0), mdiv: REG_VAL(0x14, 24, 8) },
];

unsafe fn ns2_genpll_sw_clk_init(node: *mut device_node) {
    iproc_pll_clk_setup(node, &GENPLL_SW, core::ptr::null_mut(), 0, &GENPLL_SW_CLK, GENPLL_SW_CLK.len());
}
// CLK_OF_DECLARE(ns2_genpll_sw_clk, "brcm,ns2-genpll-sw", ns2_genpll_sw_clk_init);

static LCPll_DDR: iproc_pll_ctrl = iproc_pll_ctrl {
    flags: IPROC_CLK_AON | IPROC_CLK_PLL_SPLIT_STAT_CTRL, aon: AON_VAL(0x0, 2, 1, 0),
    reset: RESET_VAL(0x4, 2, 1), dig_filter: DF_VAL(0x0, 9, 3, 5, 4, 1, 4),
    ndiv_int: REG_VAL(0x8, 4, 10), pdiv: REG_VAL(0x8, 0, 4), vco_ctrl: VCO_CTRL_VAL(0x10, 0xc), status: REG_VAL(0x0, 0, 1),
};

static LCPll_DDR_CLK: [iproc_clk_ctrl; 6] = [
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_DDR_PCIE_SATA_USB_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,18,12,0), mdiv: REG_VAL(0x14,0,8) },
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_DDR_DDR_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,19,13,0), mdiv: REG_VAL(0x14,8,8) },
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_DDR_CH2_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,20,14,0), mdiv: REG_VAL(0x10,0,8) },
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_DDR_CH3_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,21,15,0), mdiv: REG_VAL(0x10,8,8) },
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_DDR_CH4_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,22,16,0), mdiv: REG_VAL(0x10,16,8) },
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_DDR_CH5_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,23,17,0), mdiv: REG_VAL(0x10,24,8) },
];
unsafe fn ns2_lcpll_ddr_clk_init(node: *mut device_node) { iproc_pll_clk_setup(node, &LCPll_DDR, core::ptr::null_mut(), 0, &LCPll_DDR_CLK, LCPll_DDR_CLK.len()); }
// CLK_OF_DECLARE(ns2_lcpll_ddr_clk, "brcm,ns2-lcpll-ddr", ns2_lcpll_ddr_clk_init);

static LCPll_PORTS: iproc_pll_ctrl = iproc_pll_ctrl {
    flags: IPROC_CLK_AON | IPROC_CLK_PLL_SPLIT_STAT_CTRL, aon: AON_VAL(0x0, 2, 5, 4),
    reset: RESET_VAL(0x4, 2, 1), dig_filter: DF_VAL(0x0, 9, 3, 5, 4, 1, 4),
    ndiv_int: REG_VAL(0x8, 4, 10), pdiv: REG_VAL(0x8, 0, 4), vco_ctrl: VCO_CTRL_VAL(0x10, 0xc), status: REG_VAL(0x0, 0, 1),
};

static LCPll_PORTS_CLK: [iproc_clk_ctrl; 6] = [
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_PORTS_WAN_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,18,12,0), mdiv: REG_VAL(0x14,0,8) },
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_PORTS_RGMII_CLK, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,19,13,0), mdiv: REG_VAL(0x14,8,8) },
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_PORTS_CH2_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,20,14,0), mdiv: REG_VAL(0x10,0,8) },
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_PORTS_CH3_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,21,15,0), mdiv: REG_VAL(0x10,8,8) },
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_PORTS_CH4_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,22,16,0), mdiv: REG_VAL(0x10,16,8) },
    iproc_clk_ctrl { channel: BCM_NS2_LCPLL_PORTS_CH5_UNUSED, flags: IPROC_CLK_AON, enable: ENABLE_VAL(0x0,23,17,0), mdiv: REG_VAL(0x10,24,8) },
];
unsafe fn ns2_lcpll_ports_clk_init(node: *mut device_node) { iproc_pll_clk_setup(node, &LCPll_PORTS, core::ptr::null_mut(), 0, &LCPll_PORTS_CLK, LCPll_PORTS_CLK.len()); }
// CLK_OF_DECLARE(ns2_lcpll_ports_clk, "brcm,ns2-lcpll-ports", ns2_lcpll_ports_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
