// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Sam Shih <sam.shih@mediatek.com>
 * Author: Wenzhen Yu <wenzhen.yu@mediatek.com>
 */

// Dependencies supplied by the Linux clock-provider, platform-device, MediaTek
// clock, gate, and MT7986 clock-binding interfaces are intentionally external.

static SGMII0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0xe4,
    clr_ofs: 0xe4,
    sta_ofs: 0xe4,
};

// Equivalent of GATE_SGMII0(_id, _name, _parent, _shift).
macro_rules! gate_sgmii0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &SGMII0_CG_REGS, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static SGMII0_CLKS: [mtk_gate; 4] = [
    gate_sgmii0!(CLK_SGMII0_TX250M_EN, "sgmii0_tx250m_en", "top_xtal", 2),
    gate_sgmii0!(CLK_SGMII0_RX250M_EN, "sgmii0_rx250m_en", "top_xtal", 3),
    gate_sgmii0!(CLK_SGMII0_CDR_REF, "sgmii0_cdr_ref", "top_xtal", 4),
    gate_sgmii0!(CLK_SGMII0_CDR_FB, "sgmii0_cdr_fb", "top_xtal", 5),
];

static SGMII1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0xe4,
    clr_ofs: 0xe4,
    sta_ofs: 0xe4,
};

// Equivalent of GATE_SGMII1(_id, _name, _parent, _shift).
macro_rules! gate_sgmii1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &SGMII1_CG_REGS, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static SGMII1_CLKS: [mtk_gate; 4] = [
    gate_sgmii1!(CLK_SGMII1_TX250M_EN, "sgmii1_tx250m_en", "top_xtal", 2),
    gate_sgmii1!(CLK_SGMII1_RX250M_EN, "sgmii1_rx250m_en", "top_xtal", 3),
    gate_sgmii1!(CLK_SGMII1_CDR_REF, "sgmii1_cdr_ref", "top_xtal", 4),
    gate_sgmii1!(CLK_SGMII1_CDR_FB, "sgmii1_cdr_fb", "top_xtal", 5),
];

static ETH_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x30,
    clr_ofs: 0x30,
    sta_ofs: 0x30,
};

// Equivalent of GATE_ETH(_id, _name, _parent, _shift).
macro_rules! gate_eth {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &ETH_CG_REGS, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static ETH_CLKS: [mtk_gate; 5] = [
    gate_eth!(CLK_ETH_FE_EN, "eth_fe_en", "netsys_2x_sel", 6),
    gate_eth!(CLK_ETH_GP2_EN, "eth_gp2_en", "sgm_325m_sel", 7),
    gate_eth!(CLK_ETH_GP1_EN, "eth_gp1_en", "sgm_325m_sel", 8),
    gate_eth!(CLK_ETH_WOCPU1_EN, "eth_wocpu1_en", "netsys_mcu_sel", 14),
    gate_eth!(CLK_ETH_WOCPU0_EN, "eth_wocpu0_en", "netsys_mcu_sel", 15),
];

static ETH_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &ETH_CLKS,
    num_clks: ETH_CLKS.len(),
};

static SGMII0_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &SGMII0_CLKS,
    num_clks: SGMII0_CLKS.len(),
};

static SGMII1_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &SGMII1_CLKS,
    num_clks: SGMII1_CLKS.len(),
};

static OF_MATCH_CLK_MT7986_ETH: [of_device_id; 4] = [
    of_device_id { compatible: "mediatek,mt7986-ethsys", data: &ETH_DESC },
    of_device_id { compatible: "mediatek,mt7986-sgmiisys_0", data: &SGMII0_DESC },
    of_device_id { compatible: "mediatek,mt7986-sgmiisys_1", data: &SGMII1_DESC },
    of_device_id { /* sentinel */ },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt7986_eth);

static mut CLK_MT7986_ETH_DRV: platform_driver = platform_driver {
    driver: driver {
        name: "clk-mt7986-eth",
        of_match_table: &OF_MATCH_CLK_MT7986_ETH,
    },
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
};

// module_platform_driver(clk_mt7986_eth_drv);
// MODULE_DESCRIPTION("MediaTek MT7986 Ethernet clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
