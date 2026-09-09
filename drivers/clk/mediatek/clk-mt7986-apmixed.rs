// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Sam Shih <sam.shih@mediatek.com>
 * Author: Wenzhen Yu <wenzhen.yu@mediatek.com>
 */

// Dependencies supplied by the Linux clock, platform, device-tree, and
// MediaTek clock headers are intentionally left as external symbols.

const MT7986_PLL_FMAX: u64 = 2500u64 * MHZ;
const CON0_MT7986_RST_BAR: u32 = BIT(27);

macro_rules! pll_xtal {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr,
     $flags:expr, $pcwbits:expr, $pd_reg:expr, $pd_shift:expr,
     $tuner_reg:expr, $pcw_reg:expr, $pcw_shift:expr, $div_table:expr,
     $parent_name:expr) => {
        MtkPllData {
            id: $id,
            name: $name,
            reg: $reg,
            pwr_reg: $pwr_reg,
            en_mask: $en_mask,
            flags: $flags,
            rst_bar_mask: CON0_MT7986_RST_BAR,
            fmax: MT7986_PLL_FMAX,
            pcwbits: $pcwbits,
            pd_reg: $pd_reg,
            pd_shift: $pd_shift,
            tuner_reg: $tuner_reg,
            pcw_reg: $pcw_reg,
            pcw_shift: $pcw_shift,
            div_table: $div_table,
            parent_name: $parent_name,
        }
    };
}

macro_rules! pll {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr,
     $flags:expr, $pcwbits:expr, $pd_reg:expr, $pd_shift:expr,
     $tuner_reg:expr, $pcw_reg:expr, $pcw_shift:expr) => {
        pll_xtal!($id, $name, $reg, $pwr_reg, $en_mask, $flags, $pcwbits,
                  $pd_reg, $pd_shift, $tuner_reg, $pcw_reg, $pcw_shift,
                  core::ptr::null(), "clkxtal")
    };
}

static PLLS: &[MtkPllData] = &[
    pll!(CLK_APMIXED_ARMPLL, "armpll", 0x0200, 0x020C, 0x0, PLL_AO, 32,
         0x0200, 4, 0, 0x0204, 0),
    pll!(CLK_APMIXED_NET2PLL, "net2pll", 0x0210, 0x021C, 0x0, 0, 32,
         0x0210, 4, 0, 0x0214, 0),
    pll!(CLK_APMIXED_MMPLL, "mmpll", 0x0220, 0x022C, 0x0, 0, 32,
         0x0220, 4, 0, 0x0224, 0),
    pll!(CLK_APMIXED_SGMPLL, "sgmpll", 0x0230, 0x023c, 0x0, 0, 32,
         0x0230, 4, 0, 0x0234, 0),
    pll!(CLK_APMIXED_WEDMCUPLL, "wedmcupll", 0x0240, 0x024c, 0x0, 0,
         32, 0x0240, 4, 0, 0x0244, 0),
    pll!(CLK_APMIXED_NET1PLL, "net1pll", 0x0250, 0x025c, 0x0, 0, 32,
         0x0250, 4, 0, 0x0254, 0),
    pll!(CLK_APMIXED_MPLL, "mpll", 0x0260, 0x0270, 0x0, 0, 32,
         0x0260, 4, 0, 0x0264, 0),
    pll!(CLK_APMIXED_APLL2, "apll2", 0x0278, 0x0288, 0x0, 0, 32,
         0x0278, 4, 0, 0x027c, 0),
];

static OF_MATCH_CLK_MT7986_APMIXED: &[OfDeviceId] = &[
    OfDeviceId { compatible: "mediatek,mt7986-apmixedsys" },
    OfDeviceId { ..Default::default() },
];

unsafe fn clk_mt7986_apmixed_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut clk_data: *mut ClkHwOnecellData;
    let node = (*(*pdev).dev).of_node;
    let r: i32;

    clk_data = mtk_alloc_clk_data(PLLS.len());
    if clk_data.is_null() {
        return -ENOMEM;
    }

    mtk_clk_register_plls(&mut (*pdev).dev, PLLS.as_ptr(), PLLS.len(), clk_data);

    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if r != 0 {
        pr_err!("{}(): could not register clock provider: {}\n",
                "clk_mt7986_apmixed_probe", r);
        mtk_free_clk_data(clk_data);
    }
    r
}

static mut CLK_MT7986_APMIXED_DRV: PlatformDriver = PlatformDriver {
    probe: Some(clk_mt7986_apmixed_probe),
    driver: Driver {
        name: "clk-mt7986-apmixed",
        of_match_table: OF_MATCH_CLK_MT7986_APMIXED,
    },
};

// Equivalent of builtin_platform_driver(clk_mt7986_apmixed_drv).
builtin_platform_driver!(CLK_MT7986_APMIXED_DRV);

module_description!("MediaTek MT7986 apmixedsys clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
