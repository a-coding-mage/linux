// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Kevin-CW Chen <kevin-cw.chen@mediatek.com>
 */

// C dependencies supplied by the surrounding kernel crate:
// linux/clk-provider.h, linux/platform_device.h, clk-mtk.h, clk-gate.h,
// and dt-bindings/clock/mt6797-clk.h.

static VDEC0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0000,
    clr_ofs: 0x0004,
    sta_ofs: 0x0000,
};

static VDEC1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0008,
    clr_ofs: 0x000c,
    sta_ofs: 0x0008,
};

macro_rules! GATE_VDEC0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VDEC0_CG_REGS, $shift,
                  &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! GATE_VDEC1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VDEC1_CG_REGS, $shift,
                  &mtk_clk_gate_ops_setclr_inv)
    };
}

static VDEC_CLKS: [mtk_gate; 4] = [
    GATE_VDEC0!(CLK_VDEC_CKEN_ENG, "vdec_cken_eng", "vdec_sel", 8),
    GATE_VDEC0!(CLK_VDEC_ACTIVE, "vdec_active", "vdec_sel", 4),
    GATE_VDEC0!(CLK_VDEC_CKEN, "vdec_cken", "vdec_sel", 0),
    GATE_VDEC1!(CLK_VDEC_LARB1_CKEN, "vdec_larb1_cken", "mm_sel", 0),
];

static VDEC_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: VDEC_CLKS.as_ptr(),
    num_clks: VDEC_CLKS.len(),
};

static OF_MATCH_CLK_MT6797_VDEC: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt6797-vdecsys",
        data: &VDEC_DESC,
    },
    of_device_id {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt6797_vdec);

static mut CLK_MT6797_VDEC_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt6797-vdec",
        of_match_table: OF_MATCH_CLK_MT6797_VDEC.as_ptr(),
    },
};

// module_platform_driver(clk_mt6797_vdec_drv);

// MODULE_DESCRIPTION("MediaTek MT6797 Video Decoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
