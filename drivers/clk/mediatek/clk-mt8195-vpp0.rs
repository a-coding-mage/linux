// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the original C headers:
// "clk-gate.h", "clk-mtk.h", <dt-bindings/clock/mt8195-clk.h>,
// <linux/clk-provider.h>, and <linux/platform_device.h>.

static VPP0_0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x24,
    clr_ofs: 0x28,
    sta_ofs: 0x20,
};

static VPP0_1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x30,
    clr_ofs: 0x34,
    sta_ofs: 0x2c,
};

static VPP0_2_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x3c,
    clr_ofs: 0x40,
    sta_ofs: 0x38,
};

macro_rules! GATE_VPP0_0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VPP0_0_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! GATE_VPP0_1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VPP0_1_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! GATE_VPP0_2 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VPP0_2_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static VPP0_CLKS: [mtk_gate; 43] = [
    // VPP0_0
    GATE_VPP0_0!(CLK_VPP0_MDP_FG, "vpp0_mdp_fg", "top_vpp", 1),
    GATE_VPP0_0!(CLK_VPP0_STITCH, "vpp0_stitch", "top_vpp", 2),
    GATE_VPP0_0!(CLK_VPP0_PADDING, "vpp0_padding", "top_vpp", 7),
    GATE_VPP0_0!(CLK_VPP0_MDP_TCC, "vpp0_mdp_tcc", "top_vpp", 8),
    GATE_VPP0_0!(CLK_VPP0_WARP0_ASYNC_TX, "vpp0_warp0_async_tx", "top_vpp", 10),
    GATE_VPP0_0!(CLK_VPP0_WARP1_ASYNC_TX, "vpp0_warp1_async_tx", "top_vpp", 11),
    GATE_VPP0_0!(CLK_VPP0_MUTEX, "vpp0_mutex", "top_vpp", 13),
    GATE_VPP0_0!(CLK_VPP0_VPP02VPP1_RELAY, "vpp0_vpp02vpp1_relay", "top_vpp", 14),
    GATE_VPP0_0!(CLK_VPP0_VPP12VPP0_ASYNC, "vpp0_vpp12vpp0_async", "top_vpp", 15),
    GATE_VPP0_0!(CLK_VPP0_MMSYSRAM_TOP, "vpp0_mmsysram_top", "top_vpp", 16),
    GATE_VPP0_0!(CLK_VPP0_MDP_AAL, "vpp0_mdp_aal", "top_vpp", 17),
    GATE_VPP0_0!(CLK_VPP0_MDP_RSZ, "vpp0_mdp_rsz", "top_vpp", 18),
    // VPP0_1
    GATE_VPP0_1!(CLK_VPP0_SMI_COMMON, "vpp0_smi_common", "top_vpp", 0),
    GATE_VPP0_1!(CLK_VPP0_GALS_VDO0_LARB0, "vpp0_gals_vdo0_larb0", "top_vpp", 1),
    GATE_VPP0_1!(CLK_VPP0_GALS_VDO0_LARB1, "vpp0_gals_vdo0_larb1", "top_vpp", 2),
    GATE_VPP0_1!(CLK_VPP0_GALS_VENCSYS, "vpp0_gals_vencsys", "top_vpp", 3),
    GATE_VPP0_1!(CLK_VPP0_GALS_VENCSYS_CORE1, "vpp0_gals_vencsys_core1", "top_vpp", 4),
    GATE_VPP0_1!(CLK_VPP0_GALS_INFRA, "vpp0_gals_infra", "top_vpp", 5),
    GATE_VPP0_1!(CLK_VPP0_GALS_CAMSYS, "vpp0_gals_camsys", "top_vpp", 6),
    GATE_VPP0_1!(CLK_VPP0_GALS_VPP1_LARB5, "vpp0_gals_vpp1_larb5", "top_vpp", 7),
    GATE_VPP0_1!(CLK_VPP0_GALS_VPP1_LARB6, "vpp0_gals_vpp1_larb6", "top_vpp", 8),
    GATE_VPP0_1!(CLK_VPP0_SMI_REORDER, "vpp0_smi_reorder", "top_vpp", 9),
    GATE_VPP0_1!(CLK_VPP0_SMI_IOMMU, "vpp0_smi_iommu", "top_vpp", 10),
    GATE_VPP0_1!(CLK_VPP0_GALS_IMGSYS_CAMSYS, "vpp0_gals_imgsys_camsys", "top_vpp", 11),
    GATE_VPP0_1!(CLK_VPP0_MDP_RDMA, "vpp0_mdp_rdma", "top_vpp", 12),
    GATE_VPP0_1!(CLK_VPP0_MDP_WROT, "vpp0_mdp_wrot", "top_vpp", 13),
    GATE_VPP0_1!(CLK_VPP0_GALS_EMI0_EMI1, "vpp0_gals_emi0_emi1", "top_vpp", 16),
    GATE_VPP0_1!(CLK_VPP0_SMI_SUB_COMMON_REORDER, "vpp0_smi_sub_common_reorder", "top_vpp", 17),
    GATE_VPP0_1!(CLK_VPP0_SMI_RSI, "vpp0_smi_rsi", "top_vpp", 18),
    GATE_VPP0_1!(CLK_VPP0_SMI_COMMON_LARB4, "vpp0_smi_common_larb4", "top_vpp", 19),
    GATE_VPP0_1!(CLK_VPP0_GALS_VDEC_VDEC_CORE1, "vpp0_gals_vdec_vdec_core1", "top_vpp", 20),
    GATE_VPP0_1!(CLK_VPP0_GALS_VPP1_WPE, "vpp0_gals_vpp1_wpe", "top_vpp", 21),
    GATE_VPP0_1!(CLK_VPP0_GALS_VDO0_VDO1_VENCSYS_CORE1, "vpp0_gals_vdo0_vdo1_vencsys_core1", "top_vpp", 22),
    GATE_VPP0_1!(CLK_VPP0_FAKE_ENG, "vpp0_fake_eng", "top_vpp", 23),
    GATE_VPP0_1!(CLK_VPP0_MDP_HDR, "vpp0_mdp_hdr", "top_vpp", 24),
    GATE_VPP0_1!(CLK_VPP0_MDP_TDSHP, "vpp0_mdp_tdshp", "top_vpp", 25),
    GATE_VPP0_1!(CLK_VPP0_MDP_COLOR, "vpp0_mdp_color", "top_vpp", 26),
    GATE_VPP0_1!(CLK_VPP0_MDP_OVL, "vpp0_mdp_ovl", "top_vpp", 27),
    // VPP0_2
    GATE_VPP0_2!(CLK_VPP0_WARP0_RELAY, "vpp0_warp0_relay", "top_wpe_vpp", 0),
    GATE_VPP0_2!(CLK_VPP0_WARP0_MDP_DL_ASYNC, "vpp0_warp0_mdp_dl_async", "top_wpe_vpp", 1),
    GATE_VPP0_2!(CLK_VPP0_WARP1_RELAY, "vpp0_warp1_relay", "top_wpe_vpp", 2),
    GATE_VPP0_2!(CLK_VPP0_WARP1_MDP_DL_ASYNC, "vpp0_warp1_mdp_dl_async", "top_wpe_vpp", 3),
];

static VPP0_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: VPP0_CLKS.as_ptr(),
    num_clks: VPP0_CLKS.len(),
};

static CLK_MT8195_VPP0_ID_TABLE: [platform_device_id; 2] = [
    platform_device_id {
        name: "clk-mt8195-vpp0",
        driver_data: &VPP0_DESC as *const _ as kernel_ulong_t,
    },
    platform_device_id { /* sentinel */ ..platform_device_id::default() },
];

MODULE_DEVICE_TABLE!(platform, CLK_MT8195_VPP0_ID_TABLE);

static mut CLK_MT8195_VPP0_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: driver {
        name: "clk-mt8195-vpp0",
    },
    id_table: CLK_MT8195_VPP0_ID_TABLE.as_ptr(),
};

module_platform_driver!(CLK_MT8195_VPP0_DRV);

MODULE_DESCRIPTION!("MediaTek MT8195 Video Processing Pipe 0 clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
