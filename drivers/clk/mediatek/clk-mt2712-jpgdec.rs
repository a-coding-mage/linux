// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Weiyi Lu <weiyi.lu@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel translation.

static jpgdec_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// Equivalent of GATE_JPGDEC / GATE_MTK.  The referenced types, constants,
// and gate operation table are supplied by the surrounding translation.
const fn gate_jpgdec(
    id: u32,
    name: &'static str,
    parent: &'static str,
    shift: u32,
) -> mtk_gate {
    gate_mtk(id, name, parent, &jpgdec_cg_regs, shift, &mtk_clk_gate_ops_setclr_inv)
}

static jpgdec_clks: [mtk_gate; 2] = [
    gate_jpgdec(CLK_JPGDEC_JPGDEC1, "jpgdec_jpgdec1", "jpgdec_sel", 0),
    gate_jpgdec(CLK_JPGDEC_JPGDEC, "jpgdec_jpgdec", "jpgdec_sel", 4),
];

static jpgdec_desc: mtk_clk_desc = mtk_clk_desc {
    clks: jpgdec_clks.as_ptr(),
    num_clks: jpgdec_clks.len(),
};

static of_match_clk_mt2712_jpgdec: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt2712-jpgdecsys",
        data: &jpgdec_desc,
    },
    of_device_id {
        // sentinel
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt2712_jpgdec);

static mut clk_mt2712_jpgdec_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt2712-jpgdec",
        of_match_table: of_match_clk_mt2712_jpgdec.as_ptr(),
    },
};

// module_platform_driver(clk_mt2712_jpgdec_drv);

// MODULE_DESCRIPTION("MediaTek MT2712 JPEG Decoder clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
