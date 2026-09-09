// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Dependencies supplied by the Linux clock and platform-driver headers.

static WPE_TOP_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

static WPE_VPP0_0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x58,
    clr_ofs: 0x58,
    sta_ofs: 0x58,
};

static WPE_VPP0_1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x5c,
    clr_ofs: 0x5c,
    sta_ofs: 0x5c,
};

macro_rules! gate_wpe_top {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &WPE_TOP_CG_REGS, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

macro_rules! gate_wpe_vpp0_0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &WPE_VPP0_0_CG_REGS, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

macro_rules! gate_wpe_vpp0_1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &WPE_VPP0_1_CG_REGS, $shift,
                  &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static WPE_TOP_CLKS: [mtk_gate; 4] = [
    gate_wpe_top!(CLK_WPE_TOP_WPE_VPP0, "wpe_wpe_vpp0", "top_wpe_vpp", 16),
    gate_wpe_top!(CLK_WPE_TOP_SMI_LARB7, "wpe_smi_larb7", "top_wpe_vpp", 18),
    gate_wpe_top!(CLK_WPE_TOP_WPESYS_EVENT_TX, "wpe_wpesys_event_tx", "top_wpe_vpp", 20),
    gate_wpe_top!(CLK_WPE_TOP_SMI_LARB7_PCLK_EN, "wpe_smi_larb7_p_en", "top_wpe_vpp", 24),
];

static WPE_VPP0_CLKS: [mtk_gate; 23] = [
    // WPE_VPP00
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_VGEN, "wpe_vpp0_vgen", "top_img", 0),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_EXT, "wpe_vpp0_ext", "top_img", 1),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_VFC, "wpe_vpp0_vfc", "top_img", 2),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_CACH0_TOP, "wpe_vpp0_cach0_top", "top_img", 3),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_CACH0_DMA, "wpe_vpp0_cach0_dma", "top_img", 4),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_CACH1_TOP, "wpe_vpp0_cach1_top", "top_img", 5),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_CACH1_DMA, "wpe_vpp0_cach1_dma", "top_img", 6),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_CACH2_TOP, "wpe_vpp0_cach2_top", "top_img", 7),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_CACH2_DMA, "wpe_vpp0_cach2_dma", "top_img", 8),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_CACH3_TOP, "wpe_vpp0_cach3_top", "top_img", 9),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_CACH3_DMA, "wpe_vpp0_cach3_dma", "top_img", 10),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_PSP, "wpe_vpp0_psp", "top_img", 11),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_PSP2, "wpe_vpp0_psp2", "top_img", 12),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_SYNC, "wpe_vpp0_sync", "top_img", 13),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_C24, "wpe_vpp0_c24", "top_img", 14),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_MDP_CROP, "wpe_vpp0_mdp_crop", "top_img", 15),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_ISP_CROP, "wpe_vpp0_isp_crop", "top_img", 16),
    gate_wpe_vpp0_0!(CLK_WPE_VPP0_TOP, "wpe_vpp0_top", "top_img", 17),
    // WPE_VPP0_1
    gate_wpe_vpp0_1!(CLK_WPE_VPP0_VECI, "wpe_vpp0_veci", "top_img", 0),
    gate_wpe_vpp0_1!(CLK_WPE_VPP0_VEC2I, "wpe_vpp0_vec2i", "top_img", 1),
    gate_wpe_vpp0_1!(CLK_WPE_VPP0_VEC3I, "wpe_vpp0_vec3i", "top_img", 2),
    gate_wpe_vpp0_1!(CLK_WPE_VPP0_WPEO, "wpe_vpp0_wpeo", "top_img", 3),
    gate_wpe_vpp0_1!(CLK_WPE_VPP0_MSKO, "wpe_vpp0_msko", "top_img", 4),
];

static WPE_TOP_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: WPE_TOP_CLKS.as_ptr(),
    num_clks: WPE_TOP_CLKS.len(),
};

static WPE_VPP0_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: WPE_VPP0_CLKS.as_ptr(),
    num_clks: WPE_VPP0_CLKS.len(),
};

static OF_MATCH_CLK_MT8188_WPE: [of_device_id; 3] = [
    of_device_id { compatible: "mediatek,mt8188-wpesys", data: &WPE_TOP_DESC },
    of_device_id { compatible: "mediatek,mt8188-wpesys-vpp0", data: &WPE_VPP0_DESC },
    of_device_id { sentinel: true },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT8188_WPE);

static mut CLK_MT8188_WPE_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt8188-wpe",
        of_match_table: OF_MATCH_CLK_MT8188_WPE.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8188_WPE_DRV);

MODULE_DESCRIPTION!("MediaTek MT8188 Warp Engine clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
