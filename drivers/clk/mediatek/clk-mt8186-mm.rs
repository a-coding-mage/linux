// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device, clock
// binding, clk-gate, and clk-mtk headers are intentionally left external.

static MM0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static MM1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x1a4,
    clr_ofs: 0x1a8,
    sta_ofs: 0x1a0,
};

macro_rules! GATE_MM0 {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &MM0_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

macro_rules! GATE_MM1 {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &MM1_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static MM_CLKS: [mtk_gate; 26] = [
    // MM0
    GATE_MM0!(CLK_MM_DISP_MUTEX0, "mm_disp_mutex0", "top_disp", 0),
    GATE_MM0!(CLK_MM_APB_MM_BUS, "mm_apb_mm_bus", "top_disp", 1),
    GATE_MM0!(CLK_MM_DISP_OVL0, "mm_disp_ovl0", "top_disp", 2),
    GATE_MM0!(CLK_MM_DISP_RDMA0, "mm_disp_rdma0", "top_disp", 3),
    GATE_MM0!(CLK_MM_DISP_OVL0_2L, "mm_disp_ovl0_2l", "top_disp", 4),
    GATE_MM0!(CLK_MM_DISP_WDMA0, "mm_disp_wdma0", "top_disp", 5),
    GATE_MM0!(CLK_MM_DISP_RSZ0, "mm_disp_rsz0", "top_disp", 7),
    GATE_MM0!(CLK_MM_DISP_AAL0, "mm_disp_aal0", "top_disp", 8),
    GATE_MM0!(CLK_MM_DISP_CCORR0, "mm_disp_ccorr0", "top_disp", 9),
    GATE_MM0!(CLK_MM_DISP_COLOR0, "mm_disp_color0", "top_disp", 10),
    GATE_MM0!(CLK_MM_SMI_INFRA, "mm_smi_infra", "top_disp", 11),
    GATE_MM0!(CLK_MM_DISP_DSC_WRAP0, "mm_disp_dsc_wrap0", "top_disp", 12),
    GATE_MM0!(CLK_MM_DISP_GAMMA0, "mm_disp_gamma0", "top_disp", 13),
    GATE_MM0!(CLK_MM_DISP_POSTMASK0, "mm_disp_postmask0", "top_disp", 14),
    GATE_MM0!(CLK_MM_DISP_DITHER0, "mm_disp_dither0", "top_disp", 16),
    GATE_MM0!(CLK_MM_SMI_COMMON, "mm_smi_common", "top_disp", 17),
    GATE_MM0!(CLK_MM_DSI0, "mm_dsi0", "top_disp", 19),
    GATE_MM0!(CLK_MM_DISP_FAKE_ENG0, "mm_disp_fake_eng0", "top_disp", 20),
    GATE_MM0!(CLK_MM_DISP_FAKE_ENG1, "mm_disp_fake_eng1", "top_disp", 21),
    GATE_MM0!(CLK_MM_SMI_GALS, "mm_smi_gals", "top_disp", 22),
    GATE_MM0!(CLK_MM_SMI_IOMMU, "mm_smi_iommu", "top_disp", 24),
    GATE_MM0!(CLK_MM_DISP_RDMA1, "mm_disp_rdma1", "top_disp", 25),
    GATE_MM0!(CLK_MM_DISP_DPI, "mm_disp_dpi", "top_disp", 26),
    // MM1
    GATE_MM1!(CLK_MM_DSI0_DSI_CK_DOMAIN, "mm_dsi0_dsi_domain", "top_disp", 0),
    GATE_MM1!(CLK_MM_DISP_26M, "mm_disp_26m_ck", "top_disp", 10),
];

static MM_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: MM_CLKS.as_ptr(),
    num_clks: MM_CLKS.len(),
};

static CLK_MT8186_MM_ID_TABLE: [platform_device_id; 2] = [
    platform_device_id {
        name: "clk-mt8186-mm",
        driver_data: (&MM_DESC as *const mtk_clk_desc) as kernel_ulong_t,
    },
    platform_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(platform, CLK_MT8186_MM_ID_TABLE);

static mut CLK_MT8186_MM_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: driver {
        name: "clk-mt8186-mm",
    },
    id_table: CLK_MT8186_MM_ID_TABLE.as_ptr(),
};

module_platform_driver!(CLK_MT8186_MM_DRV);

MODULE_DESCRIPTION!("MediaTek MT8186 MultiMedia clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
