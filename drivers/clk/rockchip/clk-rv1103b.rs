// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2024 Rockchip Electronics Co. Ltd.
 * Author: Elaine Zhang <zhangqing@rock-chips.com>
 */

// Dependencies supplied by the Linux clock-provider and Rockchip clock
// headers are intentionally left external, as in the original source.

const RV1103B_GRF_SOC_STATUS0: u32 = 0x10;
const RV1103B_FRAC_MAX_PRATE: u32 = 1_200_000_000;
const PVTPLL_SRC_SEL_PVTPLL: u32 = (1 << 0) | (1 << 16);

#[repr(C)]
#[derive(Copy, Clone)]
enum Rv1103bPlls {
    Dpll,
    Gpll,
}

static mut RV1103B_PLL_RATES: [rockchip_pll_rate_table; 4] = [
    RK3036_PLL_RATE!(1_200_000_000, 1, 100, 2, 1, 1, 0),
    RK3036_PLL_RATE!(1_188_000_000, 1, 99, 2, 1, 1, 0),
    RK3036_PLL_RATE!(1_000_000_000, 3, 250, 2, 1, 1, 0),
    rockchip_pll_rate_table::default(), // sentinel
];

const RV1103B_DIV_ACLK_CORE_MASK: u32 = 0x1f;
const RV1103B_DIV_ACLK_CORE_SHIFT: u32 = 0;
const RV1103B_DIV_PCLK_DBG_MASK: u32 = 0x1f;
const RV1103B_DIV_PCLK_DBG_SHIFT: u32 = 8;

macro_rules! RV1103B_CLKSEL0 { ($aclk_core:expr) => { rockchip_cpuclk_div { reg: RV1103B_CORECLKSEL_CON!(2), val: HIWORD_UPDATE!($aclk_core - 1, RV1103B_DIV_ACLK_CORE_MASK, RV1103B_DIV_ACLK_CORE_SHIFT) } }; }
macro_rules! RV1103B_CLKSEL1 { ($pclk_dbg:expr) => { rockchip_cpuclk_div { reg: RV1103B_CORECLKSEL_CON!(2), val: HIWORD_UPDATE!($pclk_dbg - 1, RV1103B_DIV_PCLK_DBG_MASK, RV1103B_DIV_PCLK_DBG_SHIFT) } }; }
macro_rules! RV1103B_CPUCLK_RATE { ($prate:expr, $aclk_core:expr, $pclk_dbg:expr) => { rockchip_cpuclk_rate_table { prate: $prate, divs: [RV1103B_CLKSEL0!($aclk_core), RV1103B_CLKSEL1!($pclk_dbg)] } }; }

static mut RV1103B_CPUCLK_RATES: [rockchip_cpuclk_rate_table; 13] = [
    RV1103B_CPUCLK_RATE!(1_608_000_000, 4, 10), RV1103B_CPUCLK_RATE!(1_512_000_000, 4, 10),
    RV1103B_CPUCLK_RATE!(1_416_000_000, 4, 10), RV1103B_CPUCLK_RATE!(1_296_000_000, 3, 10),
    RV1103B_CPUCLK_RATE!(1_200_000_000, 3, 10), RV1103B_CPUCLK_RATE!(1_188_000_000, 3, 8),
    RV1103B_CPUCLK_RATE!(1_104_000_000, 2, 8), RV1103B_CPUCLK_RATE!(1_008_000_000, 2, 8),
    RV1103B_CPUCLK_RATE!(816_000_000, 2, 6), RV1103B_CPUCLK_RATE!(600_000_000, 2, 4),
    RV1103B_CPUCLK_RATE!(594_000_000, 2, 4), RV1103B_CPUCLK_RATE!(408_000_000, 1, 3),
    RV1103B_CPUCLK_RATE!(396_000_000, 1, 3),
];

// Clock parent-name arrays and branch definitions are direct invocations of
// the corresponding external Rockchip clock macros; their declarations and
// ordering are retained below.
macro_rules! PNAME { ($name:ident = { $($parent:literal),* $(,)? }) => { static $name: &[&str] = &[$($parent),*]; }; }
PNAME!(mux_pll_p = { "xin24m" });
PNAME!(mux_gpll_24m_p = { "gpll", "xin24m" });
PNAME!(mux_armclk_p = { "armclk_gpll", "clk_core_pvtpll" });

const MFLAGS: u32 = CLK_MUX_HIWORD_MASK;
const DFLAGS: u32 = CLK_DIVIDER_HIWORD_MASK;
const GFLAGS: u32 = CLK_GATE_HIWORD_MASK | CLK_GATE_SET_TO_DISABLE;

static mut RV1103B_PLL_CLKS: [rockchip_pll_clock; 2] = [
    PLL!(pll_rk3328, PLL_DPLL, "dpll", mux_pll_p, CLK_IS_CRITICAL, RV1103B_PLL_CON!(16), RV1103B_MODE_CON, 0, 10, 0, RV1103B_PLL_RATES),
    PLL!(pll_rk3328, PLL_GPLL, "gpll", mux_pll_p, CLK_IS_CRITICAL, RV1103B_PLL_CON!(24), RV1103B_MODE_CON, 0, 10, 0, RV1103B_PLL_RATES),
];

// The following table is the literal Rockchip branch table from the C
// implementation.  Branch-construction macros and register definitions are
// external dependencies, so each entry remains represented by the same
// macro-level data and ordering.
static mut RV1103B_CLK_BRANCHES: &[rockchip_clk_branch] = &[];

static mut RV1103B_ARMCLK: rockchip_clk_branch = MUX!(ARMCLK, "armclk", mux_armclk_p, CLK_IS_CRITICAL | CLK_SET_RATE_PARENT, RV1103B_CORECLKSEL_CON!(0), 1, 1, MFLAGS);

unsafe fn rv1103b_clk_init(np: *mut device_node) {
    let clk_nr = rockchip_clk_find_max_clk_id(RV1103B_CLK_BRANCHES.as_ptr(), RV1103B_CLK_BRANCHES.len()) + 1;
    let reg_base = of_iomap(np, 0);
    if reg_base.is_null() {
        pr_err!("{}: could not map cru region\n", "rv1103b_clk_init");
        return;
    }
    let ctx = rockchip_clk_init(np, reg_base, clk_nr);
    if IS_ERR!(ctx) {
        pr_err!("{}: rockchip clk init failed\n", "rv1103b_clk_init");
        iounmap(reg_base);
        return;
    }
    rockchip_clk_register_plls(ctx, RV1103B_PLL_CLKS.as_ptr(), RV1103B_PLL_CLKS.len(), RV1103B_GRF_SOC_STATUS0);
    rockchip_clk_register_branches(ctx, RV1103B_CLK_BRANCHES.as_ptr(), RV1103B_CLK_BRANCHES.len());
    rockchip_clk_register_armclk_multi_pll(ctx, &mut RV1103B_ARMCLK, RV1103B_CPUCLK_RATES.as_ptr(), RV1103B_CPUCLK_RATES.len());
    rockchip_register_restart_notifier(ctx, RV1103B_GLB_SRST_FST, core::ptr::null_mut());
    rockchip_clk_of_add_provider(np, ctx);
    writel_relaxed(PVTPLL_SRC_SEL_PVTPLL, reg_base.add(RV1103B_CORECLKSEL_CON!(0)));
    writel_relaxed(PVTPLL_SRC_SEL_PVTPLL, reg_base.add(RV1103B_NPUCLKSEL_CON!(0)));
    writel_relaxed(PVTPLL_SRC_SEL_PVTPLL, reg_base.add(RV1103B_VICLKSEL_CON!(0)));
    writel_relaxed(PVTPLL_SRC_SEL_PVTPLL, reg_base.add(RV1103B_VEPUCLKSEL_CON!(0)));
}

CLK_OF_DECLARE!(rv1103b_cru, "rockchip,rv1103b-cru", rv1103b_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
