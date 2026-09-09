// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the Linux clock and platform-driver infrastructure:
// linux/clk-provider.h, linux/platform_device.h, clk-mtk.h, clk-gate.h,
// and dt-bindings/clock/mt8192-clk.h.

static MM0_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static MM1_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x114,
    clr_ofs: 0x118,
    sta_ofs: 0x110,
};

static MM2_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x1a4,
    clr_ofs: 0x1a8,
    sta_ofs: 0x1a0,
};

macro_rules! gate_mm0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &MM0_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! gate_mm1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &MM1_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! gate_mm2 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &MM2_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static MM_CLKS: [MtkGate; 36] = [
    // MM0
    gate_mm0!(CLK_MM_DISP_MUTEX0, "mm_disp_mutex0", "disp_sel", 0),
    gate_mm0!(CLK_MM_DISP_CONFIG, "mm_disp_config", "disp_sel", 1),
    gate_mm0!(CLK_MM_DISP_OVL0, "mm_disp_ovl0", "disp_sel", 2),
    gate_mm0!(CLK_MM_DISP_RDMA0, "mm_disp_rdma0", "disp_sel", 3),
    gate_mm0!(CLK_MM_DISP_OVL0_2L, "mm_disp_ovl0_2l", "disp_sel", 4),
    gate_mm0!(CLK_MM_DISP_WDMA0, "mm_disp_wdma0", "disp_sel", 5),
    gate_mm0!(CLK_MM_DISP_UFBC_WDMA0, "mm_disp_ufbc_wdma0", "disp_sel", 6),
    gate_mm0!(CLK_MM_DISP_RSZ0, "mm_disp_rsz0", "disp_sel", 7),
    gate_mm0!(CLK_MM_DISP_AAL0, "mm_disp_aal0", "disp_sel", 8),
    gate_mm0!(CLK_MM_DISP_CCORR0, "mm_disp_ccorr0", "disp_sel", 9),
    gate_mm0!(CLK_MM_DISP_DITHER0, "mm_disp_dither0", "disp_sel", 10),
    gate_mm0!(CLK_MM_SMI_INFRA, "mm_smi_infra", "disp_sel", 11),
    gate_mm0!(CLK_MM_DISP_GAMMA0, "mm_disp_gamma0", "disp_sel", 12),
    gate_mm0!(CLK_MM_DISP_POSTMASK0, "mm_disp_postmask0", "disp_sel", 13),
    gate_mm0!(CLK_MM_DISP_DSC_WRAP0, "mm_disp_dsc_wrap0", "disp_sel", 14),
    gate_mm0!(CLK_MM_DSI0, "mm_dsi0", "disp_sel", 15),
    gate_mm0!(CLK_MM_DISP_COLOR0, "mm_disp_color0", "disp_sel", 16),
    gate_mm0!(CLK_MM_SMI_COMMON, "mm_smi_common", "disp_sel", 17),
    gate_mm0!(CLK_MM_DISP_FAKE_ENG0, "mm_disp_fake_eng0", "disp_sel", 18),
    gate_mm0!(CLK_MM_DISP_FAKE_ENG1, "mm_disp_fake_eng1", "disp_sel", 19),
    gate_mm0!(CLK_MM_MDP_TDSHP4, "mm_mdp_tdshp4", "disp_sel", 20),
    gate_mm0!(CLK_MM_MDP_RSZ4, "mm_mdp_rsz4", "disp_sel", 21),
    gate_mm0!(CLK_MM_MDP_AAL4, "mm_mdp_aal4", "disp_sel", 22),
    gate_mm0!(CLK_MM_MDP_HDR4, "mm_mdp_hdr4", "disp_sel", 23),
    gate_mm0!(CLK_MM_MDP_RDMA4, "mm_mdp_rdma4", "disp_sel", 24),
    gate_mm0!(CLK_MM_MDP_COLOR4, "mm_mdp_color4", "disp_sel", 25),
    gate_mm0!(CLK_MM_DISP_Y2R0, "mm_disp_y2r0", "disp_sel", 26),
    gate_mm0!(CLK_MM_SMI_GALS, "mm_smi_gals", "disp_sel", 27),
    gate_mm0!(CLK_MM_DISP_OVL2_2L, "mm_disp_ovl2_2l", "disp_sel", 28),
    gate_mm0!(CLK_MM_DISP_RDMA4, "mm_disp_rdma4", "disp_sel", 29),
    gate_mm0!(CLK_MM_DISP_DPI0, "mm_disp_dpi0", "disp_sel", 30),
    // MM1
    gate_mm1!(CLK_MM_SMI_IOMMU, "mm_smi_iommu", "disp_sel", 0),
    // MM2
    gate_mm2!(CLK_MM_DSI_DSI0, "mm_dsi_dsi0", "disp_sel", 0),
    gate_mm2!(CLK_MM_DPI_DPI0, "mm_dpi_dpi0", "dpi_sel", 8),
    gate_mm2!(CLK_MM_26MHZ, "mm_26mhz", "clk26m", 24),
    gate_mm2!(CLK_MM_32KHZ, "mm_32khz", "clk32k", 25),
];

static MM_DESC: MtkClkDesc = MtkClkDesc {
    clks: MM_CLKS.as_ptr(),
    num_clks: MM_CLKS.len(),
};

static CLK_MT8192_MM_ID_TABLE: [PlatformDeviceId; 2] = [
    PlatformDeviceId {
        name: "clk-mt8192-mm",
        driver_data: &MM_DESC as *const MtkClkDesc as KernelULong,
    },
    PlatformDeviceId { name: "", driver_data: 0 }, // sentinel
];

static mut CLK_MT8192_MM_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: Driver { name: "clk-mt8192-mm" },
    id_table: CLK_MT8192_MM_ID_TABLE.as_ptr(),
};

// MODULE_DEVICE_TABLE(platform, clk_mt8192_mm_id_table);
// module_platform_driver(clk_mt8192_mm_drv);
// MODULE_DESCRIPTION("MediaTek MT8192 MultiMedia clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
