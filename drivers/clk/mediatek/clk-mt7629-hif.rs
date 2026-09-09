// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2018 MediaTek Inc.
 * Author: Wenzhen Yu <Wenzhen Yu@mediatek.com>
 *         Ryder Lee <ryder.lee@mediatek.com>
 */

// Linux clock-provider, platform-device, MediaTek clock, gate, and
// mt7629 clock-binding dependencies are supplied externally.

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
    GATE_SSUSB!(CLK_SSUSB_MCU_EN, "ssusb_mcu_en", "to_usb3_mcu", 7),
    GATE_SSUSB!(CLK_SSUSB_DMA_EN, "ssusb_dma_en", "to_usb3_dma", 8),
];

static pcie_clks: [mtk_gate; 12] = [
    GATE_PCIE!(CLK_PCIE_P1_AUX_EN, "pcie_p1_aux_en", "p1_1mhz", 12),
    GATE_PCIE!(CLK_PCIE_P1_OBFF_EN, "pcie_p1_obff_en", "free_run_4mhz", 13),
    GATE_PCIE!(CLK_PCIE_P1_AHB_EN, "pcie_p1_ahb_en", "from_top_ahb", 14),
    GATE_PCIE!(CLK_PCIE_P1_AXI_EN, "pcie_p1_axi_en", "from_top_axi", 15),
    GATE_PCIE!(CLK_PCIE_P1_MAC_EN, "pcie_p1_mac_en", "pcie1_mac_en", 16),
    GATE_PCIE!(CLK_PCIE_P1_PIPE_EN, "pcie_p1_pipe_en", "pcie1_pipe_en", 17),
    GATE_PCIE!(CLK_PCIE_P0_AUX_EN, "pcie_p0_aux_en", "p0_1mhz", 18),
    GATE_PCIE!(CLK_PCIE_P0_OBFF_EN, "pcie_p0_obff_en", "free_run_4mhz", 19),
    GATE_PCIE!(CLK_PCIE_P0_AHB_EN, "pcie_p0_ahb_en", "from_top_ahb", 20),
    GATE_PCIE!(CLK_PCIE_P0_AXI_EN, "pcie_p0_axi_en", "from_top_axi", 21),
    GATE_PCIE!(CLK_PCIE_P0_MAC_EN, "pcie_p0_mac_en", "pcie0_mac_en", 22),
    GATE_PCIE!(CLK_PCIE_P0_PIPE_EN, "pcie_p0_pipe_en", "pcie0_pipe_en", 23),
];

static rst_ofs: [u16; 1] = [0x34];

static clk_rst_desc: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: rst_ofs.as_ptr(),
    rst_bank_nr: rst_ofs.len(),
};

static ssusb_desc: mtk_clk_desc = mtk_clk_desc {
    clks: ssusb_clks.as_ptr(),
    num_clks: ssusb_clks.len(),
    rst_desc: &clk_rst_desc,
};

static pcie_desc: mtk_clk_desc = mtk_clk_desc {
    clks: pcie_clks.as_ptr(),
    num_clks: pcie_clks.len(),
    rst_desc: &clk_rst_desc,
};

static of_match_clk_mt7629_hif: [of_device_id; 3] = [
    of_device_id { compatible: "mediatek,mt7629-pciesys", data: &pcie_desc },
    of_device_id { compatible: "mediatek,mt7629-ssusbsys", data: &ssusb_desc },
    of_device_id { /* sentinel */ },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt7629_hif);

static mut clk_mt7629_hif_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt7629-hif",
        of_match_table: of_match_clk_mt7629_hif.as_ptr(),
    },
};

// module_platform_driver(clk_mt7629_hif_drv);
// MODULE_DESCRIPTION("MediaTek MT2701 HIF clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
