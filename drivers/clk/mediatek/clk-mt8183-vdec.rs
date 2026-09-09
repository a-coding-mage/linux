// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

// Dependencies supplied by the surrounding kernel translation.

static VDEC0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x4,
    sta_ofs: 0x0,
};

static VDEC1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x8,
    clr_ofs: 0xc,
    sta_ofs: 0x8,
};

macro_rules! gate_vdec0_i {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VDEC0_CG_REGS, $shift,
            &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! gate_vdec1_i {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &VDEC1_CG_REGS, $shift,
            &mtk_clk_gate_ops_setclr_inv)
    };
}

static VDEC_CLKS: [mtk_gate; 2] = [
    // VDEC0
    gate_vdec0_i!(CLK_VDEC_VDEC, "vdec_vdec", "mm_sel", 0),
    // VDEC1
    gate_vdec1_i!(CLK_VDEC_LARB1, "vdec_larb1", "mm_sel", 0),
];

static VDEC_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: VDEC_CLKS.as_ptr(),
    num_clks: VDEC_CLKS.len(),
};

static OF_MATCH_CLK_MT8183_VDEC: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8183-vdecsys",
        data: &VDEC_DESC as *const mtk_clk_desc as *const core::ffi::c_void,
    },
    of_device_id {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8183_vdec);

static mut CLK_MT8183_VDEC_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8183-vdec",
        of_match_table: OF_MATCH_CLK_MT8183_VDEC.as_ptr(),
    },
};

// module_platform_driver(clk_mt8183_vdec_drv);
// MODULE_DESCRIPTION("MediaTek MT8183 Video Decoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
