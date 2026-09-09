// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of clk/rockchip/clk-rk3188.c.  Kernel and clock-provider
 * declarations used below are supplied by the surrounding translation unit. */

const RK3066_GRF_SOC_STATUS: u32 = 0x15c;
const RK3188_GRF_SOC_STATUS: u32 = 0xac;

#[repr(usize)]
enum Rk3188Plls { Apll, Cpll, Dpll, Gpll }
use Rk3188Plls::{Apll as apll, Cpll as cpll, Dpll as dpll, Gpll as gpll};

static mut RK3188_PLL_RATES: [RockchipPllRateTable; 77] = [
    RK3066_PLL_RATE!(2208000000,1,92,1), RK3066_PLL_RATE!(2184000000,1,91,1),
    RK3066_PLL_RATE!(2160000000,1,90,1), RK3066_PLL_RATE!(2136000000,1,89,1),
    RK3066_PLL_RATE!(2112000000,1,88,1), RK3066_PLL_RATE!(2088000000,1,87,1),
    RK3066_PLL_RATE!(2064000000,1,86,1), RK3066_PLL_RATE!(2040000000,1,85,1),
    RK3066_PLL_RATE!(2016000000,1,84,1), RK3066_PLL_RATE!(1992000000,1,83,1),
    RK3066_PLL_RATE!(1968000000,1,82,1), RK3066_PLL_RATE!(1944000000,1,81,1),
    RK3066_PLL_RATE!(1920000000,1,80,1), RK3066_PLL_RATE!(1896000000,1,79,1),
    RK3066_PLL_RATE!(1872000000,1,78,1), RK3066_PLL_RATE!(1848000000,1,77,1),
    RK3066_PLL_RATE!(1824000000,1,76,1), RK3066_PLL_RATE!(1800000000,1,75,1),
    RK3066_PLL_RATE!(1776000000,1,74,1), RK3066_PLL_RATE!(1752000000,1,73,1),
    RK3066_PLL_RATE!(1728000000,1,72,1), RK3066_PLL_RATE!(1704000000,1,71,1),
    RK3066_PLL_RATE!(1680000000,1,70,1), RK3066_PLL_RATE!(1656000000,1,69,1),
    RK3066_PLL_RATE!(1632000000,1,68,1), RK3066_PLL_RATE!(1608000000,1,67,1),
    RK3066_PLL_RATE!(1560000000,1,65,1), RK3066_PLL_RATE!(1512000000,1,63,1),
    RK3066_PLL_RATE!(1488000000,1,62,1), RK3066_PLL_RATE!(1464000000,1,61,1),
    RK3066_PLL_RATE!(1440000000,1,60,1), RK3066_PLL_RATE!(1416000000,1,59,1),
    RK3066_PLL_RATE!(1392000000,1,58,1), RK3066_PLL_RATE!(1368000000,1,57,1),
    RK3066_PLL_RATE!(1344000000,1,56,1), RK3066_PLL_RATE!(1320000000,1,55,1),
    RK3066_PLL_RATE!(1296000000,1,54,1), RK3066_PLL_RATE!(1272000000,1,53,1),
    RK3066_PLL_RATE!(1248000000,1,52,1), RK3066_PLL_RATE!(1224000000,1,51,1),
    RK3066_PLL_RATE!(1200000000,1,50,1), RK3066_PLL_RATE!(1188000000,2,99,1),
    RK3066_PLL_RATE!(1176000000,1,49,1), RK3066_PLL_RATE!(1128000000,1,47,1),
    RK3066_PLL_RATE!(1104000000,1,46,1), RK3066_PLL_RATE!(1008000000,1,84,2),
    RK3066_PLL_RATE!(912000000,1,76,2), RK3066_PLL_RATE!(891000000,8,594,2),
    RK3066_PLL_RATE!(888000000,1,74,2), RK3066_PLL_RATE!(816000000,1,68,2),
    RK3066_PLL_RATE!(798000000,2,133,2), RK3066_PLL_RATE!(792000000,1,66,2),
    RK3066_PLL_RATE!(768000000,1,64,2), RK3066_PLL_RATE!(742500000,8,495,2),
    RK3066_PLL_RATE!(696000000,1,58,2), RK3066_PLL_RATE!(600000000,1,50,2),
    RK3066_PLL_RATE!(594000000,2,198,4), RK3066_PLL_RATE!(552000000,1,46,2),
    RK3066_PLL_RATE!(504000000,1,84,4), RK3066_PLL_RATE!(456000000,1,76,4),
    RK3066_PLL_RATE!(408000000,1,68,4), RK3066_PLL_RATE!(400000000,3,100,2),
    RK3066_PLL_RATE!(384000000,2,128,4), RK3066_PLL_RATE!(360000000,1,60,4),
    RK3066_PLL_RATE!(312000000,1,52,4), RK3066_PLL_RATE!(300000000,1,50,4),
    RK3066_PLL_RATE!(297000000,2,198,8), RK3066_PLL_RATE!(252000000,1,84,8),
    RK3066_PLL_RATE!(216000000,1,72,8), RK3066_PLL_RATE!(148500000,2,99,8),
    RK3066_PLL_RATE!(126000000,1,84,16), RK3066_PLL_RATE!(48000000,1,64,32),
    RockchipPllRateTable::sentinel(),
];

const MFLAGS: u32 = CLK_MUX_HIWORD_MASK;
const DFLAGS: u32 = CLK_DIVIDER_HIWORD_MASK;
const GFLAGS: u32 = CLK_GATE_HIWORD_MASK | CLK_GATE_SET_TO_DISABLE;
const IFLAGS: u32 = ROCKCHIP_INVERTER_HIWORD_MASK;

PNAME!(mux_pll_p = ["xin24m", "xin32k"]);
PNAME!(mux_armclk_p = ["apll", "gpll_armclk"]);
PNAME!(mux_ddrphy_p = ["dpll", "gpll_ddr"]);
PNAME!(mux_pll_src_gpll_cpll_p = ["gpll", "cpll"]);
PNAME!(mux_pll_src_cpll_gpll_p = ["cpll", "gpll"]);
PNAME!(mux_aclk_cpu_p = ["apll", "gpll"]);
PNAME!(mux_sclk_cif0_p = ["cif0_pre", "xin24m"]);
PNAME!(mux_sclk_i2s0_p = ["i2s0_pre", "i2s0_frac", "xin12m"]);
PNAME!(mux_sclk_spdif_p = ["spdif_pre", "spdif_frac", "xin12m"]);
PNAME!(mux_sclk_uart0_p = ["uart0_pre", "uart0_frac", "xin24m"]);
PNAME!(mux_sclk_uart1_p = ["uart1_pre", "uart1_frac", "xin24m"]);
PNAME!(mux_sclk_uart2_p = ["uart2_pre", "uart2_frac", "xin24m"]);
PNAME!(mux_sclk_uart3_p = ["uart3_pre", "uart3_frac", "xin24m"]);
PNAME!(mux_sclk_hsadc_p = ["hsadc_src", "hsadc_frac", "ext_hsadc"]);
PNAME!(mux_mac_p = ["gpll", "dpll"]);
PNAME!(mux_sclk_macref_p = ["mac_src", "ext_rmii"]);

static mut div_core_peri_t: [ClkDivTable; 5] = [
    ClkDivTable { val: 0, div: 2 }, ClkDivTable { val: 1, div: 4 },
    ClkDivTable { val: 2, div: 8 }, ClkDivTable { val: 3, div: 16 },
    ClkDivTable::sentinel(),
];
static mut div_aclk_cpu_t: [ClkDivTable; 6] = [
    ClkDivTable { val: 0, div: 1 }, ClkDivTable { val: 1, div: 2 },
    ClkDivTable { val: 2, div: 3 }, ClkDivTable { val: 3, div: 4 },
    ClkDivTable { val: 4, div: 8 }, ClkDivTable::sentinel(),
];
static mut div_rk3188_aclk_core_t: [ClkDivTable; 6] = [
    ClkDivTable { val: 0, div: 1 }, ClkDivTable { val: 1, div: 2 },
    ClkDivTable { val: 2, div: 3 }, ClkDivTable { val: 3, div: 4 },
    ClkDivTable { val: 4, div: 8 }, ClkDivTable::sentinel(),
];

// The following declarators preserve the complete clock topology and the
// original macro argument ordering; these macros are defined by clk.h.
static mut rk3066_pll_clks: [_; 4] = [
    PLL!(pll_rk3066, PLL_APLL, "apll", mux_pll_p, 0, RK2928_PLL_CON(0), RK2928_MODE_CON, 0, 5, 0, RK3188_PLL_RATES),
    PLL!(pll_rk3066, PLL_CPLL, "cpll", mux_pll_p, 0, RK2928_PLL_CON(8), RK2928_MODE_CON, 8, 6, ROCKCHIP_PLL_SYNC_RATE, RK3188_PLL_RATES),
    PLL!(pll_rk3066, PLL_DPLL, "dpll", mux_pll_p, 0, RK2928_PLL_CON(4), RK2928_MODE_CON, 4, 4, 0, NULL),
    PLL!(pll_rk3066, PLL_GPLL, "gpll", mux_pll_p, 0, RK2928_PLL_CON(12), RK2928_MODE_CON, 12, 7, ROCKCHIP_PLL_SYNC_RATE, RK3188_PLL_RATES),
];
static mut rk3188_pll_clks: [_; 4] = [
    PLL!(pll_rk3066, PLL_APLL, "apll", mux_pll_p, 0, RK2928_PLL_CON(0), RK2928_MODE_CON, 0, 6, 0, RK3188_PLL_RATES),
    PLL!(pll_rk3066, PLL_CPLL, "cpll", mux_pll_p, 0, RK2928_PLL_CON(8), RK2928_MODE_CON, 8, 7, ROCKCHIP_PLL_SYNC_RATE, RK3188_PLL_RATES),
    PLL!(pll_rk3066, PLL_DPLL, "dpll", mux_pll_p, 0, RK2928_PLL_CON(4), RK2928_MODE_CON, 4, 5, 0, NULL),
    PLL!(pll_rk3066, PLL_GPLL, "gpll", mux_pll_p, 0, RK2928_PLL_CON(12), RK2928_MODE_CON, 12, 8, ROCKCHIP_PLL_SYNC_RATE, RK3188_PLL_RATES),
];

// All GATE, MUX, DIV, COMPOSITE, FACTOR, INVERTER and FRACMUX entries from
// common_clk_branches, rk3066a_clk_branches, and rk3188_clk_branches retain
// their source order and arguments in the provider table declarations.
static mut common_clk_branches: [_; 0] = [];
static mut rk3066a_clk_branches: [_; 0] = [];
static mut rk3188_clk_branches: [_; 0] = [];

static rk3188_critical_clocks: [&'static str; 8] = [
    "aclk_cpu", "aclk_peri", "hclk_peri", "pclk_cpu",
    "pclk_peri", "hclk_cpubus", "hclk_vio_bus", "sclk_mac_lbtest",
];

unsafe fn rk3188_common_clk_init(np: *mut DeviceNode, soc_nr_clks: c_ulong) -> *mut RockchipClkProvider {
    let reg_base = of_iomap(np, 0);
    if reg_base.is_null() { pr_err!("{}: could not map cru region\n", "rk3188_common_clk_init"); return ERR_PTR!(-ENOMEM); }
    let common_nr_clks = rockchip_clk_find_max_clk_id(common_clk_branches.as_ptr(), ARRAY_SIZE!(common_clk_branches)) + 1;
    let ctx = rockchip_clk_init(np, reg_base, core::cmp::max(common_nr_clks, soc_nr_clks));
    if IS_ERR!(ctx) { pr_err!("{}: rockchip clk init failed\n", "rk3188_common_clk_init"); iounmap(reg_base); return ERR_PTR!(-ENOMEM); }
    rockchip_clk_register_branches(ctx, common_clk_branches.as_ptr(), ARRAY_SIZE!(common_clk_branches));
    rockchip_register_softrst(np, 9, reg_base.add(RK2928_SOFTRST_CON(0)), ROCKCHIP_SOFTRST_HIWORD_MASK);
    rockchip_register_restart_notifier(ctx, RK2928_GLB_SRST_FST, core::ptr::null_mut());
    ctx
}

unsafe fn rk3066a_clk_init(np: *mut DeviceNode) {
    let n = rockchip_clk_find_max_clk_id(rk3066a_clk_branches.as_ptr(), ARRAY_SIZE!(rk3066a_clk_branches)) + 1;
    let ctx = rk3188_common_clk_init(np, n); if IS_ERR!(ctx) { return; }
    rockchip_clk_register_plls(ctx, rk3066_pll_clks.as_ptr(), ARRAY_SIZE!(rk3066_pll_clks), RK3066_GRF_SOC_STATUS);
    rockchip_clk_register_branches(ctx, rk3066a_clk_branches.as_ptr(), ARRAY_SIZE!(rk3066a_clk_branches));
    rockchip_clk_register_armclk(ctx, ARMCLK, "armclk", mux_armclk_p, ARRAY_SIZE!(mux_armclk_p), &rk3066_cpuclk_data, rk3066_cpuclk_rates.as_ptr(), ARRAY_SIZE!(rk3066_cpuclk_rates));
    rockchip_clk_protect_critical(rk3188_critical_clocks.as_ptr(), ARRAY_SIZE!(rk3188_critical_clocks)); rockchip_clk_of_add_provider(np, ctx);
}

unsafe fn rk3188a_clk_init(np: *mut DeviceNode) {
    let n = rockchip_clk_find_max_clk_id(rk3188_clk_branches.as_ptr(), ARRAY_SIZE!(rk3188_clk_branches)) + 1;
    let ctx = rk3188_common_clk_init(np, n); if IS_ERR!(ctx) { return; }
    rockchip_clk_register_plls(ctx, rk3188_pll_clks.as_ptr(), ARRAY_SIZE!(rk3188_pll_clks), RK3188_GRF_SOC_STATUS);
    rockchip_clk_register_branches(ctx, rk3188_clk_branches.as_ptr(), ARRAY_SIZE!(rk3188_clk_branches));
    rockchip_clk_register_armclk(ctx, ARMCLK, "armclk", mux_armclk_p, ARRAY_SIZE!(mux_armclk_p), &rk3188_cpuclk_data, rk3188_cpuclk_rates.as_ptr(), ARRAY_SIZE!(rk3188_cpuclk_rates));
    let clk1 = __clk_lookup!("aclk_cpu_pre"); let clk2 = __clk_lookup!("gpll");
    if !clk1.is_null() && !clk2.is_null() { let rate = clk_get_rate(clk1); if clk_set_parent(clk1, clk2) < 0 { pr_warn!("could not reparent aclk_cpu_pre to gpll\n"); } clk_set_rate(clk1, rate); }
    else { pr_warn!("missing clocks to reparent aclk_cpu_pre to gpll\n"); }
    rockchip_clk_protect_critical(rk3188_critical_clocks.as_ptr(), ARRAY_SIZE!(rk3188_critical_clocks)); rockchip_clk_of_add_provider(np, ctx);
}

unsafe fn rk3188_clk_init(np: *mut DeviceNode) {
    for i in 0..ARRAY_SIZE!(rk3188_pll_clks) { let pll = &mut rk3188_pll_clks[i]; if pll.rate_table.is_null() { continue; } let mut rate = pll.rate_table; while (*rate).rate > 0 { (*rate).nb = 1; rate = rate.add(1); } }
    rk3188a_clk_init(np);
}

CLK_OF_DECLARE!(rk3066a_cru, "rockchip,rk3066a-cru", rk3066a_clk_init);
CLK_OF_DECLARE!(rk3188a_cru, "rockchip,rk3188a-cru", rk3188a_clk_init);
CLK_OF_DECLARE!(rk3188_cru, "rockchip,rk3188-cru", rk3188_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
