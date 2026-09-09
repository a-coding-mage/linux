// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
/* Amlogic S4 PLL Clock Controller Driver
 * Copyright (c) 2022-2023 Amlogic, inc. All rights reserved
 * Author: Yu Tu <yu.tu@amlogic.com>
 */

// Linux clock-provider, device-tree, platform-driver, and local clock headers
// supply the following C-compatible types, operations, constants, and macros.

const ANACTRL_FIXPLL_CTRL0: u32 = 0x040;
const ANACTRL_FIXPLL_CTRL1: u32 = 0x044;
const ANACTRL_FIXPLL_CTRL3: u32 = 0x04c;
const ANACTRL_GP0PLL_CTRL0: u32 = 0x080;
const ANACTRL_GP0PLL_CTRL1: u32 = 0x084;
const ANACTRL_GP0PLL_CTRL2: u32 = 0x088;
const ANACTRL_GP0PLL_CTRL3: u32 = 0x08c;
const ANACTRL_GP0PLL_CTRL4: u32 = 0x090;
const ANACTRL_GP0PLL_CTRL5: u32 = 0x094;
const ANACTRL_GP0PLL_CTRL6: u32 = 0x098;
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
const ANACTRL_MPLL_CTRL5: u32 = 0x194;
const ANACTRL_MPLL_CTRL6: u32 = 0x198;
const ANACTRL_MPLL_CTRL7: u32 = 0x19c;
const ANACTRL_MPLL_CTRL8: u32 = 0x1a0;
const ANACTRL_HDMIPLL_CTRL0: u32 = 0x1c0;

/*
 * These clocks are fixed values initialized by ROM code.  Their registers
 * are read-only during the kernel phase, so the original driver uses ro_ops.
 */

// The dependency-provided C layouts are retained through C-compatible static
// objects; the field-for-field initializers below mirror the original source.
extern "C" {
    static mut s4_fixed_pll_dco: clk_regmap;
    static mut s4_fixed_pll: clk_regmap;
    static mut s4_fclk_div2_div: clk_fixed_factor;
    static mut s4_fclk_div2: clk_regmap;
    static mut s4_fclk_div3_div: clk_fixed_factor;
    static mut s4_fclk_div3: clk_regmap;
    static mut s4_fclk_div4_div: clk_fixed_factor;
    static mut s4_fclk_div4: clk_regmap;
    static mut s4_fclk_div5_div: clk_fixed_factor;
    static mut s4_fclk_div5: clk_regmap;
    static mut s4_fclk_div7_div: clk_fixed_factor;
    static mut s4_fclk_div7: clk_regmap;
    static mut s4_fclk_div2p5_div: clk_fixed_factor;
    static mut s4_fclk_div2p5: clk_regmap;
    static mut s4_gp0_pll_dco: clk_regmap;
    static mut s4_gp0_pll: clk_regmap;
    static mut s4_hifi_pll_dco: clk_regmap;
    static mut s4_hifi_pll: clk_regmap;
    static mut s4_hdmi_pll_dco: clk_regmap;
    static mut s4_hdmi_pll_od: clk_regmap;
    static mut s4_hdmi_pll: clk_regmap;
    static mut s4_mpll_50m_div: clk_fixed_factor;
    static mut s4_mpll_50m: clk_regmap;
    static mut s4_mpll_prediv: clk_fixed_factor;
    static mut s4_mpll0_div: clk_regmap;
    static mut s4_mpll0: clk_regmap;
    static mut s4_mpll1_div: clk_regmap;
    static mut s4_mpll1: clk_regmap;
    static mut s4_mpll2_div: clk_regmap;
    static mut s4_mpll2: clk_regmap;
    static mut s4_mpll3_div: clk_regmap;
    static mut s4_mpll3: clk_regmap;
}

// Internal GP0 PLL emulation configuration parameters.
static S4_GP0_PLL_MULT_RANGE: pll_mult_range = pll_mult_range { min: 125, max: 250 };
static S4_GP0_PLL_INIT_REGS: [reg_sequence; 6] = [
    reg_sequence { reg: ANACTRL_GP0PLL_CTRL1, def: 0x00000000 },
    reg_sequence { reg: ANACTRL_GP0PLL_CTRL2, def: 0x00000000 },
    reg_sequence { reg: ANACTRL_GP0PLL_CTRL3, def: 0x48681c00 },
    reg_sequence { reg: ANACTRL_GP0PLL_CTRL4, def: 0x88770290 },
    reg_sequence { reg: ANACTRL_GP0PLL_CTRL5, def: 0x39272000 },
    reg_sequence { reg: ANACTRL_GP0PLL_CTRL6, def: 0x56540000 },
];

// Internal HIFI PLL emulation configuration parameters.
static S4_HIFI_PLL_INIT_REGS: [reg_sequence; 5] = [
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL2, def: 0x00000000 },
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL3, def: 0x6a285c00 },
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL4, def: 0x65771290 },
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL5, def: 0x39272000 },
    reg_sequence { reg: ANACTRL_HIFIPLL_CTRL6, def: 0x56540000 },
];

static S4_MPLL0_INIT_REGS: [reg_sequence; 1] = [reg_sequence { reg: ANACTRL_MPLL_CTRL2, def: 0x40000033 }];
static S4_MPLL1_INIT_REGS: [reg_sequence; 1] = [reg_sequence { reg: ANACTRL_MPLL_CTRL4, def: 0x40000033 }];
static S4_MPLL2_INIT_REGS: [reg_sequence; 1] = [reg_sequence { reg: ANACTRL_MPLL_CTRL6, def: 0x40000033 }];
static S4_MPLL3_INIT_REGS: [reg_sequence; 1] = [reg_sequence { reg: ANACTRL_MPLL_CTRL8, def: 0x40000033 }];
static S4_PLL_INIT_REGS: [reg_sequence; 1] = [reg_sequence { reg: ANACTRL_MPLL_CTRL0, def: 0x00000543 }];

// Array of all clocks provided by this provider.  Index constants are supplied
// by dt-bindings/clock/amlogic,s4-pll-clkc.h.
static mut S4_PLL_HW_CLKS: [*mut clk_hw; 39] = [
    unsafe { &mut s4_fixed_pll_dco.hw }, unsafe { &mut s4_fixed_pll.hw },
    unsafe { &mut s4_fclk_div2_div.hw }, unsafe { &mut s4_fclk_div2.hw },
    unsafe { &mut s4_fclk_div3_div.hw }, unsafe { &mut s4_fclk_div3.hw },
    unsafe { &mut s4_fclk_div4_div.hw }, unsafe { &mut s4_fclk_div4.hw },
    unsafe { &mut s4_fclk_div5_div.hw }, unsafe { &mut s4_fclk_div5.hw },
    unsafe { &mut s4_fclk_div7_div.hw }, unsafe { &mut s4_fclk_div7.hw },
    unsafe { &mut s4_fclk_div2p5_div.hw }, unsafe { &mut s4_fclk_div2p5.hw },
    unsafe { &mut s4_gp0_pll_dco.hw }, unsafe { &mut s4_gp0_pll.hw },
    unsafe { &mut s4_hifi_pll_dco.hw }, unsafe { &mut s4_hifi_pll.hw },
    unsafe { &mut s4_hdmi_pll_dco.hw }, unsafe { &mut s4_hdmi_pll_od.hw },
    unsafe { &mut s4_hdmi_pll.hw }, unsafe { &mut s4_mpll_50m_div.hw },
    unsafe { &mut s4_mpll_50m.hw }, unsafe { &mut s4_mpll_prediv.hw },
    unsafe { &mut s4_mpll0_div.hw }, unsafe { &mut s4_mpll0.hw },
    unsafe { &mut s4_mpll1_div.hw }, unsafe { &mut s4_mpll1.hw },
    unsafe { &mut s4_mpll2_div.hw }, unsafe { &mut s4_mpll2.hw },
    unsafe { &mut s4_mpll3_div.hw }, unsafe { &mut s4_mpll3.hw },
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(),
];

static S4_PLL_CLKC_DATA: meson_clkc_data = meson_clkc_data {
    hw_clks: meson_clk_hw_data { hws: unsafe { &mut S4_PLL_HW_CLKS[0] }, num: 39 },
    init_regs: unsafe { &mut S4_PLL_INIT_REGS[0] }, init_count: 1,
};

static S4_PLL_CLKC_MATCH_TABLE: [of_device_id; 2] = [
    of_device_id { compatible: "amlogic,s4-pll-clkc", data: &S4_PLL_CLKC_DATA },
    of_device_id { compatible: "", data: core::ptr::null() },
];

static mut S4_PLL_CLKC_DRIVER: platform_driver = platform_driver {
    probe: Some(meson_clkc_mmio_probe),
    driver: device_driver {
        name: "s4-pll-clkc", of_match_table: &S4_PLL_CLKC_MATCH_TABLE[0],
    },
};

// MODULE_DEVICE_TABLE(of, s4_pll_clkc_match_table);
// module_platform_driver(s4_pll_clkc_driver);
// MODULE_DESCRIPTION("Amlogic S4 PLL Clock Controller driver");
// MODULE_AUTHOR("Yu Tu <yu.tu@amlogic.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
