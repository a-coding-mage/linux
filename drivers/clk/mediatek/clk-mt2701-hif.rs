// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: Shunli Wang <shunli.wang@mediatek.com>
 */

// C dependencies supplied by the surrounding kernel translation.

static HIF_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    sta_ofs: 0x0030,
};

// Equivalent of GATE_HIF(_id, _name, _parent, _shift).
const fn gate_hif(id: u32, name: &'static str, parent: &'static str, shift: u32) -> mtk_gate {
    gate_mtk(
        id,
        name,
        parent,
        &HIF_CG_REGS,
        shift,
        &mtk_clk_gate_ops_no_setclr_inv,
    )
}

static HIF_CLKS: [mtk_gate; 6] = [
    gate_dummy(CLK_DUMMY, "hif_dummy"),
    gate_hif(CLK_HIFSYS_USB0PHY, "usb0_phy_clk", "ethpll_500m_ck", 21),
    gate_hif(CLK_HIFSYS_USB1PHY, "usb1_phy_clk", "ethpll_500m_ck", 22),
    gate_hif(CLK_HIFSYS_PCIE0, "pcie0_clk", "ethpll_500m_ck", 24),
    gate_hif(CLK_HIFSYS_PCIE1, "pcie1_clk", "ethpll_500m_ck", 25),
    gate_hif(CLK_HIFSYS_PCIE2, "pcie2_clk", "ethpll_500m_ck", 26),
];

static RST_OFS: [u16; 1] = [0x34];

static CLK_RST_DESC: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: &RST_OFS,
    rst_bank_nr: RST_OFS.len(),
};

static HIF_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &HIF_CLKS,
    num_clks: HIF_CLKS.len(),
    rst_desc: &CLK_RST_DESC,
};

static OF_MATCH_CLK_MT2701_HIF: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt2701-hifsys",
        data: &HIF_DESC,
    },
    of_device_id {
        // sentinel
        compatible: "",
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt2701_hif);

static mut CLK_MT2701_HIF_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt2701-hif",
        of_match_table: &OF_MATCH_CLK_MT2701_HIF,
    },
};

// module_platform_driver(clk_mt2701_hif_drv);
// MODULE_DESCRIPTION("MediaTek MT2701 HIFSYS clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
