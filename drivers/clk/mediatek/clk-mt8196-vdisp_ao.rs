// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */

// Dependencies supplied by the kernel clock framework and the corresponding
// MediaTek clock bindings are intentionally left as external Rust items.

static MM_V_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static MM_V_HWV_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0030,
    clr_ofs: 0x0034,
    sta_ofs: 0x2c18,
};

// GATE_MM_AO_V(_id, _name, _parent, _shift)
// GATE_HWV_MM_V(_id, _name, _parent, _shift)

static MM_V_CLKS: [mtk_gate; 3] = [
    mtk_gate {
        id: CLK_MM_V_DISP_VDISP_AO_CONFIG,
        name: "mm_v_disp_vdisp_ao_config",
        parent_name: "disp",
        regs: &MM_V_CG_REGS,
        hwv_regs: &MM_V_HWV_REGS,
        shift: 0,
        ops: &mtk_clk_gate_hwv_ops_setclr,
        flags: CLK_OPS_PARENT_ENABLE,
    },
    mtk_gate {
        id: CLK_MM_V_DISP_DPC,
        name: "mm_v_disp_dpc",
        parent_name: "disp",
        regs: &MM_V_CG_REGS,
        hwv_regs: &MM_V_HWV_REGS,
        shift: 16,
        ops: &mtk_clk_gate_hwv_ops_setclr,
        flags: CLK_OPS_PARENT_ENABLE,
    },
    mtk_gate {
        id: CLK_MM_V_SMI_SUB_SOMM0,
        name: "mm_v_smi_sub_somm0",
        parent_name: "disp",
        regs: &MM_V_CG_REGS,
        hwv_regs: core::ptr::null(),
        shift: 2,
        ops: &mtk_clk_gate_ops_setclr,
        flags: CLK_OPS_PARENT_ENABLE | CLK_IS_CRITICAL,
    },
];

static MM_V_MCD: mtk_clk_desc = mtk_clk_desc {
    clks: &MM_V_CLKS,
    num_clks: MM_V_CLKS.len(),
};

static OF_MATCH_CLK_MT8196_VDISP_AO: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8196-vdisp-ao",
        data: &MM_V_MCD,
    },
    of_device_id {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8196_vdisp_ao);

static mut CLK_MT8196_VDISP_AO_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: device_driver {
        name: "clk-mt8196-vdisp-ao",
        of_match_table: &OF_MATCH_CLK_MT8196_VDISP_AO,
    },
};

// module_platform_driver(clk_mt8196_vdisp_ao_drv);
// MODULE_DESCRIPTION("MediaTek MT8196 vdisp_ao clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
