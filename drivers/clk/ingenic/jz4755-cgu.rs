// SPDX-License-Identifier: GPL-2.0
/*
 * Ingenic JZ4755 SoC CGU driver
 * Heavily based on JZ4725b CGU driver
 *
 * Copyright (C) 2022 Siarhei Volkau
 * Author: Siarhei Volkau <lis8215@gmail.com>
 */

// Dependencies supplied by the kernel clock, device-tree, CGU, PM, and
// dt-bindings layers are intentionally referenced but not implemented here.

const CGU_REG_CPCCR: u32 = 0x00;
const CGU_REG_CPPCR: u32 = 0x10;
const CGU_REG_CLKGR: u32 = 0x20;
const CGU_REG_OPCR: u32 = 0x24;
const CGU_REG_I2SCDR: u32 = 0x60;
const CGU_REG_LPCDR: u32 = 0x64;
const CGU_REG_MSCCDR: u32 = 0x68;
const CGU_REG_SSICDR: u32 = 0x74;
const CGU_REG_CIMCDR: u32 = 0x7c;

static mut cgu: *mut ingenic_cgu = core::ptr::null_mut();

static pll_od_encoding: [i8; 4] = [0x0, 0x1, -1, 0x3];
static jz4755_cgu_cpccr_div_table: [u8; 6] = [1, 2, 3, 4, 6, 8];
static jz4755_cgu_pll_half_div_table: [u8; 2] = [2, 1];

static jz4755_cgu_clocks: [ingenic_cgu_clk_info; JZ4755_CLK_COUNT] = [
    [JZ4755_CLK_EXT] = ingenic_cgu_clk_info { name: "ext", flags: CGU_CLK_EXT, parents: [0], ..Default::default() },
    [JZ4755_CLK_OSC32K] = ingenic_cgu_clk_info { name: "osc32k", flags: CGU_CLK_EXT, parents: [0], ..Default::default() },
    [JZ4755_CLK_PLL] = ingenic_cgu_clk_info {
        name: "pll", flags: CGU_CLK_PLL, parents: [JZ4755_CLK_EXT],
        pll: ingenic_cgu_pll_info { reg: CGU_REG_CPPCR, rate_multiplier: 1, m_shift: 23, m_bits: 9, m_offset: 2, n_shift: 18, n_bits: 5, n_offset: 2, od_shift: 16, od_bits: 2, od_max: 4, od_encoding: &pll_od_encoding, stable_bit: 10, bypass_reg: CGU_REG_CPPCR, bypass_bit: 9, enable_bit: 8 },
        ..Default::default()
    },
    [JZ4755_CLK_PLL_HALF] = ingenic_cgu_clk_info { name: "pll half", flags: CGU_CLK_DIV, parents: [JZ4755_CLK_PLL], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 21, width: 1, mask: 1, ce: -1, busy: -1, stop: -1, flags: 0, table: Some(&jz4755_cgu_pll_half_div_table) }, ..Default::default() },
    [JZ4755_CLK_EXT_HALF] = ingenic_cgu_clk_info { name: "ext half", flags: CGU_CLK_DIV, parents: [JZ4755_CLK_EXT], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 30, width: 1, mask: 1, ce: -1, busy: -1, stop: -1, flags: 0, table: None }, ..Default::default() },
    [JZ4755_CLK_CCLK] = ingenic_cgu_clk_info { name: "cclk", flags: CGU_CLK_DIV, parents: [JZ4755_CLK_PLL], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 0, width: 1, mask: 4, ce: 22, busy: -1, stop: -1, flags: 0, table: Some(&jz4755_cgu_cpccr_div_table) }, ..Default::default() },
    [JZ4755_CLK_H0CLK] = ingenic_cgu_clk_info { name: "hclk", flags: CGU_CLK_DIV, parents: [JZ4755_CLK_PLL], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 4, width: 1, mask: 4, ce: 22, busy: -1, stop: -1, flags: 0, table: Some(&jz4755_cgu_cpccr_div_table) }, ..Default::default() },
    [JZ4755_CLK_PCLK] = ingenic_cgu_clk_info { name: "pclk", flags: CGU_CLK_DIV, parents: [JZ4755_CLK_PLL], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 8, width: 1, mask: 4, ce: 22, busy: -1, stop: -1, flags: 0, table: Some(&jz4755_cgu_cpccr_div_table) }, ..Default::default() },
    [JZ4755_CLK_MCLK] = ingenic_cgu_clk_info { name: "mclk", flags: CGU_CLK_DIV, parents: [JZ4755_CLK_PLL], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 12, width: 1, mask: 4, ce: 22, busy: -1, stop: -1, flags: 0, table: Some(&jz4755_cgu_cpccr_div_table) }, ..Default::default() },
    [JZ4755_CLK_H1CLK] = ingenic_cgu_clk_info { name: "h1clk", flags: CGU_CLK_DIV, parents: [JZ4755_CLK_PLL], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 16, width: 1, mask: 4, ce: 22, busy: -1, stop: -1, flags: 0, table: Some(&jz4755_cgu_cpccr_div_table) }, ..Default::default() },
    [JZ4755_CLK_UDC] = ingenic_cgu_clk_info { name: "udc", flags: CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4755_CLK_EXT_HALF, JZ4755_CLK_PLL_HALF], mux: ingenic_cgu_mux_info { reg: CGU_REG_CPCCR, shift: 29, width: 1 }, div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 23, width: 1, mask: 6, ce: -1, busy: -1, stop: -1, flags: 0, table: None }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 10, inverted: false }, ..Default::default() },
    [JZ4755_CLK_LCD] = ingenic_cgu_clk_info { name: "lcd", flags: CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4755_CLK_PLL_HALF], div: ingenic_cgu_div_info { reg: CGU_REG_LPCDR, shift: 0, width: 1, mask: 11, ce: -1, busy: -1, stop: -1, flags: 0, table: None }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 9, inverted: false }, ..Default::default() },
    [JZ4755_CLK_MMC] = ingenic_cgu_clk_info { name: "mmc", flags: CGU_CLK_DIV, parents: [JZ4755_CLK_PLL_HALF], div: ingenic_cgu_div_info { reg: CGU_REG_MSCCDR, shift: 0, width: 1, mask: 5, ce: -1, busy: -1, stop: -1, flags: 0, table: None }, ..Default::default() },
    [JZ4755_CLK_I2S] = ingenic_cgu_clk_info { name: "i2s", flags: CGU_CLK_MUX | CGU_CLK_DIV, parents: [JZ4755_CLK_EXT_HALF, JZ4755_CLK_PLL_HALF], mux: ingenic_cgu_mux_info { reg: CGU_REG_CPCCR, shift: 31, width: 1 }, div: ingenic_cgu_div_info { reg: CGU_REG_I2SCDR, shift: 0, width: 1, mask: 9, ce: -1, busy: -1, stop: -1, flags: 0, table: None }, ..Default::default() },
    [JZ4755_CLK_SPI] = ingenic_cgu_clk_info { name: "spi", flags: CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4755_CLK_PLL_HALF], div: ingenic_cgu_div_info { reg: CGU_REG_SSICDR, shift: 0, width: 1, mask: 4, ce: -1, busy: -1, stop: -1, flags: 0, table: None }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 4, inverted: false }, ..Default::default() },
    [JZ4755_CLK_TVE] = ingenic_cgu_clk_info { name: "tve", flags: CGU_CLK_MUX | CGU_CLK_GATE, parents: [JZ4755_CLK_LCD, JZ4755_CLK_EXT], mux: ingenic_cgu_mux_info { reg: CGU_REG_LPCDR, shift: 31, width: 1 }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 18, inverted: false }, ..Default::default() },
    [JZ4755_CLK_RTC] = ingenic_cgu_clk_info { name: "rtc", flags: CGU_CLK_MUX | CGU_CLK_GATE, parents: [JZ4755_CLK_EXT512, JZ4755_CLK_OSC32K], mux: ingenic_cgu_mux_info { reg: CGU_REG_OPCR, shift: 2, width: 1 }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 2, inverted: false }, ..Default::default() },
    [JZ4755_CLK_CIM] = ingenic_cgu_clk_info { name: "cim", flags: CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4755_CLK_PLL_HALF], div: ingenic_cgu_div_info { reg: CGU_REG_CIMCDR, shift: 0, width: 1, mask: 8, ce: -1, busy: -1, stop: -1, flags: 0, table: None }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 8, inverted: false }, ..Default::default() },
    [JZ4755_CLK_UART0] = ingenic_cgu_clk_info { name: "uart0", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_EXT_HALF], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 0, inverted: false }, ..Default::default() },
    [JZ4755_CLK_UART1] = ingenic_cgu_clk_info { name: "uart1", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_EXT_HALF], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 14, inverted: false }, ..Default::default() },
    [JZ4755_CLK_UART2] = ingenic_cgu_clk_info { name: "uart2", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_EXT_HALF], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 15, inverted: false }, ..Default::default() },
    [JZ4755_CLK_ADC] = ingenic_cgu_clk_info { name: "adc", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_EXT_HALF], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 7, inverted: false }, ..Default::default() },
    [JZ4755_CLK_AIC] = ingenic_cgu_clk_info { name: "aic", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_EXT_HALF], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 5, inverted: false }, ..Default::default() },
    [JZ4755_CLK_I2C] = ingenic_cgu_clk_info { name: "i2c", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_EXT_HALF], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 3, inverted: false }, ..Default::default() },
    [JZ4755_CLK_BCH] = ingenic_cgu_clk_info { name: "bch", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_H1CLK], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 11, inverted: false }, ..Default::default() },
    [JZ4755_CLK_TCU] = ingenic_cgu_clk_info { name: "tcu", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_EXT], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 1, inverted: false }, ..Default::default() },
    [JZ4755_CLK_DMA] = ingenic_cgu_clk_info { name: "dma", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_PCLK], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 12, inverted: false }, ..Default::default() },
    [JZ4755_CLK_MMC0] = ingenic_cgu_clk_info { name: "mmc0", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_MMC], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 6, inverted: false }, ..Default::default() },
    [JZ4755_CLK_MMC1] = ingenic_cgu_clk_info { name: "mmc1", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_MMC], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 16, inverted: false }, ..Default::default() },
    [JZ4755_CLK_AUX_CPU] = ingenic_cgu_clk_info { name: "aux_cpu", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_H1CLK], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 24, inverted: false }, ..Default::default() },
    [JZ4755_CLK_AHB1] = ingenic_cgu_clk_info { name: "ahb1", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_H1CLK], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 23, inverted: false }, ..Default::default() },
    [JZ4755_CLK_IDCT] = ingenic_cgu_clk_info { name: "idct", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_H1CLK], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 22, inverted: false }, ..Default::default() },
    [JZ4755_CLK_DB] = ingenic_cgu_clk_info { name: "db", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_H1CLK], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 21, inverted: false }, ..Default::default() },
    [JZ4755_CLK_ME] = ingenic_cgu_clk_info { name: "me", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_H1CLK], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 20, inverted: false }, ..Default::default() },
    [JZ4755_CLK_MC] = ingenic_cgu_clk_info { name: "mc", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_H1CLK], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 19, inverted: false }, ..Default::default() },
    [JZ4755_CLK_TSSI] = ingenic_cgu_clk_info { name: "tssi", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_EXT_HALF], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 17, inverted: false }, ..Default::default() },
    [JZ4755_CLK_IPU] = ingenic_cgu_clk_info { name: "ipu", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_PLL_HALF], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 13, inverted: false }, ..Default::default() },
    [JZ4755_CLK_EXT512] = ingenic_cgu_clk_info { name: "ext/512", flags: CGU_CLK_FIXDIV, parents: [JZ4755_CLK_EXT], fixdiv: ingenic_cgu_fixdiv_info { div: 512 }, ..Default::default() },
    [JZ4755_CLK_UDC_PHY] = ingenic_cgu_clk_info { name: "udc_phy", flags: CGU_CLK_GATE, parents: [JZ4755_CLK_EXT_HALF], gate: ingenic_cgu_gate_info { reg: CGU_REG_OPCR, bit: 6, inverted: true }, ..Default::default() },
];

unsafe fn jz4755_cgu_init(np: *mut device_node) {
    let mut retval: i32;
    cgu = ingenic_cgu_new(&jz4755_cgu_clocks, jz4755_cgu_clocks.len(), np);
    if cgu.is_null() {
        pr_err!("{}: failed to initialise CGU\n", "jz4755_cgu_init");
        return;
    }
    retval = ingenic_cgu_register_clocks(cgu);
    if retval != 0 {
        pr_err!("{}: failed to register CGU Clocks\n", "jz4755_cgu_init");
    }
    ingenic_cgu_register_syscore(cgu);
}

// CLK_OF_DECLARE_DRIVER(jz4755_cgu, "ingenic,jz4755-cgu", jz4755_cgu_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
