// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */
// External dependencies supplied by the kernel clock framework and bindings.

static PERI_AO0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x24,
    clr_ofs: 0x28,
    sta_ofs: 0x10,
};

static PERI_AO1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x2c,
    clr_ofs: 0x30,
    sta_ofs: 0x14,
};

static PERI_AO1_HWV_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0008,
    clr_ofs: 0x000c,
    sta_ofs: 0x2c04,
};

static PERI_AO2_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x34,
    clr_ofs: 0x38,
    sta_ofs: 0x18,
};

static PERI_AO_CLKS: [mtk_gate; 34] = [
    // PERI_AO0
    mtk_gate { id: CLK_PERI_AO_UART0_BCLK, name: "peri_ao_uart0_bclk", parent_name: "uart", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 0, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_UART1_BCLK, name: "peri_ao_uart1_bclk", parent_name: "uart", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 1, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_UART2_BCLK, name: "peri_ao_uart2_bclk", parent_name: "uart", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 2, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_UART3_BCLK, name: "peri_ao_uart3_bclk", parent_name: "uart", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 3, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_UART4_BCLK, name: "peri_ao_uart4_bclk", parent_name: "uart", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 4, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_UART5_BCLK, name: "peri_ao_uart5_bclk", parent_name: "uart", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 5, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_PWM_X16W_HCLK, name: "peri_ao_pwm_x16w", parent_name: "p_axi", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 12, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_PWM_X16W_BCLK, name: "peri_ao_pwm_x16w_bclk", parent_name: "pwm", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 13, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_PWM_PWM_BCLK0, name: "peri_ao_pwm_pwm_bclk0", parent_name: "pwm", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 14, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_PWM_PWM_BCLK1, name: "peri_ao_pwm_pwm_bclk1", parent_name: "pwm", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 15, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_PWM_PWM_BCLK2, name: "peri_ao_pwm_pwm_bclk2", parent_name: "pwm", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 16, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_PWM_PWM_BCLK3, name: "peri_ao_pwm_pwm_bclk3", parent_name: "pwm", regs: &PERI_AO0_CG_REGS, hwv_regs: core::ptr::null(), shift: 17, ops: &mtk_clk_gate_ops_setclr },
    // PERI_AO1
    mtk_gate { id: CLK_PERI_AO_SPI0_BCLK, name: "peri_ao_spi0_bclk", parent_name: "spi0_b", regs: &PERI_AO1_CG_REGS, hwv_regs: &PERI_AO1_HWV_REGS, shift: 0, ops: &mtk_clk_gate_hwv_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_SPI1_BCLK, name: "peri_ao_spi1_bclk", parent_name: "spi1_b", regs: &PERI_AO1_CG_REGS, hwv_regs: &PERI_AO1_HWV_REGS, shift: 2, ops: &mtk_clk_gate_hwv_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_SPI2_BCLK, name: "peri_ao_spi2_bclk", parent_name: "spi2_b", regs: &PERI_AO1_CG_REGS, hwv_regs: &PERI_AO1_HWV_REGS, shift: 3, ops: &mtk_clk_gate_hwv_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_SPI3_BCLK, name: "peri_ao_spi3_bclk", parent_name: "spi3_b", regs: &PERI_AO1_CG_REGS, hwv_regs: &PERI_AO1_HWV_REGS, shift: 4, ops: &mtk_clk_gate_hwv_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_SPI4_BCLK, name: "peri_ao_spi4_bclk", parent_name: "spi4_b", regs: &PERI_AO1_CG_REGS, hwv_regs: &PERI_AO1_HWV_REGS, shift: 5, ops: &mtk_clk_gate_hwv_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_SPI5_BCLK, name: "peri_ao_spi5_bclk", parent_name: "spi5_b", regs: &PERI_AO1_CG_REGS, hwv_regs: &PERI_AO1_HWV_REGS, shift: 6, ops: &mtk_clk_gate_hwv_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_SPI6_BCLK, name: "peri_ao_spi6_bclk", parent_name: "spi6_b", regs: &PERI_AO1_CG_REGS, hwv_regs: &PERI_AO1_HWV_REGS, shift: 7, ops: &mtk_clk_gate_hwv_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_SPI7_BCLK, name: "peri_ao_spi7_bclk", parent_name: "spi7_b", regs: &PERI_AO1_CG_REGS, hwv_regs: &PERI_AO1_HWV_REGS, shift: 8, ops: &mtk_clk_gate_hwv_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_FLASHIF_FLASH, name: "peri_ao_flashif_flash", parent_name: "peri_ao_flashif_27m", regs: &PERI_AO1_CG_REGS, hwv_regs: core::ptr::null(), shift: 18, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_FLASHIF_27M, name: "peri_ao_flashif_27m", parent_name: "sflash", regs: &PERI_AO1_CG_REGS, hwv_regs: core::ptr::null(), shift: 19, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_FLASHIF_DRAM, name: "peri_ao_flashif_dram", parent_name: "p_axi", regs: &PERI_AO1_CG_REGS, hwv_regs: core::ptr::null(), shift: 20, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_FLASHIF_AXI, name: "peri_ao_flashif_axi", parent_name: "peri_ao_flashif_dram", regs: &PERI_AO1_CG_REGS, hwv_regs: core::ptr::null(), shift: 21, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_FLASHIF_BCLK, name: "peri_ao_flashif_bclk", parent_name: "p_axi", regs: &PERI_AO1_CG_REGS, hwv_regs: core::ptr::null(), shift: 22, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_AP_DMA_X32W_BCLK, name: "peri_ao_ap_dma_x32w_bclk", parent_name: "p_axi", regs: &PERI_AO1_CG_REGS, hwv_regs: core::ptr::null(), shift: 26, ops: &mtk_clk_gate_ops_setclr },
    // PERI_AO2
    mtk_gate { id: CLK_PERI_AO_MSDC1_MSDC_SRC, name: "peri_ao_msdc1_msdc_src", parent_name: "msdc30_1", regs: &PERI_AO2_CG_REGS, hwv_regs: core::ptr::null(), shift: 1, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_MSDC1_HCLK, name: "peri_ao_msdc1", parent_name: "peri_ao_msdc1_axi", regs: &PERI_AO2_CG_REGS, hwv_regs: core::ptr::null(), shift: 2, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_MSDC1_AXI, name: "peri_ao_msdc1_axi", parent_name: "p_axi", regs: &PERI_AO2_CG_REGS, hwv_regs: core::ptr::null(), shift: 3, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_MSDC1_HCLK_WRAP, name: "peri_ao_msdc1_h_wrap", parent_name: "peri_ao_msdc1", regs: &PERI_AO2_CG_REGS, hwv_regs: core::ptr::null(), shift: 4, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_MSDC2_MSDC_SRC, name: "peri_ao_msdc2_msdc_src", parent_name: "msdc30_2", regs: &PERI_AO2_CG_REGS, hwv_regs: core::ptr::null(), shift: 10, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_MSDC2_HCLK, name: "peri_ao_msdc2", parent_name: "peri_ao_msdc2_axi", regs: &PERI_AO2_CG_REGS, hwv_regs: core::ptr::null(), shift: 11, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_MSDC2_AXI, name: "peri_ao_msdc2_axi", parent_name: "p_axi", regs: &PERI_AO2_CG_REGS, hwv_regs: core::ptr::null(), shift: 12, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_PERI_AO_MSDC2_HCLK_WRAP, name: "peri_ao_msdc2_h_wrap", parent_name: "peri_ao_msdc2", regs: &PERI_AO2_CG_REGS, hwv_regs: core::ptr::null(), shift: 13, ops: &mtk_clk_gate_ops_setclr },
];

static PERI_AO_MCD: mtk_clk_desc = mtk_clk_desc {
    clks: &PERI_AO_CLKS,
    num_clks: PERI_AO_CLKS.len(),
};

static OF_MATCH_CLK_MT8196_PERI_AO: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt8196-pericfg-ao", data: &PERI_AO_MCD },
    of_device_id::sentinel(),
];

static mut CLK_MT8196_PERI_AO_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8196-peri-ao",
        of_match_table: &OF_MATCH_CLK_MT8196_PERI_AO,
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8196_peri_ao);
// MODULE_DESCRIPTION("MediaTek MT8196 pericfg_ao clock controller driver");
// module_platform_driver(clk_mt8196_peri_ao_drv);
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
