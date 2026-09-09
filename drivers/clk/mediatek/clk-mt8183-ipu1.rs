// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device, MediaTek
// clock, gate, and MT8183 clock-binding interfaces are intentionally external.

static ipu_core1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// Equivalent of GATE_IPU_CORE1(_id, _name, _parent, _shift), using the
// MediaTek set/clear gate operations and the local register block.
static ipu_core1_clks: [mtk_gate; 3] = [
    mtk_gate {
        id: CLK_IPU_CORE1_JTAG,
        name: "ipu_core1_jtag",
        parent_name: "dsp_sel",
        regs: &ipu_core1_cg_regs,
        shift: 0,
        ops: &mtk_clk_gate_ops_setclr,
    },
    mtk_gate {
        id: CLK_IPU_CORE1_AXI,
        name: "ipu_core1_axi",
        parent_name: "dsp_sel",
        regs: &ipu_core1_cg_regs,
        shift: 1,
        ops: &mtk_clk_gate_ops_setclr,
    },
    mtk_gate {
        id: CLK_IPU_CORE1_IPU,
        name: "ipu_core1_ipu",
        parent_name: "dsp_sel",
        regs: &ipu_core1_cg_regs,
        shift: 2,
        ops: &mtk_clk_gate_ops_setclr,
    },
];

static ipu_core1_desc: mtk_clk_desc = mtk_clk_desc {
    clks: ipu_core1_clks.as_ptr(),
    num_clks: ipu_core1_clks.len(),
};

static of_match_clk_mt8183_ipu_core1: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8183-ipu_core1",
        data: &ipu_core1_desc,
    },
    of_device_id {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8183_ipu_core1);

static mut clk_mt8183_ipu_core1_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8183-ipu_core1",
        of_match_table: of_match_clk_mt8183_ipu_core1.as_ptr(),
    },
};

// module_platform_driver(clk_mt8183_ipu_core1_drv);

// MODULE_DESCRIPTION("MediaTek MT8183 Sec. Image Processing Unit clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
