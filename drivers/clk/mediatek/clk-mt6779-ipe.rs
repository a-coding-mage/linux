// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 MediaTek Inc.
 * Author: Wendell Lin <wendell.lin@mediatek.com>
 */

// Translated dependencies:
// linux/module.h, linux/clk-provider.h, linux/platform_device.h,
// dt-bindings/clock/mt6779-clk.h, clk-mtk.h, and clk-gate.h

static ipe_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0004,
    clr_ofs: 0x0008,
    sta_ofs: 0x0000,
};

static ipe_clks: [mtk_gate; 7] = [
    GATE_MTK!(CLK_IPE_LARB7, "ipe_larb7", "ipe_sel", &ipe_cg_regs, 0,
        &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IPE_LARB8, "ipe_larb8", "ipe_sel", &ipe_cg_regs, 1,
        &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IPE_SMI_SUBCOM, "ipe_smi_subcom", "ipe_sel", &ipe_cg_regs, 2,
        &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IPE_FD, "ipe_fd", "ipe_sel", &ipe_cg_regs, 3,
        &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IPE_FE, "ipe_fe", "ipe_sel", &ipe_cg_regs, 4,
        &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IPE_RSC, "ipe_rsc", "ipe_sel", &ipe_cg_regs, 5,
        &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IPE_DPE, "ipe_dpe", "ipe_sel", &ipe_cg_regs, 6,
        &mtk_clk_gate_ops_setclr),
];

static ipe_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &ipe_clks,
    num_clks: ipe_clks.len(),
};

static of_match_clk_mt6779_ipe: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt6779-ipesys",
        data: &ipe_desc,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt6779_ipe);

static mut clk_mt6779_ipe_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt6779-ipe",
        of_match_table: &of_match_clk_mt6779_ipe,
    },
};

module_platform_driver!(clk_mt6779_ipe_drv);

MODULE_DESCRIPTION!("MediaTek MT6779 Image Processing Engine clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
