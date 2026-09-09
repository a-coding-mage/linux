// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */

// Dependencies supplied by the surrounding kernel clock implementation.

/* APMIXEDSYS PLL control register offsets */
const MAINPLL_CON0: u32 = 0x250;
const MAINPLL_CON1: u32 = 0x254;
const UNIVPLL_CON0: u32 = 0x264;
const UNIVPLL_CON1: u32 = 0x268;
const MSDCPLL_CON0: u32 = 0x278;
const MSDCPLL_CON1: u32 = 0x27c;
const ADSPPLL_CON0: u32 = 0x28c;
const ADSPPLL_CON1: u32 = 0x290;
const EMIPLL_CON0: u32 = 0x2a0;
const EMIPLL_CON1: u32 = 0x2a4;
const EMIPLL2_CON0: u32 = 0x2b4;
const EMIPLL2_CON1: u32 = 0x2b8;
const NET1PLL_CON0: u32 = 0x2c8;
const NET1PLL_CON1: u32 = 0x2cc;
const SGMIIPLL_CON0: u32 = 0x2dc;
const SGMIIPLL_CON1: u32 = 0x2e0;

/* APMIXEDSYS_GP2 PLL control register offsets */
const MAINPLL2_CON0: u32 = 0x250;
const MAINPLL2_CON1: u32 = 0x254;
const UNIVPLL2_CON0: u32 = 0x264;
const UNIVPLL2_CON1: u32 = 0x268;
const MMPLL2_CON0: u32 = 0x278;
const MMPLL2_CON1: u32 = 0x27c;
const IMGPLL_CON0: u32 = 0x28c;
const IMGPLL_CON1: u32 = 0x290;
const TVDPLL1_CON0: u32 = 0x2a0;
const TVDPLL1_CON1: u32 = 0x2a4;
const TVDPLL2_CON0: u32 = 0x2b4;
const TVDPLL2_CON1: u32 = 0x2b8;
const TVDPLL3_CON0: u32 = 0x2c8;
const TVDPLL3_CON1: u32 = 0x2cc;

const PLLEN_ALL: u32 = 0x080;
const PLLEN_ALL_SET: u32 = 0x084;
const PLLEN_ALL_CLR: u32 = 0x088;
const FENC_STATUS_CON0: u32 = 0x03c;
const MT8196_PLL_FMAX: u64 = 3800u64 * MHZ as u64;
const MT8196_PLL_FMIN: u64 = 1500u64 * MHZ as u64;
const MT8196_INTEGER_BITS: u32 = 8;

macro_rules! pll_fenc {
    ($id:expr, $name:expr, $reg:expr, $fenc_sta_ofs:expr, $fenc_sta_bit:expr,
     $flags:expr, $pd_reg:expr, $pd_shift:expr, $pcw_reg:expr, $pcw_shift:expr,
     $pcwbits:expr, $pll_en_bit:expr) => {
        mtk_pll_data {
            id: $id,
            name: $name,
            reg: $reg,
            fenc_sta_ofs: $fenc_sta_ofs,
            fenc_sta_bit: $fenc_sta_bit,
            flags: $flags,
            fmax: MT8196_PLL_FMAX,
            fmin: MT8196_PLL_FMIN,
            pd_reg: $pd_reg,
            pd_shift: $pd_shift,
            pcw_reg: $pcw_reg,
            pcw_shift: $pcw_shift,
            pcwbits: $pcwbits,
            pcwibits: MT8196_INTEGER_BITS,
            en_reg: PLLEN_ALL,
            en_set_reg: PLLEN_ALL_SET,
            en_clr_reg: PLLEN_ALL_CLR,
            pll_en_bit: $pll_en_bit,
            ops: &mtk_pll_fenc_clr_set_ops,
        }
    };
}

#[repr(C)]
struct mtk_pll_desc {
    clks: *const mtk_pll_data,
    num_clks: usize,
}

static APMIXED_PLLS: [mtk_pll_data; 8] = [
    pll_fenc!(CLK_APMIXED_MAINPLL, "mainpll", MAINPLL_CON0, FENC_STATUS_CON0, 7, PLL_AO, MAINPLL_CON1, 24, MAINPLL_CON1, 0, 22, 0),
    pll_fenc!(CLK_APMIXED_UNIVPLL, "univpll", UNIVPLL_CON0, FENC_STATUS_CON0, 6, 0, UNIVPLL_CON1, 24, UNIVPLL_CON1, 0, 22, 1),
    pll_fenc!(CLK_APMIXED_MSDCPLL, "msdcpll", MSDCPLL_CON0, FENC_STATUS_CON0, 5, 0, MSDCPLL_CON1, 24, MSDCPLL_CON1, 0, 22, 2),
    pll_fenc!(CLK_APMIXED_ADSPPLL, "adsppll", ADSPPLL_CON0, FENC_STATUS_CON0, 4, 0, ADSPPLL_CON1, 24, ADSPPLL_CON1, 0, 22, 3),
    pll_fenc!(CLK_APMIXED_EMIPLL, "emipll", EMIPLL_CON0, FENC_STATUS_CON0, 3, PLL_AO, EMIPLL_CON1, 24, EMIPLL_CON1, 0, 22, 4),
    pll_fenc!(CLK_APMIXED_EMIPLL2, "emipll2", EMIPLL2_CON0, FENC_STATUS_CON0, 2, PLL_AO, EMIPLL2_CON1, 24, EMIPLL2_CON1, 0, 22, 5),
    pll_fenc!(CLK_APMIXED_NET1PLL, "net1pll", NET1PLL_CON0, FENC_STATUS_CON0, 1, 0, NET1PLL_CON1, 24, NET1PLL_CON1, 0, 22, 6),
    pll_fenc!(CLK_APMIXED_SGMIIPLL, "sgmiipll", SGMIIPLL_CON0, FENC_STATUS_CON0, 0, 0, SGMIIPLL_CON1, 24, SGMIIPLL_CON1, 0, 22, 7),
];

static APMIXED_DESC: mtk_pll_desc = mtk_pll_desc { clks: APMIXED_PLLS.as_ptr(), num_clks: APMIXED_PLLS.len() };

static APMIXED2_PLLS: [mtk_pll_data; 7] = [
    pll_fenc!(CLK_APMIXED2_MAINPLL2, "mainpll2", MAINPLL2_CON0, FENC_STATUS_CON0, 6, 0, MAINPLL2_CON1, 24, MAINPLL2_CON1, 0, 22, 0),
    pll_fenc!(CLK_APMIXED2_UNIVPLL2, "univpll2", UNIVPLL2_CON0, FENC_STATUS_CON0, 5, 0, UNIVPLL2_CON1, 24, UNIVPLL2_CON1, 0, 22, 1),
    pll_fenc!(CLK_APMIXED2_MMPLL2, "mmpll2", MMPLL2_CON0, FENC_STATUS_CON0, 4, 0, MMPLL2_CON1, 24, MMPLL2_CON1, 0, 22, 2),
    pll_fenc!(CLK_APMIXED2_IMGPLL, "imgpll", IMGPLL_CON0, FENC_STATUS_CON0, 3, 0, IMGPLL_CON1, 24, IMGPLL_CON1, 0, 22, 3),
    pll_fenc!(CLK_APMIXED2_TVDPLL1, "tvdpll1", TVDPLL1_CON0, FENC_STATUS_CON0, 2, 0, TVDPLL1_CON1, 24, TVDPLL1_CON1, 0, 22, 4),
    pll_fenc!(CLK_APMIXED2_TVDPLL2, "tvdpll2", TVDPLL2_CON0, FENC_STATUS_CON0, 1, 0, TVDPLL2_CON1, 24, TVDPLL2_CON1, 0, 22, 5),
    pll_fenc!(CLK_APMIXED2_TVDPLL3, "tvdpll3", TVDPLL3_CON0, FENC_STATUS_CON0, 0, 0, TVDPLL3_CON1, 24, TVDPLL3_CON1, 0, 22, 6),
];

static APMIXED2_DESC: mtk_pll_desc = mtk_pll_desc { clks: APMIXED2_PLLS.as_ptr(), num_clks: APMIXED2_PLLS.len() };

// The remaining platform-driver definitions are supplied through the kernel's Rust bindings.
unsafe fn clk_mt8196_apmixed_probe(pdev: *mut platform_device) -> i32 {
    let mut clk_data: *mut clk_hw_onecell_data;
    let node = (*(*pdev).dev.of_node);
    let mcd: *const mtk_pll_desc;
    let mut r: i32;

    mcd = device_get_match_data(&(*pdev).dev);
    if mcd.is_null() { return -EINVAL; }
    clk_data = mtk_alloc_clk_data((*mcd).num_clks);
    if clk_data.is_null() { return -ENOMEM; }
    r = mtk_clk_register_plls(&mut (*pdev).dev, (*mcd).clks, (*mcd).num_clks, clk_data);
    if r != 0 { mtk_free_clk_data(clk_data); return r; }
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 { mtk_clk_unregister_plls((*mcd).clks, (*mcd).num_clks, clk_data); mtk_free_clk_data(clk_data); return r; }
    platform_set_drvdata(pdev, clk_data);
    r
}

unsafe fn clk_mt8196_apmixed_remove(pdev: *mut platform_device) {
    let mcd = device_get_match_data(&(*pdev).dev);
    let clk_data = platform_get_drvdata(pdev);
    let node = (*pdev).dev.of_node;
    of_clk_del_provider(node);
    mtk_clk_unregister_plls((*mcd).clks, (*mcd).num_clks, clk_data);
    mtk_free_clk_data(clk_data);
}

static OF_MATCH_CLK_MT8196_APMIXED: [of_device_id; 3] = [
    of_device_id { compatible: "mediatek,mt8196-apmixedsys", data: &APMIXED_DESC },
    of_device_id { compatible: "mediatek,mt8196-apmixedsys-gp2", data: &APMIXED2_DESC },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut CLK_MT8196_APMIXED_DRV: platform_driver = platform_driver {
    probe: Some(clk_mt8196_apmixed_probe),
    remove: Some(clk_mt8196_apmixed_remove),
    driver: driver {
        name: "clk-mt8196-apmixed",
        of_match_table: OF_MATCH_CLK_MT8196_APMIXED.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8196_apmixed);
// module_platform_driver(clk_mt8196_apmixed_drv);
// MODULE_DESCRIPTION("MediaTek MT8196 apmixedsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
