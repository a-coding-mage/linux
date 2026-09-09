// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 MediaTek Inc.
 * Copyright (c) 2020 BayLibre, SAS
 * Copyright (c) 2023 Collabora, Ltd.
 */

// Dependencies supplied by the kernel clock and platform-driver headers:
// dt-bindings/clock/mt8167-clk.h, linux/clk.h, linux/of.h,
// linux/platform_device.h, clk-pll.h, and clk-mtk.h.

static mut mt8167_apmixed_clk_lock: Spinlock = Spinlock::new();

const MT8167_PLL_FMAX: u64 = 2500u64 * MHZ;
const CON0_MT8167_RST_BAR: u32 = 1u32 << 27;

macro_rules! pll_b {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr,
     $flags:expr, $pcwbits:expr, $pd_reg:expr, $pd_shift:expr,
     $tuner_reg:expr, $pcw_reg:expr, $pcw_shift:expr, $div_table:expr) => {
        MtkPllData {
            id: $id, name: $name, reg: $reg, pwr_reg: $pwr_reg,
            en_mask: $en_mask, flags: $flags,
            rst_bar_mask: CON0_MT8167_RST_BAR, fmax: MT8167_PLL_FMAX,
            pcwbits: $pcwbits, pd_reg: $pd_reg, pd_shift: $pd_shift,
            tuner_reg: $tuner_reg, pcw_reg: $pcw_reg,
            pcw_shift: $pcw_shift, div_table: $div_table,
        }
    };
}

macro_rules! pll {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr,
     $flags:expr, $pcwbits:expr, $pd_reg:expr, $pd_shift:expr,
     $tuner_reg:expr, $pcw_reg:expr, $pcw_shift:expr) => {
        pll_b!($id, $name, $reg, $pwr_reg, $en_mask, $flags, $pcwbits,
               $pd_reg, $pd_shift, $tuner_reg, $pcw_reg, $pcw_shift,
               core::ptr::null())
    };
}

static mmpll_div_table: [MtkPllDivTable; 5] = [
    MtkPllDivTable { div: 0, freq: MT8167_PLL_FMAX },
    MtkPllDivTable { div: 1, freq: 1_000_000_000 },
    MtkPllDivTable { div: 2, freq: 604_500_000 },
    MtkPllDivTable { div: 3, freq: 253_500_000 },
    MtkPllDivTable { div: 4, freq: 126_750_000 },
];

static plls: [MtkPllData; 8] = [
    pll!(CLK_APMIXED_ARMPLL, "armpll", 0x0100, 0x0110, 0, 0, 21, 0x0104, 24, 0, 0x0104, 0),
    pll!(CLK_APMIXED_MAINPLL, "mainpll", 0x0120, 0x0130, 0, HAVE_RST_BAR, 21, 0x0124, 24, 0, 0x0124, 0),
    pll!(CLK_APMIXED_UNIVPLL, "univpll", 0x0140, 0x0150, 0x30000000, HAVE_RST_BAR, 7, 0x0144, 24, 0, 0x0144, 0),
    pll_b!(CLK_APMIXED_MMPLL, "mmpll", 0x0160, 0x0170, 0, 0, 21, 0x0164, 24, 0, 0x0164, 0, mmpll_div_table.as_ptr()),
    pll!(CLK_APMIXED_APLL1, "apll1", 0x0180, 0x0190, 0, 0, 31, 0x0180, 1, 0x0194, 0x0184, 0),
    pll!(CLK_APMIXED_APLL2, "apll2", 0x01A0, 0x01B0, 0, 0, 31, 0x01A0, 1, 0x01B4, 0x01A4, 0),
    pll!(CLK_APMIXED_TVDPLL, "tvdpll", 0x01C0, 0x01D0, 0, 0, 21, 0x01C4, 24, 0, 0x01C4, 0),
    pll!(CLK_APMIXED_LVDSPLL, "lvdspll", 0x01E0, 0x01F0, 0, 0, 21, 0x01E4, 24, 0, 0x01E4, 0),
];

macro_rules! div_adj_flag {
    ($id:expr, $name:expr, $parent:expr, $reg:expr, $shift:expr, $width:expr, $flag:expr) => {
        MtkClkDivider { id: $id, name: $name, parent_name: $parent,
            div_reg: $reg, div_shift: $shift, div_width: $width,
            clk_divider_flags: $flag }
    };
}

static adj_divs: [MtkClkDivider; 1] = [
    div_adj_flag!(CLK_APMIXED_HDMI_REF, "hdmi_ref", "tvdpll", 0x1c4, 24, 3, CLK_DIVIDER_POWER_OF_TWO),
];

unsafe fn clk_mt8167_apmixed_probe(pdev: *mut PlatformDevice) -> i32 {
    let base: *mut core::ffi::c_void;
    let clk_data: *mut ClkHwOnecellData;
    let node = (*pdev).dev.of_node;
    let dev = &mut (*pdev).dev;
    let mut ret: i32;

    base = devm_platform_ioremap_resource(pdev, 0);
    if is_err(base) { return ptr_err(base); }
    clk_data = mtk_devm_alloc_clk_data(dev, MT8167_CLK_APMIXED_NR_CLK);
    if clk_data.is_null() { return -12; }
    ret = mtk_clk_register_plls(dev, plls.as_ptr(), plls.len(), clk_data);
    if ret != 0 { return ret; }
    ret = mtk_clk_register_dividers(dev, adj_divs.as_ptr(), adj_divs.len(), base,
                                    &raw mut mt8167_apmixed_clk_lock, clk_data);
    if ret != 0 { goto_unregister_plls!(adj_divs, plls, clk_data, ret); }
    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if ret != 0 {
        mtk_clk_unregister_dividers(adj_divs.as_ptr(), adj_divs.len(), clk_data);
        mtk_clk_unregister_plls(plls.as_ptr(), plls.len(), clk_data);
        return ret;
    }
    0
}

static of_match_clk_mt8167_apmixed: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "mediatek,mt8167-apmixedsys" },
    OfDeviceId::sentinel(),
];

static mut clk_mt8167_apmixed_drv: PlatformDriver = PlatformDriver {
    probe: Some(clk_mt8167_apmixed_probe),
    driver: Driver { name: "clk-mt8167-apmixed", of_match_table: of_match_clk_mt8167_apmixed.as_ptr() },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8167_apmixed)
// builtin_platform_driver(clk_mt8167_apmixed_drv)
// MODULE_DESCRIPTION("MediaTek MT8167 apmixedsys clocks driver")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
