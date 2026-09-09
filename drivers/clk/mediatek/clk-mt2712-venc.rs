// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Weiyi Lu <weiyi.lu@mediatek.com>
 */

// Dependencies supplied by the Linux clock-provider, platform-device, and
// MediaTek clock headers are intentionally referenced but not implemented here.

static venc_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// Equivalent of GATE_VENC(_id, _name, _parent, _shift), using the
// GATE_MTK declaration supplied by clk-mtk.h/clk-gate.h.
macro_rules! gate_venc {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &venc_cg_regs, $shift,
                  &mtk_clk_gate_ops_setclr_inv)
    };
}

static venc_clks: [mtk_gate; 3] = [
    gate_venc!(CLK_VENC_SMI_COMMON_CON, "venc_smi", "mm_sel", 0),
    gate_venc!(CLK_VENC_VENC, "venc_venc", "venc_sel", 4),
    gate_venc!(CLK_VENC_SMI_LARB6, "venc_smi_larb6", "jpgdec_sel", 12),
];

static venc_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &venc_clks,
    num_clks: venc_clks.len(),
};

static of_match_clk_mt2712_venc: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt2712-vencsys",
        data: &venc_desc,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt2712_venc);

static mut clk_mt2712_venc_drv: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt2712-venc",
        of_match_table: &of_match_clk_mt2712_venc,
    },
};

module_platform_driver!(clk_mt2712_venc_drv);

MODULE_DESCRIPTION!("MediaTek MT2712 Video Encoders clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
