// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021 MediaTek Inc.
 *               Chun-Jie Chen <chun-jie.chen@mediatek.com>
 * Copyright (c) 2023 Collabora Ltd.
 *               AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

const APMIXED_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x14,
    clr_ofs: 0x14,
    sta_ofs: 0x14,
};

macro_rules! GATE_APMIXED {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &APMIXED_CG_REGS, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static APMIXED_CLKS: [mtk_gate; 1] = [
    GATE_APMIXED!(CLK_APMIXED_MIPID26M, "mipid26m", "clk26m", 16),
];

const MT8192_PLL_FMAX: usize = 3800usize * MHZ;
const MT8192_PLL_FMIN: usize = 1500usize * MHZ;
const MT8192_INTEGER_BITS: u32 = 8;

macro_rules! PLL {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr, $flags:expr,
     $rst_bar_mask:expr, $pcwbits:expr, $pd_reg:expr, $pd_shift:expr,
     $tuner_reg:expr, $tuner_en_reg:expr, $tuner_en_bit:expr,
     $pcw_reg:expr, $pcw_shift:expr, $pcw_chg_reg:expr,
     $en_reg:expr, $pll_en_bit:expr) => {
        mtk_pll_data {
            id: $id, name: $name, reg: $reg, pwr_reg: $pwr_reg,
            en_mask: $en_mask, flags: $flags, rst_bar_mask: $rst_bar_mask,
            fmax: MT8192_PLL_FMAX, fmin: MT8192_PLL_FMIN,
            pcwbits: $pcwbits, pcwibits: MT8192_INTEGER_BITS,
            pd_reg: $pd_reg, pd_shift: $pd_shift, tuner_reg: $tuner_reg,
            tuner_en_reg: $tuner_en_reg, tuner_en_bit: $tuner_en_bit,
            pcw_reg: $pcw_reg, pcw_shift: $pcw_shift,
            pcw_chg_reg: $pcw_chg_reg, en_reg: $en_reg,
            pll_en_bit: $pll_en_bit,
        }
    };
}

macro_rules! PLL_B {
    ($($args:expr),*) => { PLL!($($args),*, 0, 0, 0) };
}

static PLLS: [mtk_pll_data; 10] = [
    PLL_B!(CLK_APMIXED_MAINPLL, "mainpll", 0x0340, 0x034c, 0xff000000, HAVE_RST_BAR, BIT!(23), 22, 0x0344, 24, 0, 0, 0, 0x0344, 0),
    PLL_B!(CLK_APMIXED_UNIVPLL, "univpll", 0x0308, 0x0314, 0xff000000, HAVE_RST_BAR, BIT!(23), 22, 0x030c, 24, 0, 0, 0, 0x030c, 0),
    PLL!(CLK_APMIXED_USBPLL, "usbpll", 0x03c4, 0x03cc, 0x00000000, 0, 0, 22, 0x03c4, 24, 0, 0, 0, 0x03c4, 0, 0x03c4, 0x03cc, 2),
    PLL_B!(CLK_APMIXED_MSDCPLL, "msdcpll", 0x0350, 0x035c, 0x00000000, 0, 0, 22, 0x0354, 24, 0, 0, 0, 0x0354, 0),
    PLL_B!(CLK_APMIXED_MMPLL, "mmpll", 0x0360, 0x036c, 0xff000000, HAVE_RST_BAR, BIT!(23), 22, 0x0364, 24, 0, 0, 0, 0x0364, 0),
    PLL_B!(CLK_APMIXED_ADSPPLL, "adsppll", 0x0370, 0x037c, 0xff000000, HAVE_RST_BAR, BIT!(23), 22, 0x0374, 24, 0, 0, 0, 0x0374, 0),
    PLL_B!(CLK_APMIXED_MFGPLL, "mfgpll", 0x0268, 0x0274, 0x00000000, 0, 0, 22, 0x026c, 24, 0, 0, 0, 0x026c, 0),
    PLL_B!(CLK_APMIXED_TVDPLL, "tvdpll", 0x0380, 0x038c, 0x00000000, 0, 0, 22, 0x0384, 24, 0, 0, 0, 0x0384, 0),
    PLL_B!(CLK_APMIXED_APLL1, "apll1", 0x0318, 0x0328, 0x00000000, 0, 0, 32, 0x031c, 24, 0x0040, 0x000c, 0, 0x0320, 0),
    PLL_B!(CLK_APMIXED_APLL2, "apll2", 0x032c, 0x033c, 0x00000000, 0, 0, 32, 0x0330, 24, 0, 0, 0, 0x0334, 0),
];

#[repr(C)]
enum fh_pll_id {
    FH_ARMPLL_LL, FH_ARMPLL_BL0, FH_ARMPLL_BL1, FH_ARMPLL_BL2, FH_ARMPLL_BL3,
    FH_CCIPLL, FH_MFGPLL, FH_MEMPLL, FH_MPLL, FH_MMPLL, FH_MAINPLL,
    FH_MSDCPLL, FH_ADSPPLL, FH_APUPLL, FH_TVDPLL, FH_NR_FH,
}

macro_rules! FH {
    ($pllid:expr, $fhid:expr, $offset:expr) => { mtk_pllfh_data {
        data: mtk_pllfh_data_inner {
            pll_id: $pllid, fh_id: $fhid, fh_ver: FHCTL_PLLFH_V2,
            fhx_offset: $offset, dds_mask: GENMASK!(21, 0),
            slope0_value: 0x6003c97, slope1_value: 0x6003c97,
            sfstrx_en: BIT!(2), frddsx_en: BIT!(1), fhctlx_en: BIT!(0),
            tgl_org: BIT!(31), dvfs_tri: BIT!(31), pcwchg: BIT!(31),
            dt_val: 0x0, df_val: 0x9, updnlmt_shft: 16,
            msk_frddsx_dys: GENMASK!(23, 20), msk_frddsx_dts: GENMASK!(19, 16),
        },
    }};
}

static mut PLLFHS: [mtk_pllfh_data; 6] = [
    FH!(CLK_APMIXED_MFGPLL, FH_MFGPLL, 0xb4), FH!(CLK_APMIXED_MMPLL, FH_MMPLL, 0xf0),
    FH!(CLK_APMIXED_MAINPLL, FH_MAINPLL, 0x104), FH!(CLK_APMIXED_MSDCPLL, FH_MSDCPLL, 0x118),
    FH!(CLK_APMIXED_ADSPPLL, FH_ADSPPLL, 0x12c), FH!(CLK_APMIXED_TVDPLL, FH_TVDPLL, 0x154),
];

static OF_MATCH_CLK_MT8192_APMIXED: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt8192-apmixedsys" },
    of_device_id { /* sentinel */ },
];

unsafe fn clk_mt8192_apmixed_probe(pdev: *mut platform_device) -> i32 {
    let mut clk_data: *mut clk_hw_onecell_data;
    let node = (*pdev).dev.of_node;
    let fhctl_node: *const u8 = b"mediatek,mt8192-fhctl\0".as_ptr();
    let mut r: i32;

    clk_data = mtk_alloc_clk_data(CLK_APMIXED_NR_CLK);
    if clk_data.is_null() { return -ENOMEM; }
    fhctl_parse_dt(fhctl_node, PLLFHS.as_mut_ptr(), ARRAY_SIZE!(PLLFHS));
    r = mtk_clk_register_pllfhs(&mut (*pdev).dev, PLLS.as_ptr(), ARRAY_SIZE!(PLLS), PLLFHS.as_mut_ptr(), ARRAY_SIZE!(PLLFHS), clk_data);
    if r != 0 { goto!(free_clk_data); }
    r = mtk_clk_register_gates(&mut (*pdev).dev, node, APMIXED_CLKS.as_ptr(), ARRAY_SIZE!(APMIXED_CLKS), clk_data);
    if r != 0 { goto!(unregister_plls); }
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 { goto!(unregister_gates); }
    return r;
unregister_gates:
    mtk_clk_unregister_gates(APMIXED_CLKS.as_ptr(), ARRAY_SIZE!(APMIXED_CLKS), clk_data);
unregister_plls:
    mtk_clk_unregister_pllfhs(PLLS.as_ptr(), ARRAY_SIZE!(PLLS), PLLFHS.as_mut_ptr(), ARRAY_SIZE!(PLLFHS), clk_data);
free_clk_data:
    mtk_free_clk_data(clk_data); r
}

unsafe fn clk_mt8192_apmixed_remove(pdev: *mut platform_device) {
    let node = (*pdev).dev.of_node;
    let clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider(node);
    mtk_clk_unregister_gates(APMIXED_CLKS.as_ptr(), ARRAY_SIZE!(APMIXED_CLKS), clk_data);
    mtk_clk_unregister_pllfhs(PLLS.as_ptr(), ARRAY_SIZE!(PLLS), PLLFHS.as_mut_ptr(), ARRAY_SIZE!(PLLFHS), clk_data);
    mtk_free_clk_data(clk_data);
}

static mut CLK_MT8192_APMIXED_DRV: platform_driver = platform_driver {
    driver: driver { name: "clk-mt8192-apmixed", of_match_table: OF_MATCH_CLK_MT8192_APMIXED.as_ptr() },
    probe: clk_mt8192_apmixed_probe,
    remove: clk_mt8192_apmixed_remove,
};

module_platform_driver!(CLK_MT8192_APMIXED_DRV);
MODULE_DESCRIPTION!("MediaTek MT8192 apmixed clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
