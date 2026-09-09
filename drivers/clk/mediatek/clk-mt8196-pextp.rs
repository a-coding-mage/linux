// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */

// Dependencies supplied by the kernel clock, reset, and platform-driver code.

const MT8196_PEXTP_RST0_SET_OFFSET: u16 = 0x8;

static PEXT_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x18,
    clr_ofs: 0x1c,
    sta_ofs: 0x14,
};

macro_rules! GATE_PEXT {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate {
            id: $id,
            name: $name,
            parent_name: $parent,
            regs: &PEXT_CG_REGS,
            shift: $shift,
            ops: &mtk_clk_gate_ops_setclr,
        }
    };
}

static PEXT_CLKS: [mtk_gate; 8] = [
    GATE_PEXT!(CLK_PEXT_PEXTP_MAC_P0_TL, "pext_pm0_tl", "tl", 0),
    GATE_PEXT!(CLK_PEXT_PEXTP_MAC_P0_REF, "pext_pm0_ref", "clk26m", 1),
    GATE_PEXT!(CLK_PEXT_PEXTP_PHY_P0_MCU_BUS, "pext_pp0_mcu_bus", "clk26m", 6),
    GATE_PEXT!(CLK_PEXT_PEXTP_PHY_P0_PEXTP_REF, "pext_pp0_pextp_ref", "clk26m", 7),
    GATE_PEXT!(CLK_PEXT_PEXTP_MAC_P0_AXI_250, "pext_pm0_axi_250", "ufs_pexpt0_mem_sub", 12),
    GATE_PEXT!(CLK_PEXT_PEXTP_MAC_P0_AHB_APB, "pext_pm0_ahb_apb", "ufs_pextp0_axi", 13),
    GATE_PEXT!(CLK_PEXT_PEXTP_MAC_P0_PL_P, "pext_pm0_pl_p", "clk26m", 14),
    GATE_PEXT!(CLK_PEXT_PEXTP_VLP_AO_P0_LP, "pext_pextp_vlp_ao_p0_lp", "clk26m", 19),
];

static PEXT_RST_OFS: [u16; 1] = [MT8196_PEXTP_RST0_SET_OFFSET];

static PEXT_RST_IDX_MAP: [u16; 2] = [
    MT8196_PEXTP0_RST0_PCIE0_MAC: 0,
    MT8196_PEXTP0_RST0_PCIE0_PHY: 1,
];

static PEXT_RST_DESC: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SET_CLR,
    rst_bank_ofs: &PEXT_RST_OFS,
    rst_bank_nr: PEXT_RST_OFS.len(),
    rst_idx_map: &PEXT_RST_IDX_MAP,
    rst_idx_map_nr: PEXT_RST_IDX_MAP.len(),
};

static PEXT_MCD: mtk_clk_desc = mtk_clk_desc {
    clks: &PEXT_CLKS,
    num_clks: PEXT_CLKS.len(),
    rst_desc: &PEXT_RST_DESC,
};

static PEXT1_CLKS: [mtk_gate; 16] = [
    GATE_PEXT!(CLK_PEXT1_PEXTP_MAC_P1_TL, "pext1_pm1_tl", "tl_p1", 0),
    GATE_PEXT!(CLK_PEXT1_PEXTP_MAC_P1_REF, "pext1_pm1_ref", "clk26m", 1),
    GATE_PEXT!(CLK_PEXT1_PEXTP_MAC_P2_TL, "pext1_pm2_tl", "tl_p2", 2),
    GATE_PEXT!(CLK_PEXT1_PEXTP_MAC_P2_REF, "pext1_pm2_ref", "clk26m", 3),
    GATE_PEXT!(CLK_PEXT1_PEXTP_PHY_P1_MCU_BUS, "pext1_pp1_mcu_bus", "clk26m", 8),
    GATE_PEXT!(CLK_PEXT1_PEXTP_PHY_P1_PEXTP_REF, "pext1_pp1_pextp_ref", "clk26m", 9),
    GATE_PEXT!(CLK_PEXT1_PEXTP_PHY_P2_MCU_BUS, "pext1_pp2_mcu_bus", "clk26m", 10),
    GATE_PEXT!(CLK_PEXT1_PEXTP_PHY_P2_PEXTP_REF, "pext1_pp2_pextp_ref", "clk26m", 11),
    GATE_PEXT!(CLK_PEXT1_PEXTP_MAC_P1_AXI_250, "pext1_pm1_axi_250", "pextp1_usb_axi", 16),
    GATE_PEXT!(CLK_PEXT1_PEXTP_MAC_P1_AHB_APB, "pext1_pm1_ahb_apb", "pextp1_usb_mem_sub", 17),
    GATE_PEXT!(CLK_PEXT1_PEXTP_MAC_P1_PL_P, "pext1_pm1_pl_p", "clk26m", 18),
    GATE_PEXT!(CLK_PEXT1_PEXTP_MAC_P2_AXI_250, "pext1_pm2_axi_250", "pextp1_usb_axi", 19),
    GATE_PEXT!(CLK_PEXT1_PEXTP_MAC_P2_AHB_APB, "pext1_pm2_ahb_apb", "pextp1_usb_mem_sub", 20),
    GATE_PEXT!(CLK_PEXT1_PEXTP_MAC_P2_PL_P, "pext1_pm2_pl_p", "clk26m", 21),
    GATE_PEXT!(CLK_PEXT1_PEXTP_VLP_AO_P1_LP, "pext1_pextp_vlp_ao_p1_lp", "clk26m", 26),
    GATE_PEXT!(CLK_PEXT1_PEXTP_VLP_AO_P2_LP, "pext1_pextp_vlp_ao_p2_lp", "clk26m", 27),
];

static PEXT1_RST_IDX_MAP: [u16; 10] = [
    MT8196_PEXTP1_RST0_PCIE1_MAC: 0,
    MT8196_PEXTP1_RST0_PCIE1_PHY: 1,
    MT8196_PEXTP1_RST0_PCIE2_MAC: 8,
    MT8196_PEXTP1_RST0_PCIE2_PHY: 9,
];

static PEXT1_RST_DESC: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SET_CLR,
    rst_bank_ofs: &PEXT_RST_OFS,
    rst_bank_nr: PEXT_RST_OFS.len(),
    rst_idx_map: &PEXT1_RST_IDX_MAP,
    rst_idx_map_nr: PEXT1_RST_IDX_MAP.len(),
};

static PEXT1_MCD: mtk_clk_desc = mtk_clk_desc {
    clks: &PEXT1_CLKS,
    num_clks: PEXT1_CLKS.len(),
    rst_desc: &PEXT1_RST_DESC,
};

static OF_MATCH_CLK_MT8196_PEXTP: [of_device_id; 3] = [
    of_device_id { compatible: "mediatek,mt8196-pextp0cfg-ao", data: &PEXT_MCD },
    of_device_id { compatible: "mediatek,mt8196-pextp1cfg-ao", data: &PEXT1_MCD },
    of_device_id { ..Default::default() },
];

static mut CLK_MT8196_PEXTP_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8196-pextp",
        of_match_table: &OF_MATCH_CLK_MT8196_PEXTP,
    },
};

// module_platform_driver(CLK_MT8196_PEXTP_DRV);
// MODULE_DESCRIPTION("MediaTek MT8196 PCIe transmit phy clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
