// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ingenic JZ4740 SoC CGU driver
 *
 * Copyright (c) 2015 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/* CGU register offsets */
const CGU_REG_CPCCR: u32 = 0x00;
const CGU_REG_LCR: u32 = 0x04;
const CGU_REG_CPPCR: u32 = 0x10;
const CGU_REG_CLKGR: u32 = 0x20;
const CGU_REG_SCR: u32 = 0x24;
const CGU_REG_I2SCDR: u32 = 0x60;
const CGU_REG_LPCDR: u32 = 0x64;
const CGU_REG_MSCCDR: u32 = 0x68;
const CGU_REG_UHCCDR: u32 = 0x6c;
const CGU_REG_SSICDR: u32 = 0x74;

/* bits within a PLL control register */
const PLLCTL_M_SHIFT: u32 = 23;
const PLLCTL_M_MASK: u32 = 0x1ff << PLLCTL_M_SHIFT;
const PLLCTL_N_SHIFT: u32 = 18;
const PLLCTL_N_MASK: u32 = 0x1f << PLLCTL_N_SHIFT;
const PLLCTL_OD_SHIFT: u32 = 16;
const PLLCTL_OD_MASK: u32 = 0x3 << PLLCTL_OD_SHIFT;
const PLLCTL_STABLE: u32 = 1 << 10;
const PLLCTL_BYPASS: u32 = 1 << 9;
const PLLCTL_ENABLE: u32 = 1 << 8;

/* bits within the LCR register */
const LCR_SLEEP: u32 = 1 << 0;

/* bits within the CLKGR register */
const CLKGR_UDC: u32 = 1 << 11;

static mut cgu: *mut ingenic_cgu = core::ptr::null_mut();

static pll_od_encoding: [i8; 4] = [0x0, 0x1, -1, 0x3];

static jz4740_cgu_cpccr_div_table: [u8; 10] = [1, 2, 3, 4, 6, 8, 12, 16, 24, 32];

static jz4740_cgu_pll_half_div_table: [u8; 2] = [2, 1];

static jz4740_cgu_clocks: [ingenic_cgu_clk_info; JZ4740_CLK_COUNT] = [
    [JZ4740_CLK_EXT] = ingenic_cgu_clk_info { name: "ext", typ: CGU_CLK_EXT },
    [JZ4740_CLK_RTC] = ingenic_cgu_clk_info { name: "rtc", typ: CGU_CLK_EXT },
    [JZ4740_CLK_PLL] = ingenic_cgu_clk_info {
        name: "pll", typ: CGU_CLK_PLL,
        parents: [JZ4740_CLK_EXT, -1, -1, -1],
        pll: ingenic_cgu_pll_info {
            reg: CGU_REG_CPPCR, rate_multiplier: 1, m_shift: 23, m_bits: 9,
            m_offset: 2, n_shift: 18, n_bits: 5, n_offset: 2,
            od_shift: 16, od_bits: 2, od_max: 4,
            od_encoding: &pll_od_encoding, stable_bit: 10,
            bypass_reg: CGU_REG_CPPCR, bypass_bit: 9, enable_bit: 8,
        },
    },
    [JZ4740_CLK_PLL_HALF] = ingenic_cgu_clk_info {
        name: "pll half", typ: CGU_CLK_DIV,
        parents: [JZ4740_CLK_PLL, -1, -1, -1],
        div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 21, busy_shift: 1, bits: 1, busy_bit: -1, stop_bit: -1, gate_bit: -1, flags: 0, table: &jz4740_cgu_pll_half_div_table },
    },
    [JZ4740_CLK_CCLK] = ingenic_cgu_clk_info {
        name: "cclk", typ: CGU_CLK_DIV, flags: CLK_IS_CRITICAL,
        parents: [JZ4740_CLK_PLL, -1, -1, -1],
        // Disabling the CPU clock or any parent clocks will hang the system; mark it critical.
        div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 0, busy_shift: 1, bits: 4, busy_bit: 22, stop_bit: -1, gate_bit: -1, flags: 0, table: &jz4740_cgu_cpccr_div_table },
    },
    [JZ4740_CLK_HCLK] = ingenic_cgu_clk_info { name: "hclk", typ: CGU_CLK_DIV, parents: [JZ4740_CLK_PLL, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 4, busy_shift: 1, bits: 4, busy_bit: 22, stop_bit: -1, gate_bit: -1, flags: 0, table: &jz4740_cgu_cpccr_div_table } },
    [JZ4740_CLK_PCLK] = ingenic_cgu_clk_info { name: "pclk", typ: CGU_CLK_DIV, parents: [JZ4740_CLK_PLL, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 8, busy_shift: 1, bits: 4, busy_bit: 22, stop_bit: -1, gate_bit: -1, flags: 0, table: &jz4740_cgu_cpccr_div_table } },
    [JZ4740_CLK_MCLK] = ingenic_cgu_clk_info { name: "mclk", typ: CGU_CLK_DIV, flags: CLK_IS_CRITICAL, parents: [JZ4740_CLK_PLL, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 12, busy_shift: 1, bits: 4, busy_bit: 22, stop_bit: -1, gate_bit: -1, flags: 0, table: &jz4740_cgu_cpccr_div_table } },
    [JZ4740_CLK_LCD] = ingenic_cgu_clk_info { name: "lcd", typ: CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4740_CLK_PLL_HALF, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 16, busy_shift: 1, bits: 5, busy_bit: 22, stop_bit: -1, gate_bit: -1, flags: 0, table: &jz4740_cgu_cpccr_div_table }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 10, active_low: false } },
    [JZ4740_CLK_LCD_PCLK] = ingenic_cgu_clk_info { name: "lcd_pclk", typ: CGU_CLK_DIV, parents: [JZ4740_CLK_PLL_HALF, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_LPCDR, shift: 0, busy_shift: 1, bits: 11, busy_bit: -1, stop_bit: -1, gate_bit: -1, flags: 0, table: core::ptr::null() } },
    [JZ4740_CLK_I2S] = ingenic_cgu_clk_info { name: "i2s", typ: CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4740_CLK_EXT, JZ4740_CLK_PLL_HALF, -1, -1], mux: ingenic_cgu_mux_info { reg: CGU_REG_CPCCR, shift: 31, bits: 1 }, div: ingenic_cgu_div_info { reg: CGU_REG_I2SCDR, shift: 0, busy_shift: 1, bits: 9, busy_bit: -1, stop_bit: -1, gate_bit: -1, flags: 0, table: core::ptr::null() }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 6, active_low: false } },
    [JZ4740_CLK_SPI] = ingenic_cgu_clk_info { name: "spi", typ: CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4740_CLK_EXT, JZ4740_CLK_PLL, -1, -1], mux: ingenic_cgu_mux_info { reg: CGU_REG_SSICDR, shift: 31, bits: 1 }, div: ingenic_cgu_div_info { reg: CGU_REG_SSICDR, shift: 0, busy_shift: 1, bits: 4, busy_bit: -1, stop_bit: -1, gate_bit: -1, flags: 0, table: core::ptr::null() }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 4, active_low: false } },
    [JZ4740_CLK_MMC] = ingenic_cgu_clk_info { name: "mmc", typ: CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4740_CLK_PLL_HALF, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_MSCCDR, shift: 0, busy_shift: 1, bits: 5, busy_bit: -1, stop_bit: -1, gate_bit: -1, flags: 0, table: core::ptr::null() }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 7, active_low: false } },
    [JZ4740_CLK_UHC] = ingenic_cgu_clk_info { name: "uhc", typ: CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4740_CLK_PLL_HALF, -1, -1, -1], div: ingenic_cgu_div_info { reg: CGU_REG_UHCCDR, shift: 0, busy_shift: 1, bits: 4, busy_bit: -1, stop_bit: -1, gate_bit: -1, flags: 0, table: core::ptr::null() }, gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 14, active_low: false } },
    [JZ4740_CLK_UDC] = ingenic_cgu_clk_info { name: "udc", typ: CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE, parents: [JZ4740_CLK_EXT, JZ4740_CLK_PLL_HALF, -1, -1], mux: ingenic_cgu_mux_info { reg: CGU_REG_CPCCR, shift: 29, bits: 1 }, div: ingenic_cgu_div_info { reg: CGU_REG_CPCCR, shift: 23, busy_shift: 1, bits: 6, busy_bit: -1, stop_bit: -1, gate_bit: -1, flags: 0, table: core::ptr::null() }, gate: ingenic_cgu_gate_info { reg: CGU_REG_SCR, bit: 6, active_low: true } },
    [JZ4740_CLK_UART0] = ingenic_cgu_clk_info { name: "uart0", typ: CGU_CLK_GATE, parents: [JZ4740_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 0, active_low: false } },
    [JZ4740_CLK_UART1] = ingenic_cgu_clk_info { name: "uart1", typ: CGU_CLK_GATE, parents: [JZ4740_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 15, active_low: false } },
    [JZ4740_CLK_DMA] = ingenic_cgu_clk_info { name: "dma", typ: CGU_CLK_GATE, parents: [JZ4740_CLK_PCLK, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 12, active_low: false } },
    [JZ4740_CLK_IPU] = ingenic_cgu_clk_info { name: "ipu", typ: CGU_CLK_GATE, parents: [JZ4740_CLK_PCLK, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 13, active_low: false } },
    [JZ4740_CLK_ADC] = ingenic_cgu_clk_info { name: "adc", typ: CGU_CLK_GATE, parents: [JZ4740_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 8, active_low: false } },
    [JZ4740_CLK_I2C] = ingenic_cgu_clk_info { name: "i2c", typ: CGU_CLK_GATE, parents: [JZ4740_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 3, active_low: false } },
    [JZ4740_CLK_AIC] = ingenic_cgu_clk_info { name: "aic", typ: CGU_CLK_GATE, parents: [JZ4740_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 5, active_low: false } },
    [JZ4740_CLK_TCU] = ingenic_cgu_clk_info { name: "tcu", typ: CGU_CLK_GATE, parents: [JZ4740_CLK_EXT, -1, -1, -1], gate: ingenic_cgu_gate_info { reg: CGU_REG_CLKGR, bit: 1, active_low: false } },
];

unsafe fn jz4740_cgu_init(np: *mut device_node) {
    let mut retval: i32;
    cgu = ingenic_cgu_new(jz4740_cgu_clocks.as_ptr(), jz4740_cgu_clocks.len(), np);
    if cgu.is_null() {
        pr_err!("{}: failed to initialise CGU\n", "jz4740_cgu_init");
        return;
    }
    retval = ingenic_cgu_register_clocks(cgu);
    if retval != 0 {
        pr_err!("{}: failed to register CGU Clocks\n", "jz4740_cgu_init");
    }
    ingenic_cgu_register_syscore(cgu);
}

// CLK_OF_DECLARE_DRIVER(jz4740_cgu, "ingenic,jz4740-cgu", jz4740_cgu_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
