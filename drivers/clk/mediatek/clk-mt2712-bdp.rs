// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Weiyi Lu <weiyi.lu@mediatek.com>
 */

// Translated from the Linux clock-provider, platform-device, MediaTek clock,
// clock-gate, and MT2712 clock binding headers.

static BDP_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x100,
    clr_ofs: 0x100,
    sta_ofs: 0x100,
};

macro_rules! gate_bdp {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &BDP_CG_REGS, $shift, &mtk_clk_gate_ops_no_setclr)
    };
}

static BDP_CLKS: [mtk_gate; 26] = [
    gate_bdp!(CLK_BDP_BRIDGE_B, "bdp_bridge_b", "mm_sel", 0),
    gate_bdp!(CLK_BDP_BRIDGE_DRAM, "bdp_bridge_d", "mm_sel", 1),
    gate_bdp!(CLK_BDP_LARB_DRAM, "bdp_larb_d", "mm_sel", 2),
    gate_bdp!(CLK_BDP_WR_CHANNEL_VDI_PXL, "bdp_vdi_pxl", "tvd_sel", 3),
    gate_bdp!(CLK_BDP_WR_CHANNEL_VDI_DRAM, "bdp_vdi_d", "mm_sel", 4),
    gate_bdp!(CLK_BDP_WR_CHANNEL_VDI_B, "bdp_vdi_b", "mm_sel", 5),
    gate_bdp!(CLK_BDP_MT_B, "bdp_fmt_b", "mm_sel", 9),
    gate_bdp!(CLK_BDP_DISPFMT_27M, "bdp_27m", "di_sel", 10),
    gate_bdp!(CLK_BDP_DISPFMT_27M_VDOUT, "bdp_27m_vdout", "di_sel", 11),
    gate_bdp!(CLK_BDP_DISPFMT_27_74_74, "bdp_27_74_74", "di_sel", 12),
    gate_bdp!(CLK_BDP_DISPFMT_2FS, "bdp_2fs", "di_sel", 13),
    gate_bdp!(CLK_BDP_DISPFMT_2FS_2FS74_148, "bdp_2fs74_148", "di_sel", 14),
    gate_bdp!(CLK_BDP_DISPFMT_B, "bdp_b", "mm_sel", 15),
    gate_bdp!(CLK_BDP_VDO_DRAM, "bdp_vdo_d", "mm_sel", 16),
    gate_bdp!(CLK_BDP_VDO_2FS, "bdp_vdo_2fs", "di_sel", 17),
    gate_bdp!(CLK_BDP_VDO_B, "bdp_vdo_b", "mm_sel", 18),
    gate_bdp!(CLK_BDP_WR_CHANNEL_DI_PXL, "bdp_di_pxl", "di_sel", 19),
    gate_bdp!(CLK_BDP_WR_CHANNEL_DI_DRAM, "bdp_di_d", "mm_sel", 20),
    gate_bdp!(CLK_BDP_WR_CHANNEL_DI_B, "bdp_di_b", "mm_sel", 21),
    gate_bdp!(CLK_BDP_NR_AGENT, "bdp_nr_agent", "nr_sel", 22),
    gate_bdp!(CLK_BDP_NR_DRAM, "bdp_nr_d", "mm_sel", 23),
    gate_bdp!(CLK_BDP_NR_B, "bdp_nr_b", "mm_sel", 24),
    gate_bdp!(CLK_BDP_BRIDGE_RT_B, "bdp_bridge_rt_b", "mm_sel", 25),
    gate_bdp!(CLK_BDP_BRIDGE_RT_DRAM, "bdp_bridge_rt_d", "mm_sel", 26),
    gate_bdp!(CLK_BDP_LARB_RT_DRAM, "bdp_larb_rt_d", "mm_sel", 27),
    gate_bdp!(CLK_BDP_TVD_TDC, "bdp_tvd_tdc", "mm_sel", 28),
    gate_bdp!(CLK_BDP_TVD_54, "bdp_tvd_clk_54", "tvd_sel", 29),
    gate_bdp!(CLK_BDP_TVD_CBUS, "bdp_tvd_cbus", "mm_sel", 30),
];

static BDP_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: BDP_CLKS.as_ptr(),
    num_clks: BDP_CLKS.len(),
};

static OF_MATCH_CLK_MT2712_BDP: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt2712-bdpsys",
        data: &BDP_DESC,
    },
    of_device_id {
        // sentinel
    },
];

static mut CLK_MT2712_BDP_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt2712-bdp",
        of_match_table: OF_MATCH_CLK_MT2712_BDP.as_ptr(),
    },
};

module_platform_driver!(CLK_MT2712_BDP_DRV);

module_description!("MediaTek MT2712 BDP clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
