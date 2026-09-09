// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Yassine Oudjana <y.oudjana@protonmail.com>
 */

// Dependencies supplied by the Linux clock-provider, platform-device,
// clk-gate, clk-mtk, and MediaTek MT6735 clock-binding interfaces.

const IMG_CG_CON: u32 = 0x00;
const IMG_CG_SET: u32 = 0x04;
const IMG_CG_CLR: u32 = 0x08;

static mut imgsys_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: IMG_CG_SET,
    clr_ofs: IMG_CG_CLR,
    sta_ofs: IMG_CG_CON,
};

static imgsys_gates: [mtk_gate; 8] = [
    GATE_MTK!(CLK_IMG_SMI_LARB2, "smi_larb2", "mm_sel", &imgsys_cg_regs, 0, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IMG_CAM_SMI, "cam_smi", "mm_sel", &imgsys_cg_regs, 5, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IMG_CAM_CAM, "cam_cam", "mm_sel", &imgsys_cg_regs, 6, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IMG_SEN_TG, "sen_tg", "mm_sel", &imgsys_cg_regs, 7, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IMG_SEN_CAM, "sen_cam", "mm_sel", &imgsys_cg_regs, 8, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IMG_CAM_SV, "cam_sv", "mm_sel", &imgsys_cg_regs, 9, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IMG_SUFOD, "sufod", "mm_sel", &imgsys_cg_regs, 10, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IMG_FD, "fd", "mm_sel", &imgsys_cg_regs, 11, &mtk_clk_gate_ops_setclr),
];

static imgsys_clks: mtk_clk_desc = mtk_clk_desc {
    clks: &imgsys_gates,
    num_clks: imgsys_gates.len(),
};

static of_match_mt6735_imgsys: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt6735-imgsys",
        data: &imgsys_clks,
    },
    of_device_id {
        /* sentinel */
        ..of_device_id::default()
    },
];

static mut clk_mt6735_imgsys: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: "clk-mt6735-imgsys",
        of_match_table: &of_match_mt6735_imgsys,
    },
};

module_platform_driver!(clk_mt6735_imgsys);

module_author!("Yassine Oudjana <y.oudjana@protonmail.com>");
module_description!("MediaTek MT6735 imgsys clock driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
