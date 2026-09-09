// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023 MediaTek Inc.
 * Author: Sam Shih <sam.shih@mediatek.com>
 * Author: Xiufeng Li <Xiufeng.Li@mediatek.com>
 */

// C dependencies supplied by the surrounding kernel translation.

static ETHDMA_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x30,
    clr_ofs: 0x30,
    sta_ofs: 0x30,
};

macro_rules! gate_ethdma {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate {
            id: $id,
            name: $name,
            parent_name: $parent,
            regs: &ETHDMA_CG_REGS,
            shift: $shift,
            ops: &mtk_clk_gate_ops_no_setclr_inv,
        }
    };
}

static ETHDMA_CLKS: [mtk_gate; 9] = [
    gate_ethdma!(CLK_ETHDMA_XGP1_EN, "ethdma_xgp1_en", "top_xtal", 0),
    gate_ethdma!(CLK_ETHDMA_XGP2_EN, "ethdma_xgp2_en", "top_xtal", 1),
    gate_ethdma!(CLK_ETHDMA_XGP3_EN, "ethdma_xgp3_en", "top_xtal", 2),
    gate_ethdma!(CLK_ETHDMA_FE_EN, "ethdma_fe_en", "netsys_2x_sel", 6),
    gate_ethdma!(CLK_ETHDMA_GP2_EN, "ethdma_gp2_en", "top_xtal", 7),
    gate_ethdma!(CLK_ETHDMA_GP1_EN, "ethdma_gp1_en", "top_xtal", 8),
    gate_ethdma!(CLK_ETHDMA_GP3_EN, "ethdma_gp3_en", "top_xtal", 10),
    gate_ethdma!(CLK_ETHDMA_ESW_EN, "ethdma_esw_en", "netsys_gsw_sel", 16),
    gate_ethdma!(CLK_ETHDMA_CRYPT0_EN, "ethdma_crypt0_en", "eip197_sel", 29),
];

static ETHDMA_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &ETHDMA_CLKS,
    num_clks: ETHDMA_CLKS.len(),
};

static SGMII_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0xe4,
    clr_ofs: 0xe4,
    sta_ofs: 0xe4,
};

macro_rules! gate_sgmii {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate {
            id: $id,
            name: $name,
            parent_name: $parent,
            regs: &SGMII_CG_REGS,
            shift: $shift,
            ops: &mtk_clk_gate_ops_no_setclr_inv,
        }
    };
}

static SGMII0_CLKS: [mtk_gate; 2] = [
    gate_sgmii!(CLK_SGM0_TX_EN, "sgm0_tx_en", "top_xtal", 2),
    gate_sgmii!(CLK_SGM0_RX_EN, "sgm0_rx_en", "top_xtal", 3),
];

static SGMII0_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &SGMII0_CLKS,
    num_clks: SGMII0_CLKS.len(),
};

static SGMII1_CLKS: [mtk_gate; 2] = [
    gate_sgmii!(CLK_SGM1_TX_EN, "sgm1_tx_en", "top_xtal", 2),
    gate_sgmii!(CLK_SGM1_RX_EN, "sgm1_rx_en", "top_xtal", 3),
];

static SGMII1_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &SGMII1_CLKS,
    num_clks: SGMII1_CLKS.len(),
};

static ETHWARP_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x14,
    clr_ofs: 0x14,
    sta_ofs: 0x14,
};

macro_rules! gate_ethwarp {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate {
            id: $id,
            name: $name,
            parent_name: $parent,
            regs: &ETHWARP_CG_REGS,
            shift: $shift,
            ops: &mtk_clk_gate_ops_no_setclr_inv,
        }
    };
}

static ETHWARP_CLKS: [mtk_gate; 3] = [
    gate_ethwarp!(CLK_ETHWARP_WOCPU2_EN, "ethwarp_wocpu2_en", "netsys_mcu_sel", 13),
    gate_ethwarp!(CLK_ETHWARP_WOCPU1_EN, "ethwarp_wocpu1_en", "netsys_mcu_sel", 14),
    gate_ethwarp!(CLK_ETHWARP_WOCPU0_EN, "ethwarp_wocpu0_en", "netsys_mcu_sel", 15),
];

static ETHWARP_RST_OFS: [u16; 1] = [0x8];

static ETHWARP_IDX_MAP: [u16; 10] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 9,
];

static ETHWARP_RST_DESC: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: &ETHWARP_RST_OFS,
    rst_bank_nr: ETHWARP_RST_OFS.len(),
    rst_idx_map: &ETHWARP_IDX_MAP,
    rst_idx_map_nr: ETHWARP_IDX_MAP.len(),
};

static ETHWARP_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &ETHWARP_CLKS,
    num_clks: ETHWARP_CLKS.len(),
    rst_desc: &ETHWARP_RST_DESC,
};

static OF_MATCH_CLK_MT7988_ETH: [of_device_id; 5] = [
    of_device_id { compatible: "mediatek,mt7988-ethsys", data: &ETHDMA_DESC },
    of_device_id { compatible: "mediatek,mt7988-sgmiisys0", data: &SGMII0_DESC },
    of_device_id { compatible: "mediatek,mt7988-sgmiisys1", data: &SGMII1_DESC },
    of_device_id { compatible: "mediatek,mt7988-ethwarp", data: &ETHWARP_DESC },
    of_device_id { ..Default::default() },
];

static mut CLK_MT7988_ETH_DRV: platform_driver = platform_driver {
    driver: driver {
        name: "clk-mt7988-eth",
        of_match_table: &OF_MATCH_CLK_MT7988_ETH,
    },
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
};

// module_platform_driver(CLK_MT7988_ETH_DRV);
// MODULE_DESCRIPTION("MediaTek MT7988 Ethernet clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
