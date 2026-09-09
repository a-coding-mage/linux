// SPDX-License-Identifier: GPL-2.0
/*
 * PLL clock descriptions for TI DA850/OMAP-L138/AM18XX
 *
 * Copyright (C) 2018 David Lechner <david@lechnology.com>
 */

// Linux dependencies and "pll.h" are supplied by other translation units.

const OCSEL_OCSRC_OSCIN: u32 = 0x14;
const OCSEL_OCSRC_PLL1_OBSCLK: u32 = 0x1e;

const fn ocsel_ocsrc_pll0_sysclk(n: u32) -> u32 { 0x16 + n }
const fn ocsel_ocsrc_pll1_sysclk(n: u32) -> u32 { 0x16 + n }

static da850_pll0_info: davinci_pll_clk_info = davinci_pll_clk_info {
    name: "pll0",
    unlock_reg: CFGCHIP(0),
    unlock_mask: CFGCHIP0_PLL_MASTER_LOCK,
    pllm_mask: GENMASK(4, 0),
    pllm_min: 4,
    pllm_max: 32,
    pllout_min_rate: 300000000,
    pllout_max_rate: 600000000,
    flags: PLL_HAS_CLKMODE | PLL_HAS_PREDIV | PLL_HAS_POSTDIV | PLL_HAS_EXTCLKSRC,
};

/*
 * NB: Technically, the clocks flagged as SYSCLK_FIXED_DIV are "fixed ratio",
 * meaning that we could change the divider as long as we keep the correct
 * ratio between all of the clocks, but we don't support that because there is
 * currently not a need for it.
 */

SYSCLK!(1, pll0_sysclk1, pll0_pllen, 5, SYSCLK_FIXED_DIV);
SYSCLK!(2, pll0_sysclk2, pll0_pllen, 5, SYSCLK_FIXED_DIV);
SYSCLK!(3, pll0_sysclk3, pll0_pllen, 5, 0);
SYSCLK!(4, pll0_sysclk4, pll0_pllen, 5, SYSCLK_FIXED_DIV);
SYSCLK!(5, pll0_sysclk5, pll0_pllen, 5, 0);
SYSCLK!(6, pll0_sysclk6, pll0_pllen, 5, SYSCLK_ARM_RATE | SYSCLK_FIXED_DIV);
SYSCLK!(7, pll0_sysclk7, pll0_pllen, 5, 0);

static da850_pll0_obsclk_parent_names: [&str; 9] = [
    "oscin", "pll0_sysclk1", "pll0_sysclk2", "pll0_sysclk3",
    "pll0_sysclk4", "pll0_sysclk5", "pll0_sysclk6", "pll0_sysclk7",
    "pll1_obsclk",
];

static mut da850_pll0_obsclk_table: [u32; 9] = [
    OCSEL_OCSRC_OSCIN,
    ocsel_ocsrc_pll0_sysclk(1), ocsel_ocsrc_pll0_sysclk(2),
    ocsel_ocsrc_pll0_sysclk(3), ocsel_ocsrc_pll0_sysclk(4),
    ocsel_ocsrc_pll0_sysclk(5), ocsel_ocsrc_pll0_sysclk(6),
    ocsel_ocsrc_pll0_sysclk(7), OCSEL_OCSRC_PLL1_OBSCLK,
];

static da850_pll0_obsclk_info: davinci_pll_obsclk_info = davinci_pll_obsclk_info {
    name: "pll0_obsclk",
    parent_names: &da850_pll0_obsclk_parent_names,
    num_parents: da850_pll0_obsclk_parent_names.len(),
    table: unsafe { &da850_pll0_obsclk_table },
    ocsrc_mask: GENMASK(4, 0),
};

unsafe fn da850_pll0_init(dev: *mut device, base: *mut core::ffi::c_void, cfgchip: *mut regmap) -> i32 {
    let mut clk: *mut clk;
    davinci_pll_clk_register(dev, &da850_pll0_info, "ref_clk", base, cfgchip);
    clk = davinci_pll_sysclk_register(dev, &pll0_sysclk1, base);
    clk_register_clkdev(clk, "pll0_sysclk1", "da850-psc0");
    clk = davinci_pll_sysclk_register(dev, &pll0_sysclk2, base);
    clk_register_clkdev(clk, "pll0_sysclk2", "da850-psc0");
    clk_register_clkdev(clk, "pll0_sysclk2", "da850-psc1");
    clk_register_clkdev(clk, "pll0_sysclk2", "da850-async3-clksrc");
    clk = davinci_pll_sysclk_register(dev, &pll0_sysclk3, base);
    clk_register_clkdev(clk, "pll0_sysclk3", "da850-async1-clksrc");
    clk = davinci_pll_sysclk_register(dev, &pll0_sysclk4, base);
    clk_register_clkdev(clk, "pll0_sysclk4", "da850-psc0");
    clk_register_clkdev(clk, "pll0_sysclk4", "da850-psc1");
    davinci_pll_sysclk_register(dev, &pll0_sysclk5, base);
    clk = davinci_pll_sysclk_register(dev, &pll0_sysclk6, base);
    clk_register_clkdev(clk, "pll0_sysclk6", "da850-psc0");
    davinci_pll_sysclk_register(dev, &pll0_sysclk7, base);
    davinci_pll_auxclk_register(dev, "pll0_auxclk", base);
    clk = clk_register_fixed_factor(dev, "async2", "pll0_auxclk", CLK_IS_CRITICAL, 1, 1);
    clk_register_clkdev(clk, core::ptr::null(), "i2c_davinci.1");
    clk_register_clkdev(clk, "timer0", core::ptr::null());
    clk_register_clkdev(clk, core::ptr::null(), "davinci-wdt");
    davinci_pll_obsclk_register(dev, &da850_pll0_obsclk_info, base);
    0
}

static da850_pll0_sysclk_info: [*const davinci_pll_sysclk_info; 8] = [
    &pll0_sysclk1, &pll0_sysclk2, &pll0_sysclk3, &pll0_sysclk4,
    &pll0_sysclk5, &pll0_sysclk6, &pll0_sysclk7, core::ptr::null(),
];

unsafe fn of_da850_pll0_init(node: *mut device_node) {
    let base = of_iomap(node, 0);
    if base.is_null() { pr_err!("%s: ioremap failed\n", "of_da850_pll0_init"); return; }
    let cfgchip = syscon_regmap_lookup_by_compatible("ti,da830-cfgchip");
    of_davinci_pll_init(core::ptr::null_mut(), node, &da850_pll0_info,
        &da850_pll0_obsclk_info, da850_pll0_sysclk_info.as_ptr(), 7, base, cfgchip);
}

static da850_pll1_info: davinci_pll_clk_info = davinci_pll_clk_info {
    name: "pll1", unlock_reg: CFGCHIP(3), unlock_mask: CFGCHIP3_PLL1_MASTER_LOCK,
    pllm_mask: GENMASK(4, 0), pllm_min: 4, pllm_max: 32,
    pllout_min_rate: 300000000, pllout_max_rate: 600000000, flags: PLL_HAS_POSTDIV,
};

SYSCLK!(1, pll1_sysclk1, pll1_pllen, 5, SYSCLK_ALWAYS_ENABLED);
SYSCLK!(2, pll1_sysclk2, pll1_pllen, 5, 0);
SYSCLK!(3, pll1_sysclk3, pll1_pllen, 5, 0);

static da850_pll1_obsclk_parent_names: [&str; 4] = ["oscin", "pll1_sysclk1", "pll1_sysclk2", "pll1_sysclk3"];
static mut da850_pll1_obsclk_table: [u32; 4] = [OCSEL_OCSRC_OSCIN, ocsel_ocsrc_pll1_sysclk(1), ocsel_ocsrc_pll1_sysclk(2), ocsel_ocsrc_pll1_sysclk(3)];
static da850_pll1_obsclk_info: davinci_pll_obsclk_info = davinci_pll_obsclk_info {
    name: "pll1_obsclk", parent_names: &da850_pll1_obsclk_parent_names,
    num_parents: da850_pll1_obsclk_parent_names.len(), table: unsafe { &da850_pll1_obsclk_table },
    ocsrc_mask: GENMASK(4, 0),
};

unsafe fn da850_pll1_init(dev: *mut device, base: *mut core::ffi::c_void, cfgchip: *mut regmap) -> i32 {
    let mut clk: *mut clk;
    davinci_pll_clk_register(dev, &da850_pll1_info, "oscin", base, cfgchip);
    davinci_pll_sysclk_register(dev, &pll1_sysclk1, base);
    clk = davinci_pll_sysclk_register(dev, &pll1_sysclk2, base);
    clk_register_clkdev(clk, "pll1_sysclk2", "da850-async3-clksrc");
    davinci_pll_sysclk_register(dev, &pll1_sysclk3, base);
    davinci_pll_obsclk_register(dev, &da850_pll1_obsclk_info, base);
    0
}

static da850_pll1_sysclk_info: [*const davinci_pll_sysclk_info; 4] = [&pll1_sysclk1, &pll1_sysclk2, &pll1_sysclk3, core::ptr::null()];

unsafe fn of_da850_pll1_init(dev: *mut device, base: *mut core::ffi::c_void, cfgchip: *mut regmap) -> i32 {
    of_davinci_pll_init(dev, (*dev).of_node, &da850_pll1_info, &da850_pll1_obsclk_info,
        da850_pll1_sysclk_info.as_ptr(), 3, base, cfgchip)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
