// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Chen Zhong <chen.zhong@mediatek.com>
 *         Sean Wang <sean.wang@mediatek.com>
 */

// Translated from the Linux MediaTek MT7622 HIF clocks implementation.

macro_rules! GATE_PCIE {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &pcie_cg_regs, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

macro_rules! GATE_SSUSB {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &ssusb_cg_regs, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static pcie_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x30,
    clr_ofs: 0x30,
    sta_ofs: 0x30,
};

static ssusb_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x30,
    clr_ofs: 0x30,
    sta_ofs: 0x30,
};

static ssusb_clks: [mtk_gate; 6] = [
    GATE_SSUSB!(CLK_SSUSB_U2_PHY_1P_EN, "ssusb_u2_phy_1p", "to_u2_phy_1p", 0),
    GATE_SSUSB!(CLK_SSUSB_U2_PHY_EN, "ssusb_u2_phy_en", "to_u2_phy", 1),
    GATE_SSUSB!(CLK_SSUSB_REF_EN, "ssusb_ref_en", "to_usb3_ref", 5),
    GATE_SSUSB!(CLK_SSUSB_SYS_EN, "ssusb_sys_en", "to_usb3_sys", 6),
    GATE_SSUSB!(CLK_SSUSB_MCU_EN, "ssusb_mcu_en", "axi_sel", 7),
    GATE_SSUSB!(CLK_SSUSB_DMA_EN, "ssusb_dma_en", "hif_sel", 8),
];

static pcie_clks: [mtk_gate; 17] = [
    GATE_PCIE!(CLK_PCIE_P1_AUX_EN, "pcie_p1_aux_en", "p1_1mhz", 12),
    GATE_PCIE!(CLK_PCIE_P1_OBFF_EN, "pcie_p1_obff_en", "free_run_4mhz", 13),
    GATE_PCIE!(CLK_PCIE_P1_AHB_EN, "pcie_p1_ahb_en", "axi_sel", 14),
    GATE_PCIE!(CLK_PCIE_P1_AXI_EN, "pcie_p1_axi_en", "hif_sel", 15),
    GATE_PCIE!(CLK_PCIE_P1_MAC_EN, "pcie_p1_mac_en", "pcie1_mac_en", 16),
    GATE_PCIE!(CLK_PCIE_P1_PIPE_EN, "pcie_p1_pipe_en", "pcie1_pipe_en", 17),
    GATE_PCIE!(CLK_PCIE_P0_AUX_EN, "pcie_p0_aux_en", "p0_1mhz", 18),
    GATE_PCIE!(CLK_PCIE_P0_OBFF_EN, "pcie_p0_obff_en", "free_run_4mhz", 19),
    GATE_PCIE!(CLK_PCIE_P0_AHB_EN, "pcie_p0_ahb_en", "axi_sel", 20),
    GATE_PCIE!(CLK_PCIE_P0_AXI_EN, "pcie_p0_axi_en", "hif_sel", 21),
    GATE_PCIE!(CLK_PCIE_P0_MAC_EN, "pcie_p0_mac_en", "pcie0_mac_en", 22),
    GATE_PCIE!(CLK_PCIE_P0_PIPE_EN, "pcie_p0_pipe_en", "pcie0_pipe_en", 23),
    GATE_PCIE!(CLK_SATA_AHB_EN, "sata_ahb_en", "axi_sel", 26),
    GATE_PCIE!(CLK_SATA_AXI_EN, "sata_axi_en", "hif_sel", 27),
    GATE_PCIE!(CLK_SATA_ASIC_EN, "sata_asic_en", "sata_asic", 28),
    GATE_PCIE!(CLK_SATA_RBC_EN, "sata_rbc_en", "sata_rbc", 29),
    GATE_PCIE!(CLK_SATA_PM_EN, "sata_pm_en", "univpll2_d4", 30),
];

static rst_ofs: [u16; 1] = [0x34];

static clk_rst_desc: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: &rst_ofs,
    rst_bank_nr: rst_ofs.len(),
};

static ssusb_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &ssusb_clks,
    num_clks: ssusb_clks.len(),
    rst_desc: &clk_rst_desc,
};

static pcie_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &pcie_clks,
    num_clks: pcie_clks.len(),
    rst_desc: &clk_rst_desc,
};

static of_match_clk_mt7622_hif: [of_device_id; 3] = [
    of_device_id { compatible: "mediatek,mt7622-pciesys", data: &pcie_desc },
    of_device_id { compatible: "mediatek,mt7622-ssusbsys", data: &ssusb_desc },
    of_device_id { /* sentinel */ },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt7622_hif);

static mut clk_mt7622_hif_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt7622-hif",
        of_match_table: &of_match_clk_mt7622_hif,
    },
};

module_platform_driver!(clk_mt7622_hif_drv);

MODULE_DESCRIPTION!("MediaTek MT7622 HIF clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
