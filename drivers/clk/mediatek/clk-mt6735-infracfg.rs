// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Yassine Oudjana <y.oudjana@protonmail.com>
 */

// Linux clock-provider, platform-device, clk-gate, clk-mtk, and DT bindings
// are supplied by the surrounding crate.

const INFRA_RST0: u16 = 0x30;
const INFRA_GLOBALCON_PDN0: u16 = 0x40;
const INFRA_PDN1: u16 = 0x44;
const INFRA_PDN_STA: u16 = 0x48;
const RST_NR_PER_BANK: usize = 32;

static mut infra_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: INFRA_GLOBALCON_PDN0,
    clr_ofs: INFRA_PDN1,
    sta_ofs: INFRA_PDN_STA,
};

static infracfg_gates: [mtk_gate; 18] = [
    GATE_MTK!(CLK_INFRA_DBG, "dbg", "axi_sel", &infra_cg_regs, 0, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_GCE, "gce", "axi_sel", &infra_cg_regs, 1, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_TRBG, "trbg", "axi_sel", &infra_cg_regs, 2, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_CPUM, "cpum", "axi_sel", &infra_cg_regs, 3, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_DEVAPC, "devapc", "axi_sel", &infra_cg_regs, 4, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_AUDIO, "audio", "aud_intbus_sel", &infra_cg_regs, 5, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_GCPU, "gcpu", "axi_sel", &infra_cg_regs, 6, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_L2C_SRAM, "l2csram", "axi_sel", &infra_cg_regs, 7, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_M4U, "m4u", "axi_sel", &infra_cg_regs, 8, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_CLDMA, "cldma", "axi_sel", &infra_cg_regs, 12, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_CONNMCU_BUS, "connmcu_bus", "axi_sel", &infra_cg_regs, 15, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_KP, "kp", "axi_sel", &infra_cg_regs, 16, &mtk_clk_gate_ops_setclr),
    GATE_MTK_FLAGS!(CLK_INFRA_APXGPT, "apxgpt", "axi_sel", &infra_cg_regs, 18, &mtk_clk_gate_ops_setclr, CLK_IS_CRITICAL),
    GATE_MTK!(CLK_INFRA_SEJ, "sej", "axi_sel", &infra_cg_regs, 19, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_CCIF0_AP, "ccif0ap", "axi_sel", &infra_cg_regs, 20, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_CCIF1_AP, "ccif1ap", "axi_sel", &infra_cg_regs, 21, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_PMIC_SPI, "pmicspi", "pmicspi_sel", &infra_cg_regs, 22, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_PMIC_WRAP, "pmicwrap", "axi_sel", &infra_cg_regs, 23, &mtk_clk_gate_ops_setclr),
];

static mut infracfg_rst_bank_ofs: [u16; 1] = [INFRA_RST0];

// C designated initializers are represented as the equivalent sparse mapping.
static mut infracfg_rst_idx_map: [(u32, u32); 20] = [
    (MT6735_INFRA_RST0_EMI_REG, 0 * RST_NR_PER_BANK as u32 + 0),
    (MT6735_INFRA_RST0_DRAMC0_AO, 0 * RST_NR_PER_BANK as u32 + 1),
    (MT6735_INFRA_RST0_AP_CIRQ_EINT, 0 * RST_NR_PER_BANK as u32 + 3),
    (MT6735_INFRA_RST0_APXGPT, 0 * RST_NR_PER_BANK as u32 + 4),
    (MT6735_INFRA_RST0_SCPSYS, 0 * RST_NR_PER_BANK as u32 + 5),
    (MT6735_INFRA_RST0_KP, 0 * RST_NR_PER_BANK as u32 + 6),
    (MT6735_INFRA_RST0_PMIC_WRAP, 0 * RST_NR_PER_BANK as u32 + 7),
    (MT6735_INFRA_RST0_CLDMA_AO_TOP, 0 * RST_NR_PER_BANK as u32 + 8),
    (MT6735_INFRA_RST0_USBSIF_TOP, 0 * RST_NR_PER_BANK as u32 + 9),
    (MT6735_INFRA_RST0_EMI, 0 * RST_NR_PER_BANK as u32 + 16),
    (MT6735_INFRA_RST0_CCIF, 0 * RST_NR_PER_BANK as u32 + 17),
    (MT6735_INFRA_RST0_DRAMC0, 0 * RST_NR_PER_BANK as u32 + 18),
    (MT6735_INFRA_RST0_EMI_AO_REG, 0 * RST_NR_PER_BANK as u32 + 19),
    (MT6735_INFRA_RST0_CCIF_AO, 0 * RST_NR_PER_BANK as u32 + 20),
    (MT6735_INFRA_RST0_TRNG, 0 * RST_NR_PER_BANK as u32 + 21),
    (MT6735_INFRA_RST0_SYS_CIRQ, 0 * RST_NR_PER_BANK as u32 + 22),
    (MT6735_INFRA_RST0_GCE, 0 * RST_NR_PER_BANK as u32 + 23),
    (MT6735_INFRA_RST0_M4U, 0 * RST_NR_PER_BANK as u32 + 24),
    (MT6735_INFRA_RST0_CCIF1, 0 * RST_NR_PER_BANK as u32 + 25),
    (MT6735_INFRA_RST0_CLDMA_TOP_PD, 0 * RST_NR_PER_BANK as u32 + 26),
];

static infracfg_resets: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: &mut infracfg_rst_bank_ofs,
    rst_bank_nr: 1,
    rst_idx_map: &mut infracfg_rst_idx_map,
    rst_idx_map_nr: 20,
};

static infracfg_clks: mtk_clk_desc = mtk_clk_desc {
    clks: &infracfg_gates,
    num_clks: 18,
    rst_desc: &infracfg_resets,
};

static of_match_mt6735_infracfg: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt6735-infracfg", data: &infracfg_clks },
    of_device_id { /* sentinel */ },
];

static mut clk_mt6735_infracfg: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt6735-infracfg",
        of_match_table: &of_match_mt6735_infracfg,
    },
};

// MODULE_DEVICE_TABLE(of, of_match_mt6735_infracfg);
// module_platform_driver(clk_mt6735_infracfg);
// MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
// MODULE_DESCRIPTION("MediaTek MT6735 infracfg clock and reset driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
