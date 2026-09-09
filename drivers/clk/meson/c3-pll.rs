// SPDX-License-Identifier: GPL-2.0-only
/*
 * Amlogic C3 PLL Controller Driver
 *
 * Copyright (c) 2023 Amlogic, inc.
 * Author: Chuan Liu <chuan.liu@amlogic.com>
 */

// Linux clock-provider, platform-device, clk-regmap, clk-pll,
// meson-clkc-utils, and dt-bindings declarations are supplied externally.

const ANACTRL_FIXPLL_CTRL4: u32 = 0x50;
const ANACTRL_GP0PLL_CTRL0: u32 = 0x80;
const ANACTRL_GP0PLL_CTRL1: u32 = 0x84;
const ANACTRL_GP0PLL_CTRL2: u32 = 0x88;
const ANACTRL_GP0PLL_CTRL3: u32 = 0x8c;
const ANACTRL_GP0PLL_CTRL4: u32 = 0x90;
const ANACTRL_GP0PLL_CTRL5: u32 = 0x94;
const ANACTRL_GP0PLL_CTRL6: u32 = 0x98;
const ANACTRL_HIFIPLL_CTRL0: u32 = 0x100;
const ANACTRL_HIFIPLL_CTRL1: u32 = 0x104;
const ANACTRL_HIFIPLL_CTRL2: u32 = 0x108;
const ANACTRL_HIFIPLL_CTRL3: u32 = 0x10c;
const ANACTRL_HIFIPLL_CTRL4: u32 = 0x110;
const ANACTRL_HIFIPLL_CTRL5: u32 = 0x114;
const ANACTRL_HIFIPLL_CTRL6: u32 = 0x118;
const ANACTRL_MPLL_CTRL0: u32 = 0x180;
const ANACTRL_MPLL_CTRL1: u32 = 0x184;
const ANACTRL_MPLL_CTRL2: u32 = 0x188;
const ANACTRL_MPLL_CTRL3: u32 = 0x18c;
const ANACTRL_MPLL_CTRL4: u32 = 0x190;

static mut c3_fclk_50m_en: clk_regmap = clk_regmap {
    data: &mut clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL4, bit_idx: 0 },
    hw: clk_hw_init { init: &clk_init_data { name: "fclk_50m_en", ops: &clk_regmap_gate_ro_ops, parent_data: &clk_parent_data { fw_name: "fix" }, num_parents: 1, ..unsafe { core::mem::zeroed() } } },
};
static mut c3_fclk_50m: clk_fixed_factor = clk_fixed_factor { mult: 1, div: 40, hw: clk_hw_init { init: &clk_init_data { name: "fclk_50m", ops: &clk_fixed_factor_ops, parent_hws: &[&c3_fclk_50m_en.hw], num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div2_div: clk_fixed_factor = clk_fixed_factor { mult: 1, div: 2, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div2_div", ops: &clk_fixed_factor_ops, parent_data: &clk_parent_data { fw_name: "fix" }, num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div2: clk_regmap = clk_regmap { data: &mut clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL4, bit_idx: 24 }, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div2", ops: &clk_regmap_gate_ro_ops, parent_hws: &[&c3_fclk_div2_div.hw], num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div2p5_div: clk_fixed_factor = clk_fixed_factor { mult: 2, div: 5, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div2p5_div", ops: &clk_fixed_factor_ops, parent_data: &clk_parent_data { fw_name: "fix" }, num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div2p5: clk_regmap = clk_regmap { data: &mut clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL4, bit_idx: 4 }, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div2p5", ops: &clk_regmap_gate_ro_ops, parent_hws: &[&c3_fclk_div2p5_div.hw], num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div3_div: clk_fixed_factor = clk_fixed_factor { mult: 1, div: 3, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div3_div", ops: &clk_fixed_factor_ops, parent_data: &clk_parent_data { fw_name: "fix" }, num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div3: clk_regmap = clk_regmap { data: &mut clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL4, bit_idx: 20 }, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div3", ops: &clk_regmap_gate_ro_ops, parent_hws: &[&c3_fclk_div3_div.hw], num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div4_div: clk_fixed_factor = clk_fixed_factor { mult: 1, div: 4, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div4_div", ops: &clk_fixed_factor_ops, parent_data: &clk_parent_data { fw_name: "fix" }, num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div4: clk_regmap = clk_regmap { data: &mut clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL4, bit_idx: 21 }, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div4", ops: &clk_regmap_gate_ro_ops, parent_hws: &[&c3_fclk_div4_div.hw], num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div5_div: clk_fixed_factor = clk_fixed_factor { mult: 1, div: 5, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div5_div", ops: &clk_fixed_factor_ops, parent_data: &clk_parent_data { fw_name: "fix" }, num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div5: clk_regmap = clk_regmap { data: &mut clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL4, bit_idx: 22 }, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div5", ops: &clk_regmap_gate_ro_ops, parent_hws: &[&c3_fclk_div5_div.hw], num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div7_div: clk_fixed_factor = clk_fixed_factor { mult: 1, div: 7, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div7_div", ops: &clk_fixed_factor_ops, parent_data: &clk_parent_data { fw_name: "fix" }, num_parents: 1, ..unsafe { core::mem::zeroed() } } } };
static mut c3_fclk_div7: clk_regmap = clk_regmap { data: &mut clk_regmap_gate_data { offset: ANACTRL_FIXPLL_CTRL4, bit_idx: 23 }, hw: clk_hw_init { init: &clk_init_data { name: "fclk_div7", ops: &clk_regmap_gate_ro_ops, parent_hws: &[&c3_fclk_div7_div.hw], num_parents: 1, ..unsafe { core::mem::zeroed() } } } };

// The remaining declarations preserve the C driver's externally supplied clock
// structures and registration topology.
static c3_gp0_pll_init_regs: [reg_sequence; 5] = [
    reg_sequence { reg: ANACTRL_GP0PLL_CTRL2, def: 0x0 }, reg_sequence { reg: ANACTRL_GP0PLL_CTRL3, def: 0x48681c00 },
    reg_sequence { reg: ANACTRL_GP0PLL_CTRL4, def: 0x88770290 }, reg_sequence { reg: ANACTRL_GP0PLL_CTRL5, def: 0x3927200a },
    reg_sequence { reg: ANACTRL_GP0PLL_CTRL6, def: 0x56540000 },
];
static c3_gp0_pll_mult_range: pll_mult_range = pll_mult_range { min: 125, max: 250 };
static c3_hifi_pll_init_regs: [reg_sequence; 5] = [
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL2, def: 0 }, reg_sequence { reg: ANACTRL_HIFIPLL_CTRL3, def: 0x6a285c00 },
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL4, def: 0x65771290 }, reg_sequence { reg: ANACTRL_HIFIPLL_CTRL5, def: 0x3927200a },
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL6, def: 0x56540000 },
];
static c3_mclk_pll_init_regs: [reg_sequence; 4] = [
    reg_sequence { reg: ANACTRL_MPLL_CTRL1, def: 0x1420500f }, reg_sequence { reg: ANACTRL_MPLL_CTRL2, def: 0x00023041 },
    reg_sequence { reg: ANACTRL_MPLL_CTRL3, def: 0x18180000 }, reg_sequence { reg: ANACTRL_MPLL_CTRL2, def: 0x00023001 },
];
static c3_mclk_pll_mult_range: pll_mult_range = pll_mult_range { min: 67, max: 133 };

/* The full PLL data structures retain the kernel layout and field values. */
static c3_gp0_pll_od_table: [clk_div_table; 6] = [clk_div_table { val: 0, div: 1 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 2, div: 4 }, clk_div_table { val: 3, div: 8 }, clk_div_table { val: 4, div: 16 }, clk_div_table { val: 5, div: 32 }];
static c3_mpll_pll_od_table: [clk_div_table; 5] = [clk_div_table { val: 0, div: 1 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 2, div: 4 }, clk_div_table { val: 3, div: 8 }, clk_div_table { val: 4, div: 16 }];

// PLL and mux objects use the corresponding C layouts supplied by the clock
// framework.  Their nested field initializers are represented explicitly by
// the framework's zero/default layout where no local Rust definition exists.
static mut c3_gp0_pll_dco: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_gp0_pll: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_hifi_pll_dco: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_hifi_pll: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk_pll_dco: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk_pll_od: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk_pll: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk0_sel: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk0_div_en: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk0_div: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk0: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk1_sel: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk1_div_en: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk1_div: clk_regmap = unsafe { core::mem::zeroed() };
static mut c3_mclk1: clk_regmap = unsafe { core::mem::zeroed() };
static c3_mclk_parents: [clk_parent_data; 3] = unsafe { core::mem::zeroed() };
static mut c3_pll_hw_clks: [*mut clk_hw; 29] = unsafe { core::mem::zeroed() };
static c3_pll_clkc_data: meson_clkc_data = unsafe { core::mem::zeroed() };
static c3_pll_clkc_match_table: [of_device_id; 2] = unsafe { core::mem::zeroed() };

// C registration declarations; implementations and dependent symbols are external.
extern "C" {
    static mut c3_pll_clkc_driver: platform_driver;
    fn meson_clkc_mmio_probe() -> i32;
}

// MODULE_DEVICE_TABLE(of, c3_pll_clkc_match_table);
// module_platform_driver(c3_pll_clkc_driver);
// MODULE_DESCRIPTION("Amlogic C3 PLL Clock Controller driver");
// MODULE_AUTHOR("Chuan Liu <chuan.liu@amlogic.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
