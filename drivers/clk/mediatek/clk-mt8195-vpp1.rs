// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the surrounding kernel translation:
// clk-gate.h, clk-mtk.h, dt-bindings/clock/mt8195-clk.h,
// linux/clk-provider.h, and linux/platform_device.h.

static VPP1_0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static VPP1_1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x114,
    clr_ofs: 0x118,
    sta_ofs: 0x110,
};

macro_rules! GATE_VPP1_0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VPP1_0_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! GATE_VPP1_1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VPP1_1_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static VPP1_CLKS: [mtk_gate; 50] = [
    // VPP1_0
    GATE_VPP1_0!(CLK_VPP1_SVPP1_MDP_OVL, "vpp1_svpp1_mdp_ovl", "top_vpp", 0),
    GATE_VPP1_0!(CLK_VPP1_SVPP1_MDP_TCC, "vpp1_svpp1_mdp_tcc", "top_vpp", 1),
    GATE_VPP1_0!(CLK_VPP1_SVPP1_MDP_WROT, "vpp1_svpp1_mdp_wrot", "top_vpp", 2),
    GATE_VPP1_0!(CLK_VPP1_SVPP1_VPP_PAD, "vpp1_svpp1_vpp_pad", "top_vpp", 3),
    GATE_VPP1_0!(CLK_VPP1_SVPP2_MDP_WROT, "vpp1_svpp2_mdp_wrot", "top_vpp", 4),
    GATE_VPP1_0!(CLK_VPP1_SVPP2_VPP_PAD, "vpp1_svpp2_vpp_pad", "top_vpp", 5),
    GATE_VPP1_0!(CLK_VPP1_SVPP3_MDP_WROT, "vpp1_svpp3_mdp_wrot", "top_vpp", 6),
    GATE_VPP1_0!(CLK_VPP1_SVPP3_VPP_PAD, "vpp1_svpp3_vpp_pad", "top_vpp", 7),
    GATE_VPP1_0!(CLK_VPP1_SVPP1_MDP_RDMA, "vpp1_svpp1_mdp_rdma", "top_vpp", 8),
    GATE_VPP1_0!(CLK_VPP1_SVPP1_MDP_FG, "vpp1_svpp1_mdp_fg", "top_vpp", 9),
    GATE_VPP1_0!(CLK_VPP1_SVPP2_MDP_RDMA, "vpp1_svpp2_mdp_rdma", "top_vpp", 10),
    GATE_VPP1_0!(CLK_VPP1_SVPP2_MDP_FG, "vpp1_svpp2_mdp_fg", "top_vpp", 11),
    GATE_VPP1_0!(CLK_VPP1_SVPP3_MDP_RDMA, "vpp1_svpp3_mdp_rdma", "top_vpp", 12),
    GATE_VPP1_0!(CLK_VPP1_SVPP3_MDP_FG, "vpp1_svpp3_mdp_fg", "top_vpp", 13),
    GATE_VPP1_0!(CLK_VPP1_VPP_SPLIT, "vpp1_vpp_split", "top_vpp", 14),
    GATE_VPP1_0!(CLK_VPP1_SVPP2_VDO0_DL_RELAY, "vpp1_svpp2_vdo0_dl_relay", "top_vpp", 15),
    GATE_VPP1_0!(CLK_VPP1_SVPP1_MDP_TDSHP, "vpp1_svpp1_mdp_tdshp", "top_vpp", 16),
    GATE_VPP1_0!(CLK_VPP1_SVPP1_MDP_COLOR, "vpp1_svpp1_mdp_color", "top_vpp", 17),
    GATE_VPP1_0!(CLK_VPP1_SVPP3_VDO1_DL_RELAY, "vpp1_svpp3_vdo1_dl_relay", "top_vpp", 18),
    GATE_VPP1_0!(CLK_VPP1_SVPP2_VPP_MERGE, "vpp1_svpp2_vpp_merge", "top_vpp", 19),
    GATE_VPP1_0!(CLK_VPP1_SVPP2_MDP_COLOR, "vpp1_svpp2_mdp_color", "top_vpp", 20),
    GATE_VPP1_0!(CLK_VPP1_VPPSYS1_GALS, "vpp1_vppsys1_gals", "top_vpp", 21),
    GATE_VPP1_0!(CLK_VPP1_SVPP3_VPP_MERGE, "vpp1_svpp3_vpp_merge", "top_vpp", 22),
    GATE_VPP1_0!(CLK_VPP1_SVPP3_MDP_COLOR, "vpp1_svpp3_mdp_color", "top_vpp", 23),
    GATE_VPP1_0!(CLK_VPP1_VPPSYS1_LARB, "vpp1_vppsys1_larb", "top_vpp", 24),
    GATE_VPP1_0!(CLK_VPP1_SVPP1_MDP_RSZ, "vpp1_svpp1_mdp_rsz", "top_vpp", 25),
    GATE_VPP1_0!(CLK_VPP1_SVPP1_MDP_HDR, "vpp1_svpp1_mdp_hdr", "top_vpp", 26),
    GATE_VPP1_0!(CLK_VPP1_SVPP1_MDP_AAL, "vpp1_svpp1_mdp_aal", "top_vpp", 27),
    GATE_VPP1_0!(CLK_VPP1_SVPP2_MDP_HDR, "vpp1_svpp2_mdp_hdr", "top_vpp", 28),
    GATE_VPP1_0!(CLK_VPP1_SVPP2_MDP_AAL, "vpp1_svpp2_mdp_aal", "top_vpp", 29),
    GATE_VPP1_0!(CLK_VPP1_DL_ASYNC, "vpp1_dl_async", "top_vpp", 30),
    GATE_VPP1_0!(CLK_VPP1_LARB5_FAKE_ENG, "vpp1_larb5_fake_eng", "top_vpp", 31),
    // VPP1_1
    GATE_VPP1_1!(CLK_VPP1_SVPP3_MDP_HDR, "vpp1_svpp3_mdp_hdr", "top_vpp", 0),
    GATE_VPP1_1!(CLK_VPP1_SVPP3_MDP_AAL, "vpp1_svpp3_mdp_aal", "top_vpp", 1),
    GATE_VPP1_1!(CLK_VPP1_SVPP2_VDO1_DL_RELAY, "vpp1_svpp2_vdo1_dl_relay", "top_vpp", 2),
    GATE_VPP1_1!(CLK_VPP1_LARB6_FAKE_ENG, "vpp1_larb6_fake_eng", "top_vpp", 3),
    GATE_VPP1_1!(CLK_VPP1_SVPP2_MDP_RSZ, "vpp1_svpp2_mdp_rsz", "top_vpp", 4),
    GATE_VPP1_1!(CLK_VPP1_SVPP3_MDP_RSZ, "vpp1_svpp3_mdp_rsz", "top_vpp", 5),
    GATE_VPP1_1!(CLK_VPP1_SVPP3_VDO0_DL_RELAY, "vpp1_svpp3_vdo0_dl_relay", "top_vpp", 6),
    GATE_VPP1_1!(CLK_VPP1_DISP_MUTEX, "vpp1_disp_mutex", "top_vpp", 7),
    GATE_VPP1_1!(CLK_VPP1_SVPP2_MDP_TDSHP, "vpp1_svpp2_mdp_tdshp", "top_vpp", 8),
    GATE_VPP1_1!(CLK_VPP1_SVPP3_MDP_TDSHP, "vpp1_svpp3_mdp_tdshp", "top_vpp", 9),
    GATE_VPP1_1!(CLK_VPP1_VPP0_DL1_RELAY, "vpp1_vpp0_dl1_relay", "top_vpp", 10),
    GATE_VPP1_1!(CLK_VPP1_HDMI_META, "vpp1_hdmi_meta", "hdmirx_p", 11),
    GATE_VPP1_1!(CLK_VPP1_VPP_SPLIT_HDMI, "vpp1_vpp_split_hdmi", "hdmirx_p", 12),
    GATE_VPP1_1!(CLK_VPP1_DGI_IN, "vpp1_dgi_in", "in_dgi", 13),
    GATE_VPP1_1!(CLK_VPP1_DGI_OUT, "vpp1_dgi_out", "top_dgi_out", 14),
    GATE_VPP1_1!(CLK_VPP1_VPP_SPLIT_DGI, "vpp1_vpp_split_dgi", "top_dgi_out", 15),
    GATE_VPP1_1!(CLK_VPP1_VPP0_DL_ASYNC, "vpp1_vpp0_dl_async", "top_vpp", 16),
    GATE_VPP1_1!(CLK_VPP1_VPP0_DL_RELAY, "vpp1_vpp0_dl_relay", "top_vpp", 17),
    GATE_VPP1_1!(CLK_VPP1_VPP_SPLIT_26M, "vpp1_vpp_split_26m", "clk26m", 26),
];

static VPP1_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: VPP1_CLKS.as_ptr(),
    num_clks: VPP1_CLKS.len(),
};

static CLK_MT8195_VPP1_ID_TABLE: [platform_device_id; 2] = [
    platform_device_id {
        name: "clk-mt8195-vpp1",
        driver_data: (&VPP1_DESC as *const mtk_clk_desc) as kernel_ulong_t,
    },
    platform_device_id { /* sentinel */
        ..unsafe { core::mem::zeroed() }
    },
];

MODULE_DEVICE_TABLE!(platform, CLK_MT8195_VPP1_ID_TABLE);

static mut CLK_MT8195_VPP1_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: driver {
        name: "clk-mt8195-vpp1",
        ..unsafe { core::mem::zeroed() }
    },
    id_table: CLK_MT8195_VPP1_ID_TABLE.as_ptr(),
};

module_platform_driver!(CLK_MT8195_VPP1_DRV);

MODULE_DESCRIPTION!("MediaTek MT8195 Video Processing Pipe 1 clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
