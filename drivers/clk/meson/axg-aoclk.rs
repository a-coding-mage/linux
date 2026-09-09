// SPDX-License-Identifier: GPL-2.0+
/*
 * Amlogic Meson-AXG Clock Controller Driver
 *
 * Copyright (c) 2016 Baylibre SAS.
 * Author: Michael Turquette <mturquette@baylibre.com>
 *
 * Copyright (c) 2018 Amlogic, inc.
 * Author: Qiufang Dai <qiufang.dai@amlogic.com>
 */
// Dependencies supplied by the surrounding kernel translation are intentionally external.

const AO_RTI_PWR_CNTL_REG1: u32 = 0x0C;
const AO_RTI_PWR_CNTL_REG0: u32 = 0x10;
const AO_RTI_GEN_CNTL_REG0: u32 = 0x40;
const AO_OSCIN_CNTL: u32 = 0x58;
const AO_CRT_CLK_CNTL1: u32 = 0x68;
const AO_SAR_CLK: u32 = 0x90;
const AO_RTC_ALT_CLK_CNTL0: u32 = 0x94;
const AO_RTC_ALT_CLK_CNTL1: u32 = 0x98;

static axg_ao_pclk_parents: clk_parent_data = clk_parent_data { fw_name: "mpeg-clk" };

macro_rules! AXG_AO_GATE {
    ($name:ident, $bit:expr, $flags:expr) => {
        static mut axg_ao_$name: clk_regmap = MESON_PCLK!(axg_ao_$name, AO_RTI_GEN_CNTL_REG0, $bit, &axg_ao_pclk_parents, $flags);
    };
}

AXG_AO_GATE!(remote, 0, CLK_IGNORE_UNUSED);
AXG_AO_GATE!(i2c_master, 1, CLK_IGNORE_UNUSED);
AXG_AO_GATE!(i2c_slave, 2, CLK_IGNORE_UNUSED);
AXG_AO_GATE!(uart1, 3, CLK_IGNORE_UNUSED);
AXG_AO_GATE!(uart2, 5, CLK_IGNORE_UNUSED);
AXG_AO_GATE!(ir_blaster, 6, CLK_IGNORE_UNUSED);
AXG_AO_GATE!(saradc, 7, CLK_IGNORE_UNUSED);

static mut axg_ao_cts_oscin: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: AO_RTI_PWR_CNTL_REG0, bit_idx: 14 },
    hw: clk_hw { init: &clk_init_data { name: "cts_oscin", ops: &clk_regmap_gate_ro_ops, parent_data: &clk_parent_data { fw_name: "xtal" }, num_parents: 1 } },
};
static mut axg_ao_32k_pre: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: AO_RTC_ALT_CLK_CNTL0, bit_idx: 31 },
    hw: clk_hw { init: &clk_init_data { name: "axg_ao_32k_pre", ops: &clk_regmap_gate_ops, parent_hws: &[&axg_ao_cts_oscin.hw], num_parents: 1 } },
};
static axg_32k_div_table: [meson_clk_dualdiv_param; 2] = [
    meson_clk_dualdiv_param { dual: 1, n1: 733, m1: 8, n2: 732, m2: 11 },
    meson_clk_dualdiv_param { ..Default::default() },
];
static mut axg_ao_32k_div: clk_regmap = clk_regmap {
    data: &meson_clk_dualdiv_data {
        n1: meson_clk_dualdiv_cfg { reg_off: AO_RTC_ALT_CLK_CNTL0, shift: 0, width: 12 },
        n2: meson_clk_dualdiv_cfg { reg_off: AO_RTC_ALT_CLK_CNTL0, shift: 12, width: 12 },
        m1: meson_clk_dualdiv_cfg { reg_off: AO_RTC_ALT_CLK_CNTL1, shift: 0, width: 12 },
        m2: meson_clk_dualdiv_cfg { reg_off: AO_RTC_ALT_CLK_CNTL1, shift: 12, width: 12 },
        dual: meson_clk_dualdiv_cfg { reg_off: AO_RTC_ALT_CLK_CNTL0, shift: 28, width: 1 },
        table: &axg_32k_div_table,
    },
    hw: clk_hw { init: &clk_init_data { name: "axg_ao_32k_div", ops: &meson_clk_dualdiv_ops, parent_hws: &[&axg_ao_32k_pre.hw], num_parents: 1 } },
};
static mut axg_ao_32k_sel: clk_regmap = clk_regmap {
    data: &clk_regmap_mux_data { offset: AO_RTC_ALT_CLK_CNTL1, mask: 0x1, shift: 24, flags: CLK_MUX_ROUND_CLOSEST },
    hw: clk_hw { init: &clk_init_data { name: "axg_ao_32k_sel", ops: &clk_regmap_mux_ops, parent_hws: &[&axg_ao_32k_div.hw, &axg_ao_32k_pre.hw], num_parents: 2, flags: CLK_SET_RATE_PARENT } },
};
static mut axg_ao_32k: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: AO_RTC_ALT_CLK_CNTL0, bit_idx: 30 },
    hw: clk_hw { init: &clk_init_data { name: "axg_ao_32k", ops: &clk_regmap_gate_ops, parent_hws: &[&axg_ao_32k_sel.hw], num_parents: 1, flags: CLK_SET_RATE_PARENT } },
};
static mut axg_ao_cts_rtc_oscin: clk_regmap = clk_regmap {
    data: &clk_regmap_mux_data { offset: AO_RTI_PWR_CNTL_REG0, mask: 0x1, shift: 10, flags: CLK_MUX_ROUND_CLOSEST },
    hw: clk_hw { init: &clk_init_data { name: "axg_ao_cts_rtc_oscin", ops: &clk_regmap_mux_ops, parent_data: &[clk_parent_data { hw: &axg_ao_32k.hw }, clk_parent_data { fw_name: "ext_32k-0" }], num_parents: 2, flags: CLK_SET_RATE_PARENT } },
};
static mut axg_ao_clk81: clk_regmap = clk_regmap {
    data: &clk_regmap_mux_data { offset: AO_RTI_PWR_CNTL_REG0, mask: 0x1, shift: 8, flags: CLK_MUX_ROUND_CLOSEST },
    hw: clk_hw { init: &clk_init_data {
        /* NOTE: this is one of the infamous clock the pwm driver can request directly by its global name. */
        name: "axg_ao_clk81", ops: &clk_regmap_mux_ro_ops,
        parent_data: &[clk_parent_data { fw_name: "mpeg-clk" }, clk_parent_data { hw: &axg_ao_cts_rtc_oscin.hw }], num_parents: 2, flags: CLK_SET_RATE_PARENT,
    } },
};
static mut axg_ao_saradc_mux: clk_regmap = clk_regmap {
    data: &clk_regmap_mux_data { offset: AO_SAR_CLK, mask: 0x3, shift: 9 },
    hw: clk_hw { init: &clk_init_data { name: "ao_saradc_mux", ops: &clk_regmap_mux_ops, parent_data: &[clk_parent_data { fw_name: "xtal" }, clk_parent_data { hw: &axg_ao_clk81.hw }], num_parents: 2 } },
};
static mut axg_ao_saradc_div: clk_regmap = clk_regmap {
    data: &clk_regmap_div_data { offset: AO_SAR_CLK, shift: 0, width: 8 },
    hw: clk_hw { init: &clk_init_data { name: "ao_saradc_div", ops: &clk_regmap_divider_ops, parent_hws: &[&axg_ao_saradc_mux.hw], num_parents: 1, flags: CLK_SET_RATE_PARENT } },
};
static mut axg_ao_saradc_gate: clk_regmap = clk_regmap {
    data: &clk_regmap_gate_data { offset: AO_SAR_CLK, bit_idx: 8 },
    hw: clk_hw { init: &clk_init_data { name: "ao_saradc_gate", ops: &clk_regmap_gate_ops, parent_hws: &[&axg_ao_saradc_div.hw], num_parents: 1, flags: CLK_SET_RATE_PARENT } },
};

static axg_ao_reset: [u32; 24] = {
    let mut r = [0; 24];
    r[RESET_AO_REMOTE as usize] = 16; r[RESET_AO_I2C_MASTER as usize] = 18;
    r[RESET_AO_I2C_SLAVE as usize] = 19; r[RESET_AO_UART1 as usize] = 17;
    r[RESET_AO_UART2 as usize] = 22; r[RESET_AO_IR_BLASTER as usize] = 23; r
};
static mut axg_ao_hw_clks: [*mut clk_hw; 17] = [
    &mut axg_ao_remote.hw, &mut axg_ao_i2c_master.hw, &mut axg_ao_i2c_slave.hw,
    &mut axg_ao_uart1.hw, &mut axg_ao_uart2.hw, &mut axg_ao_ir_blaster.hw,
    &mut axg_ao_saradc.hw, &mut axg_ao_clk81.hw, &mut axg_ao_saradc_mux.hw,
    &mut axg_ao_saradc_div.hw, &mut axg_ao_saradc_gate.hw, &mut axg_ao_cts_oscin.hw,
    &mut axg_ao_32k_pre.hw, &mut axg_ao_32k_div.hw, &mut axg_ao_32k_sel.hw,
    &mut axg_ao_32k.hw, &mut axg_ao_cts_rtc_oscin.hw,
];
static axg_ao_clkc_data: meson_aoclk_data = meson_aoclk_data {
    reset_reg: AO_RTI_GEN_CNTL_REG0, num_reset: axg_ao_reset.len(), reset: &axg_ao_reset,
    clkc_data: meson_clk_hw_data { hw_clks: clk_hw_onecell_data { hws: &axg_ao_hw_clks, num: axg_ao_hw_clks.len() } },
};
static axg_ao_clkc_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "amlogic,meson-axg-aoclkc", data: &axg_ao_clkc_data.clkc_data }, of_device_id::empty(),
];
MODULE_DEVICE_TABLE!(of, axg_ao_clkc_match_table);
static axg_ao_clkc_driver: platform_driver = platform_driver {
    probe: meson_aoclkc_probe,
    driver: device_driver { name: "axg-ao-clkc", of_match_table: &axg_ao_clkc_match_table },
};
module_platform_driver!(axg_ao_clkc_driver);
MODULE_DESCRIPTION!("Amlogic AXG Always-ON Clock Controller driver");
MODULE_LICENSE!("GPL");
MODULE_IMPORT_NS!("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
