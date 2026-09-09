// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Dependencies supplied by the Linux clock-provider, platform-device, and
// MediaTek clock headers are intentionally left as external Rust items.

static IPE_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// GATE_IPE(_id, _name, _parent, _shift) expands to:
// GATE_MTK(_id, _name, _parent, &ipe_cg_regs, _shift, &mtk_clk_gate_ops_setclr)

const IPE_SYS_SMI_LARB_RST_OFF: u32 = 0xC;

static IPE_CLKS: [mtk_gate; 5] = [
    GATE_MTK!(CLK_IPE_DPE, "ipe_dpe", "top_ipe", &IPE_CG_REGS, 0, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IPE_FDVT, "ipe_fdvt", "top_ipe", &IPE_CG_REGS, 1, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IPE_ME, "ipe_me", "top_ipe", &IPE_CG_REGS, 2, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IPESYS_TOP, "ipesys_top", "top_ipe", &IPE_CG_REGS, 3, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_IPE_SMI_LARB12, "ipe_smi_larb12", "top_ipe", &IPE_CG_REGS, 4, &mtk_clk_gate_ops_setclr),
];

/* Reset for SMI larb 12 */
static mut IPE_SYS_RST_OFS: [u16; 1] = [IPE_SYS_SMI_LARB_RST_OFF as u16];

static IPE_SYS_RST_DESC: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: unsafe { &IPE_SYS_RST_OFS },
    rst_bank_nr: IPE_SYS_RST_OFS.len(),
};

static IPE_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: unsafe { &IPE_CLKS },
    num_clks: IPE_CLKS.len(),
    rst_desc: &IPE_SYS_RST_DESC,
};

static OF_MATCH_CLK_MT8188_IPE: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8188-ipesys",
        data: &IPE_DESC,
    },
    of_device_id {
        // sentinel
    },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT8188_IPE);

static mut CLK_MT8188_IPE_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8188-ipe",
        of_match_table: &OF_MATCH_CLK_MT8188_IPE,
    },
};

module_platform_driver!(CLK_MT8188_IPE_DRV);

MODULE_DESCRIPTION!("MediaTek MT8188 Image Processing Engine clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
