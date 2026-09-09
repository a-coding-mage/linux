// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */
// C dependencies: dt-bindings/clock/mediatek,mt8196-clock.h,
// linux/clk-provider.h, linux/module.h, linux/of_device.h,
// linux/platform_device.h, clk-gate.h, and clk-mtk.h.

static MM10_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static MM10_HWV_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0010,
    clr_ofs: 0x0014,
    sta_ofs: 0x2c08,
};

static MM11_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x114,
    clr_ofs: 0x118,
    sta_ofs: 0x110,
};

static MM11_HWV_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0018,
    clr_ofs: 0x001c,
    sta_ofs: 0x2c0c,
};

macro_rules! gate_hwv_mm10 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate { id: $id, name: $name, parent_name: $parent,
            regs: &MM10_CG_REGS, hwv_regs: &MM10_HWV_REGS, shift: $shift,
            ops: &mtk_clk_gate_hwv_ops_setclr, flags: CLK_OPS_PARENT_ENABLE }
    };
}
macro_rules! gate_hwv_mm11 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate { id: $id, name: $name, parent_name: $parent,
            regs: &MM11_CG_REGS, hwv_regs: &MM11_HWV_REGS, shift: $shift,
            ops: &mtk_clk_gate_hwv_ops_setclr }
    };
}

static MM1_CLKS: [mtk_gate; 57] = [
    // MM10
    gate_hwv_mm10!(CLK_MM1_DISPSYS1_CONFIG, "mm1_dispsys1_config", "disp", 0),
    gate_hwv_mm10!(CLK_MM1_DISPSYS1_S_CONFIG, "mm1_dispsys1_s_config", "disp", 1),
    gate_hwv_mm10!(CLK_MM1_DISP_MUTEX0, "mm1_disp_mutex0", "disp", 2),
    gate_hwv_mm10!(CLK_MM1_DISP_DLI_ASYNC20, "mm1_disp_dli_async20", "disp", 3),
    gate_hwv_mm10!(CLK_MM1_DISP_DLI_ASYNC21, "mm1_disp_dli_async21", "disp", 4),
    gate_hwv_mm10!(CLK_MM1_DISP_DLI_ASYNC22, "mm1_disp_dli_async22", "disp", 5),
    gate_hwv_mm10!(CLK_MM1_DISP_DLI_ASYNC23, "mm1_disp_dli_async23", "disp", 6),
    gate_hwv_mm10!(CLK_MM1_DISP_DLI_ASYNC24, "mm1_disp_dli_async24", "disp", 7),
    gate_hwv_mm10!(CLK_MM1_DISP_DLI_ASYNC25, "mm1_disp_dli_async25", "disp", 8),
    gate_hwv_mm10!(CLK_MM1_DISP_DLI_ASYNC26, "mm1_disp_dli_async26", "disp", 9),
    gate_hwv_mm10!(CLK_MM1_DISP_DLI_ASYNC27, "mm1_disp_dli_async27", "disp", 10),
    gate_hwv_mm10!(CLK_MM1_DISP_DLI_ASYNC28, "mm1_disp_dli_async28", "disp", 11),
    gate_hwv_mm10!(CLK_MM1_DISP_RELAY0, "mm1_disp_relay0", "disp", 12),
    gate_hwv_mm10!(CLK_MM1_DISP_RELAY1, "mm1_disp_relay1", "disp", 13),
    gate_hwv_mm10!(CLK_MM1_DISP_RELAY2, "mm1_disp_relay2", "disp", 14),
    gate_hwv_mm10!(CLK_MM1_DISP_RELAY3, "mm1_disp_relay3", "disp", 15),
    gate_hwv_mm10!(CLK_MM1_DISP_DP_INTF0, "mm1_DP_CLK", "disp", 16),
    gate_hwv_mm10!(CLK_MM1_DISP_DP_INTF1, "mm1_disp_dp_intf1", "disp", 17),
    gate_hwv_mm10!(CLK_MM1_DISP_DSC_WRAP0, "mm1_disp_dsc_wrap0", "disp", 18),
    gate_hwv_mm10!(CLK_MM1_DISP_DSC_WRAP1, "mm1_disp_dsc_wrap1", "disp", 19),
    gate_hwv_mm10!(CLK_MM1_DISP_DSC_WRAP2, "mm1_disp_dsc_wrap2", "disp", 20),
    gate_hwv_mm10!(CLK_MM1_DISP_DSC_WRAP3, "mm1_disp_dsc_wrap3", "disp", 21),
    gate_hwv_mm10!(CLK_MM1_DISP_DSI0, "mm1_CLK0", "disp", 22),
    gate_hwv_mm10!(CLK_MM1_DISP_DSI1, "mm1_CLK1", "disp", 23),
    gate_hwv_mm10!(CLK_MM1_DISP_DSI2, "mm1_CLK2", "disp", 24),
    gate_hwv_mm10!(CLK_MM1_DISP_DVO0, "mm1_disp_dvo0", "disp", 25),
    gate_hwv_mm10!(CLK_MM1_DISP_GDMA0, "mm1_disp_gdma0", "disp", 26),
    gate_hwv_mm10!(CLK_MM1_DISP_MERGE0, "mm1_disp_merge0", "disp", 27),
    gate_hwv_mm10!(CLK_MM1_DISP_MERGE1, "mm1_disp_merge1", "disp", 28),
    gate_hwv_mm10!(CLK_MM1_DISP_MERGE2, "mm1_disp_merge2", "disp", 29),
    gate_hwv_mm10!(CLK_MM1_DISP_ODDMR0, "mm1_disp_oddmr0", "disp", 30),
    gate_hwv_mm10!(CLK_MM1_DISP_POSTALIGN0, "mm1_disp_postalign0", "disp", 31),
    // MM11
    gate_hwv_mm11!(CLK_MM1_DISP_DITHER2, "mm1_disp_dither2", "disp", 0),
    gate_hwv_mm11!(CLK_MM1_DISP_R2Y0, "mm1_disp_r2y0", "disp", 1),
    gate_hwv_mm11!(CLK_MM1_DISP_SPLITTER0, "mm1_disp_splitter0", "disp", 2),
    gate_hwv_mm11!(CLK_MM1_DISP_SPLITTER1, "mm1_disp_splitter1", "disp", 3),
    gate_hwv_mm11!(CLK_MM1_DISP_SPLITTER2, "mm1_disp_splitter2", "disp", 4),
    gate_hwv_mm11!(CLK_MM1_DISP_SPLITTER3, "mm1_disp_splitter3", "disp", 5),
    gate_hwv_mm11!(CLK_MM1_DISP_VDCM0, "mm1_disp_vdcm0", "disp", 6),
    gate_hwv_mm11!(CLK_MM1_DISP_WDMA1, "mm1_disp_wdma1", "disp", 7),
    gate_hwv_mm11!(CLK_MM1_DISP_WDMA2, "mm1_disp_wdma2", "disp", 8),
    gate_hwv_mm11!(CLK_MM1_DISP_WDMA3, "mm1_disp_wdma3", "disp", 9),
    gate_hwv_mm11!(CLK_MM1_DISP_WDMA4, "mm1_disp_wdma4", "disp", 10),
    gate_hwv_mm11!(CLK_MM1_MDP_RDMA1, "mm1_mdp_rdma1", "disp", 11),
    gate_hwv_mm11!(CLK_MM1_SMI_LARB0, "mm1_smi_larb0", "disp", 12),
    gate_hwv_mm11!(CLK_MM1_MOD1, "mm1_mod1", "clk26m", 13),
    gate_hwv_mm11!(CLK_MM1_MOD2, "mm1_mod2", "clk26m", 14),
    gate_hwv_mm11!(CLK_MM1_MOD3, "mm1_mod3", "clk26m", 15),
    gate_hwv_mm11!(CLK_MM1_MOD4, "mm1_mod4", "dp0", 16),
    gate_hwv_mm11!(CLK_MM1_MOD5, "mm1_mod5", "dp1", 17),
    gate_hwv_mm11!(CLK_MM1_MOD6, "mm1_mod6", "dp1", 18),
    gate_hwv_mm11!(CLK_MM1_CG0, "mm1_cg0", "disp", 20),
    gate_hwv_mm11!(CLK_MM1_CG1, "mm1_cg1", "disp", 21),
    gate_hwv_mm11!(CLK_MM1_CG2, "mm1_cg2", "disp", 22),
    gate_hwv_mm11!(CLK_MM1_CG3, "mm1_cg3", "disp", 23),
    gate_hwv_mm11!(CLK_MM1_CG4, "mm1_cg4", "disp", 24),
    gate_hwv_mm11!(CLK_MM1_CG5, "mm1_cg5", "disp", 25),
    gate_hwv_mm11!(CLK_MM1_CG6, "mm1_cg6", "disp", 26),
    gate_hwv_mm11!(CLK_MM1_CG7, "mm1_cg7", "disp", 27),
    gate_hwv_mm11!(CLK_MM1_F26M, "mm1_f26m_ck", "clk26m", 28),
];

static MM1_MCD: mtk_clk_desc = mtk_clk_desc {
    clks: MM1_CLKS.as_ptr(),
    num_clks: MM1_CLKS.len(),
};

static CLK_MT8196_DISP1_ID_TABLE: [platform_device_id; 2] = [
    platform_device_id { name: "clk-mt8196-disp1", driver_data: &MM1_MCD as *const _ as kernel_ulong_t },
    platform_device_id { name: "", driver_data: 0 },
];

static mut CLK_MT8196_DISP1_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: driver { name: "clk-mt8196-disp1" },
    id_table: CLK_MT8196_DISP1_ID_TABLE.as_ptr(),
};

// MODULE_DEVICE_TABLE(platform, clk_mt8196_disp1_id_table);
// module_platform_driver(clk_mt8196_disp1_drv);
// MODULE_DESCRIPTION("MediaTek MT8196 disp1 clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
