// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */

// External declarations supplied by the MediaTek clock framework and bindings.

static mdp0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static mdp1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x114,
    clr_ofs: 0x118,
    sta_ofs: 0x110,
};

static mdp2_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x124,
    clr_ofs: 0x128,
    sta_ofs: 0x120,
};

macro_rules! GATE_MDP0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &mdp0_cg_regs,
            shift: $shift, flags: CLK_OPS_PARENT_ENABLE, ops: &mtk_clk_gate_ops_setclr }
    };
}

macro_rules! GATE_MDP1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &mdp1_cg_regs,
            shift: $shift, ops: &mtk_clk_gate_ops_setclr }
    };
}

macro_rules! GATE_MDP2 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate { id: $id, name: $name, parent_name: $parent, regs: &mdp2_cg_regs,
            shift: $shift, flags: CLK_OPS_PARENT_ENABLE, ops: &mtk_clk_gate_ops_setclr }
    };
}

static mdp1_clks: [mtk_gate; 39] = [
    GATE_MDP0!(CLK_MDP1_MDP_MUTEX0, "mdp1_mdp_mutex0", "mdp", 0),
    GATE_MDP0!(CLK_MDP1_SMI0, "mdp1_smi0", "mdp", 1),
    GATE_MDP0!(CLK_MDP1_APB_BUS, "mdp1_apb_bus", "mdp", 2),
    GATE_MDP0!(CLK_MDP1_MDP_RDMA0, "mdp1_mdp_rdma0", "mdp", 3),
    GATE_MDP0!(CLK_MDP1_MDP_RDMA1, "mdp1_mdp_rdma1", "mdp", 4),
    GATE_MDP0!(CLK_MDP1_MDP_RDMA2, "mdp1_mdp_rdma2", "mdp", 5),
    GATE_MDP0!(CLK_MDP1_MDP_BIRSZ0, "mdp1_mdp_birsz0", "mdp", 6),
    GATE_MDP0!(CLK_MDP1_MDP_HDR0, "mdp1_mdp_hdr0", "mdp", 7),
    GATE_MDP0!(CLK_MDP1_MDP_AAL0, "mdp1_mdp_aal0", "mdp", 8),
    GATE_MDP0!(CLK_MDP1_MDP_RSZ0, "mdp1_mdp_rsz0", "mdp", 9),
    GATE_MDP0!(CLK_MDP1_MDP_RSZ2, "mdp1_mdp_rsz2", "mdp", 10),
    GATE_MDP0!(CLK_MDP1_MDP_TDSHP0, "mdp1_mdp_tdshp0", "mdp", 11),
    GATE_MDP0!(CLK_MDP1_MDP_COLOR0, "mdp1_mdp_color0", "mdp", 12),
    GATE_MDP0!(CLK_MDP1_MDP_WROT0, "mdp1_mdp_wrot0", "mdp", 13),
    GATE_MDP0!(CLK_MDP1_MDP_WROT1, "mdp1_mdp_wrot1", "mdp", 14),
    GATE_MDP0!(CLK_MDP1_MDP_WROT2, "mdp1_mdp_wrot2", "mdp", 15),
    GATE_MDP0!(CLK_MDP1_MDP_FAKE_ENG0, "mdp1_mdp_fake_eng0", "mdp", 16),
    GATE_MDP0!(CLK_MDP1_APB_DB, "mdp1_apb_db", "mdp", 17),
    GATE_MDP0!(CLK_MDP1_MDP_DLI_ASYNC0, "mdp1_mdp_dli_async0", "mdp", 18),
    GATE_MDP0!(CLK_MDP1_MDP_DLI_ASYNC1, "mdp1_mdp_dli_async1", "mdp", 19),
    GATE_MDP0!(CLK_MDP1_MDP_DLO_ASYNC0, "mdp1_mdp_dlo_async0", "mdp", 20),
    GATE_MDP0!(CLK_MDP1_MDP_DLO_ASYNC1, "mdp1_mdp_dlo_async1", "mdp", 21),
    GATE_MDP0!(CLK_MDP1_MDP_DLI_ASYNC2, "mdp1_mdp_dli_async2", "mdp", 22),
    GATE_MDP0!(CLK_MDP1_MDP_DLO_ASYNC2, "mdp1_mdp_dlo_async2", "mdp", 23),
    GATE_MDP0!(CLK_MDP1_MDP_DLO_ASYNC3, "mdp1_mdp_dlo_async3", "mdp", 24),
    GATE_MDP0!(CLK_MDP1_IMG_DL_ASYNC0, "mdp1_img_dl_async0", "mdp", 25),
    GATE_MDP0!(CLK_MDP1_MDP_RROT0, "mdp1_mdp_rrot0", "mdp", 26),
    GATE_MDP0!(CLK_MDP1_MDP_MERGE0, "mdp1_mdp_merge0", "mdp", 27),
    GATE_MDP0!(CLK_MDP1_MDP_C3D0, "mdp1_mdp_c3d0", "mdp", 28),
    GATE_MDP0!(CLK_MDP1_MDP_FG0, "mdp1_mdp_fg0", "mdp", 29),
    GATE_MDP0!(CLK_MDP1_MDP_CLA2, "mdp1_mdp_cla2", "mdp", 30),
    GATE_MDP0!(CLK_MDP1_MDP_DLO_ASYNC4, "mdp1_mdp_dlo_async4", "mdp", 31),
    GATE_MDP1!(CLK_MDP1_VPP_RSZ0, "mdp1_vpp_rsz0", "mdp", 0),
    GATE_MDP1!(CLK_MDP1_VPP_RSZ1, "mdp1_vpp_rsz1", "mdp", 1),
    GATE_MDP1!(CLK_MDP1_MDP_DLO_ASYNC5, "mdp1_mdp_dlo_async5", "mdp", 2),
    GATE_MDP1!(CLK_MDP1_IMG0, "mdp1_img0", "mdp", 3),
    GATE_MDP1!(CLK_MDP1_F26M, "mdp1_f26m", "clk26m", 27),
    GATE_MDP2!(CLK_MDP1_IMG_DL_RELAY0, "mdp1_img_dl_relay0", "mdp", 0),
    GATE_MDP2!(CLK_MDP1_IMG_DL_RELAY1, "mdp1_img_dl_relay1", "mdp", 8),
];

static mdp1_mcd: mtk_clk_desc = mtk_clk_desc { clks: mdp1_clks.as_ptr(), num_clks: mdp1_clks.len(), need_runtime_pm: true };

static mdp_clks: [mtk_gate; 39] = [
    GATE_MDP0!(CLK_MDP_MDP_MUTEX0, "mdp_mdp_mutex0", "mdp", 0), GATE_MDP0!(CLK_MDP_SMI0, "mdp_smi0", "mdp", 1),
    GATE_MDP0!(CLK_MDP_APB_BUS, "mdp_apb_bus", "mdp", 2), GATE_MDP0!(CLK_MDP_MDP_RDMA0, "mdp_mdp_rdma0", "mdp", 3),
    GATE_MDP0!(CLK_MDP_MDP_RDMA1, "mdp_mdp_rdma1", "mdp", 4), GATE_MDP0!(CLK_MDP_MDP_RDMA2, "mdp_mdp_rdma2", "mdp", 5),
    GATE_MDP0!(CLK_MDP_MDP_BIRSZ0, "mdp_mdp_birsz0", "mdp", 6), GATE_MDP0!(CLK_MDP_MDP_HDR0, "mdp_mdp_hdr0", "mdp", 7),
    GATE_MDP0!(CLK_MDP_MDP_AAL0, "mdp_mdp_aal0", "mdp", 8), GATE_MDP0!(CLK_MDP_MDP_RSZ0, "mdp_mdp_rsz0", "mdp", 9),
    GATE_MDP0!(CLK_MDP_MDP_RSZ2, "mdp_mdp_rsz2", "mdp", 10), GATE_MDP0!(CLK_MDP_MDP_TDSHP0, "mdp_mdp_tdshp0", "mdp", 11),
    GATE_MDP0!(CLK_MDP_MDP_COLOR0, "mdp_mdp_color0", "mdp", 12), GATE_MDP0!(CLK_MDP_MDP_WROT0, "mdp_mdp_wrot0", "mdp", 13),
    GATE_MDP0!(CLK_MDP_MDP_WROT1, "mdp_mdp_wrot1", "mdp", 14), GATE_MDP0!(CLK_MDP_MDP_WROT2, "mdp_mdp_wrot2", "mdp", 15),
    GATE_MDP0!(CLK_MDP_MDP_FAKE_ENG0, "mdp_mdp_fake_eng0", "mdp", 16), GATE_MDP0!(CLK_MDP_APB_DB, "mdp_apb_db", "mdp", 17),
    GATE_MDP0!(CLK_MDP_MDP_DLI_ASYNC0, "mdp_mdp_dli_async0", "mdp", 18), GATE_MDP0!(CLK_MDP_MDP_DLI_ASYNC1, "mdp_mdp_dli_async1", "mdp", 19),
    GATE_MDP0!(CLK_MDP_MDP_DLO_ASYNC0, "mdp_mdp_dlo_async0", "mdp", 20), GATE_MDP0!(CLK_MDP_MDP_DLO_ASYNC1, "mdp_mdp_dlo_async1", "mdp", 21),
    GATE_MDP0!(CLK_MDP_MDP_DLI_ASYNC2, "mdp_mdp_dli_async2", "mdp", 22), GATE_MDP0!(CLK_MDP_MDP_DLO_ASYNC2, "mdp_mdp_dlo_async2", "mdp", 23),
    GATE_MDP0!(CLK_MDP_MDP_DLO_ASYNC3, "mdp_mdp_dlo_async3", "mdp", 24), GATE_MDP0!(CLK_MDP_IMG_DL_ASYNC0, "mdp_img_dl_async0", "mdp", 25),
    GATE_MDP0!(CLK_MDP_MDP_RROT0, "mdp_mdp_rrot0", "mdp", 26), GATE_MDP0!(CLK_MDP_MDP_MERGE0, "mdp_mdp_merge0", "mdp", 27),
    GATE_MDP0!(CLK_MDP_MDP_C3D0, "mdp_mdp_c3d0", "mdp", 28), GATE_MDP0!(CLK_MDP_MDP_FG0, "mdp_mdp_fg0", "mdp", 29),
    GATE_MDP0!(CLK_MDP_MDP_CLA2, "mdp_mdp_cla2", "mdp", 30), GATE_MDP0!(CLK_MDP_MDP_DLO_ASYNC4, "mdp_mdp_dlo_async4", "mdp", 31),
    GATE_MDP1!(CLK_MDP_VPP_RSZ0, "mdp_vpp_rsz0", "mdp", 0), GATE_MDP1!(CLK_MDP_VPP_RSZ1, "mdp_vpp_rsz1", "mdp", 1),
    GATE_MDP1!(CLK_MDP_MDP_DLO_ASYNC5, "mdp_mdp_dlo_async5", "mdp", 2), GATE_MDP1!(CLK_MDP_IMG0, "mdp_img0", "mdp", 3),
    GATE_MDP1!(CLK_MDP_F26M, "mdp_f26m", "clk26m", 27), GATE_MDP2!(CLK_MDP_IMG_DL_RELAY0, "mdp_img_dl_relay0", "mdp", 0),
    GATE_MDP2!(CLK_MDP_IMG_DL_RELAY1, "mdp_img_dl_relay1", "mdp", 8),
];

static mdp_mcd: mtk_clk_desc = mtk_clk_desc { clks: mdp_clks.as_ptr(), num_clks: mdp_clks.len(), need_runtime_pm: true };

static of_match_clk_mt8196_mdpsys: [of_device_id; 3] = [
    of_device_id { compatible: "mediatek,mt8196-mdpsys0", data: &mdp_mcd },
    of_device_id { compatible: "mediatek,mt8196-mdpsys1", data: &mdp1_mcd },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut clk_mt8196_mdpsys_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver { name: "clk-mt8196-mdpsys", of_match_table: of_match_clk_mt8196_mdpsys.as_ptr() },
};

module_platform_driver!(clk_mt8196_mdpsys_drv);

module_description!("MediaTek MT8196 Multimedia Data Path clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
