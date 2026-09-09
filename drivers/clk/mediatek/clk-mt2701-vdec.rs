// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: Shunli Wang <shunli.wang@mediatek.com>
 */

// Dependencies supplied by the kernel clock-provider, platform-device,
// MediaTek clock, gate, and MT2701 clock-binding interfaces.
use crate::{
    mtk_clk_desc, mtk_clk_gate_ops_setclr_inv, mtk_clk_simple_probe,
    mtk_clk_simple_remove, mtk_gate, mtk_gate_regs, of_device_id,
    platform_driver, ARRAY_SIZE, CLK_DUMMY, CLK_VDEC_CKGEN, CLK_VDEC_LARB,
    GATE_DUMMY, GATE_MTK,
};

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

// C macros GATE_VDEC0 and GATE_VDEC1 expand to GATE_MTK with the
// corresponding register block and inverted set/clear gate operations.
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

static VDEC_CLKS: [mtk_gate; 3] = [
    GATE_DUMMY!(CLK_DUMMY, "vdec_dummy"),
    GATE_VDEC0!(CLK_VDEC_CKGEN, "vdec_cken", "vdec_sel", 0),
    GATE_VDEC1!(CLK_VDEC_LARB, "vdec_larb_cken", "mm_sel", 0),
];

static VDEC_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: VDEC_CLKS.as_ptr(),
    num_clks: ARRAY_SIZE!(VDEC_CLKS),
};

static OF_MATCH_CLK_MT2701_VDEC: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt2701-vdecsys",
        data: &VDEC_DESC,
    },
    of_device_id {
        // sentinel
        ..Default::default()
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt2701_vdec);

static mut CLK_MT2701_VDEC_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: crate::device_driver {
        name: "clk-mt2701-vdec",
        of_match_table: OF_MATCH_CLK_MT2701_VDEC.as_ptr(),
    },
};

// module_platform_driver(clk_mt2701_vdec_drv);
// MODULE_DESCRIPTION("MediaTek MT2701 Video Decoders clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
