// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: Shunli Wang <shunli.wang@mediatek.com>
 */

// Linux clock-provider, platform-device, clk-mtk, clk-gate, and MT2701 clock
// binding dependencies are supplied by the surrounding translation unit.

static DISP0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0104,
    clr_ofs: 0x0108,
    sta_ofs: 0x0100,
};

static DISP1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0114,
    clr_ofs: 0x0118,
    sta_ofs: 0x0110,
};

macro_rules! GATE_DISP0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &DISP0_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! GATE_DISP1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &DISP1_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static MM_CLKS: [mtk_gate; 34] = [
    GATE_DUMMY!(CLK_DUMMY, "mm_dummy"),
    GATE_DISP0!(CLK_MM_SMI_COMMON, "mm_smi_comm", "mm_sel", 0),
    GATE_DISP0!(CLK_MM_SMI_LARB0, "mm_smi_larb0", "mm_sel", 1),
    GATE_DISP0!(CLK_MM_CMDQ, "mm_cmdq", "mm_sel", 2),
    GATE_DISP0!(CLK_MM_MUTEX, "mm_mutex", "mm_sel", 3),
    GATE_DISP0!(CLK_MM_DISP_COLOR, "mm_disp_color", "mm_sel", 4),
    GATE_DISP0!(CLK_MM_DISP_BLS, "mm_disp_bls", "mm_sel", 5),
    GATE_DISP0!(CLK_MM_DISP_WDMA, "mm_disp_wdma", "mm_sel", 6),
    GATE_DISP0!(CLK_MM_DISP_RDMA, "mm_disp_rdma", "mm_sel", 7),
    GATE_DISP0!(CLK_MM_DISP_OVL, "mm_disp_ovl", "mm_sel", 8),
    GATE_DISP0!(CLK_MM_MDP_TDSHP, "mm_mdp_tdshp", "mm_sel", 9),
    GATE_DISP0!(CLK_MM_MDP_WROT, "mm_mdp_wrot", "mm_sel", 10),
    GATE_DISP0!(CLK_MM_MDP_WDMA, "mm_mdp_wdma", "mm_sel", 11),
    GATE_DISP0!(CLK_MM_MDP_RSZ1, "mm_mdp_rsz1", "mm_sel", 12),
    GATE_DISP0!(CLK_MM_MDP_RSZ0, "mm_mdp_rsz0", "mm_sel", 13),
    GATE_DISP0!(CLK_MM_MDP_RDMA, "mm_mdp_rdma", "mm_sel", 14),
    GATE_DISP0!(CLK_MM_MDP_BLS_26M, "mm_mdp_bls_26m", "pwm_sel", 15),
    GATE_DISP0!(CLK_MM_CAM_MDP, "mm_cam_mdp", "mm_sel", 16),
    GATE_DISP0!(CLK_MM_FAKE_ENG, "mm_fake_eng", "mm_sel", 17),
    GATE_DISP0!(CLK_MM_MUTEX_32K, "mm_mutex_32k", "rtc_sel", 18),
    GATE_DISP0!(CLK_MM_DISP_RDMA1, "mm_disp_rdma1", "mm_sel", 19),
    GATE_DISP0!(CLK_MM_DISP_UFOE, "mm_disp_ufoe", "mm_sel", 20),
    GATE_DISP1!(CLK_MM_DSI_ENGINE, "mm_dsi_eng", "mm_sel", 0),
    GATE_DISP1!(CLK_MM_DSI_DIG, "mm_dsi_dig", "dsi0_lntc_dsi", 1),
    GATE_DISP1!(CLK_MM_DPI_DIGL, "mm_dpi_digl", "dpi0_sel", 2),
    GATE_DISP1!(CLK_MM_DPI_ENGINE, "mm_dpi_eng", "mm_sel", 3),
    GATE_DISP1!(CLK_MM_DPI1_DIGL, "mm_dpi1_digl", "dpi1_sel", 4),
    GATE_DISP1!(CLK_MM_DPI1_ENGINE, "mm_dpi1_eng", "mm_sel", 5),
    GATE_DISP1!(CLK_MM_TVE_OUTPUT, "mm_tve_output", "tve_sel", 6),
    GATE_DISP1!(CLK_MM_TVE_INPUT, "mm_tve_input", "dpi0_sel", 7),
    GATE_DISP1!(CLK_MM_HDMI_PIXEL, "mm_hdmi_pixel", "dpi1_sel", 8),
    GATE_DISP1!(CLK_MM_HDMI_PLL, "mm_hdmi_pll", "hdmi_sel", 9),
    GATE_DISP1!(CLK_MM_HDMI_AUDIO, "mm_hdmi_audio", "apll_sel", 10),
    GATE_DISP1!(CLK_MM_HDMI_SPDIF, "mm_hdmi_spdif", "apll_sel", 11),
    GATE_DISP1!(CLK_MM_TVE_FMM, "mm_tve_fmm", "mm_sel", 14),
];

static MM_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: MM_CLKS.as_ptr(),
    num_clks: MM_CLKS.len(),
};

static CLK_MT2701_MM_ID_TABLE: [platform_device_id; 2] = [
    platform_device_id { name: "clk-mt2701-mm", driver_data: &MM_DESC as *const _ as kernel_ulong_t },
    platform_device_id { /* sentinel */ ..platform_device_id::default() },
];

module_device_table!(platform, CLK_MT2701_MM_ID_TABLE);

static mut CLK_MT2701_MM_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: driver { name: "clk-mt2701-mm" },
    id_table: CLK_MT2701_MM_ID_TABLE.as_ptr(),
};

module_platform_driver!(CLK_MT2701_MM_DRV);

module_description!("MediaTek MT2701 MultiMedia ddp clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
