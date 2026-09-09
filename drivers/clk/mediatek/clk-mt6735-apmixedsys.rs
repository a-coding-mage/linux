// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Yassine Oudjana <y.oudjana@protonmail.com>
 */

// Translated from clk-mt6735-apmixedsys.c.
// Kernel headers and symbols referenced below are supplied by other files.

const AP_PLL_CON_5: u32 = 0x014;
const ARMPLL_CON0: u32 = 0x200;
const ARMPLL_CON1: u32 = 0x204;
const ARMPLL_PWR_CON0: u32 = 0x20c;
const MAINPLL_CON0: u32 = 0x210;
const MAINPLL_CON1: u32 = 0x214;
const MAINPLL_PWR_CON0: u32 = 0x21c;
const UNIVPLL_CON0: u32 = 0x220;
const UNIVPLL_CON1: u32 = 0x224;
const UNIVPLL_PWR_CON0: u32 = 0x22c;
const MMPLL_CON0: u32 = 0x230;
const MMPLL_CON1: u32 = 0x234;
const MMPLL_PWR_CON0: u32 = 0x23c;
const MSDCPLL_CON0: u32 = 0x240;
const MSDCPLL_CON1: u32 = 0x244;
const MSDCPLL_PWR_CON0: u32 = 0x24c;
const VENCPLL_CON0: u32 = 0x250;
const VENCPLL_CON1: u32 = 0x254;
const VENCPLL_PWR_CON0: u32 = 0x25c;
const TVDPLL_CON0: u32 = 0x260;
const TVDPLL_CON1: u32 = 0x264;
const TVDPLL_PWR_CON0: u32 = 0x26c;
const APLL1_CON0: u32 = 0x270;
const APLL1_CON1: u32 = 0x274;
const APLL1_CON2: u32 = 0x278;
const APLL1_PWR_CON0: u32 = 0x280;
const APLL2_CON0: u32 = 0x284;
const APLL2_CON1: u32 = 0x288;
const APLL2_CON2: u32 = 0x28c;
const APLL2_PWR_CON0: u32 = 0x294;

const CON0_RST_BAR: u32 = 1u32 << 24;

static AP___MIXEDSYS_PLLS: [mtk_pll_data; 9] = [
    mtk_pll_data { id: CLK_APMIXED_ARMPLL, name: "armpll", parent_name: "clk26m", reg: ARMPLL_CON0, pwr_reg: ARMPLL_PWR_CON0, en_mask: 0x00000001, rst_bar_mask: 0, pd_reg: ARMPLL_CON1, pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0, tuner_en_bit: 0, pcw_reg: ARMPLL_CON1, pcw_chg_reg: ARMPLL_CON1, pcwbits: 21, flags: PLL_AO },
    mtk_pll_data { id: CLK_APMIXED_MAINPLL, name: "mainpll", parent_name: "clk26m", reg: MAINPLL_CON0, pwr_reg: MAINPLL_PWR_CON0, en_mask: 0xf0000101, rst_bar_mask: CON0_RST_BAR, pd_reg: MAINPLL_CON1, pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0, tuner_en_bit: 0, pcw_reg: MAINPLL_CON1, pcw_chg_reg: MAINPLL_CON1, pcwbits: 21, flags: HAVE_RST_BAR },
    mtk_pll_data { id: CLK_APMIXED_UNIVPLL, name: "univpll", parent_name: "clk26m", reg: UNIVPLL_CON0, pwr_reg: UNIVPLL_PWR_CON0, en_mask: 0xfc000001, rst_bar_mask: CON0_RST_BAR, pd_reg: UNIVPLL_CON1, pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0, tuner_en_bit: 0, pcw_reg: UNIVPLL_CON1, pcw_chg_reg: UNIVPLL_CON1, pcwbits: 21, flags: HAVE_RST_BAR },
    mtk_pll_data { id: CLK_APMIXED_MMPLL, name: "mmpll", parent_name: "clk26m", reg: MMPLL_CON0, pwr_reg: MMPLL_PWR_CON0, en_mask: 0x00000001, rst_bar_mask: 0, pd_reg: MMPLL_CON1, pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0, tuner_en_bit: 0, pcw_reg: MMPLL_CON1, pcw_chg_reg: MMPLL_CON1, pcwbits: 21, flags: 0 },
    mtk_pll_data { id: CLK_APMIXED_MSDCPLL, name: "msdcpll", parent_name: "clk26m", reg: MSDCPLL_CON0, pwr_reg: MSDCPLL_PWR_CON0, en_mask: 0x00000001, rst_bar_mask: 0, pd_reg: MSDCPLL_CON1, pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0, tuner_en_bit: 0, pcw_reg: MSDCPLL_CON1, pcw_chg_reg: MSDCPLL_CON1, pcwbits: 21, flags: 0 },
    mtk_pll_data { id: CLK_APMIXED_VENCPLL, name: "vencpll", parent_name: "clk26m", reg: VENCPLL_CON0, pwr_reg: VENCPLL_PWR_CON0, en_mask: 0x00000001, rst_bar_mask: CON0_RST_BAR, pd_reg: VENCPLL_CON1, pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0, tuner_en_bit: 0, pcw_reg: VENCPLL_CON1, pcw_chg_reg: VENCPLL_CON1, pcwbits: 21, flags: HAVE_RST_BAR },
    mtk_pll_data { id: CLK_APMIXED_TVDPLL, name: "tvdpll", parent_name: "clk26m", reg: TVDPLL_CON0, pwr_reg: TVDPLL_PWR_CON0, en_mask: 0x00000001, rst_bar_mask: 0, pd_reg: TVDPLL_CON1, pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0, tuner_en_bit: 0, pcw_reg: TVDPLL_CON1, pcw_chg_reg: TVDPLL_CON1, pcwbits: 21, flags: 0 },
    mtk_pll_data { id: CLK_APMIXED_APLL1, name: "apll1", parent_name: "clk26m", reg: APLL1_CON0, pwr_reg: APLL1_PWR_CON0, en_mask: 0x00000001, rst_bar_mask: 0, pd_reg: APLL1_CON0, pd_shift: 4, tuner_reg: APLL1_CON2, tuner_en_reg: AP_PLL_CON_5, tuner_en_bit: 0, pcw_reg: APLL1_CON1, pcw_chg_reg: APLL1_CON1, pcwbits: 31, flags: 0 },
    mtk_pll_data { id: CLK_APMIXED_APLL2, name: "apll2", parent_name: "clk26m", reg: APLL2_CON0, pwr_reg: APLL2_PWR_CON0, en_mask: 0x00000001, rst_bar_mask: 0, pd_reg: APLL2_CON0, pd_shift: 4, tuner_reg: APLL2_CON2, tuner_en_reg: AP_PLL_CON_5, tuner_en_bit: 1, pcw_reg: APLL2_CON1, pcw_chg_reg: APLL2_CON1, pcwbits: 31, flags: 0 },
];

unsafe fn clk_mt6735_apmixed_probe(pdev: *mut platform_device) -> i32 {
    let base = devm_ioremap_resource(&mut (*pdev).dev, platform_get_resource(pdev, IORESOURCE_MEM, 0));
    if is_err(base) { return ptr_err(base); }

    let clk_data = mtk_devm_alloc_clk_data(&mut (*pdev).dev, AP___MIXEDSYS_PLLS.len());
    if clk_data.is_null() { return -ENOMEM; }
    platform_set_drvdata(pdev, clk_data);

    let mut ret = mtk_clk_register_plls(&mut (*pdev).dev, AP___MIXEDSYS_PLLS.as_ptr(), AP___MIXEDSYS_PLLS.len(), clk_data);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "Failed to register PLLs: %d\n", ret);
        return ret;
    }
    ret = devm_of_clk_add_hw_provider(&mut (*pdev).dev, of_clk_hw_onecell_get, clk_data);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "Failed to register clock provider: %d\n", ret);
        mtk_clk_unregister_plls(AP___MIXEDSYS_PLLS.as_ptr(), AP___MIXEDSYS_PLLS.len(), clk_data);
    }
    ret
}

unsafe fn clk_mt6735_apmixed_remove(pdev: *mut platform_device) {
    let clk_data = platform_get_drvdata(pdev);
    mtk_clk_unregister_plls(AP___MIXEDSYS_PLLS.as_ptr(), AP___MIXEDSYS_PLLS.len(), clk_data);
}

static OF_MATCH_MT6735_APMIXEDSYS: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt6735-apmixedsys" },
    of_device_id { /* sentinel */ },
];

static mut CLK_MT6735_APMIXEDSYS: platform_driver = platform_driver {
    probe: Some(clk_mt6735_apmixed_probe),
    remove: Some(clk_mt6735_apmixed_remove),
    driver: device_driver { name: "clk-mt6735-apmixedsys", of_match_table: OF_MATCH_MT6735_APMIXEDSYS.as_ptr() },
};

// MODULE_DEVICE_TABLE(of, of_match_mt6735_apmixedsys);
// module_platform_driver(clk_mt6735_apmixedsys);
// MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
// MODULE_DESCRIPTION("MediaTek MT6735 apmixedsys clock driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
