// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2015 Broadcom Corporation

// Linux kernel dependencies supplied by the surrounding translation unit.
// dt-bindings/clock/bcm-nsp.h and clk-iproc.h provide the referenced symbols.

macro_rules! reg_val {
    ($o:expr, $s:expr, $w:expr) => {
        iproc_reg_ctrl { offset: $o, shift: $s, width: $w }
    };
}

macro_rules! aon_val {
    ($o:expr, $pw:expr, $ps:expr, $is:expr) => {
        iproc_aon_ctrl { offset: $o, pwr_width: $pw, pwr_shift: $ps, iso_shift: $is }
    };
}

macro_rules! reset_val {
    ($o:expr, $rs:expr, $prs:expr) => {
        iproc_reset_ctrl { offset: $o, reset_shift: $rs, p_reset_shift: $prs }
    };
}

macro_rules! df_val {
    ($o:expr, $kis:expr, $kiw:expr, $kps:expr, $kpw:expr, $kas:expr, $kaw:expr) => {
        iproc_dig_filter_ctrl {
            offset: $o, ki_shift: $kis, ki_width: $kiw, kp_shift: $kps,
            kp_width: $kpw, ka_shift: $kas, ka_width: $kaw,
        }
    };
}

macro_rules! enable_val {
    ($o:expr, $es:expr, $hs:expr, $bs:expr) => {
        iproc_enable_ctrl { offset: $o, enable_shift: $es, hold_shift: $hs, bypass_shift: $bs }
    };
}

unsafe extern "C" {
    fn iproc_armpll_setup(node: *mut device_node);
    fn iproc_pll_clk_setup(
        node: *mut device_node,
        pll: *const iproc_pll_ctrl,
        clk: *const core::ffi::c_void,
        num_clk: usize,
        clocks: *const iproc_clk_ctrl,
        num_clocks: usize,
    );
}

unsafe fn nsp_armpll_init(node: *mut device_node) {
    iproc_armpll_setup(node);
}

// CLK_OF_DECLARE(nsp_armpll, "brcm,nsp-armpll", nsp_armpll_init);

static GENPLL: iproc_pll_ctrl = iproc_pll_ctrl {
    flags: IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_EMBED_PWRCTRL,
    aon: aon_val!(0x0, 1, 12, 0),
    reset: reset_val!(0x0, 11, 10),
    dig_filter: df_val!(0x0, 4, 3, 0, 4, 7, 3),
    ndiv_int: reg_val!(0x14, 20, 10),
    ndiv_frac: reg_val!(0x14, 0, 20),
    pdiv: reg_val!(0x18, 24, 3),
    status: reg_val!(0x20, 12, 1),
};

static GENPLL_CLK: [iproc_clk_ctrl; 6] = [
    iproc_clk_ctrl { channel: BCM_NSP_GENPLL_PHY_CLK, flags: IPROC_CLK_AON, enable: enable_val!(0x4, 12, 6, 18), mdiv: reg_val!(0x18, 16, 8) },
    iproc_clk_ctrl { channel: BCM_NSP_GENPLL_ENET_SW_CLK, flags: IPROC_CLK_AON, enable: enable_val!(0x4, 13, 7, 19), mdiv: reg_val!(0x18, 8, 8) },
    iproc_clk_ctrl { channel: BCM_NSP_GENPLL_USB_PHY_REF_CLK, flags: IPROC_CLK_AON, enable: enable_val!(0x4, 14, 8, 20), mdiv: reg_val!(0x18, 0, 8) },
    iproc_clk_ctrl { channel: BCM_NSP_GENPLL_IPROCFAST_CLK, flags: IPROC_CLK_AON, enable: enable_val!(0x4, 15, 9, 21), mdiv: reg_val!(0x1c, 16, 8) },
    iproc_clk_ctrl { channel: BCM_NSP_GENPLL_SATA1_CLK, flags: IPROC_CLK_AON, enable: enable_val!(0x4, 16, 10, 22), mdiv: reg_val!(0x1c, 8, 8) },
    iproc_clk_ctrl { channel: BCM_NSP_GENPLL_SATA2_CLK, flags: IPROC_CLK_AON, enable: enable_val!(0x4, 17, 11, 23), mdiv: reg_val!(0x1c, 0, 8) },
];

unsafe fn nsp_genpll_clk_init(node: *mut device_node) {
    iproc_pll_clk_setup(node, &GENPLL, core::ptr::null(), 0, GENPLL_CLK.as_ptr(), GENPLL_CLK.len());
}

// CLK_OF_DECLARE(nsp_genpll_clk, "brcm,nsp-genpll", nsp_genpll_clk_init);

static LCPLL0: iproc_pll_ctrl = iproc_pll_ctrl {
    flags: IPROC_CLK_PLL_HAS_NDIV_FRAC | IPROC_CLK_EMBED_PWRCTRL,
    aon: aon_val!(0x0, 1, 24, 0),
    reset: reset_val!(0x0, 23, 22),
    dig_filter: df_val!(0x0, 16, 3, 12, 4, 19, 4),
    ndiv_int: reg_val!(0x4, 20, 8),
    ndiv_frac: reg_val!(0x4, 0, 20),
    pdiv: reg_val!(0x4, 28, 3),
    status: reg_val!(0x10, 12, 1),
};

static LCPLL0_CLK: [iproc_clk_ctrl; 3] = [
    iproc_clk_ctrl { channel: BCM_NSP_LCPLL0_PCIE_PHY_REF_CLK, flags: IPROC_CLK_AON, enable: enable_val!(0x0, 6, 3, 9), mdiv: reg_val!(0x8, 24, 8) },
    iproc_clk_ctrl { channel: BCM_NSP_LCPLL0_SDIO_CLK, flags: IPROC_CLK_AON, enable: enable_val!(0x0, 7, 4, 10), mdiv: reg_val!(0x8, 16, 8) },
    iproc_clk_ctrl { channel: BCM_NSP_LCPLL0_DDR_PHY_CLK, flags: IPROC_CLK_AON, enable: enable_val!(0x0, 8, 5, 11), mdiv: reg_val!(0x8, 8, 8) },
];

unsafe fn nsp_lcpll0_clk_init(node: *mut device_node) {
    iproc_pll_clk_setup(node, &LCPLL0, core::ptr::null(), 0, LCPLL0_CLK.as_ptr(), LCPLL0_CLK.len());
}

// CLK_OF_DECLARE(nsp_lcpll0_clk, "brcm,nsp-lcpll0", nsp_lcpll0_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
