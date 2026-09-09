// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the surrounding kernel translation.

static WPE_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

static WPE_VPP0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x58,
    clr_ofs: 0x58,
    sta_ofs: 0x58,
};

static WPE_VPP1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x5c,
    clr_ofs: 0x5c,
    sta_ofs: 0x5c,
};

macro_rules! GATE_WPE {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &WPE_CG_REGS, $shift, &mtk_clk_gate_ops_no_setclr_inv)
    };
}

macro_rules! GATE_WPE_VPP0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &WPE_VPP0_CG_REGS, $shift, &mtk_clk_gate_ops_no_setclr_inv)
    };
}

macro_rules! GATE_WPE_VPP1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &WPE_VPP1_CG_REGS, $shift, &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static WPE_CLKS: [mtk_gate; 7] = [
    GATE_WPE!(CLK_WPE_VPP0, "wpe_vpp0", "top_wpe_vpp", 16),
    GATE_WPE!(CLK_WPE_VPP1, "wpe_vpp1", "top_wpe_vpp", 17),
    GATE_WPE!(CLK_WPE_SMI_LARB7, "wpe_smi_larb7", "top_wpe_vpp", 18),
    GATE_WPE!(CLK_WPE_SMI_LARB8, "wpe_smi_larb8", "top_wpe_vpp", 19),
    GATE_WPE!(CLK_WPE_EVENT_TX, "wpe_event_tx", "top_wpe_vpp", 20),
    GATE_WPE!(CLK_WPE_SMI_LARB7_P, "wpe_smi_larb7_p", "top_wpe_vpp", 24),
    GATE_WPE!(CLK_WPE_SMI_LARB8_P, "wpe_smi_larb8_p", "top_wpe_vpp", 25),
];

static WPE_VPP0_CLKS: [mtk_gate; 23] = [
    GATE_WPE_VPP0!(CLK_WPE_VPP0_VGEN, "wpe_vpp0_vgen", "top_img", 0),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_EXT, "wpe_vpp0_ext", "top_img", 1),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_VFC, "wpe_vpp0_vfc", "top_img", 2),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_CACH0_TOP, "wpe_vpp0_cach0_top", "top_img", 3),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_CACH0_DMA, "wpe_vpp0_cach0_dma", "top_img", 4),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_CACH1_TOP, "wpe_vpp0_cach1_top", "top_img", 5),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_CACH1_DMA, "wpe_vpp0_cach1_dma", "top_img", 6),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_CACH2_TOP, "wpe_vpp0_cach2_top", "top_img", 7),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_CACH2_DMA, "wpe_vpp0_cach2_dma", "top_img", 8),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_CACH3_TOP, "wpe_vpp0_cach3_top", "top_img", 9),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_CACH3_DMA, "wpe_vpp0_cach3_dma", "top_img", 10),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_PSP, "wpe_vpp0_psp", "top_img", 11),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_PSP2, "wpe_vpp0_psp2", "top_img", 12),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_SYNC, "wpe_vpp0_sync", "top_img", 13),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_C24, "wpe_vpp0_c24", "top_img", 14),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_MDP_CROP, "wpe_vpp0_mdp_crop", "top_img", 15),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_ISP_CROP, "wpe_vpp0_isp_crop", "top_img", 16),
    GATE_WPE_VPP0!(CLK_WPE_VPP0_TOP, "wpe_vpp0_top", "top_img", 17),
    GATE_WPE_VPP1!(CLK_WPE_VPP0_VECI, "wpe_vpp0_veci", "top_img", 0),
    GATE_WPE_VPP1!(CLK_WPE_VPP0_VEC2I, "wpe_vpp0_vec2i", "top_img", 1),
    GATE_WPE_VPP1!(CLK_WPE_VPP0_VEC3I, "wpe_vpp0_vec3i", "top_img", 2),
    GATE_WPE_VPP1!(CLK_WPE_VPP0_WPEO, "wpe_vpp0_wpeo", "top_img", 3),
    GATE_WPE_VPP1!(CLK_WPE_VPP0_MSKO, "wpe_vpp0_msko", "top_img", 4),
];

static WPE_VPP1_CLKS: [mtk_gate; 23] = [
    GATE_WPE_VPP0!(CLK_WPE_VPP1_VGEN, "wpe_vpp1_vgen", "top_img", 0),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_EXT, "wpe_vpp1_ext", "top_img", 1),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_VFC, "wpe_vpp1_vfc", "top_img", 2),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_CACH0_TOP, "wpe_vpp1_cach0_top", "top_img", 3),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_CACH0_DMA, "wpe_vpp1_cach0_dma", "top_img", 4),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_CACH1_TOP, "wpe_vpp1_cach1_top", "top_img", 5),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_CACH1_DMA, "wpe_vpp1_cach1_dma", "top_img", 6),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_CACH2_TOP, "wpe_vpp1_cach2_top", "top_img", 7),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_CACH2_DMA, "wpe_vpp1_cach2_dma", "top_img", 8),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_CACH3_TOP, "wpe_vpp1_cach3_top", "top_img", 9),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_CACH3_DMA, "wpe_vpp1_cach3_dma", "top_img", 10),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_PSP, "wpe_vpp1_psp", "top_img", 11),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_PSP2, "wpe_vpp1_psp2", "top_img", 12),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_SYNC, "wpe_vpp1_sync", "top_img", 13),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_C24, "wpe_vpp1_c24", "top_img", 14),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_MDP_CROP, "wpe_vpp1_mdp_crop", "top_img", 15),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_ISP_CROP, "wpe_vpp1_isp_crop", "top_img", 16),
    GATE_WPE_VPP0!(CLK_WPE_VPP1_TOP, "wpe_vpp1_top", "top_img", 17),
    GATE_WPE_VPP1!(CLK_WPE_VPP1_VECI, "wpe_vpp1_veci", "top_img", 0),
    GATE_WPE_VPP1!(CLK_WPE_VPP1_VEC2I, "wpe_vpp1_vec2i", "top_img", 1),
    GATE_WPE_VPP1!(CLK_WPE_VPP1_VEC3I, "wpe_vpp1_vec3i", "top_img", 2),
    GATE_WPE_VPP1!(CLK_WPE_VPP1_WPEO, "wpe_vpp1_wpeo", "top_img", 3),
    GATE_WPE_VPP1!(CLK_WPE_VPP1_MSKO, "wpe_vpp1_msko", "top_img", 4),
];

static WPE_DESC: mtk_clk_desc = mtk_clk_desc { clks: &WPE_CLKS, num_clks: WPE_CLKS.len() };
static WPE_VPP0_DESC: mtk_clk_desc = mtk_clk_desc { clks: &WPE_VPP0_CLKS, num_clks: WPE_VPP0_CLKS.len() };
static WPE_VPP1_DESC: mtk_clk_desc = mtk_clk_desc { clks: &WPE_VPP1_CLKS, num_clks: WPE_VPP1_CLKS.len() };

static OF_MATCH_CLK_MT8195_WPE: [of_device_id; 4] = [
    of_device_id { compatible: "mediatek,mt8195-wpesys", data: &WPE_DESC },
    of_device_id { compatible: "mediatek,mt8195-wpesys_vpp0", data: &WPE_VPP0_DESC },
    of_device_id { compatible: "mediatek,mt8195-wpesys_vpp1", data: &WPE_VPP1_DESC },
    of_device_id { ..Default::default() },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT8195_WPE);

static mut CLK_MT8195_WPE_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8195-wpe",
        of_match_table: OF_MATCH_CLK_MT8195_WPE.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8195_WPE_DRV);

MODULE_DESCRIPTION!("MediaTek MT8195 Warp Engine clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
