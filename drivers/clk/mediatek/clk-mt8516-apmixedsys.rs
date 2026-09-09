// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2019 MediaTek Inc.
 *               James Liao <jamesjj.liao@mediatek.com>
 *               Fabien Parent <fparent@baylibre.com>
 *
 * Copyright (c) 2023 Collabora, Ltd.
 *               AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the kernel clock, device-tree, and platform-driver bindings.

const MT8516_PLL_FMAX: u64 = 1502u64 * MHZ;
const CON0_MT8516_RST_BAR: u32 = 1u32 << 27;

macro_rules! PLL_B {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr, $flags:expr,
     $pcwbits:expr, $pd_reg:expr, $pd_shift:expr, $tuner_reg:expr, $pcw_reg:expr,
     $pcw_shift:expr, $div_table:expr) => {
        mtk_pll_data {
            id: $id,
            name: $name,
            reg: $reg,
            pwr_reg: $pwr_reg,
            en_mask: $en_mask,
            flags: $flags,
            rst_bar_mask: CON0_MT8516_RST_BAR,
            fmax: MT8516_PLL_FMAX,
            pcwbits: $pcwbits,
            pd_reg: $pd_reg,
            pd_shift: $pd_shift,
            tuner_reg: $tuner_reg,
            pcw_reg: $pcw_reg,
            pcw_shift: $pcw_shift,
            div_table: $div_table,
        }
    };
}

macro_rules! PLL {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr, $flags:expr,
     $pcwbits:expr, $pd_reg:expr, $pd_shift:expr, $tuner_reg:expr, $pcw_reg:expr,
     $pcw_shift:expr) => {
        PLL_B!($id, $name, $reg, $pwr_reg, $en_mask, $flags, $pcwbits,
               $pd_reg, $pd_shift, $tuner_reg, $pcw_reg, $pcw_shift, core::ptr::null())
    };
}

static mmpll_div_table: [mtk_pll_div_table; 6] = [
    mtk_pll_div_table { div: 0, freq: MT8516_PLL_FMAX },
    mtk_pll_div_table { div: 1, freq: 1000000000 },
    mtk_pll_div_table { div: 2, freq: 604500000 },
    mtk_pll_div_table { div: 3, freq: 253500000 },
    mtk_pll_div_table { div: 4, freq: 126750000 },
    mtk_pll_div_table { div: 0, freq: 0 }, // sentinel
];

static plls: [mtk_pll_data; 6] = [
    PLL!(CLK_APMIXED_ARMPLL, "armpll", 0x0100, 0x0110, 0, 0,
         21, 0x0104, 24, 0, 0x0104, 0),
    PLL!(CLK_APMIXED_MAINPLL, "mainpll", 0x0120, 0x0130, 0,
         HAVE_RST_BAR, 21, 0x0124, 24, 0, 0x0124, 0),
    PLL!(CLK_APMIXED_UNIVPLL, "univpll", 0x0140, 0x0150, 0x30000000,
         HAVE_RST_BAR, 7, 0x0144, 24, 0, 0x0144, 0),
    PLL_B!(CLK_APMIXED_MMPLL, "mmpll", 0x0160, 0x0170, 0, 0,
           21, 0x0164, 24, 0, 0x0164, 0, mmpll_div_table.as_ptr()),
    PLL!(CLK_APMIXED_APLL1, "apll1", 0x0180, 0x0190, 0, 0,
         31, 0x0180, 1, 0x0194, 0x0184, 0),
    PLL!(CLK_APMIXED_APLL2, "apll2", 0x01A0, 0x01B0, 0, 0,
         31, 0x01A0, 1, 0x01B4, 0x01A4, 0),
];

unsafe fn clk_mt8516_apmixed_probe(pdev: *mut platform_device) -> i32 {
    let base: *mut core::ffi::c_void;
    let clk_data: *mut clk_hw_onecell_data;
    let node = (*pdev).dev.of_node;
    let dev = &mut (*pdev).dev;
    let mut ret: i32;

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    clk_data = mtk_devm_alloc_clk_data(dev, CLK_APMIXED_NR_CLK);
    if clk_data.is_null() {
        return -ENOMEM;
    }

    ret = mtk_clk_register_plls(dev, plls.as_ptr(), plls.len(), clk_data);
    if ret != 0 {
        return ret;
    }

    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if ret != 0 {
        mtk_clk_unregister_plls(plls.as_ptr(), plls.len(), clk_data);
        return ret;
    }

    return 0;
}

static of_match_clk_mt8516_apmixed: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt8516-apmixedsys" },
    of_device_id { /* sentinel */ },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt8516_apmixed);

static mut clk_mt8516_apmixed_drv: platform_driver = platform_driver {
    probe: Some(clk_mt8516_apmixed_probe),
    driver: driver {
        name: "clk-mt8516-apmixed",
        of_match_table: of_match_clk_mt8516_apmixed.as_ptr(),
    },
};

builtin_platform_driver!(clk_mt8516_apmixed_drv);

MODULE_DESCRIPTION!("MediaTek MT8516 apmixedsys clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
