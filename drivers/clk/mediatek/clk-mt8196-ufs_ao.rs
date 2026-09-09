// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */

// External clock, reset, kernel, and local driver definitions are supplied by
// the surrounding kernel translation unit.

const MT8196_UFSAO_RST0_SET_OFFSET: u16 = 0x48;
const MT8196_UFSAO_RST1_SET_OFFSET: u16 = 0x148;

static ufsao0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x108,
    clr_ofs: 0x10c,
    sta_ofs: 0x104,
};

static ufsao1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x8,
    clr_ofs: 0xc,
    sta_ofs: 0x4,
};

const fn gate_ufsao0(id: u32, name: &'static str, parent_name: &'static str, shift: u8) -> mtk_gate {
    mtk_gate {
        id,
        name,
        parent_name,
        regs: &ufsao0_cg_regs,
        shift,
        ops: &mtk_clk_gate_ops_setclr,
    }
}

const fn gate_ufsao1(id: u32, name: &'static str, parent_name: &'static str, shift: u8) -> mtk_gate {
    mtk_gate {
        id,
        name,
        parent_name,
        regs: &ufsao1_cg_regs,
        shift,
        ops: &mtk_clk_gate_ops_setclr,
    }
}

static ufsao_clks: [mtk_gate; 8] = [
    // UFSAO0
    gate_ufsao0(CLK_UFSAO_UFSHCI_UFS, "ufsao_ufshci_ufs", "ufs", 0),
    gate_ufsao0(CLK_UFSAO_UFSHCI_AES, "ufsao_ufshci_aes", "aes_ufsfde", 1),
    // UFSAO1
    gate_ufsao1(CLK_UFSAO_UNIPRO_TX_SYM, "ufsao_unipro_tx_sym", "clk26m", 0),
    gate_ufsao1(CLK_UFSAO_UNIPRO_RX_SYM0, "ufsao_unipro_rx_sym0", "clk26m", 1),
    gate_ufsao1(CLK_UFSAO_UNIPRO_RX_SYM1, "ufsao_unipro_rx_sym1", "clk26m", 2),
    gate_ufsao1(CLK_UFSAO_UNIPRO_SYS, "ufsao_unipro_sys", "ufs", 3),
    gate_ufsao1(CLK_UFSAO_UNIPRO_SAP, "ufsao_unipro_sap", "clk26m", 4),
    gate_ufsao1(CLK_UFSAO_PHY_SAP, "ufsao_phy_sap", "clk26m", 8),
];

static ufsao_rst_ofs: [u16; 2] = [
    MT8196_UFSAO_RST0_SET_OFFSET,
    MT8196_UFSAO_RST1_SET_OFFSET,
];

// C designated initializers preserve the reset-index mapping; the constants
// and bank size are provided by the reset bindings.
static ufsao_rst_idx_map: [(u32, u16); 4] = [
    (MT8196_UFSAO_RST0_UFS_MPHY, 8),
    (MT8196_UFSAO_RST1_UFS_UNIPRO, 1 * RST_NR_PER_BANK + 0),
    (MT8196_UFSAO_RST1_UFS_CRYPTO, 1 * RST_NR_PER_BANK + 1),
    (MT8196_UFSAO_RST1_UFSHCI, 1 * RST_NR_PER_BANK + 2),
];

static ufsao_rst_desc: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SET_CLR,
    rst_bank_ofs: &ufsao_rst_ofs,
    rst_bank_nr: ufsao_rst_ofs.len(),
    rst_idx_map: &ufsao_rst_idx_map,
    rst_idx_map_nr: ufsao_rst_idx_map.len(),
};

static ufsao_mcd: mtk_clk_desc = mtk_clk_desc {
    clks: &ufsao_clks,
    num_clks: ufsao_clks.len(),
    rst_desc: &ufsao_rst_desc,
};

static of_match_clk_mt8196_ufs_ao: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8196-ufscfg-ao",
        data: &ufsao_mcd,
    },
    of_device_id { /* sentinel */ },
];

module_device_table_of!(of_match_clk_mt8196_ufs_ao);

static mut clk_mt8196_ufs_ao_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt8196-ufs-ao",
        of_match_table: &of_match_clk_mt8196_ufs_ao,
    },
};

module_platform_driver!(clk_mt8196_ufs_ao_drv);
module_description!("MediaTek MT8196 ufs_ao clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
