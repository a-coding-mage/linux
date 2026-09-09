// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Kevin Chen <kevin-cw.chen@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel clock framework.

static MM0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0104,
    clr_ofs: 0x0108,
    sta_ofs: 0x0100,
};

static MM1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0114,
    clr_ofs: 0x0118,
    sta_ofs: 0x0110,
};

macro_rules! GATE_MM0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &MM0_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! GATE_MM1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &MM1_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static MM_CLKS: [mtk_gate; 42] = [
    GATE_MM0!(CLK_MM_SMI_COMMON, "mm_smi_common", "mm_sel", 0),
    GATE_MM0!(CLK_MM_SMI_LARB0, "mm_smi_larb0", "mm_sel", 1),
    GATE_MM0!(CLK_MM_SMI_LARB5, "mm_smi_larb5", "mm_sel", 2),
    GATE_MM0!(CLK_MM_CAM_MDP, "mm_cam_mdp", "mm_sel", 3),
    GATE_MM0!(CLK_MM_MDP_RDMA0, "mm_mdp_rdma0", "mm_sel", 4),
    GATE_MM0!(CLK_MM_MDP_RDMA1, "mm_mdp_rdma1", "mm_sel", 5),
    GATE_MM0!(CLK_MM_MDP_RSZ0, "mm_mdp_rsz0", "mm_sel", 6),
    GATE_MM0!(CLK_MM_MDP_RSZ1, "mm_mdp_rsz1", "mm_sel", 7),
    GATE_MM0!(CLK_MM_MDP_RSZ2, "mm_mdp_rsz2", "mm_sel", 8),
    GATE_MM0!(CLK_MM_MDP_TDSHP, "mm_mdp_tdshp", "mm_sel", 9),
    GATE_MM0!(CLK_MM_MDP_COLOR, "mm_mdp_color", "mm_sel", 10),
    GATE_MM0!(CLK_MM_MDP_WDMA, "mm_mdp_wdma", "mm_sel", 11),
    GATE_MM0!(CLK_MM_MDP_WROT0, "mm_mdp_wrot0", "mm_sel", 12),
    GATE_MM0!(CLK_MM_MDP_WROT1, "mm_mdp_wrot1", "mm_sel", 13),
    GATE_MM0!(CLK_MM_FAKE_ENG, "mm_fake_eng", "mm_sel", 14),
    GATE_MM0!(CLK_MM_DISP_OVL0, "mm_disp_ovl0", "mm_sel", 15),
    GATE_MM0!(CLK_MM_DISP_OVL1, "mm_disp_ovl1", "mm_sel", 16),
    GATE_MM0!(CLK_MM_DISP_OVL0_2L, "mm_disp_ovl0_2l", "mm_sel", 17),
    GATE_MM0!(CLK_MM_DISP_OVL1_2L, "mm_disp_ovl1_2l", "mm_sel", 18),
    GATE_MM0!(CLK_MM_DISP_RDMA0, "mm_disp_rdma0", "mm_sel", 19),
    GATE_MM0!(CLK_MM_DISP_RDMA1, "mm_disp_rdma1", "mm_sel", 20),
    GATE_MM0!(CLK_MM_DISP_WDMA0, "mm_disp_wdma0", "mm_sel", 21),
    GATE_MM0!(CLK_MM_DISP_WDMA1, "mm_disp_wdma1", "mm_sel", 22),
    GATE_MM0!(CLK_MM_DISP_COLOR, "mm_disp_color", "mm_sel", 23),
    GATE_MM0!(CLK_MM_DISP_CCORR, "mm_disp_ccorr", "mm_sel", 24),
    GATE_MM0!(CLK_MM_DISP_AAL, "mm_disp_aal", "mm_sel", 25),
    GATE_MM0!(CLK_MM_DISP_GAMMA, "mm_disp_gamma", "mm_sel", 26),
    GATE_MM0!(CLK_MM_DISP_OD, "mm_disp_od", "mm_sel", 27),
    GATE_MM0!(CLK_MM_DISP_DITHER, "mm_disp_dither", "mm_sel", 28),
    GATE_MM0!(CLK_MM_DISP_UFOE, "mm_disp_ufoe", "mm_sel", 29),
    GATE_MM0!(CLK_MM_DISP_DSC, "mm_disp_dsc", "mm_sel", 30),
    GATE_MM0!(CLK_MM_DISP_SPLIT, "mm_disp_split", "mm_sel", 31),
    GATE_MM1!(CLK_MM_DSI0_MM_CLOCK, "mm_dsi0_mm_clock", "mm_sel", 0),
    GATE_MM1!(CLK_MM_DSI1_MM_CLOCK, "mm_dsi1_mm_clock", "mm_sel", 2),
    GATE_MM1!(CLK_MM_DPI_MM_CLOCK, "mm_dpi_mm_clock", "mm_sel", 4),
    GATE_MM1!(CLK_MM_DPI_INTERFACE_CLOCK, "mm_dpi_interface_clock", "dpi0_sel", 5),
    GATE_MM1!(CLK_MM_LARB4_AXI_ASIF_MM_CLOCK, "mm_larb4_axi_asif_mm_clock", "mm_sel", 6),
    GATE_MM1!(CLK_MM_LARB4_AXI_ASIF_MJC_CLOCK, "mm_larb4_axi_asif_mjc_clock", "mjc_sel", 7),
    GATE_MM1!(CLK_MM_DISP_OVL0_MOUT_CLOCK, "mm_disp_ovl0_mout_clock", "mm_sel", 8),
    GATE_MM1!(CLK_MM_FAKE_ENG2, "mm_fake_eng2", "mm_sel", 9),
    GATE_MM1!(CLK_MM_DSI0_INTERFACE_CLOCK, "mm_dsi0_interface_clock", "clk26m", 1),
    GATE_MM1!(CLK_MM_DSI1_INTERFACE_CLOCK, "mm_dsi1_interface_clock", "clk26m", 3),
];

static MM_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: MM_CLKS.as_ptr(),
    num_clks: MM_CLKS.len(),
};

static CLK_MT6797_MM_ID_TABLE: [platform_device_id; 2] = [
    platform_device_id { name: "clk-mt6797-mm", driver_data: &MM_DESC as *const _ as kernel_ulong_t },
    platform_device_id { /* sentinel */ name: "", driver_data: 0 },
];

static mut CLK_MT6797_MM_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: driver { name: "clk-mt6797-mm" },
    id_table: CLK_MT6797_MM_ID_TABLE.as_ptr(),
};

// MODULE_DEVICE_TABLE(platform, clk_mt6797_mm_id_table);
// module_platform_driver(clk_mt6797_mm_drv);
// MODULE_DESCRIPTION("MediaTek MT6797 MultiMedia clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
