// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the kernel clock framework and MT8173 clock bindings.

macro_rules! GATE_VDEC {
    ($id:expr, $name:expr, $parent:expr, $regs:expr) => {
        GATE_MTK!($id, $name, $parent, $regs, 0, &mtk_clk_gate_ops_setclr_inv)
    };
}

static vdec0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0000,
    clr_ofs: 0x0004,
    sta_ofs: 0x0000,
};

static vdec1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0008,
    clr_ofs: 0x000c,
    sta_ofs: 0x0008,
};

static vdec_clks: [mtk_gate; 3] = [
    GATE_DUMMY!(CLK_DUMMY, "vdec_dummy"),
    GATE_VDEC!(CLK_VDEC_CKEN, "vdec_cken", "vdec_sel", &vdec0_cg_regs),
    GATE_VDEC!(CLK_VDEC_LARB_CKEN, "vdec_larb_cken", "mm_sel", &vdec1_cg_regs),
];

static vdec_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &vdec_clks,
    num_clks: vdec_clks.len(),
};

static of_match_clk_mt8173_vdecsys: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8173-vdecsys",
        data: &vdec_desc,
    },
    of_device_id {
        // sentinel
        ..Default::default()
    },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt8173_vdecsys);

static mut clk_mt8173_vdecsys_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8173-vdecsys",
        of_match_table: &of_match_clk_mt8173_vdecsys,
        ..Default::default()
    },
};

module_platform_driver!(clk_mt8173_vdecsys_drv);

MODULE_DESCRIPTION!("MediaTek MT8173 vdecsys clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
