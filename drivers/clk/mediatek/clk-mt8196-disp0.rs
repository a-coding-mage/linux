// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */
// Dependency intent preserved from the C includes:
// dt-bindings/clock/mediatek,mt8196-clock.h, linux/clk-provider.h,
// linux/module.h, linux/of_device.h, linux/platform_device.h,
// clk-gate.h, and clk-mtk.h.

static const mm0_cg_regs: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static const mm0_hwv_regs: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0020,
    clr_ofs: 0x0024,
    sta_ofs: 0x2c10,
};

static const mm1_cg_regs: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x114,
    clr_ofs: 0x118,
    sta_ofs: 0x110,
};

static const mm1_hwv_regs: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0028,
    clr_ofs: 0x002c,
    sta_ofs: 0x2c14,
};

macro_rules! gate_mm0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        MtkGate { id: $id, name: $name, parent_name: $parent,
            regs: &mm0_cg_regs, shift: $shift,
            flags: CLK_OPS_PARENT_ENABLE, ops: &mtk_clk_gate_ops_setclr }
    };
}

macro_rules! gate_hwv_mm0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        MtkGate { id: $id, name: $name, parent_name: $parent,
            regs: &mm0_cg_regs, hwv_regs: &mm0_hwv_regs, shift: $shift,
            ops: &mtk_clk_gate_hwv_ops_setclr, flags: CLK_OPS_PARENT_ENABLE }
    };
}

macro_rules! gate_mm1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        MtkGate { id: $id, name: $name, parent_name: $parent,
            regs: &mm1_cg_regs, shift: $shift,
            flags: CLK_OPS_PARENT_ENABLE, ops: &mtk_clk_gate_ops_setclr }
    };
}

macro_rules! gate_hwv_mm1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        MtkGate { id: $id, name: $name, parent_name: $parent,
            regs: &mm1_cg_regs, hwv_regs: &mm1_hwv_regs, shift: $shift,
            ops: &mtk_clk_gate_hwv_ops_setclr, flags: CLK_OPS_PARENT_ENABLE }
    };
}

static mm_clks: &[MtkGate] = &[
    gate_hwv_mm0!(CLK_MM_CONFIG, "mm_config", "disp", 0),
    gate_hwv_mm0!(CLK_MM_DISP_MUTEX0, "mm_disp_mutex0", "disp", 1),
    gate_hwv_mm0!(CLK_MM_DISP_AAL0, "mm_disp_aal0", "disp", 2),
    gate_hwv_mm0!(CLK_MM_DISP_AAL1, "mm_disp_aal1", "disp", 3),
    gate_mm0!(CLK_MM_DISP_C3D0, "mm_disp_c3d0", "disp", 4),
    gate_mm0!(CLK_MM_DISP_C3D1, "mm_disp_c3d1", "disp", 5),
    gate_mm0!(CLK_MM_DISP_C3D2, "mm_disp_c3d2", "disp", 6),
    gate_mm0!(CLK_MM_DISP_C3D3, "mm_disp_c3d3", "disp", 7),
    gate_mm0!(CLK_MM_DISP_CCORR0, "mm_disp_ccorr0", "disp", 8),
    gate_mm0!(CLK_MM_DISP_CCORR1, "mm_disp_ccorr1", "disp", 9),
    gate_mm0!(CLK_MM_DISP_CCORR2, "mm_disp_ccorr2", "disp", 10),
    gate_mm0!(CLK_MM_DISP_CCORR3, "mm_disp_ccorr3", "disp", 11),
    gate_mm0!(CLK_MM_DISP_CHIST0, "mm_disp_chist0", "disp", 12),
    gate_mm0!(CLK_MM_DISP_CHIST1, "mm_disp_chist1", "disp", 13),
    gate_mm0!(CLK_MM_DISP_COLOR0, "mm_disp_color0", "disp", 14),
    gate_mm0!(CLK_MM_DISP_COLOR1, "mm_disp_color1", "disp", 15),
    gate_mm0!(CLK_MM_DISP_DITHER0, "mm_disp_dither0", "disp", 16),
    gate_mm0!(CLK_MM_DISP_DITHER1, "mm_disp_dither1", "disp", 17),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC0, "mm_disp_dli_async0", "disp", 18),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC1, "mm_disp_dli_async1", "disp", 19),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC2, "mm_disp_dli_async2", "disp", 20),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC3, "mm_disp_dli_async3", "disp", 21),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC4, "mm_disp_dli_async4", "disp", 22),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC5, "mm_disp_dli_async5", "disp", 23),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC6, "mm_disp_dli_async6", "disp", 24),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC7, "mm_disp_dli_async7", "disp", 25),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC8, "mm_disp_dli_async8", "disp", 26),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC9, "mm_disp_dli_async9", "disp", 27),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC10, "mm_disp_dli_async10", "disp", 28),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC11, "mm_disp_dli_async11", "disp", 29),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC12, "mm_disp_dli_async12", "disp", 30),
    gate_hwv_mm0!(CLK_MM_DISP_DLI_ASYNC13, "mm_disp_dli_async13", "disp", 31),
    gate_hwv_mm1!(CLK_MM_DISP_DLI_ASYNC14, "mm_disp_dli_async14", "disp", 0),
    gate_hwv_mm1!(CLK_MM_DISP_DLI_ASYNC15, "mm_disp_dli_async15", "disp", 1),
    gate_hwv_mm1!(CLK_MM_DISP_DLO_ASYNC0, "mm_disp_dlo_async0", "disp", 2),
    gate_hwv_mm1!(CLK_MM_DISP_DLO_ASYNC1, "mm_disp_dlo_async1", "disp", 3),
    gate_hwv_mm1!(CLK_MM_DISP_DLO_ASYNC2, "mm_disp_dlo_async2", "disp", 4),
    gate_hwv_mm1!(CLK_MM_DISP_DLO_ASYNC3, "mm_disp_dlo_async3", "disp", 5),
    gate_hwv_mm1!(CLK_MM_DISP_DLO_ASYNC4, "mm_disp_dlo_async4", "disp", 6),
    gate_hwv_mm1!(CLK_MM_DISP_DLO_ASYNC5, "mm_disp_dlo_async5", "disp", 7),
    gate_hwv_mm1!(CLK_MM_DISP_DLO_ASYNC6, "mm_disp_dlo_async6", "disp", 8),
    gate_hwv_mm1!(CLK_MM_DISP_DLO_ASYNC7, "mm_disp_dlo_async7", "disp", 9),
    gate_hwv_mm1!(CLK_MM_DISP_DLO_ASYNC8, "mm_disp_dlo_async8", "disp", 10),
    gate_mm1!(CLK_MM_DISP_GAMMA0, "mm_disp_gamma0", "disp", 11),
    gate_mm1!(CLK_MM_DISP_GAMMA1, "mm_disp_gamma1", "disp", 12),
    gate_mm1!(CLK_MM_MDP_AAL0, "mm_mdp_aal0", "disp", 13),
    gate_mm1!(CLK_MM_MDP_AAL1, "mm_mdp_aal1", "disp", 14),
    gate_hwv_mm1!(CLK_MM_MDP_RDMA0, "mm_mdp_rdma0", "disp", 15),
    gate_hwv_mm1!(CLK_MM_DISP_POSTMASK0, "mm_disp_postmask0", "disp", 16),
    gate_hwv_mm1!(CLK_MM_DISP_POSTMASK1, "mm_disp_postmask1", "disp", 17),
    gate_hwv_mm1!(CLK_MM_MDP_RSZ0, "mm_mdp_rsz0", "disp", 18),
    gate_hwv_mm1!(CLK_MM_MDP_RSZ1, "mm_mdp_rsz1", "disp", 19),
    gate_hwv_mm1!(CLK_MM_DISP_SPR0, "mm_disp_spr0", "disp", 20),
    gate_mm1!(CLK_MM_DISP_TDSHP0, "mm_disp_tdshp0", "disp", 21),
    gate_mm1!(CLK_MM_DISP_TDSHP1, "mm_disp_tdshp1", "disp", 22),
    gate_hwv_mm1!(CLK_MM_DISP_WDMA0, "mm_disp_wdma0", "disp", 23),
    gate_hwv_mm1!(CLK_MM_DISP_Y2R0, "mm_disp_y2r0", "disp", 24),
    gate_hwv_mm1!(CLK_MM_SMI_SUB_COMM0, "mm_ssc", "disp", 25),
    gate_hwv_mm1!(CLK_MM_DISP_FAKE_ENG0, "mm_disp_fake_eng0", "disp", 26),
];

static mm_mcd: MtkClkDesc = MtkClkDesc {
    clks: mm_clks,
    num_clks: mm_clks.len(),
};

static clk_mt8196_disp0_id_table: &[PlatformDeviceId] = &[
    PlatformDeviceId { name: "clk-mt8196-disp0", driver_data: &mm_mcd as *const _ as KernelULong },
    PlatformDeviceId { /* sentinel */ },
];
module_device_table!(platform, clk_mt8196_disp0_id_table);

static mut clk_mt8196_disp0_drv: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: Driver { name: "clk-mt8196-disp0" },
    id_table: clk_mt8196_disp0_id_table,
};
module_platform_driver!(clk_mt8196_disp0_drv);

module_description!("MediaTek MT8196 disp0 clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
