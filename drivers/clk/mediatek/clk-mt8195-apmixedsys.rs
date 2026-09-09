// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the surrounding kernel crate:
// clk-fhctl.h, clk-gate.h, clk-mtk.h, clk-pll.h, clk-pllfh.h,
// dt-bindings/clock/mt8195-clk.h, and linux/platform_device.h.

static APmixed_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x8,
    clr_ofs: 0x8,
    sta_ofs: 0x8,
};

macro_rules! GATE_APMIXED {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &APmixed_CG_REGS, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static APMIXED_CLKS: [mtk_gate; 1] = [
    GATE_APMIXED!(CLK_APMIXED_PLL_SSUSB26M, "pll_ssusb26m", "clk26m", 1),
];

const MT8195_PLL_FMAX: u64 = 3800u64 * MHZ;
const MT8195_PLL_FMIN: u64 = 1500u64 * MHZ;
const MT8195_INTEGER_BITS: u32 = 8;

macro_rules! PLL {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr,
     $flags:expr, $rst_bar_mask:expr, $pcwbits:expr, $pd_reg:expr,
     $pd_shift:expr, $tuner_reg:expr, $tuner_en_reg:expr, $tuner_en_bit:expr,
     $pcw_reg:expr, $pcw_shift:expr, $pcw_chg_reg:expr, $en_reg:expr,
     $pll_en_bit:expr) => {
        mtk_pll_data {
            id: $id, name: $name, reg: $reg, pwr_reg: $pwr_reg,
            en_mask: $en_mask, flags: $flags, rst_bar_mask: $rst_bar_mask,
            fmax: MT8195_PLL_FMAX, fmin: MT8195_PLL_FMIN,
            pcwbits: $pcwbits, pcwibits: MT8195_INTEGER_BITS,
            pd_reg: $pd_reg, pd_shift: $pd_shift, tuner_reg: $tuner_reg,
            tuner_en_reg: $tuner_en_reg, tuner_en_bit: $tuner_en_bit,
            pcw_reg: $pcw_reg, pcw_shift: $pcw_shift,
            pcw_chg_reg: $pcw_chg_reg, en_reg: $en_reg,
            pll_en_bit: $pll_en_bit,
        }
    };
}

static PLLS: [mtk_pll_data; 22] = [
    PLL!(CLK_APMIXED_NNAPLL, "nnapll", 0x0390, 0x03a0, 0, 0, 0, 22, 0x0398, 24, 0, 0, 0, 0x0398, 0, 0x0398, 0, 9),
    PLL!(CLK_APMIXED_RESPLL, "respll", 0x0190, 0x0320, 0, 0, 0, 22, 0x0198, 24, 0, 0, 0, 0x0198, 0, 0x0198, 0, 9),
    PLL!(CLK_APMIXED_ETHPLL, "ethpll", 0x0360, 0x0370, 0, 0, 0, 22, 0x0368, 24, 0, 0, 0, 0x0368, 0, 0x0368, 0, 9),
    PLL!(CLK_APMIXED_MSDCPLL, "msdcpll", 0x0710, 0x0720, 0, 0, 0, 22, 0x0718, 24, 0, 0, 0, 0x0718, 0, 0x0718, 0, 9),
    PLL!(CLK_APMIXED_TVDPLL1, "tvdpll1", 0x00a0, 0x00b0, 0, 0, 0, 22, 0x00a8, 24, 0, 0, 0, 0x00a8, 0, 0x00a8, 0, 9),
    PLL!(CLK_APMIXED_TVDPLL2, "tvdpll2", 0x00c0, 0x00d0, 0, 0, 0, 22, 0x00c8, 24, 0, 0, 0, 0x00c8, 0, 0x00c8, 0, 9),
    PLL!(CLK_APMIXED_MMPLL, "mmpll", 0x00e0, 0x00f0, 0xff000000, HAVE_RST_BAR, BIT!(23), 22, 0x00e8, 24, 0, 0, 0, 0x00e8, 0, 0x00e8, 0, 9),
    PLL!(CLK_APMIXED_MAINPLL, "mainpll", 0x01d0, 0x01e0, 0xff000000, HAVE_RST_BAR, BIT!(23), 22, 0x01d8, 24, 0, 0, 0, 0x01d8, 0, 0x01d8, 0, 9),
    PLL!(CLK_APMIXED_VDECPLL, "vdecpll", 0x0890, 0x08a0, 0, 0, 0, 22, 0x0898, 24, 0, 0, 0, 0x0898, 0, 0x0898, 0, 9),
    PLL!(CLK_APMIXED_IMGPLL, "imgpll", 0x0100, 0x0110, 0, 0, 0, 22, 0x0108, 24, 0, 0, 0, 0x0108, 0, 0x0108, 0, 9),
    PLL!(CLK_APMIXED_UNIVPLL, "univpll", 0x01f0, 0x0700, 0xff000000, HAVE_RST_BAR, BIT!(23), 22, 0x01f8, 24, 0, 0, 0, 0x01f8, 0, 0x01f8, 0, 9),
    PLL!(CLK_APMIXED_HDMIPLL1, "hdmipll1", 0x08c0, 0x08d0, 0, 0, 0, 22, 0x08c8, 24, 0, 0, 0, 0x08c8, 0, 0x08c8, 0, 9),
    PLL!(CLK_APMIXED_HDMIPLL2, "hdmipll2", 0x0870, 0x0880, 0, 0, 0, 22, 0x0878, 24, 0, 0, 0, 0x0878, 0, 0x0878, 0, 9),
    PLL!(CLK_APMIXED_HDMIRX_APLL, "hdmirx_apll", 0x08e0, 0x0dd4, 0, 0, 0, 32, 0x08e8, 24, 0, 0, 0, 0x08ec, 0, 0x08e8, 0, 9),
    PLL!(CLK_APMIXED_USB1PLL, "usb1pll", 0x01a0, 0x01b0, 0, 0, 0, 22, 0x01a8, 24, 0, 0, 0, 0x01a8, 0, 0x01a8, 0, 9),
    PLL!(CLK_APMIXED_ADSPPLL, "adsppll", 0x07e0, 0x07f0, 0, 0, 0, 22, 0x07e8, 24, 0, 0, 0, 0x07e8, 0, 0x07e8, 0, 9),
    PLL!(CLK_APMIXED_APLL1, "apll1", 0x07c0, 0x0dc0, 0, 0, 0, 32, 0x07c8, 24, 0x0470, 0x0000, 12, 0x07cc, 0, 0x07c8, 0, 9),
    PLL!(CLK_APMIXED_APLL2, "apll2", 0x0780, 0x0dc4, 0, 0, 0, 32, 0x0788, 24, 0x0474, 0x0000, 13, 0x078c, 0, 0x0788, 0, 9),
    PLL!(CLK_APMIXED_APLL3, "apll3", 0x0760, 0x0dc8, 0, 0, 0, 32, 0x0768, 24, 0x0478, 0x0000, 14, 0x076c, 0, 0x0768, 0, 9),
    PLL!(CLK_APMIXED_APLL4, "apll4", 0x0740, 0x0dcc, 0, 0, 0, 32, 0x0748, 24, 0x047c, 0x0000, 15, 0x074c, 0, 0x0748, 0, 9),
    PLL!(CLK_APMIXED_APLL5, "apll5", 0x07a0, 0x0dd0, 0x100000, 0, 0, 32, 0x07a8, 24, 0x0480, 0x0000, 16, 0x07ac, 0, 0x07a8, 0, 9),
    PLL!(CLK_APMIXED_MFGPLL, "mfgpll", 0x0340, 0x0350, 0, 0, 0, 22, 0x0348, 24, 0, 0, 0, 0x0348, 0, 0x0348, 0, 9),
    PLL!(CLK_APMIXED_DGIPLL, "dgipll", 0x0150, 0x0160, 0, 0, 0, 22, 0x0158, 24, 0, 0, 0, 0x0158, 0, 0x0158, 0, 9),
];

#[repr(C)]
enum FhPllId {
    FH_ARMPLL_LL,
    FH_ARMPLL_BL,
    FH_MEMPLL,
    FH_ADSPPLL,
    FH_NNAPLL,
    FH_CCIPLL,
    FH_MFGPLL,
    FH_TVDPLL2,
    FH_MPLL,
    FH_MMPLL,
    FH_MAINPLL,
    FH_MSDCPLL,
    FH_IMGPLL,
    FH_VDECPLL,
    FH_TVDPLL1,
    FH_NR_FH,
}

macro_rules! FH {
    ($pllid:expr, $fhid:expr, $offset:expr) => {
        mtk_pllfh_data { data: mtk_pllfh_data_inner {
            pll_id: $pllid, fh_id: $fhid, fh_ver: FHCTL_PLLFH_V2,
            fhx_offset: $offset, dds_mask: GENMASK!(21, 0),
            slope0_value: 0x6003c97, slope1_value: 0x6003c97,
            sfstrx_en: BIT!(2), frddsx_en: BIT!(1), fhctlx_en: BIT!(0),
            tgl_org: BIT!(31), dvfs_tri: BIT!(31), pcwchg: BIT!(31),
            dt_val: 0x0, df_val: 0x9, updnlmt_shft: 16,
            msk_frddsx_dys: GENMASK!(23, 20), msk_frddsx_dts: GENMASK!(19, 16),
        }}
    };
}

static mut PLLFHS: [mtk_pllfh_data; 10] = [
    FH!(CLK_APMIXED_ADSPPLL, FH_ADSPPLL, 0x78), FH!(CLK_APMIXED_NNAPLL, FH_NNAPLL, 0x8c),
    FH!(CLK_APMIXED_MFGPLL, FH_MFGPLL, 0xb4), FH!(CLK_APMIXED_TVDPLL2, FH_TVDPLL2, 0xc8),
    FH!(CLK_APMIXED_MMPLL, FH_MMPLL, 0xf0), FH!(CLK_APMIXED_MAINPLL, FH_MAINPLL, 0x104),
    FH!(CLK_APMIXED_MSDCPLL, FH_MSDCPLL, 0x118), FH!(CLK_APMIXED_IMGPLL, FH_IMGPLL, 0x12c),
    FH!(CLK_APMIXED_VDECPLL, FH_VDECPLL, 0x140), FH!(CLK_APMIXED_TVDPLL2, FH_TVDPLL1, 0x154),
];

static OF_MATCH_CLK_MT8195_APMIXED: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt8195-apmixedsys" },
    of_device_id {},
];

unsafe fn clk_mt8195_apmixed_probe(pdev: *mut platform_device) -> c_int {
    let mut clk_data: *mut clk_hw_onecell_data;
    let node = (*pdev).dev.of_node;
    let fhctl_node: *const u8 = b"mediatek,mt8195-fhctl\0".as_ptr();
    let mut r: c_int;

    clk_data = mtk_alloc_clk_data(CLK_APMIXED_NR_CLK);
    if clk_data.is_null() { return -ENOMEM; }

    fhctl_parse_dt(fhctl_node, PLLFHS.as_mut_ptr(), ARRAY_SIZE!(PLLFHS));
    r = mtk_clk_register_pllfhs(&mut (*pdev).dev, PLLS.as_ptr(), ARRAY_SIZE!(PLLS),
                                PLLFHS.as_mut_ptr(), ARRAY_SIZE!(PLLFHS), clk_data);
    if r != 0 { mtk_free_clk_data(clk_data); return r; }
    r = mtk_clk_register_gates(&mut (*pdev).dev, node, APMIXED_CLKS.as_ptr(), ARRAY_SIZE!(APMIXED_CLKS), clk_data);
    if r != 0 { mtk_clk_unregister_pllfhs(PLLS.as_ptr(), ARRAY_SIZE!(PLLS), PLLFHS.as_mut_ptr(), ARRAY_SIZE!(PLLFHS), clk_data); mtk_free_clk_data(clk_data); return r; }
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 { mtk_clk_unregister_gates(APMIXED_CLKS.as_ptr(), ARRAY_SIZE!(APMIXED_CLKS), clk_data); mtk_clk_unregister_pllfhs(PLLS.as_ptr(), ARRAY_SIZE!(PLLS), PLLFHS.as_mut_ptr(), ARRAY_SIZE!(PLLFHS), clk_data); mtk_free_clk_data(clk_data); return r; }
    platform_set_drvdata(pdev, clk_data);
    r
}

unsafe fn clk_mt8195_apmixed_remove(pdev: *mut platform_device) {
    let node = (*pdev).dev.of_node;
    let clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider(node);
    mtk_clk_unregister_gates(APMIXED_CLKS.as_ptr(), ARRAY_SIZE!(APMIXED_CLKS), clk_data);
    mtk_clk_unregister_pllfhs(PLLS.as_ptr(), ARRAY_SIZE!(PLLS), PLLFHS.as_mut_ptr(), ARRAY_SIZE!(PLLFHS), clk_data);
    mtk_free_clk_data(clk_data);
}

static mut CLK_MT8195_APMIXED_DRV: platform_driver = platform_driver {
    probe: Some(clk_mt8195_apmixed_probe),
    remove: Some(clk_mt8195_apmixed_remove),
    driver: driver { name: "clk-mt8195-apmixed", of_match_table: OF_MATCH_CLK_MT8195_APMIXED.as_ptr() },
};

module_platform_driver!(CLK_MT8195_APMIXED_DRV);
// MODULE_DESCRIPTION("MediaTek MT8195 apmixedsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
