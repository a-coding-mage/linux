// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the surrounding kernel translation.

static VDO0_0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static VDO0_1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x114,
    clr_ofs: 0x118,
    sta_ofs: 0x110,
};

static VDO0_2_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x124,
    clr_ofs: 0x128,
    sta_ofs: 0x120,
};

macro_rules! gate_vdo0_0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &VDO0_0_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! gate_vdo0_1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &VDO0_1_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! gate_vdo0_2 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &VDO0_2_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! gate_vdo0_2_flags {
    ($id:expr, $name:expr, $parent:expr, $shift:expr, $flags:expr) => {
        gate_mtk_flags!($id, $name, $parent, &VDO0_2_CG_REGS, $shift,
                        &mtk_clk_gate_ops_setclr, $flags)
    };
}

static VDO0_CLKS: [mtk_gate; 49] = [
    // VDO0_0
    gate_vdo0_0!(CLK_VDO0_DISP_OVL0, "vdo0_disp_ovl0", "top_vpp", 0),
    gate_vdo0_0!(CLK_VDO0_DISP_COLOR0, "vdo0_disp_color0", "top_vpp", 2),
    gate_vdo0_0!(CLK_VDO0_DISP_COLOR1, "vdo0_disp_color1", "top_vpp", 3),
    gate_vdo0_0!(CLK_VDO0_DISP_CCORR0, "vdo0_disp_ccorr0", "top_vpp", 4),
    gate_vdo0_0!(CLK_VDO0_DISP_CCORR1, "vdo0_disp_ccorr1", "top_vpp", 5),
    gate_vdo0_0!(CLK_VDO0_DISP_AAL0, "vdo0_disp_aal0", "top_vpp", 6),
    gate_vdo0_0!(CLK_VDO0_DISP_AAL1, "vdo0_disp_aal1", "top_vpp", 7),
    gate_vdo0_0!(CLK_VDO0_DISP_GAMMA0, "vdo0_disp_gamma0", "top_vpp", 8),
    gate_vdo0_0!(CLK_VDO0_DISP_GAMMA1, "vdo0_disp_gamma1", "top_vpp", 9),
    gate_vdo0_0!(CLK_VDO0_DISP_DITHER0, "vdo0_disp_dither0", "top_vpp", 10),
    gate_vdo0_0!(CLK_VDO0_DISP_DITHER1, "vdo0_disp_dither1", "top_vpp", 11),
    gate_vdo0_0!(CLK_VDO0_DISP_OVL1, "vdo0_disp_ovl1", "top_vpp", 16),
    gate_vdo0_0!(CLK_VDO0_DISP_WDMA0, "vdo0_disp_wdma0", "top_vpp", 17),
    gate_vdo0_0!(CLK_VDO0_DISP_WDMA1, "vdo0_disp_wdma1", "top_vpp", 18),
    gate_vdo0_0!(CLK_VDO0_DISP_RDMA0, "vdo0_disp_rdma0", "top_vpp", 19),
    gate_vdo0_0!(CLK_VDO0_DISP_RDMA1, "vdo0_disp_rdma1", "top_vpp", 20),
    gate_vdo0_0!(CLK_VDO0_DSI0, "vdo0_dsi0", "top_vpp", 21),
    gate_vdo0_0!(CLK_VDO0_DSI1, "vdo0_dsi1", "top_vpp", 22),
    gate_vdo0_0!(CLK_VDO0_DSC_WRAP0, "vdo0_dsc_wrap0", "top_vpp", 23),
    gate_vdo0_0!(CLK_VDO0_VPP_MERGE0, "vdo0_vpp_merge0", "top_vpp", 24),
    gate_vdo0_0!(CLK_VDO0_DP_INTF0, "vdo0_dp_intf0", "top_vpp", 25),
    gate_vdo0_0!(CLK_VDO0_DISP_MUTEX0, "vdo0_disp_mutex0", "top_vpp", 26),
    gate_vdo0_0!(CLK_VDO0_DISP_IL_ROT0, "vdo0_disp_il_rot0", "top_vpp", 27),
    gate_vdo0_0!(CLK_VDO0_APB_BUS, "vdo0_apb_bus", "top_vpp", 28),
    gate_vdo0_0!(CLK_VDO0_FAKE_ENG0, "vdo0_fake_eng0", "top_vpp", 29),
    gate_vdo0_0!(CLK_VDO0_FAKE_ENG1, "vdo0_fake_eng1", "top_vpp", 30),
    // VDO0_1
    gate_vdo0_1!(CLK_VDO0_DL_ASYNC0, "vdo0_dl_async0", "top_vpp", 0),
    gate_vdo0_1!(CLK_VDO0_DL_ASYNC1, "vdo0_dl_async1", "top_vpp", 1),
    gate_vdo0_1!(CLK_VDO0_DL_ASYNC2, "vdo0_dl_async2", "top_vpp", 2),
    gate_vdo0_1!(CLK_VDO0_DL_ASYNC3, "vdo0_dl_async3", "top_vpp", 3),
    gate_vdo0_1!(CLK_VDO0_DL_ASYNC4, "vdo0_dl_async4", "top_vpp", 4),
    gate_vdo0_1!(CLK_VDO0_DISP_MONITOR0, "vdo0_disp_monitor0", "top_vpp", 5),
    gate_vdo0_1!(CLK_VDO0_DISP_MONITOR1, "vdo0_disp_monitor1", "top_vpp", 6),
    gate_vdo0_1!(CLK_VDO0_DISP_MONITOR2, "vdo0_disp_monitor2", "top_vpp", 7),
    gate_vdo0_1!(CLK_VDO0_DISP_MONITOR3, "vdo0_disp_monitor3", "top_vpp", 8),
    gate_vdo0_1!(CLK_VDO0_DISP_MONITOR4, "vdo0_disp_monitor4", "top_vpp", 9),
    gate_vdo0_1!(CLK_VDO0_SMI_GALS, "vdo0_smi_gals", "top_vpp", 10),
    gate_vdo0_1!(CLK_VDO0_SMI_COMMON, "vdo0_smi_common", "top_vpp", 11),
    gate_vdo0_1!(CLK_VDO0_SMI_EMI, "vdo0_smi_emi", "top_vpp", 12),
    gate_vdo0_1!(CLK_VDO0_SMI_IOMMU, "vdo0_smi_iommu", "top_vpp", 13),
    gate_vdo0_1!(CLK_VDO0_SMI_LARB, "vdo0_smi_larb", "top_vpp", 14),
    gate_vdo0_1!(CLK_VDO0_SMI_RSI, "vdo0_smi_rsi", "top_vpp", 15),
    // VDO0_2
    gate_vdo0_2!(CLK_VDO0_DSI0_DSI, "vdo0_dsi0_dsi", "top_dsi_occ", 0),
    gate_vdo0_2!(CLK_VDO0_DSI1_DSI, "vdo0_dsi1_dsi", "top_dsi_occ", 8),
    gate_vdo0_2_flags!(CLK_VDO0_DP_INTF0_DP_INTF, "vdo0_dp_intf0_dp_intf",
                       "top_edp", 16, CLK_SET_RATE_PARENT),
];

static VDO0_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &VDO0_CLKS,
    num_clks: array_size!(&VDO0_CLKS),
};

static CLK_MT8195_VDO0_ID_TABLE: [platform_device_id; 2] = [
    platform_device_id {
        name: "clk-mt8195-vdo0",
        driver_data: &VDO0_DESC as *const _ as kernel_ulong_t,
    },
    platform_device_id { /* sentinel */ },
];

module_device_table!(platform, CLK_MT8195_VDO0_ID_TABLE);

static mut CLK_MT8195_VDO0_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: driver {
        name: "clk-mt8195-vdo0",
    },
    id_table: &CLK_MT8195_VDO0_ID_TABLE,
};

module_platform_driver!(CLK_MT8195_VDO0_DRV);

module_description!("MediaTek MT8195 Video Output 0 clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
