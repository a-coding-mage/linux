// SPDX-License-Identifier: GPL-2.0+
/*
 * Amlogic Meson-AXG Clock Controller Driver
 *
 * Copyright (c) 2016 Baylibre SAS.
 * Author: Michael Turquette <mturquette@baylibre.com>
 *
 * Copyright (c) 2019 Baylibre SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 */

// Linux clock, platform, reset, syscon, module, Meson AO-clock, regmap,
// dual-divider, clock-binding, and reset-binding dependencies are supplied by
// the surrounding translation unit.

const AO_RTI_STATUS_REG3: u32 = 0x0c;
const AO_RTI_PWR_CNTL_REG0: u32 = 0x10;
const AO_RTI_GEN_CNTL_REG0: u32 = 0x40;
const AO_CLK_GATE0: u32 = 0x4c;
const AO_CLK_GATE0_SP: u32 = 0x50;
const AO_OSCIN_CNTL: u32 = 0x58;
const AO_CEC_CLK_CNTL_REG0: u32 = 0x74;
const AO_CEC_CLK_CNTL_REG1: u32 = 0x78;
const AO_SAR_CLK: u32 = 0x90;
const AO_RTC_ALT_CLK_CNTL0: u32 = 0x94;
const AO_RTC_ALT_CLK_CNTL1: u32 = 0x98;

static g12a_ao_pclk_parents: clk_parent_data = clk_parent_data { fw_name: "mpeg-clk" };

// The gates below retain CLK_IGNORE_UNUSED for historical compatibility.
macro_rules! G12A_AO_PCLK {
    ($name:ident, $reg:expr, $bit:expr, $flags:expr) => {
        static mut $name: clk_regmap = meson_pclk!($reg, $bit, &g12a_ao_pclk_parents, $flags);
    };
}

G12A_AO_PCLK!(g12a_ao_ahb, AO_CLK_GATE0, 0, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_ir_in, AO_CLK_GATE0, 1, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_i2c_m0, AO_CLK_GATE0, 2, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_i2c_s0, AO_CLK_GATE0, 3, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_uart, AO_CLK_GATE0, 4, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_prod_i2c, AO_CLK_GATE0, 5, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_uart2, AO_CLK_GATE0, 6, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_ir_out, AO_CLK_GATE0, 7, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_saradc, AO_CLK_GATE0, 8, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_mailbox, AO_CLK_GATE0_SP, 0, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_m3, AO_CLK_GATE0_SP, 1, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_ahb_sram, AO_CLK_GATE0_SP, 2, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_rti, AO_CLK_GATE0_SP, 3, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_m4_fclk, AO_CLK_GATE0_SP, 4, CLK_IGNORE_UNUSED);
G12A_AO_PCLK!(g12a_ao_m4_hclk, AO_CLK_GATE0_SP, 5, CLK_IGNORE_UNUSED);

static mut g12a_ao_cts_oscin: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: AO_RTI_PWR_CNTL_REG0, bit_idx: 14 },
    hw: clk_init_data { name: "cts_oscin", ops: &clk_regmap_gate_ro_ops,
        parent_data: &clk_parent_data { fw_name: "xtal" }, num_parents: 1 },
};

static g12a_32k_div_table: [meson_clk_dualdiv_param; 2] = [
    meson_clk_dualdiv_param { dual: 1, n1: 733, m1: 8, n2: 732, m2: 11 },
    meson_clk_dualdiv_param { dual: 0, n1: 0, m1: 0, n2: 0, m2: 0 },
];

// 32k_by_oscin clock
static mut g12a_ao_32k_by_oscin_pre: clk_regmap = clk_regmap_gate!("ao_32k_by_oscin_pre", AO_RTC_ALT_CLK_CNTL0, 31, &g12a_ao_cts_oscin.hw, clk_regmap_gate_ops);
static mut g12a_ao_32k_by_oscin_div: clk_regmap = meson_dualdiv!("ao_32k_by_oscin_div", AO_RTC_ALT_CLK_CNTL0, AO_RTC_ALT_CLK_CNTL1, 28, &g12a_ao_32k_by_oscin_pre.hw, &g12a_32k_div_table);
static mut g12a_ao_32k_by_oscin_sel: clk_regmap = clk_regmap_mux!("ao_32k_by_oscin_sel", AO_RTC_ALT_CLK_CNTL1, 0x1, 24, &g12a_ao_32k_by_oscin_div.hw, &g12a_ao_32k_by_oscin_pre.hw);
static mut g12a_ao_32k_by_oscin: clk_regmap = clk_regmap_gate!("ao_32k_by_oscin", AO_RTC_ALT_CLK_CNTL0, 30, &g12a_ao_32k_by_oscin_sel.hw, clk_regmap_gate_ops);

// cec clock
static mut g12a_ao_cec_pre: clk_regmap = clk_regmap_gate!("ao_cec_pre", AO_CEC_CLK_CNTL_REG0, 31, &g12a_ao_cts_oscin.hw, clk_regmap_gate_ops);
static mut g12a_ao_cec_div: clk_regmap = meson_dualdiv!("ao_cec_div", AO_CEC_CLK_CNTL_REG0, AO_CEC_CLK_CNTL_REG1, 28, &g12a_ao_cec_pre.hw, &g12a_32k_div_table);
static mut g12a_ao_cec_sel: clk_regmap = clk_regmap_mux!("ao_cec_sel", AO_CEC_CLK_CNTL_REG1, 0x1, 24, &g12a_ao_cec_div.hw, &g12a_ao_cec_pre.hw);
static mut g12a_ao_cec: clk_regmap = clk_regmap_gate!("ao_cec", AO_CEC_CLK_CNTL_REG0, 30, &g12a_ao_cec_sel.hw, clk_regmap_gate_ops);

static mut g12a_ao_cts_rtc_oscin: clk_regmap = clk_regmap_mux_data!("ao_cts_rtc_oscin", AO_RTI_PWR_CNTL_REG0, 0x1, 10,
    parent_data![hw(&g12a_ao_32k_by_oscin.hw), fw_name("ext-32k-0")], clk_regmap_mux_ops);
static mut g12a_ao_clk81: clk_regmap = clk_regmap_mux_data!("g12a_ao_clk81", AO_RTI_PWR_CNTL_REG0, 0x1, 8,
    parent_data![fw_name("mpeg-clk"), hw(&g12a_ao_cts_rtc_oscin.hw)], clk_regmap_mux_ro_ops);
static mut g12a_ao_saradc_mux: clk_regmap = clk_regmap_mux_data!("ao_saradc_mux", AO_SAR_CLK, 0x3, 9,
    parent_data![fw_name("xtal"), hw(&g12a_ao_clk81.hw)], clk_regmap_mux_ops);
static mut g12a_ao_saradc_div: clk_regmap = clk_regmap_div!("ao_saradc_div", AO_SAR_CLK, 0, 8, &g12a_ao_saradc_mux.hw);
static mut g12a_ao_saradc_gate: clk_regmap = clk_regmap_gate!("ao_saradc_gate", AO_SAR_CLK, 8, &g12a_ao_saradc_div.hw, clk_regmap_gate_ops);

static g12a_ao_reset: [u32; 24] = [
    /* RESET_AO_IR_IN */ 16, /* RESET_AO_UART */ 17, /* RESET_AO_I2C_M */ 18,
    /* RESET_AO_I2C_S */ 19, /* RESET_AO_SAR_ADC */ 20, 0, /* RESET_AO_UART2 */ 22,
    /* RESET_AO_IR_OUT */ 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

static mut g12a_ao_hw_clks: [*mut clk_hw; 29] = [
    &mut g12a_ao_ahb.hw, &mut g12a_ao_ir_in.hw, &mut g12a_ao_i2c_m0.hw,
    &mut g12a_ao_i2c_s0.hw, &mut g12a_ao_uart.hw, &mut g12a_ao_prod_i2c.hw,
    &mut g12a_ao_uart2.hw, &mut g12a_ao_ir_out.hw, &mut g12a_ao_saradc.hw,
    &mut g12a_ao_mailbox.hw, &mut g12a_ao_m3.hw, &mut g12a_ao_ahb_sram.hw,
    &mut g12a_ao_rti.hw, &mut g12a_ao_m4_fclk.hw, &mut g12a_ao_m4_hclk.hw,
    &mut g12a_ao_clk81.hw, &mut g12a_ao_saradc_mux.hw, &mut g12a_ao_saradc_div.hw,
    &mut g12a_ao_saradc_gate.hw, &mut g12a_ao_cts_oscin.hw,
    &mut g12a_ao_32k_by_oscin_pre.hw, &mut g12a_ao_32k_by_oscin_div.hw,
    &mut g12a_ao_32k_by_oscin_sel.hw, &mut g12a_ao_32k_by_oscin.hw,
    &mut g12a_ao_cec_pre.hw, &mut g12a_ao_cec_div.hw, &mut g12a_ao_cec_sel.hw,
    &mut g12a_ao_cec.hw, &mut g12a_ao_cts_rtc_oscin.hw,
];

static g12a_ao_clkc_data: meson_aoclk_data = meson_aoclk_data {
    reset_reg: AO_RTI_GEN_CNTL_REG0, num_reset: g12a_ao_reset.len(), reset: &g12a_ao_reset,
    clkc_data: meson_clkc_data { hw_clks: meson_hw_clks { hws: &g12a_ao_hw_clks, num: g12a_ao_hw_clks.len() } },
};

static g12a_ao_clkc_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "amlogic,meson-g12a-aoclkc", data: &g12a_ao_clkc_data.clkc_data },
    of_device_id::default(),
];

static mut g12a_ao_clkc_driver: platform_driver = platform_driver {
    probe: Some(meson_aoclkc_probe),
    driver: driver { name: "g12a-aoclkc", of_match_table: &g12a_ao_clkc_match_table },
};

module_platform_driver!(g12a_ao_clkc_driver);
module_description!("Amlogic G12A Always-ON Clock Controller driver");
module_license!("GPL");
module_import_ns!("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
