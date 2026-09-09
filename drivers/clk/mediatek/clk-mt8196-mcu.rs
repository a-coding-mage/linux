// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */
// Dependencies supplied by the kernel clock, device-tree, and platform code.

const ARMPLL_LL_CON0: u32 = 0x008;
const ARMPLL_LL_CON1: u32 = 0x00c;
const ARMPLL_LL_CON2: u32 = 0x010;
const ARMPLL_LL_CON3: u32 = 0x014;
const ARMPLL_BL_CON0: u32 = 0x008;
const ARMPLL_BL_CON1: u32 = 0x00c;
const ARMPLL_BL_CON2: u32 = 0x010;
const ARMPLL_BL_CON3: u32 = 0x014;
const ARMPLL_B_CON0: u32 = 0x008;
const ARMPLL_B_CON1: u32 = 0x00c;
const ARMPLL_B_CON2: u32 = 0x010;
const ARMPLL_B_CON3: u32 = 0x014;
const CCIPLL_CON0: u32 = 0x008;
const CCIPLL_CON1: u32 = 0x00c;
const CCIPLL_CON2: u32 = 0x010;
const CCIPLL_CON3: u32 = 0x014;
const PTPPLL_CON0: u32 = 0x008;
const PTPPLL_CON1: u32 = 0x00c;
const PTPPLL_CON2: u32 = 0x010;
const PTPPLL_CON3: u32 = 0x014;

const MT8196_PLL_FMAX: u64 = 3800u64 * MHZ;
const MT8196_PLL_FMIN: u64 = 1500u64 * MHZ;
const MT8196_INTEGER_BITS: u32 = 8;

static CPU_BL_PLLS: [mtk_pll_data; 1] = [mtk_pll_data {
    id: CLK_CPBL_ARMPLL_BL, name: c"armpll-bl", reg: ARMPLL_BL_CON0,
    en_reg: ARMPLL_BL_CON0, en_mask: 0, pll_en_bit: 0, flags: PLL_AO,
    rst_bar_mask: BIT(0), fmax: MT8196_PLL_FMAX, fmin: MT8196_PLL_FMIN,
    pd_reg: ARMPLL_BL_CON1, pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0,
    tuner_en_bit: 0, pcw_reg: ARMPLL_BL_CON1, pcw_shift: 0, pcwbits: 22,
    pcwibits: MT8196_INTEGER_BITS,
}];

static CPU_B_PLLS: [mtk_pll_data; 1] = [mtk_pll_data {
    id: CLK_CPB_ARMPLL_B, name: c"armpll-b", reg: ARMPLL_B_CON0,
    en_reg: ARMPLL_B_CON0, en_mask: 0, pll_en_bit: 0, flags: PLL_AO,
    rst_bar_mask: BIT(0), fmax: MT8196_PLL_FMAX, fmin: MT8196_PLL_FMIN,
    pd_reg: ARMPLL_B_CON1, pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0,
    tuner_en_bit: 0, pcw_reg: ARMPLL_B_CON1, pcw_shift: 0, pcwbits: 22,
    pcwibits: MT8196_INTEGER_BITS,
}];

static CPU_LL_PLLS: [mtk_pll_data; 1] = [mtk_pll_data {
    id: CLK_CPLL_ARMPLL_LL, name: c"armpll-ll", reg: ARMPLL_LL_CON0,
    en_reg: ARMPLL_LL_CON0, en_mask: 0, pll_en_bit: 0, flags: PLL_AO,
    rst_bar_mask: BIT(0), fmax: MT8196_PLL_FMAX, fmin: MT8196_PLL_FMIN,
    pd_reg: ARMPLL_LL_CON1, pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0,
    tuner_en_bit: 0, pcw_reg: ARMPLL_LL_CON1, pcw_shift: 0, pcwbits: 22,
    pcwibits: MT8196_INTEGER_BITS,
}];

static CCI_PLLS: [mtk_pll_data; 1] = [mtk_pll_data {
    id: CLK_CCIPLL, name: c"ccipll", reg: CCIPLL_CON0, en_reg: CCIPLL_CON0,
    en_mask: 0, pll_en_bit: 0, flags: PLL_AO, rst_bar_mask: BIT(0),
    fmax: MT8196_PLL_FMAX, fmin: MT8196_PLL_FMIN, pd_reg: CCIPLL_CON1,
    pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0, tuner_en_bit: 0,
    pcw_reg: CCIPLL_CON1, pcw_shift: 0, pcwbits: 22,
    pcwibits: MT8196_INTEGER_BITS,
}];

static PTP_PLLS: [mtk_pll_data; 1] = [mtk_pll_data {
    id: CLK_PTPPLL, name: c"ptppll", reg: PTPPLL_CON0, en_reg: PTPPLL_CON0,
    en_mask: 0, pll_en_bit: 0, flags: PLL_AO, rst_bar_mask: BIT(0),
    fmax: MT8196_PLL_FMAX, fmin: MT8196_PLL_FMIN, pd_reg: PTPPLL_CON1,
    pd_shift: 24, tuner_reg: 0, tuner_en_reg: 0, tuner_en_bit: 0,
    pcw_reg: PTPPLL_CON1, pcw_shift: 0, pcwbits: 22,
    pcwibits: MT8196_INTEGER_BITS,
}];

static OF_MATCH_CLK_MT8196_MCU: [of_device_id; 6] = [
    of_device_id { compatible: c"mediatek,mt8196-armpll-bl-pll-ctrl", data: &CPU_BL_PLLS },
    of_device_id { compatible: c"mediatek,mt8196-armpll-b-pll-ctrl", data: &CPU_B_PLLS },
    of_device_id { compatible: c"mediatek,mt8196-armpll-ll-pll-ctrl", data: &CPU_LL_PLLS },
    of_device_id { compatible: c"mediatek,mt8196-ccipll-pll-ctrl", data: &CCI_PLLS },
    of_device_id { compatible: c"mediatek,mt8196-ptppll-pll-ctrl", data: &PTP_PLLS },
    of_device_id { sentinel: true },
];

unsafe fn clk_mt8196_mcu_probe(pdev: *mut platform_device) -> c_int {
    let plls: *const mtk_pll_data;
    let clk_data: *mut clk_hw_onecell_data;
    let node = (*(*pdev).dev.of_node);
    let num_plls: c_int = 1;
    let mut r: c_int;

    plls = of_device_get_match_data(&(*pdev).dev);
    if plls.is_null() { return -EINVAL; }
    clk_data = mtk_alloc_clk_data(num_plls);
    if clk_data.is_null() { return -ENOMEM; }
    r = mtk_clk_register_plls(&mut (*pdev).dev, plls, num_plls, clk_data);
    if r != 0 { mtk_free_clk_data(clk_data); return r; }
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 { mtk_clk_unregister_plls(plls, num_plls, clk_data); mtk_free_clk_data(clk_data); return r; }
    platform_set_drvdata(pdev, clk_data);
    r
}

unsafe fn clk_mt8196_mcu_remove(pdev: *mut platform_device) {
    let plls = of_device_get_match_data(&(*pdev).dev);
    let clk_data = platform_get_drvdata(pdev);
    let node = (*pdev).dev.of_node;
    of_clk_del_provider(node);
    mtk_clk_unregister_plls(plls, 1, clk_data);
    mtk_free_clk_data(clk_data);
}

static mut CLK_MT8196_MCU_DRV: platform_driver = platform_driver {
    probe: Some(clk_mt8196_mcu_probe),
    remove: Some(clk_mt8196_mcu_remove),
    driver: driver { name: c"clk-mt8196-mcu", of_match_table: OF_MATCH_CLK_MT8196_MCU.as_ptr() },
};

// Equivalent of module_platform_driver(CLK_MT8196_MCU_DRV).
// MODULE_DESCRIPTION("MediaTek MT8196 mcusys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
