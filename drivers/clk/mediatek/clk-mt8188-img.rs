// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Dependencies supplied by the corresponding clock-provider and platform modules.

static IMGSYS_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! gate_imgsys {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &IMGSYS_CG_REGS, $shift, &MTK_CLK_GATE_OPS_SETCLR)
    };
}

const IMG_SYS_SMI_LARB_RST_OFF: u16 = 0xC;

static IMGSYS_MAIN_CLKS: [MtkGate; 10] = [
    gate_imgsys!(CLK_IMGSYS_MAIN_LARB9, "imgsys_main_larb9", "top_img", 0),
    gate_imgsys!(CLK_IMGSYS_MAIN_TRAW0, "imgsys_main_traw0", "top_img", 1),
    gate_imgsys!(CLK_IMGSYS_MAIN_TRAW1, "imgsys_main_traw1", "top_img", 2),
    gate_imgsys!(CLK_IMGSYS_MAIN_VCORE_GALS, "imgsys_main_vcore_gals", "top_img", 3),
    gate_imgsys!(CLK_IMGSYS_MAIN_DIP0, "imgsys_main_dip0", "top_img", 8),
    gate_imgsys!(CLK_IMGSYS_MAIN_WPE0, "imgsys_main_wpe0", "top_img", 9),
    gate_imgsys!(CLK_IMGSYS_MAIN_IPE, "imgsys_main_ipe", "top_img", 10),
    gate_imgsys!(CLK_IMGSYS_MAIN_WPE1, "imgsys_main_wpe1", "top_img", 12),
    gate_imgsys!(CLK_IMGSYS_MAIN_WPE2, "imgsys_main_wpe2", "top_img", 13),
    gate_imgsys!(CLK_IMGSYS_MAIN_GALS, "imgsys_main_gals", "top_img", 31),
];

static IMGSYS_WPE1_CLKS: [MtkGate; 2] = [
    gate_imgsys!(CLK_IMGSYS_WPE1_LARB11, "imgsys_wpe1_larb11", "top_img", 0),
    gate_imgsys!(CLK_IMGSYS_WPE1, "imgsys_wpe1", "top_img", 1),
];

static IMGSYS_WPE2_CLKS: [MtkGate; 2] = [
    gate_imgsys!(CLK_IMGSYS_WPE2_LARB11, "imgsys_wpe2_larb11", "top_img", 0),
    gate_imgsys!(CLK_IMGSYS_WPE2, "imgsys_wpe2", "top_img", 1),
];

static IMGSYS_WPE3_CLKS: [MtkGate; 2] = [
    gate_imgsys!(CLK_IMGSYS_WPE3_LARB11, "imgsys_wpe3_larb11", "top_img", 0),
    gate_imgsys!(CLK_IMGSYS_WPE3, "imgsys_wpe3", "top_img", 1),
];

static IMGSYS1_DIP_TOP_CLKS: [MtkGate; 2] = [
    gate_imgsys!(CLK_IMGSYS1_DIP_TOP_LARB10, "imgsys1_dip_larb10", "top_img", 0),
    gate_imgsys!(CLK_IMGSYS1_DIP_TOP_DIP_TOP, "imgsys1_dip_dip_top", "top_img", 1),
];

static IMGSYS1_DIP_NR_CLKS: [MtkGate; 2] = [
    gate_imgsys!(CLK_IMGSYS1_DIP_NR_LARB15, "imgsys1_dip_nr_larb15", "top_img", 0),
    gate_imgsys!(CLK_IMGSYS1_DIP_NR_DIP_NR, "imgsys1_dip_nr_dip_nr", "top_img", 1),
];

/* Reset for SMI larb 10/11a/11b/11c/15 */
static IMG_SYS_RST_OFS: [u16; 1] = [IMG_SYS_SMI_LARB_RST_OFF];

static IMG_SYS_RST_DESC: MtkClkRstDesc = MtkClkRstDesc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: IMG_SYS_RST_OFS,
    rst_bank_nr: IMG_SYS_RST_OFS.len(),
};

static IMGSYS_MAIN_DESC: MtkClkDesc = MtkClkDesc {
    clks: IMGSYS_MAIN_CLKS,
    num_clks: IMGSYS_MAIN_CLKS.len(),
};

static IMGSYS_WPE1_DESC: MtkClkDesc = MtkClkDesc {
    clks: IMGSYS_WPE1_CLKS,
    num_clks: IMGSYS_WPE1_CLKS.len(),
    rst_desc: Some(&IMG_SYS_RST_DESC),
};

static IMGSYS_WPE2_DESC: MtkClkDesc = MtkClkDesc {
    clks: IMGSYS_WPE2_CLKS,
    num_clks: IMGSYS_WPE2_CLKS.len(),
    rst_desc: Some(&IMG_SYS_RST_DESC),
};

static IMGSYS_WPE3_DESC: MtkClkDesc = MtkClkDesc {
    clks: IMGSYS_WPE3_CLKS,
    num_clks: IMGSYS_WPE3_CLKS.len(),
    rst_desc: Some(&IMG_SYS_RST_DESC),
};

static IMGSYS1_DIP_TOP_DESC: MtkClkDesc = MtkClkDesc {
    clks: IMGSYS1_DIP_TOP_CLKS,
    num_clks: IMGSYS1_DIP_TOP_CLKS.len(),
    rst_desc: Some(&IMG_SYS_RST_DESC),
};

static IMGSYS1_DIP_NR_DESC: MtkClkDesc = MtkClkDesc {
    clks: IMGSYS1_DIP_NR_CLKS,
    num_clks: IMGSYS1_DIP_NR_CLKS.len(),
    rst_desc: Some(&IMG_SYS_RST_DESC),
};

static OF_MATCH_CLK_MT8188_IMGSYS_MAIN: [OfDeviceId; 7] = [
    OfDeviceId { compatible: "mediatek,mt8188-imgsys", data: &IMGSYS_MAIN_DESC },
    OfDeviceId { compatible: "mediatek,mt8188-imgsys-wpe1", data: &IMGSYS_WPE1_DESC },
    OfDeviceId { compatible: "mediatek,mt8188-imgsys-wpe2", data: &IMGSYS_WPE2_DESC },
    OfDeviceId { compatible: "mediatek,mt8188-imgsys-wpe3", data: &IMGSYS_WPE3_DESC },
    OfDeviceId { compatible: "mediatek,mt8188-imgsys1-dip-top", data: &IMGSYS1_DIP_TOP_DESC },
    OfDeviceId { compatible: "mediatek,mt8188-imgsys1-dip-nr", data: &IMGSYS1_DIP_NR_DESC },
    OfDeviceId::SENTINEL,
];

static mut CLK_MT8188_IMGSYS_MAIN_DRV: PlatformDriver = PlatformDriver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: Driver {
        name: "clk-mt8188-imgsys_main",
        of_match_table: &OF_MATCH_CLK_MT8188_IMGSYS_MAIN,
    },
};

module_device_table_of!(OF_MATCH_CLK_MT8188_IMGSYS_MAIN);
module_platform_driver!(CLK_MT8188_IMGSYS_MAIN_DRV);

module_description!("MediaTek MT8188 imgsys clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
