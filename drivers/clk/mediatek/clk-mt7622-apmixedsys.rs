// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Copyright (c) 2023 Collabora, Ltd.
 *               AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// C dependencies:
// <dt-bindings/clock/mt7622-clk.h>, <linux/clk.h>, <linux/of.h>,
// <linux/platform_device.h>, "clk-gate.h", "clk-mtk.h", "clk-pll.h"

const MT7622_PLL_FMAX: usize = 2500usize * MHZ;
const CON0_MT7622_RST_BAR: u32 = 1u32 << 27;

// C macro PLL_xtal(_id, _name, _reg, _pwr_reg, _en_mask, _flags, _pcwbits,
// _pd_reg, _pd_shift, _tuner_reg, _pcw_reg, _pcw_shift, _div_table,
// _parent_name) initializes an mtk_pll_data value.
macro_rules! pll_xtal {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr, $flags:expr,
     $pcwbits:expr, $pd_reg:expr, $pd_shift:expr, $tuner_reg:expr, $pcw_reg:expr,
     $pcw_shift:expr, $div_table:expr, $parent_name:expr) => {
        MtkPllData {
            id: $id, name: $name, reg: $reg, pwr_reg: $pwr_reg, en_mask: $en_mask,
            flags: $flags, rst_bar_mask: CON0_MT7622_RST_BAR, fmax: MT7622_PLL_FMAX,
            pcwbits: $pcwbits, pd_reg: $pd_reg, pd_shift: $pd_shift,
            tuner_reg: $tuner_reg, pcw_reg: $pcw_reg, pcw_shift: $pcw_shift,
            div_table: $div_table, parent_name: $parent_name,
        }
    };
}

macro_rules! pll {
    ($id:expr, $name:expr, $reg:expr, $pwr_reg:expr, $en_mask:expr, $flags:expr,
     $pcwbits:expr, $pd_reg:expr, $pd_shift:expr, $tuner_reg:expr, $pcw_reg:expr,
     $pcw_shift:expr) => {
        pll_xtal!($id, $name, $reg, $pwr_reg, $en_mask, $flags, $pcwbits,
                  $pd_reg, $pd_shift, $tuner_reg, $pcw_reg, $pcw_shift,
                  core::ptr::null(), "clkxtal")
    };
}

static AP混合_CG_REGS: MtkGateRegs = MtkGateRegs { set_ofs: 0x8, clr_ofs: 0x8, sta_ofs: 0x8 };

macro_rules! gate_apmixed_ao {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk_flags!($id, $name, $parent, &AP混合_CG_REGS, $shift,
                        &mtk_clk_gate_ops_no_setclr_inv, CLK_IS_CRITICAL)
    };
}

static PLLS: [MtkPllData; 9] = [
    pll!(CLK_APMIXED_ARMPLL, "armpll", 0x0200, 0x020C, 0, PLL_AO, 21, 0x0204, 24, 0, 0x0204, 0),
    pll!(CLK_APMIXED_MAINPLL, "mainpll", 0x0210, 0x021C, 0, HAVE_RST_BAR, 21, 0x0214, 24, 0, 0x0214, 0),
    pll!(CLK_APMIXED_UNIV2PLL, "univ2pll", 0x0220, 0x022C, 0, HAVE_RST_BAR, 7, 0x0224, 24, 0, 0x0224, 14),
    pll!(CLK_APMIXED_ETH1PLL, "eth1pll", 0x0300, 0x0310, 0, 0, 21, 0x0300, 1, 0, 0x0304, 0),
    pll!(CLK_APMIXED_ETH2PLL, "eth2pll", 0x0314, 0x0320, 0, 0, 21, 0x0314, 1, 0, 0x0318, 0),
    pll!(CLK_APMIXED_AUD1PLL, "aud1pll", 0x0324, 0x0330, 0, 0, 31, 0x0324, 1, 0, 0x0328, 0),
    pll!(CLK_APMIXED_AUD2PLL, "aud2pll", 0x0334, 0x0340, 0, 0, 31, 0x0334, 1, 0, 0x0338, 0),
    pll!(CLK_APMIXED_TRGPLL, "trgpll", 0x0344, 0x0354, 0, 0, 21, 0x0344, 1, 0, 0x0348, 0),
    pll!(CLK_APMIXED_SGMIPLL, "sgmipll", 0x0358, 0x0368, 0, 0, 21, 0x0358, 1, 0, 0x035C, 0),
];

static APMIXED_CLKS: [MtkGate; 1] = [
    gate_apmixed_ao!(CLK_APMIXED_MAIN_CORE_EN, "main_core_en", "mainpll", 5),
];

unsafe fn clk_mt7622_apmixed_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut base: *mut core::ffi::c_void;
    let mut clk_data: *mut ClkHwOnecellData;
    let node = (*pdev).dev.of_node;
    let dev = &mut (*pdev).dev;
    let mut ret: i32;

    base = devm_platform_ioremap_resource(pdev, 0);
    if is_err(base) { return ptr_err(base); }

    clk_data = mtk_devm_alloc_clk_data(dev, CLK_APMIXED_NR_CLK);
    if clk_data.is_null() { return -ENOMEM; }

    ret = mtk_clk_register_plls(dev, PLLS.as_ptr(), PLLS.len(), clk_data);
    if ret != 0 { return ret; }

    ret = mtk_clk_register_gates(dev, node, APMIXED_CLKS.as_ptr(), APMIXED_CLKS.len(), clk_data);
    if ret != 0 { goto_unregister_plls!(); }

    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if ret != 0 { goto_unregister_gates!(); }
    return 0;

    goto_unregister_gates!();
    mtk_clk_unregister_gates(APMIXED_CLKS.as_ptr(), APMIXED_CLKS.len(), clk_data);
    goto_unregister_plls!();
    mtk_clk_unregister_plls(PLLS.as_ptr(), PLLS.len(), clk_data);
    ret
}

unsafe fn clk_mt7622_apmixed_remove(pdev: *mut PlatformDevice) {
    let node = (*pdev).dev.of_node;
    let clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider(node);
    mtk_clk_unregister_gates(APMIXED_CLKS.as_ptr(), APMIXED_CLKS.len(), clk_data);
    mtk_clk_unregister_plls(PLLS.as_ptr(), PLLS.len(), clk_data);
}

static OF_MATCH_CLK_MT7622_APMIXED: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "mediatek,mt7622-apmixedsys" },
    OfDeviceId { compatible: core::ptr::null() },
];

static mut CLK_MT7622_APMIXED_DRV: PlatformDriver = PlatformDriver {
    probe: Some(clk_mt7622_apmixed_probe),
    remove: Some(clk_mt7622_apmixed_remove),
    driver: Driver { name: "clk-mt7622-apmixed", of_match_table: OF_MATCH_CLK_MT7622_APMIXED.as_ptr() },
};

// module_platform_driver!(CLK_MT7622_APMIXED_DRV);
// MODULE_DESCRIPTION("MediaTek MT7622 apmixedsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
