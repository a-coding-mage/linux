// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 MediaTek Inc.
 */

// Dependencies supplied by the kernel clock and platform headers:
// dt-bindings/clock/mediatek,mt8365-clk.h
// linux/clk-provider.h, linux/platform_device.h
// clk-gate.h, clk-mtk.h

static vdec0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x4,
    sta_ofs: 0x0,
};

static vdec1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x8,
    clr_ofs: 0xc,
    sta_ofs: 0x8,
};

// C macros translated as direct wrappers around the externally supplied gate
// constructor and gate-operation objects.
macro_rules! GATE_VDEC0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &vdec0_cg_regs, $shift,
                  &mtk_clk_gate_ops_setclr_inv)
    };
}

macro_rules! GATE_VDEC1 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &vdec1_cg_regs, $shift,
                  &mtk_clk_gate_ops_setclr_inv)
    };
}

static vdec_clks: [mtk_gate; 2] = [
    /* VDEC0 */
    GATE_VDEC0!(CLK_VDEC_VDEC, "vdec_fvdec_ck", "mm_sel", 0),
    /* VDEC1 */
    GATE_VDEC1!(CLK_VDEC_LARB1, "vdec_flarb1_ck", "mm_sel", 0),
];

static vdec_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &vdec_clks,
    num_clks: vdec_clks.len(),
};

static of_match_clk_mt8365_vdec: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8365-vdecsys",
        data: &vdec_desc,
    },
    of_device_id {
        /* sentinel */
        ..Default::default()
    },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt8365_vdec);

static mut clk_mt8365_vdec_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8365-vdec",
        of_match_table: &of_match_clk_mt8365_vdec,
    },
};

module_platform_driver!(clk_mt8365_vdec_drv);

MODULE_DESCRIPTION!("MediaTek MT8365 Video Decoders clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
