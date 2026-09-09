// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device, MediaTek
// clock, gate, and MT8183 clock-binding interfaces are intentionally external.

static mm0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static mm1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x114,
    clr_ofs: 0x118,
    sta_ofs: 0x110,
};

macro_rules! GATE_MM0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &mm0_cg_regs, $shift,
            &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! GATE_MM1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &mm1_cg_regs, $shift,
            &mtk_clk_gate_ops_setclr)
    };
}

static mm_clks: [mtk_gate; 45] = [
    /* MM0 */
    GATE_MM0!(CLK_MM_SMI_COMMON, "mm_smi_common", "mm_sel", 0),
    GATE_MM0!(CLK_MM_SMI_LARB0, "mm_smi_larb0", "mm_sel", 1),
    GATE_MM0!(CLK_MM_SMI_LARB1, "mm_smi_larb1", "mm_sel", 2),
    GATE_MM0!(CLK_MM_GALS_COMM0, "mm_gals_comm0", "mm_sel", 3),
    GATE_MM0!(CLK_MM_GALS_COMM1, "mm_gals_comm1", "mm_sel", 4),
    GATE_MM0!(CLK_MM_GALS_CCU2MM, "mm_gals_ccu2mm", "mm_sel", 5),
    GATE_MM0!(CLK_MM_GALS_IPU12MM, "mm_gals_ipu12mm", "mm_sel", 6),
    GATE_MM0!(CLK_MM_GALS_IMG2MM, "mm_gals_img2mm", "mm_sel", 7),
    GATE_MM0!(CLK_MM_GALS_CAM2MM, "mm_gals_cam2mm", "mm_sel", 8),
    GATE_MM0!(CLK_MM_GALS_IPU2MM, "mm_gals_ipu2mm", "mm_sel", 9),
    GATE_MM0!(CLK_MM_MDP_DL_TXCK, "mm_mdp_dl_txck", "mm_sel", 10),
    GATE_MM0!(CLK_MM_IPU_DL_TXCK, "mm_ipu_dl_txck", "mm_sel", 11),
    GATE_MM0!(CLK_MM_MDP_RDMA0, "mm_mdp_rdma0", "mm_sel", 12),
    GATE_MM0!(CLK_MM_MDP_RDMA1, "mm_mdp_rdma1", "mm_sel", 13),
    GATE_MM0!(CLK_MM_MDP_RSZ0, "mm_mdp_rsz0", "mm_sel", 14),
    GATE_MM0!(CLK_MM_MDP_RSZ1, "mm_mdp_rsz1", "mm_sel", 15),
    GATE_MM0!(CLK_MM_MDP_TDSHP, "mm_mdp_tdshp", "mm_sel", 16),
    GATE_MM0!(CLK_MM_MDP_WROT0, "mm_mdp_wrot0", "mm_sel", 17),
    GATE_MM0!(CLK_MM_MDP_WDMA0, "mm_mdp_wdma0", "mm_sel", 18),
    GATE_MM0!(CLK_MM_FAKE_ENG, "mm_fake_eng", "mm_sel", 19),
    GATE_MM0!(CLK_MM_DISP_OVL0, "mm_disp_ovl0", "mm_sel", 20),
    GATE_MM0!(CLK_MM_DISP_OVL0_2L, "mm_disp_ovl0_2l", "mm_sel", 21),
    GATE_MM0!(CLK_MM_DISP_OVL1_2L, "mm_disp_ovl1_2l", "mm_sel", 22),
    GATE_MM0!(CLK_MM_DISP_RDMA0, "mm_disp_rdma0", "mm_sel", 23),
    GATE_MM0!(CLK_MM_DISP_RDMA1, "mm_disp_rdma1", "mm_sel", 24),
    GATE_MM0!(CLK_MM_DISP_WDMA0, "mm_disp_wdma0", "mm_sel", 25),
    GATE_MM0!(CLK_MM_DISP_COLOR0, "mm_disp_color0", "mm_sel", 26),
    GATE_MM0!(CLK_MM_DISP_CCORR0, "mm_disp_ccorr0", "mm_sel", 27),
    GATE_MM0!(CLK_MM_DISP_AAL0, "mm_disp_aal0", "mm_sel", 28),
    GATE_MM0!(CLK_MM_DISP_GAMMA0, "mm_disp_gamma0", "mm_sel", 29),
    GATE_MM0!(CLK_MM_DISP_DITHER0, "mm_disp_dither0", "mm_sel", 30),
    GATE_MM0!(CLK_MM_DISP_SPLIT, "mm_disp_split", "mm_sel", 31),
    /* MM1 */
    GATE_MM1!(CLK_MM_DSI0_MM, "mm_dsi0_mm", "mm_sel", 0),
    GATE_MM1!(CLK_MM_DSI0_IF, "mm_dsi0_if", "mm_sel", 1),
    GATE_MM1!(CLK_MM_DPI_MM, "mm_dpi_mm", "mm_sel", 2),
    GATE_MM1!(CLK_MM_DPI_IF, "mm_dpi_if", "dpi0_sel", 3),
    GATE_MM1!(CLK_MM_FAKE_ENG2, "mm_fake_eng2", "mm_sel", 4),
    GATE_MM1!(CLK_MM_MDP_DL_RX, "mm_mdp_dl_rx", "mm_sel", 5),
    GATE_MM1!(CLK_MM_IPU_DL_RX, "mm_ipu_dl_rx", "mm_sel", 6),
    GATE_MM1!(CLK_MM_26M, "mm_26m", "f_f26m_ck", 7),
    GATE_MM1!(CLK_MM_MMSYS_R2Y, "mm_mmsys_r2y", "mm_sel", 8),
    GATE_MM1!(CLK_MM_DISP_RSZ, "mm_disp_rsz", "mm_sel", 9),
    GATE_MM1!(CLK_MM_MDP_AAL, "mm_mdp_aal", "mm_sel", 10),
    GATE_MM1!(CLK_MM_MDP_CCORR, "mm_mdp_ccorr", "mm_sel", 11),
    GATE_MM1!(CLK_MM_DBI_MM, "mm_dbi_mm", "mm_sel", 12),
    GATE_MM1!(CLK_MM_DBI_IF, "mm_dbi_if", "dpi0_sel", 13),
];

static mm_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &mm_clks,
    num_clks: ARRAY_SIZE!(mm_clks),
};

static clk_mt8183_mm_id_table: [platform_device_id; 2] = [
    platform_device_id { name: "clk-mt8183-mm", driver_data: &mm_desc as kernel_ulong_t },
    platform_device_id { /* sentinel */ },
];

MODULE_DEVICE_TABLE!(platform, clk_mt8183_mm_id_table);

static mut clk_mt8183_mm_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: driver { name: "clk-mt8183-mm" },
    id_table: &clk_mt8183_mm_id_table,
};

module_platform_driver!(clk_mt8183_mm_drv);

MODULE_DESCRIPTION!("MediaTek MT8183 MultiMedia clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
