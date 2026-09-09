// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device,
// MediaTek clock, clock-gate, and MT8192 clock-binding interfaces.

static MSdc_TOP_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

macro_rules! GATE_MSDC_TOP {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &MSdc_TOP_CG_REGS, $shift, &mtk_clk_gate_ops_no_setclr_inv)
    };
}

static MSDC_TOP_CLKS: [MtkGate; 15] = [
    GATE_MSDC_TOP!(CLK_MSDC_TOP_AES_0P, "msdc_top_aes_0p", "aes_msdcfde_sel", 0),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_SRC_0P, "msdc_top_src_0p", "infra_msdc0_src", 1),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_SRC_1P, "msdc_top_src_1p", "infra_msdc1_src", 2),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_SRC_2P, "msdc_top_src_2p", "infra_msdc2_src", 3),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_P_MSDC0, "msdc_top_p_msdc0", "axi_sel", 4),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_P_MSDC1, "msdc_top_p_msdc1", "axi_sel", 5),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_P_MSDC2, "msdc_top_p_msdc2", "axi_sel", 6),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_P_CFG, "msdc_top_p_cfg", "axi_sel", 7),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_AXI, "msdc_top_axi", "axi_sel", 8),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_H_MST_0P, "msdc_top_h_mst_0p", "infra_msdc0", 9),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_H_MST_1P, "msdc_top_h_mst_1p", "infra_msdc1", 10),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_H_MST_2P, "msdc_top_h_mst_2p", "infra_msdc2", 11),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_MEM_OFF_DLY_26M, "msdc_top_mem_off_dly_26m", "clk26m", 12),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_32K, "msdc_top_32k", "clk32k", 13),
    GATE_MSDC_TOP!(CLK_MSDC_TOP_AHB2AXI_BRG_AXI, "msdc_top_ahb2axi_brg_axi", "axi_sel", 14),
];

static MSDC_TOP_DESC: MtkClkDesc = MtkClkDesc {
    clks: &MSDC_TOP_CLKS,
    num_clks: MSDC_TOP_CLKS.len(),
};

static OF_MATCH_CLK_MT8192_MSDC: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: "mediatek,mt8192-msdc_top",
        data: &MSDC_TOP_DESC,
    },
    OfDeviceId { /* sentinel */ },
];

MODULE_DEVICE_TABLE!(of, OF_MATCH_CLK_MT8192_MSDC);

static mut CLK_MT8192_MSDC_DRV: PlatformDriver = PlatformDriver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: Driver {
        name: "clk-mt8192-msdc",
        of_match_table: &OF_MATCH_CLK_MT8192_MSDC,
    },
};

module_platform_driver!(CLK_MT8192_MSDC_DRV);

MODULE_DESCRIPTION!("MediaTek MT8192 MMC/SD Controller clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
