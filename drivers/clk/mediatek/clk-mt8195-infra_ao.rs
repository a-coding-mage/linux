// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>
// Translated from clk-mt8195-infra_ao.c. External kernel types and symbols are
// intentionally referenced as dependencies supplied by the surrounding tree.

static INFRA_AO0_CG_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x80, clr_ofs: 0x84, sta_ofs: 0x90 };
static INFRA_AO1_CG_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x88, clr_ofs: 0x8c, sta_ofs: 0x94 };
static INFRA_AO2_CG_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0xa4, clr_ofs: 0xa8, sta_ofs: 0xac };
static INFRA_AO3_CG_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0xc0, clr_ofs: 0xc4, sta_ofs: 0xc8 };
static INFRA_AO4_CG_REGS: mtk_gate_regs = mtk_gate_regs { set_ofs: 0xe0, clr_ofs: 0xe4, sta_ofs: 0xe8 };

macro_rules! gate_infra_ao {
    ($n:ident, $regs:ident, $id:ident, $name:literal, $parent:literal, $shift:expr) => {
        GATE_MTK_FLAGS!($id, $name, $parent, &$regs, $shift, &mtk_clk_gate_ops_setclr, 0)
    };
    ($n:ident, $regs:ident, $id:ident, $name:literal, $parent:literal, $shift:expr, $flag:ident) => {
        GATE_MTK_FLAGS!($id, $name, $parent, &$regs, $shift, &mtk_clk_gate_ops_setclr, $flag)
    };
}

static INFRA_AO_CLKS: &[mtk_gate] = &[
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_PMIC_TMR, "infra_ao_pmic_tmr", "top_pwrap_ulposc", 0),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_PMIC_AP, "infra_ao_pmic_ap", "top_pwrap_ulposc", 1),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_PMIC_MD, "infra_ao_pmic_md", "top_pwrap_ulposc", 2),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_PMIC_CONN, "infra_ao_pmic_conn", "top_pwrap_ulposc", 3),
    // infra_ao_sej is main clock is for secure engine with JTAG support
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_SEJ, "infra_ao_sej", "top_axi", 5, CLK_IS_CRITICAL),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_APXGPT, "infra_ao_apxgpt", "top_axi", 6),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_GCE, "infra_ao_gce", "top_axi", 8),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_GCE2, "infra_ao_gce2", "top_axi", 9),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_THERM, "infra_ao_therm", "top_axi", 10),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_PWM_H, "infra_ao_pwm_h", "top_axi", 15),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_PWM1, "infra_ao_pwm1", "top_pwm", 16),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_PWM2, "infra_ao_pwm2", "top_pwm", 17),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_PWM3, "infra_ao_pwm3", "top_pwm", 18),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_PWM4, "infra_ao_pwm4", "top_pwm", 19),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_PWM, "infra_ao_pwm", "top_pwm", 21),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_UART0, "infra_ao_uart0", "top_uart", 22),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_UART1, "infra_ao_uart1", "top_uart", 23),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_UART2, "infra_ao_uart2", "top_uart", 24),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_UART3, "infra_ao_uart3", "top_uart", 25),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_UART4, "infra_ao_uart4", "top_uart", 26),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_GCE_26M, "infra_ao_gce_26m", "clk26m", 27),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_CQ_DMA_FPC, "infra_ao_cq_dma_fpc", "fpc", 28),
    gate_infra_ao!(AO0, INFRA_AO0_CG_REGS, CLK_INFRA_AO_UART5, "infra_ao_uart5", "top_uart", 29),
];

static mut INFRA_AO_RST_OFS: [u16; 5] = [
    INFRA_RST0_SET_OFFSET, INFRA_RST1_SET_OFFSET, INFRA_RST2_SET_OFFSET,
    INFRA_RST3_SET_OFFSET, INFRA_RST4_SET_OFFSET,
];

static mut INFRA_AO_IDX_MAP: [u16; 6] = [
    [MT8195_INFRA_RST0_THERM_CTRL_SWRST] = 0 * RST_NR_PER_BANK + 0,
    [MT8195_INFRA_RST2_USBSIF_P1_SWRST] = 2 * RST_NR_PER_BANK + 18,
    [MT8195_INFRA_RST2_PCIE_P0_SWRST] = 2 * RST_NR_PER_BANK + 26,
    [MT8195_INFRA_RST2_PCIE_P1_SWRST] = 2 * RST_NR_PER_BANK + 27,
    [MT8195_INFRA_RST3_THERM_CTRL_PTP_SWRST] = 3 * RST_NR_PER_BANK + 5,
    [MT8195_INFRA_RST4_THERM_CTRL_MCU_SWRST] = 4 * RST_NR_PER_BANK + 10,
];

static mut INFRA_AO_RST_DESC: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SET_CLR,
    rst_bank_ofs: unsafe { &mut INFRA_AO_RST_OFS },
    rst_bank_nr: core::mem::size_of_val(unsafe { &INFRA_AO_RST_OFS }),
    rst_idx_map: unsafe { &mut INFRA_AO_IDX_MAP },
    rst_idx_map_nr: core::mem::size_of_val(unsafe { &INFRA_AO_IDX_MAP }),
};

static INFRA_AO_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: &INFRA_AO_CLKS,
    num_clks: core::mem::size_of_val(&INFRA_AO_CLKS),
    rst_desc: unsafe { &mut INFRA_AO_RST_DESC },
};

static OF_MATCH_CLK_MT8195_INFRA_AO: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt8195-infracfg_ao", data: &INFRA_AO_DESC },
    of_device_id { /* sentinel */ compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut CLK_MT8195_INFRA_AO_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8195-infra_ao",
        of_match_table: &OF_MATCH_CLK_MT8195_INFRA_AO,
    },
};

// module_platform_driver(clk_mt8195_infra_ao_drv);
// MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_infra_ao);
// MODULE_DESCRIPTION("MediaTek MT8195 infracfg clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
