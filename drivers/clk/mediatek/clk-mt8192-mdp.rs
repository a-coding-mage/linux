// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the Linux clock, platform-device, MediaTek clock,
// gate, and MT8192 clock-binding interfaces are intentionally external.

static mdp0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static mdp1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x124,
    clr_ofs: 0x128,
    sta_ofs: 0x120,
};

macro_rules! GATE_MDP0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &mdp0_cg_regs, $shift,
                  &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! GATE_MDP1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &mdp1_cg_regs, $shift,
                  &mtk_clk_gate_ops_setclr)
    };
}

static mdp_clks: [mtk_gate; 22] = [
    // MDP0
    GATE_MDP0!(CLK_MDP_RDMA0, "mdp_mdp_rdma0", "mdp_sel", 0),
    GATE_MDP0!(CLK_MDP_TDSHP0, "mdp_mdp_tdshp0", "mdp_sel", 1),
    GATE_MDP0!(CLK_MDP_IMG_DL_ASYNC0, "mdp_img_dl_async0", "mdp_sel", 2),
    GATE_MDP0!(CLK_MDP_IMG_DL_ASYNC1, "mdp_img_dl_async1", "mdp_sel", 3),
    GATE_MDP0!(CLK_MDP_RDMA1, "mdp_mdp_rdma1", "mdp_sel", 4),
    GATE_MDP0!(CLK_MDP_TDSHP1, "mdp_mdp_tdshp1", "mdp_sel", 5),
    GATE_MDP0!(CLK_MDP_SMI0, "mdp_smi0", "mdp_sel", 6),
    GATE_MDP0!(CLK_MDP_APB_BUS, "mdp_apb_bus", "mdp_sel", 7),
    GATE_MDP0!(CLK_MDP_WROT0, "mdp_mdp_wrot0", "mdp_sel", 8),
    GATE_MDP0!(CLK_MDP_RSZ0, "mdp_mdp_rsz0", "mdp_sel", 9),
    GATE_MDP0!(CLK_MDP_HDR0, "mdp_mdp_hdr0", "mdp_sel", 10),
    GATE_MDP0!(CLK_MDP_MUTEX0, "mdp_mdp_mutex0", "mdp_sel", 11),
    GATE_MDP0!(CLK_MDP_WROT1, "mdp_mdp_wrot1", "mdp_sel", 12),
    GATE_MDP0!(CLK_MDP_RSZ1, "mdp_mdp_rsz1", "mdp_sel", 13),
    GATE_MDP0!(CLK_MDP_HDR1, "mdp_mdp_hdr1", "mdp_sel", 14),
    GATE_MDP0!(CLK_MDP_FAKE_ENG0, "mdp_mdp_fake_eng0", "mdp_sel", 15),
    GATE_MDP0!(CLK_MDP_AAL0, "mdp_mdp_aal0", "mdp_sel", 16),
    GATE_MDP0!(CLK_MDP_AAL1, "mdp_mdp_aal1", "mdp_sel", 17),
    GATE_MDP0!(CLK_MDP_COLOR0, "mdp_mdp_color0", "mdp_sel", 18),
    GATE_MDP0!(CLK_MDP_COLOR1, "mdp_mdp_color1", "mdp_sel", 19),
    // MDP1
    GATE_MDP1!(CLK_MDP_IMG_DL_RELAY0_ASYNC0, "mdp_img_dl_relay0_async0", "mdp_sel", 0),
    GATE_MDP1!(CLK_MDP_IMG_DL_RELAY1_ASYNC1, "mdp_img_dl_relay1_async1", "mdp_sel", 8),
];

static mdp_desc: mtk_clk_desc = mtk_clk_desc {
    clks: mdp_clks.as_ptr(),
    num_clks: mdp_clks.len(),
};

static of_match_clk_mt8192_mdp: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8192-mdpsys",
        data: &mdp_desc,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt8192_mdp);

static mut clk_mt8192_mdp_drv: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8192-mdp",
        of_match_table: of_match_clk_mt8192_mdp.as_ptr(),
    },
};

module_platform_driver!(clk_mt8192_mdp_drv);

MODULE_DESCRIPTION!("MediaTek MT8192 Multimedia Data Path clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
