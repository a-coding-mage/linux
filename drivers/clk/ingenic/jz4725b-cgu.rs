// SPDX-License-Identifier: GPL-2.0
/*
 * Ingenic JZ4725B SoC CGU driver
 *
 * Copyright (C) 2018 Paul Cercueil
 * Author: Paul Cercueil <paul@crapouillou.net>
 */

// C dependencies supplied by the surrounding kernel translation.

/* CGU register offsets */
const CGU_REG_CPCCR: u32 = 0x00;
const CGU_REG_LCR: u32 = 0x04;
const CGU_REG_CPPCR: u32 = 0x10;
const CGU_REG_CLKGR: u32 = 0x20;
const CGU_REG_OPCR: u32 = 0x24;
const CGU_REG_I2SCDR: u32 = 0x60;
const CGU_REG_LPCDR: u32 = 0x64;
const CGU_REG_MSCCDR: u32 = 0x68;
const CGU_REG_SSICDR: u32 = 0x74;
const CGU_REG_CIMCDR: u32 = 0x78;

/* bits within the LCR register */
const LCR_SLEEP: u32 = 1 << 0;

static mut cgu: *mut ingenic_cgu = core::ptr::null_mut();

static pll_od_encoding: [i8; 4] = [0x0, 0x1, -1, 0x3];

static jz4725b_cgu_cpccr_div_table: [u8; 6] = [1, 2, 3, 4, 6, 8];

static jz4725b_cgu_pll_half_div_table: [u8; 2] = [2, 1];

static jz4725b_cgu_clocks: [ingenic_cgu_clk_info; JZ4725B_NR_CLKS] = [
    /* External clocks */
    [JZ4725B_CLK_EXT] = ingenic_cgu_clk_info { name: "ext", typ: CGU_CLK_EXT },
    [JZ4725B_CLK_OSC32K] = ingenic_cgu_clk_info { name: "osc32k", typ: CGU_CLK_EXT },

    [JZ4725B_CLK_PLL] = ingenic_cgu_clk_info {
        name: "pll", typ: CGU_CLK_PLL,
        parents: [JZ4725B_CLK_EXT, -1, -1, -1],
        pll: ingenic_cgu_pll_info {
            reg: CGU_REG_CPPCR, rate_multiplier: 1, m_shift: 23, m_bits: 9,
            m_offset: 2, n_shift: 18, n_bits: 5, n_offset: 2, od_shift: 16,
            od_bits: 2, od_max: 4, od_encoding: &pll_od_encoding, stable_bit: 10,
            bypass_reg: CGU_REG_CPPCR, bypass_bit: 9, enable_bit: 8,
        },
    },

    /* Muxes & dividers */
    [JZ4725B_CLK_PLL_HALF] = ingenic_cgu_clk_info {
        name: "pll half", typ: CGU_CLK_DIV,
        parents: [JZ4725B_CLK_PLL, -1, -1, -1],
        div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 21, bits: 1, div: 1, mux_shift: -1, mux_bits: -1, flags: -1, table: Some(&jz4725b_cgu_pll_half_div_table) },
    },
    [JZ4725B_CLK_CCLK] = ingenic_cgu_clk_info {
        name: "cclk", typ: CGU_CLK_DIV, flags: CLK_IS_CRITICAL,
        parents: [JZ4725B_CLK_PLL, -1, -1, -1],
        div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 0, bits: 1, div: 4, mux_shift: 22, mux_bits: -1, flags: -1, table: Some(&jz4725b_cgu_cpccr_div_table) },
    },
    [JZ4725B_CLK_HCLK] = ingenic_cgu_clk_info { name: "hclk", typ: CGU_CLK_DIV, parents: [JZ4725B_CLK_PLL, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 4, bits: 1, div: 4, mux_shift: 22, mux_bits: -1, flags: -1, table: Some(&jz4725b_cgu_cpccr_div_table) } },
    [JZ4725B_CLK_PCLK] = ingenic_cgu_clk_info { name: "pclk", typ: CGU_CLK_DIV, parents: [JZ4725B_CLK_PLL, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 8, bits: 1, div: 4, mux_shift: 22, mux_bits: -1, flags: -1, table: Some(&jz4725b_cgu_cpccr_div_table) } },
    [JZ4725B_CLK_MCLK] = ingenic_cgu_clk_info { name: "mclk", typ: CGU_CLK_DIV, flags: CLK_IS_CRITICAL, parents: [JZ4725B_CLK_PLL, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 12, bits: 1, div: 4, mux_shift: 22, mux_bits: -1, flags: -1, table: Some(&jz4725b_cgu_cpccr_div_table) } },
    [JZ4725B_CLK_IPU] = ingenic_cgu_clk_info { name: "ipu", typ: CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4725B_CLK_PLL, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 16, bits: 1, div: 4, mux_shift: 22, mux_bits: -1, flags: -1, table: Some(&jz4725b_cgu_cpccr_div_table) }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 13, inverted: false } },
    [JZ4725B_CLK_LCD] = ingenic_cgu_clk_info { name: "lcd", typ: CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4725B_CLK_PLL_HALF, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_LPCDR, shift: 0, bits: 1, div: 11, mux_shift: -1, mux_bits: -1, flags: -1, table: None }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 9, inverted: false } },
    [JZ4725B_CLK_I2S] = ingenic_cgu_clk_info { name: "i2s", typ: CGU_CLK_MUX | CGU_CLK_DIV, parents: [JZ4725B_CLK_EXT, JZ4725B_CLK_PLL_HALF, -1, -1], mux: ingenic_cgu_mux_info { reg: CGU_REG_CPCCR, shift: 31, bits: 1 }, div: ingenic_cgu_div_info { reg: CGU_REG_I2SCDR, shift: 0, bits: 1, div: 9, mux_shift: -1, mux_bits: -1, flags: -1, table: None } },
    [JZ4725B_CLK_SPI] = ingenic_cgu_clk_info { name: "spi", typ: CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4725B_CLK_EXT, JZ4725B_CLK_PLL, -1, -1], mux: ingenic_cgu_mux_info { reg: CGU_REG_SSICDR, shift: 31, bits: 1 }, div: ingenic_cgu_div_info { reg: CGU_REG_SSICDR, shift: 0, bits: 1, div: 4, mux_shift: -1, mux_bits: -1, flags: -1, table: None }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 4, inverted: false } },
    [JZ4725B_CLK_MMC_MUX] = ingenic_cgu_clk_info { name: "mmc_mux", typ: CGU_CLK_DIV, parents: [JZ4725B_CLK_PLL_HALF, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_MSCCDR, shift: 0, bits: 1, div: 5, mux_shift: -1, mux_bits: -1, flags: -1, table: None } },
    [JZ4725B_CLK_UDC] = ingenic_cgu_clk_info { name: "udc", typ: CGU_CLK_MUX | CGU_CLK_DIV, parents: [JZ4725B_CLK_EXT, JZ4725B_CLK_PLL_HALF, -1, -1], mux: ingenic_cgu_mux_info { reg: CGU_REG_CPCCR, shift: 29, bits: 1 }, div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 23, bits: 1, div: 6, mux_shift: -1, mux_bits: -1, flags: -1, table: None } },

    /* Gate-only clocks */
    [JZ4725B_CLK_UART] = ingenic_cgu_clk_info { name: "uart", typ: CGU_CLK_GATE, parents: [JZ4725B_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 0, inverted: false } },
    [JZ4725B_CLK_DMA] = ingenic_cgu_clk_info { name: "dma", typ: CGU_CLK_GATE, parents: [JZ4725B_CLK_PCLK, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 12, inverted: false } },
    [JZ4725B_CLK_ADC] = ingenic_cgu_clk_info { name: "adc", typ: CGU_CLK_GATE, parents: [JZ4725B_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 7, inverted: false } },
    [JZ4725B_CLK_I2C] = ingenic_cgu_clk_info { name: "i2c", typ: CGU_CLK_GATE, parents: [JZ4725B_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 3, inverted: false } },
    [JZ4725B_CLK_AIC] = ingenic_cgu_clk_info { name: "aic", typ: CGU_CLK_GATE, parents: [JZ4725B_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 5, inverted: false } },
    [JZ4725B_CLK_MMC0] = ingenic_cgu_clk_info { name: "mmc0", typ: CGU_CLK_GATE, parents: [JZ4725B_CLK_MMC_MUX, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 6, inverted: false } },
    [JZ4725B_CLK_MMC1] = ingenic_cgu_clk_info { name: "mmc1", typ: CGU_CLK_GATE, parents: [JZ4725B_CLK_MMC_MUX, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 16, inverted: false } },
    [JZ4725B_CLK_BCH] = ingenic_cgu_clk_info { name: "bch", typ: CGU_CLK_GATE, parents: [JZ4725B_CLK_MCLK, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 11, inverted: false } },
    [JZ4725B_CLK_TCU] = ingenic_cgu_clk_info { name: "tcu", typ: CGU_CLK_GATE, parents: [JZ4725B_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 1, inverted: false } },
    [JZ4725B_CLK_EXT512] = ingenic_cgu_clk_info { name: "ext/512", typ: CGU_CLK_FIXDIV, parents: [JZ4725B_CLK_EXT, -1, -1, -1], fixdiv: ingenic_cgu_fixdiv_info { div: 256 } },
    [JZ4725B_CLK_RTC] = ingenic_cgu_clk_info { name: "rtc", typ: CGU_CLK_MUX, parents: [JZ4725B_CLK_EXT512, JZ4725B_CLK_OSC32K, -1, -1], mux: ingenic_cgu_mux_info { reg: CGU_REG_OPCR, shift: 2, bits: 1 } },
    [JZ4725B_CLK_UDC_PHY] = ingenic_cgu_clk_info { name: "udc_phy", typ: CGU_CLK_GATE, parents: [JZ4725B_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_OPCR, bit: 6, inverted: true } },
];

unsafe fn jz4725b_cgu_init(np: *mut device_node) {
    let mut retval: i32;
    cgu = ingenic_cgu_new(jz4725b_cgu_clocks.as_ptr(), jz4725b_cgu_clocks.len(), np);
    if cgu.is_null() {
        pr_err!("jz4725b_cgu_init: failed to initialise CGU\n");
        return;
    }
    retval = ingenic_cgu_register_clocks(cgu);
    if retval != 0 {
        pr_err!("jz4725b_cgu_init: failed to register CGU Clocks\n");
    }
    ingenic_cgu_register_syscore(cgu);
}

// CLK_OF_DECLARE_DRIVER(jz4725b_cgu, "ingenic,jz4725b-cgu", jz4725b_cgu_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
