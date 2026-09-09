// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 */

// External dependencies supplied by the kernel clock and platform-driver code:
// clk-gate, clk-mtk, dt-bindings/clock/mt8173-clk.

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

macro_rules! gate_mm0 {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &MM0_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! gate_mm1 {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &MM1_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static MT8173_MM_CLKS: [mtk_gate; 51] = [
    gate_dummy!(CLK_DUMMY, "mm_dummy"),
    // MM0
    gate_mm0!(CLK_MM_SMI_COMMON, "mm_smi_common", "mm_sel", 0),
    gate_mm0!(CLK_MM_SMI_LARB0, "mm_smi_larb0", "mm_sel", 1),
    gate_mm0!(CLK_MM_CAM_MDP, "mm_cam_mdp", "mm_sel", 2),
    gate_mm0!(CLK_MM_MDP_RDMA0, "mm_mdp_rdma0", "mm_sel", 3),
    gate_mm0!(CLK_MM_MDP_RDMA1, "mm_mdp_rdma1", "mm_sel", 4),
    gate_mm0!(CLK_MM_MDP_RSZ0, "mm_mdp_rsz0", "mm_sel", 5),
    gate_mm0!(CLK_MM_MDP_RSZ1, "mm_mdp_rsz1", "mm_sel", 6),
    gate_mm0!(CLK_MM_MDP_RSZ2, "mm_mdp_rsz2", "mm_sel", 7),
    gate_mm0!(CLK_MM_MDP_TDSHP0, "mm_mdp_tdshp0", "mm_sel", 8),
    gate_mm0!(CLK_MM_MDP_TDSHP1, "mm_mdp_tdshp1", "mm_sel", 9),
    gate_mm0!(CLK_MM_MDP_WDMA, "mm_mdp_wdma", "mm_sel", 11),
    gate_mm0!(CLK_MM_MDP_WROT0, "mm_mdp_wrot0", "mm_sel", 12),
    gate_mm0!(CLK_MM_MDP_WROT1, "mm_mdp_wrot1", "mm_sel", 13),
    gate_mm0!(CLK_MM_FAKE_ENG, "mm_fake_eng", "mm_sel", 14),
    gate_mm0!(CLK_MM_MUTEX_32K, "mm_mutex_32k", "rtc_sel", 15),
    gate_mm0!(CLK_MM_DISP_OVL0, "mm_disp_ovl0", "mm_sel", 16),
    gate_mm0!(CLK_MM_DISP_OVL1, "mm_disp_ovl1", "mm_sel", 17),
    gate_mm0!(CLK_MM_DISP_RDMA0, "mm_disp_rdma0", "mm_sel", 18),
    gate_mm0!(CLK_MM_DISP_RDMA1, "mm_disp_rdma1", "mm_sel", 19),
    gate_mm0!(CLK_MM_DISP_RDMA2, "mm_disp_rdma2", "mm_sel", 20),
    gate_mm0!(CLK_MM_DISP_WDMA0, "mm_disp_wdma0", "mm_sel", 21),
    gate_mm0!(CLK_MM_DISP_WDMA1, "mm_disp_wdma1", "mm_sel", 22),
    gate_mm0!(CLK_MM_DISP_COLOR0, "mm_disp_color0", "mm_sel", 23),
    gate_mm0!(CLK_MM_DISP_COLOR1, "mm_disp_color1", "mm_sel", 24),
    gate_mm0!(CLK_MM_DISP_AAL, "mm_disp_aal", "mm_sel", 25),
    gate_mm0!(CLK_MM_DISP_GAMMA, "mm_disp_gamma", "mm_sel", 26),
    gate_mm0!(CLK_MM_DISP_UFOE, "mm_disp_ufoe", "mm_sel", 27),
    gate_mm0!(CLK_MM_DISP_SPLIT0, "mm_disp_split0", "mm_sel", 28),
    gate_mm0!(CLK_MM_DISP_SPLIT1, "mm_disp_split1", "mm_sel", 29),
    gate_mm0!(CLK_MM_DISP_MERGE, "mm_disp_merge", "mm_sel", 30),
    gate_mm0!(CLK_MM_DISP_OD, "mm_disp_od", "mm_sel", 31),
    // MM1
    gate_mm1!(CLK_MM_DISP_PWM0MM, "mm_disp_pwm0mm", "mm_sel", 0),
    gate_mm1!(CLK_MM_DISP_PWM026M, "mm_disp_pwm026m", "pwm_sel", 1),
    gate_mm1!(CLK_MM_DISP_PWM1MM, "mm_disp_pwm1mm", "mm_sel", 2),
    gate_mm1!(CLK_MM_DISP_PWM126M, "mm_disp_pwm126m", "pwm_sel", 3),
    gate_mm1!(CLK_MM_DSI0_ENGINE, "mm_dsi0_engine", "mm_sel", 4),
    gate_mm1!(CLK_MM_DSI0_DIGITAL, "mm_dsi0_digital", "dsi0_dig", 5),
    gate_mm1!(CLK_MM_DSI1_ENGINE, "mm_dsi1_engine", "mm_sel", 6),
    gate_mm1!(CLK_MM_DSI1_DIGITAL, "mm_dsi1_digital", "dsi1_dig", 7),
    gate_mm1!(CLK_MM_DPI_PIXEL, "mm_dpi_pixel", "dpi0_sel", 8),
    gate_mm1!(CLK_MM_DPI_ENGINE, "mm_dpi_engine", "mm_sel", 9),
    gate_mm1!(CLK_MM_DPI1_PIXEL, "mm_dpi1_pixel", "lvds_pxl", 10),
    gate_mm1!(CLK_MM_DPI1_ENGINE, "mm_dpi1_engine", "mm_sel", 11),
    gate_mm1!(CLK_MM_HDMI_PIXEL, "mm_hdmi_pixel", "dpi0_sel", 12),
    gate_mm1!(CLK_MM_HDMI_PLLCK, "mm_hdmi_pllck", "hdmi_sel", 13),
    gate_mm1!(CLK_MM_HDMI_AUDIO, "mm_hdmi_audio", "apll1", 14),
    gate_mm1!(CLK_MM_HDMI_SPDIF, "mm_hdmi_spdif", "apll2", 15),
    gate_mm1!(CLK_MM_LVDS_PIXEL, "mm_lvds_pixel", "lvds_pxl", 16),
    gate_mm1!(CLK_MM_LVDS_CTS, "mm_lvds_cts", "lvds_cts", 17),
    gate_mm1!(CLK_MM_SMI_LARB4, "mm_smi_larb4", "mm_sel", 18),
    gate_mm1!(CLK_MM_HDMI_HDCP, "mm_hdmi_hdcp", "hdcp_sel", 19),
    gate_mm1!(CLK_MM_HDMI_HDCP24M, "mm_hdmi_hdcp24m", "hdcp_24m_sel", 20),
];

static MM_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &MT8173_MM_CLKS,
    num_clks: MT8173_MM_CLKS.len(),
};

static CLK_MT8173_MM_ID_TABLE: [platform_device_id; 2] = [
    platform_device_id {
        name: "clk-mt8173-mm",
        driver_data: &MM_DESC as *const _ as kernel_ulong_t,
    },
    platform_device_id { /* sentinel */ },
];

module_device_table!(platform, CLK_MT8173_MM_ID_TABLE);

static mut CLK_MT8173_MM_DRV: platform_driver = platform_driver {
    driver: driver {
        name: "clk-mt8173-mm",
    },
    id_table: &CLK_MT8173_MM_ID_TABLE,
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
};

module_platform_driver!(CLK_MT8173_MM_DRV);

module_description!("MediaTek MT8173 MultiMedia clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
