// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 MediaTek Inc.
 * Copyright (c) 2020 BayLibre, SAS
 * Author: James Liao <jamesjj.liao@mediatek.com>
 *         Fabien Parent <fparent@baylibre.com>
 */

// Dependencies supplied by the Linux clock-provider, platform-device,
// MediaTek clock, gate, and MT8167 clock-binding interfaces.

static MFG_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! GATE_MFG {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &MFG_CG_REGS, $shift, &mtk_clk_gate_ops_setclr)
    };
}

static MFG_CLKS: [mtk_gate; 4] = [
    GATE_MFG!(CLK_MFG_BAXI, "mfg_baxi", "ahb_infra_sel", 0),
    GATE_MFG!(CLK_MFG_BMEM, "mfg_bmem", "gfmux_emi1x_sel", 1),
    GATE_MFG!(CLK_MFG_BG3D, "mfg_bg3d", "mfg_mm", 2),
    GATE_MFG!(CLK_MFG_B26M, "mfg_b26m", "clk26m_ck", 3),
];

static MFG_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: MFG_CLKS,
    num_clks: MFG_CLKS.len(),
};

static OF_MATCH_CLK_MT8167_MFGCFG: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8167-mfgcfg",
        data: &MFG_DESC,
    },
    of_device_id {
        // sentinel
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8167_mfgcfg);

static mut CLK_MT8167_MFGCFG_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8167-mfgcfg",
        of_match_table: &OF_MATCH_CLK_MT8167_MFGCFG,
    },
};

// module_platform_driver(clk_mt8167_mfgcfg_drv);

// MODULE_DESCRIPTION("MediaTek MT8167 GPU mfg clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
