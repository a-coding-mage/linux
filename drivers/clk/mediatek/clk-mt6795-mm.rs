// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the surrounding kernel clock framework.

macro_rules! GATE_MM0 {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &mm0_cg_regs, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! GATE_MM1 {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &mm1_cg_regs, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static mm0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0104,
    clr_ofs: 0x0108,
    sta_ofs: 0x0100,
};

static mm1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0114,
    clr_ofs: 0x0118,
    sta_ofs: 0x0110,
};

static mm_gates: &[mtk_gate] = &[
    // MM0
    GATE_MM0!(CLK_MM_SMI_COMMON, "mm_smi_common", "mm_sel", 0),
    GATE_MM0!(CLK_MM_SMI_LARB0, "mm_smi_larb0", "mm_sel", 1),
    GATE_MM0!(CLK_MM_CAM_MDP, "mm_cam_mdp", "mm_sel", 2),
    GATE_MM0!(CLK_MM_MDP_RDMA0, "mm_mdp_rdma0", "mm_sel", 3),
    GATE_MM0!(CLK_MM_MDP_RDMA1, "mm_mdp_rdma1", "mm_sel", 4),
    GATE_MM0!(CLK_MM_MDP_RSZ0, "mm_mdp_rsz0", "mm_sel", 5),
    GATE_MM0!(CLK_MM_MDP_RSZ1, "mm_mdp_rsz1", "mm_sel", 6),
    GATE_MM0!(CLK_MM_MDP_RSZ2, "mm_mdp_rsz2", "mm_sel", 7),
    GATE_MM0!(CLK_MM_MDP_TDSHP0, "mm_mdp_tdshp0", "mm_sel", 8),
    GATE_MM0!(CLK_MM_MDP_TDSHP1, "mm_mdp_tdshp1", "mm_sel", 9),
    GATE_MM0!(CLK_MM_MDP_CROP, "mm_mdp_crop", "mm_sel", 10),
    GATE_MM0!(CLK_MM_MDP_WDMA, "mm_mdp_wdma", "mm_sel", 11),
    GATE_MM0!(CLK_MM_MDP_WROT0, "mm_mdp_wrot0", "mm_sel", 12),
    GATE_MM0!(CLK_MM_MDP_WROT1, "mm_mdp_wrot1", "mm_sel", 13),
    GATE_MM0!(CLK_MM_FAKE_ENG, "mm_fake_eng", "mm_sel", 14),
    GATE_MM0!(CLK_MM_MUTEX_32K, "mm_mutex_32k", "clk32k", 15),
    GATE_MM0!(CLK_MM_DISP_OVL0, "mm_disp_ovl0", "mm_sel", 16),
    GATE_MM0!(CLK_MM_DISP_OVL1, "mm_disp_ovl1", "mm_sel", 17),
    GATE_MM0!(CLK_MM_DISP_RDMA0, "mm_disp_rdma0", "mm_sel", 18),
    GATE_MM0!(CLK_MM_DISP_RDMA1, "mm_disp_rdma1", "mm_sel", 19),
    GATE_MM0!(CLK_MM_DISP_RDMA2, "mm_disp_rdma2", "mm_sel", 20),
    GATE_MM0!(CLK_MM_DISP_WDMA0, "mm_disp_wdma0", "mm_sel", 21),
    GATE_MM0!(CLK_MM_DISP_WDMA1, "mm_disp_wdma1", "mm_sel", 22),
    GATE_MM0!(CLK_MM_DISP_COLOR0, "mm_disp_color0", "mm_sel", 23),
    GATE_MM0!(CLK_MM_DISP_COLOR1, "mm_disp_color1", "mm_sel", 24),
    GATE_MM0!(CLK_MM_DISP_AAL, "mm_disp_aal", "mm_sel", 25),
    GATE_MM0!(CLK_MM_DISP_GAMMA, "mm_disp_gamma", "mm_sel", 26),
    GATE_MM0!(CLK_MM_DISP_UFOE, "mm_disp_ufoe", "mm_sel", 27),
    GATE_MM0!(CLK_MM_DISP_SPLIT0, "mm_disp_split0", "mm_sel", 28),
    GATE_MM0!(CLK_MM_DISP_SPLIT1, "mm_disp_split1", "mm_sel", 29),
    GATE_MM0!(CLK_MM_DISP_MERGE, "mm_disp_merge", "mm_sel", 30),
    GATE_MM0!(CLK_MM_DISP_OD, "mm_disp_od", "mm_sel", 31),

    // MM1
    GATE_MM1!(CLK_MM_DISP_PWM0MM, "mm_disp_pwm0mm", "mm_sel", 0),
    GATE_MM1!(CLK_MM_DISP_PWM026M, "mm_disp_pwm026m", "pwm_sel", 1),
    GATE_MM1!(CLK_MM_DISP_PWM1MM, "mm_disp_pwm1mm", "mm_sel", 2),
    GATE_MM1!(CLK_MM_DISP_PWM126M, "mm_disp_pwm126m", "pwm_sel", 3),
    GATE_MM1!(CLK_MM_DSI0_ENGINE, "mm_dsi0_engine", "mm_sel", 4),
    GATE_MM1!(CLK_MM_DSI0_DIGITAL, "mm_dsi0_digital", "dsi0_dig", 5),
    GATE_MM1!(CLK_MM_DSI1_ENGINE, "mm_dsi1_engine", "mm_sel", 6),
    GATE_MM1!(CLK_MM_DSI1_DIGITAL, "mm_dsi1_digital", "dsi1_dig", 7),
    GATE_MM1!(CLK_MM_DPI_PIXEL, "mm_dpi_pixel", "dpi0_sel", 8),
    GATE_MM1!(CLK_MM_DPI_ENGINE, "mm_dpi_engine", "mm_sel", 9),
];

static mm_desc: mtk_clk_desc = mtk_clk_desc {
    clks: mm_gates,
    num_clks: ARRAY_SIZE!(mm_gates),
};

static clk_mt6795_mm_id_table: &[platform_device_id] = &[
    platform_device_id { name: "clk-mt6795-mm", driver_data: &mm_desc as *const _ as kernel_ulong_t },
    platform_device_id { /* sentinel */ ..platform_device_id::default() },
];

static mut clk_mt6795_mm_drv: platform_driver = platform_driver {
    driver: driver { name: "clk-mt6795-mm" },
    id_table: clk_mt6795_mm_id_table,
    probe: mtk_clk_pdev_probe,
    remove: mtk_clk_pdev_remove,
};

MODULE_DEVICE_TABLE!(platform, clk_mt6795_mm_id_table);
module_platform_driver!(clk_mt6795_mm_drv);

MODULE_DESCRIPTION!("MediaTek MT6795 MMSYS clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
