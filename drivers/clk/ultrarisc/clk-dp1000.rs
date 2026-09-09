// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2026 UltraRISC Technology (Shanghai) Co., Ltd.
 */

// Linux and dt-bindings dependencies are supplied by the surrounding kernel
// translation unit, together with the declarations from clk-ultrarisc.h.

const DP1000_PLL_CFG1_OFFSET: u32 = 0x400;
const DP1000_PLL_CFG2_OFFSET: u32 = 0x404;

const DP1000_CCR_UART_OFFSET: u32 = 0x220;
const DP1000_CCR_I2C_OFFSET: u32 = 0x224;
const DP1000_CCR_GMAC_OFFSET: u32 = 0x228;
const DP1000_CCR_SPI_OFFSET: u32 = 0x22c;
const DP1000_PERI_CLKENA_OFFSET: u32 = 0x270;

const DP1000_CCR_LOAD: u32 = 1 << 16;

const DP1000_PERI_MAX_RATE: u64 = 62500000;
const DP1000_CLK_NUM: usize = 21;

static DP1000_PLL_LAYOUT: ultrarisc_pll_layout = ultrarisc_pll_layout {
    cfg1_offset: DP1000_PLL_CFG1_OFFSET,
    cfg2_offset: DP1000_PLL_CFG2_OFFSET,
    frac_mask: (1 << 24) - 1,
    m_mask: ((1 << 8) - 1) << 16,
    n_mask: ((1 << 6) - 1) << 6,
    oddiv1_mask: (1 << 2) - 1,
    oddiv2_mask: ((1 << 2) - 1) << 3,
};

static DP1000_PLLS: [ultrarisc_pll_desc; 1] = [
    ultrarisc_pll_desc {
        id: DP1000_CLK_SYSPLL,
        name: "syspll_clk",
    },
];

static DP1000_FIXED_FACTOR_CLKS: [ultrarisc_fixed_factor_desc; 6] = [
    ultrarisc_fixed_factor_desc { id: DP1000_CLK_SYSPLL_DIV2, name: "syspll_div2_clk", parent_id: DP1000_CLK_SYSPLL, mult: 1, div: 2 },
    ultrarisc_fixed_factor_desc { id: DP1000_CLK_SUBSYS, name: "subsys_clk", parent_id: DP1000_CLK_SYSPLL_DIV2, mult: 1, div: 2 },
    ultrarisc_fixed_factor_desc { id: DP1000_CLK_PCIE_DBI, name: "pcie_dbi_clk", parent_id: DP1000_CLK_SYSPLL, mult: 1, div: 10 },
    ultrarisc_fixed_factor_desc { id: DP1000_CLK_PCIEX4_CORE, name: "pciex4_core_clk", parent_id: DP1000_CLK_SYSPLL, mult: 1, div: 2 },
    ultrarisc_fixed_factor_desc { id: DP1000_CLK_PCIEX16_CORE, name: "pciex16_core_clk", parent_id: DP1000_CLK_SYSPLL, mult: 1, div: 1 },
    ultrarisc_fixed_factor_desc { id: DP1000_CLK_PCIE_AUX, name: "pcie_aux_clk", parent_id: DP1000_CLK_SYSPLL, mult: 1, div: 40 },
];

static DP1000_DIVIDER_CLKS: [ultrarisc_divider_desc; 4] = [
    ultrarisc_divider_desc { id: DP1000_CLK_GMAC, name: "gmac_clk", offset: DP1000_CCR_GMAC_OFFSET, parent_id: DP1000_CLK_SYSPLL_DIV2, max_rate: 0, load_mask: DP1000_CCR_LOAD, div_shift: 8, div_width: 4, gate_bit: 0, divider_flags: CLK_DIVIDER_ONE_BASED, gate_flags: 0 },
    ultrarisc_divider_desc { id: DP1000_CLK_UART_ROOT, name: "uart_root_clk", offset: DP1000_CCR_UART_OFFSET, parent_id: DP1000_CLK_SUBSYS, max_rate: DP1000_PERI_MAX_RATE, load_mask: DP1000_CCR_LOAD, div_shift: 8, div_width: 4, gate_bit: 0, divider_flags: CLK_DIVIDER_ONE_BASED, gate_flags: 0 },
    ultrarisc_divider_desc { id: DP1000_CLK_I2C_ROOT, name: "i2c_root_clk", offset: DP1000_CCR_I2C_OFFSET, parent_id: DP1000_CLK_SUBSYS, max_rate: DP1000_PERI_MAX_RATE, load_mask: DP1000_CCR_LOAD, div_shift: 8, div_width: 4, gate_bit: 0, divider_flags: CLK_DIVIDER_ONE_BASED, gate_flags: 0 },
    ultrarisc_divider_desc { id: DP1000_CLK_SPI_ROOT, name: "spi_root_clk", offset: DP1000_CCR_SPI_OFFSET, parent_id: DP1000_CLK_SUBSYS, max_rate: DP1000_PERI_MAX_RATE, load_mask: DP1000_CCR_LOAD, div_shift: 8, div_width: 4, gate_bit: 0, divider_flags: CLK_DIVIDER_ONE_BASED, gate_flags: 0 },
];

static DP1000_GATE_CLKS: [ultrarisc_gate_desc; 10] = [
    ultrarisc_gate_desc { id: DP1000_CLK_UART0, name: "uart0_clk", offset: DP1000_PERI_CLKENA_OFFSET, parent_id: DP1000_CLK_UART_ROOT, gate_bit: 0, gate_flags: 0 },
    ultrarisc_gate_desc { id: DP1000_CLK_UART1, name: "uart1_clk", offset: DP1000_PERI_CLKENA_OFFSET, parent_id: DP1000_CLK_UART_ROOT, gate_bit: 1, gate_flags: 0 },
    ultrarisc_gate_desc { id: DP1000_CLK_UART2, name: "uart2_clk", offset: DP1000_PERI_CLKENA_OFFSET, parent_id: DP1000_CLK_UART_ROOT, gate_bit: 2, gate_flags: 0 },
    ultrarisc_gate_desc { id: DP1000_CLK_UART3, name: "uart3_clk", offset: DP1000_PERI_CLKENA_OFFSET, parent_id: DP1000_CLK_UART_ROOT, gate_bit: 3, gate_flags: 0 },
    ultrarisc_gate_desc { id: DP1000_CLK_I2C0, name: "i2c0_clk", offset: DP1000_PERI_CLKENA_OFFSET, parent_id: DP1000_CLK_I2C_ROOT, gate_bit: 4, gate_flags: 0 },
    ultrarisc_gate_desc { id: DP1000_CLK_I2C1, name: "i2c1_clk", offset: DP1000_PERI_CLKENA_OFFSET, parent_id: DP1000_CLK_I2C_ROOT, gate_bit: 5, gate_flags: 0 },
    ultrarisc_gate_desc { id: DP1000_CLK_I2C2, name: "i2c2_clk", offset: DP1000_PERI_CLKENA_OFFSET, parent_id: DP1000_CLK_I2C_ROOT, gate_bit: 6, gate_flags: 0 },
    ultrarisc_gate_desc { id: DP1000_CLK_I2C3, name: "i2c3_clk", offset: DP1000_PERI_CLKENA_OFFSET, parent_id: DP1000_CLK_I2C_ROOT, gate_bit: 7, gate_flags: 0 },
    ultrarisc_gate_desc { id: DP1000_CLK_SPI0, name: "spi0_clk", offset: DP1000_PERI_CLKENA_OFFSET, parent_id: DP1000_CLK_SPI_ROOT, gate_bit: 8, gate_flags: 0 },
    ultrarisc_gate_desc { id: DP1000_CLK_SPI1, name: "spi1_clk", offset: DP1000_PERI_CLKENA_OFFSET, parent_id: DP1000_CLK_SPI_ROOT, gate_bit: 9, gate_flags: 0 },
];

static DP1000_CLK_SOC_DATA: ultrarisc_clk_soc_data = ultrarisc_clk_soc_data {
    num_clks: DP1000_CLK_NUM,
    pll_layout: &DP1000_PLL_LAYOUT,
    plls: DP1000_PLLS.as_ptr(),
    num_plls: DP1000_PLLS.len(),
    fixed_factors: DP1000_FIXED_FACTOR_CLKS.as_ptr(),
    num_fixed_factors: DP1000_FIXED_FACTOR_CLKS.len(),
    dividers: DP1000_DIVIDER_CLKS.as_ptr(),
    num_dividers: DP1000_DIVIDER_CLKS.len(),
    gates: DP1000_GATE_CLKS.as_ptr(),
    num_gates: DP1000_GATE_CLKS.len(),
};

unsafe fn dp1000_clk_probe(pdev: *mut platform_device) -> i32 {
    ultrarisc_clk_probe(pdev, &DP1000_CLK_SOC_DATA)
}

static DP1000_CLK_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "ultrarisc,dp1000-clk" },
    of_device_id { compatible: "" },
];

static mut DP1000_CLK_DRIVER: platform_driver = platform_driver {
    probe: Some(dp1000_clk_probe),
    driver: driver {
        name: "ultrarisc-dp1000-clk",
        of_match_table: DP1000_CLK_OF_MATCH.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, dp1000_clk_of_match);
// module_platform_driver(dp1000_clk_driver);
// MODULE_IMPORT_NS("CLK_ULTRARISC");
// MODULE_DESCRIPTION("UltraRISC DP1000 clock controller");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
