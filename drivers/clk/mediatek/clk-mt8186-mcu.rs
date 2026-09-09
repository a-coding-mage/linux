// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// C dependencies: linux/clk-provider.h, linux/platform_device.h,
// dt-bindings/clock/mt8186-clk.h, and clk-mtk.h.

static MCU_ARMPLL_LL_PARENTS: [&str; 4] = [
    "clk26m",
    "armpll_ll",
    "mainpll",
    "univpll_d2",
];

static MCU_ARMPLL_BL_PARENTS: [&str; 4] = [
    "clk26m",
    "armpll_bl",
    "mainpll",
    "univpll_d2",
];

static MCU_ARMPLL_BUS_PARENTS: [&str; 4] = [
    "clk26m",
    "ccipll",
    "mainpll",
    "univpll_d2",
];

/*
 * We only configure the CPU muxes when adjust CPU frequency in MediaTek CPUFreq Driver.
 * Other fields like divider always keep the same value. (set once in bootloader)
 */
static mut MCU_MUXES: [mtk_composite; 3] = [
    /* CPU_PLLDIV_CFG0 */
    MUX!(CLK_MCU_ARMPLL_LL_SEL, "mcu_armpll_ll_sel", MCU_ARMPLL_LL_PARENTS, 0x2A0, 9, 2),
    /* CPU_PLLDIV_CFG1 */
    MUX!(CLK_MCU_ARMPLL_BL_SEL, "mcu_armpll_bl_sel", MCU_ARMPLL_BL_PARENTS, 0x2A4, 9, 2),
    /* BUS_PLLDIV_CFG */
    MUX!(CLK_MCU_ARMPLL_BUS_SEL, "mcu_armpll_bus_sel", MCU_ARMPLL_BUS_PARENTS, 0x2E0, 9, 2),
];

static MCU_DESC: mtk_clk_desc = mtk_clk_desc {
    composite_clks: unsafe { &mut MCU_MUXES },
    num_composite_clks: MCU_MUXES.len(),
};

static OF_MATCH_CLK_MT8186_MCU: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8186-mcusys",
        data: &MCU_DESC,
    },
    of_device_id::sentinel(),
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT8186_MCU);

static mut CLK_MT8186_MCU_DRV: platform_driver = platform_driver {
    driver: driver {
        name: "clk-mt8186-mcu",
        of_match_table: &OF_MATCH_CLK_MT8186_MCU,
    },
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
};

module_platform_driver!(CLK_MT8186_MCU_DRV);

MODULE_DESCRIPTION!("MediaTek MT8186 mcusys clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
