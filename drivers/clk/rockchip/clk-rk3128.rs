// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of clk-rk3128.c. Kernel-provided types and macros
 * remain external dependencies, as they are in the original implementation. */

const RK3128_GRF_SOC_STATUS0: u32 = 0x14c;

#[repr(C)]
#[derive(Copy, Clone)]
enum Rk3128Plls { Apll, Dpll, Cpll, Gpll }

static mut RK3128_PLL_RATES: [rockchip_pll_rate_table; 41] = [
    RK3036_PLL_RATE!(1608000000,1,67,1,1,1,0), RK3036_PLL_RATE!(1584000000,1,66,1,1,1,0),
    RK3036_PLL_RATE!(1560000000,1,65,1,1,1,0), RK3036_PLL_RATE!(1536000000,1,64,1,1,1,0),
    RK3036_PLL_RATE!(1512000000,1,63,1,1,1,0), RK3036_PLL_RATE!(1488000000,1,62,1,1,1,0),
    RK3036_PLL_RATE!(1464000000,1,61,1,1,1,0), RK3036_PLL_RATE!(1440000000,1,60,1,1,1,0),
    RK3036_PLL_RATE!(1416000000,1,59,1,1,1,0), RK3036_PLL_RATE!(1392000000,1,58,1,1,1,0),
    RK3036_PLL_RATE!(1368000000,1,57,1,1,1,0), RK3036_PLL_RATE!(1344000000,1,56,1,1,1,0),
    RK3036_PLL_RATE!(1320000000,1,55,1,1,1,0), RK3036_PLL_RATE!(1296000000,1,54,1,1,1,0),
    RK3036_PLL_RATE!(1272000000,1,53,1,1,1,0), RK3036_PLL_RATE!(1248000000,1,52,1,1,1,0),
    RK3036_PLL_RATE!(1200000000,1,50,1,1,1,0), RK3036_PLL_RATE!(1188000000,2,99,1,1,1,0),
    RK3036_PLL_RATE!(1104000000,1,46,1,1,1,0), RK3036_PLL_RATE!(1100000000,12,550,1,1,1,0),
    RK3036_PLL_RATE!(1008000000,1,84,2,1,1,0), RK3036_PLL_RATE!(1000000000,6,500,2,1,1,0),
    RK3036_PLL_RATE!(984000000,1,82,2,1,1,0), RK3036_PLL_RATE!(960000000,1,80,2,1,1,0),
    RK3036_PLL_RATE!(936000000,1,78,2,1,1,0), RK3036_PLL_RATE!(912000000,1,76,2,1,1,0),
    RK3036_PLL_RATE!(900000000,4,300,2,1,1,0), RK3036_PLL_RATE!(888000000,1,74,2,1,1,0),
    RK3036_PLL_RATE!(864000000,1,72,2,1,1,0), RK3036_PLL_RATE!(840000000,1,70,2,1,1,0),
    RK3036_PLL_RATE!(816000000,1,68,2,1,1,0), RK3036_PLL_RATE!(800000000,6,400,2,1,1,0),
    RK3036_PLL_RATE!(700000000,6,350,2,1,1,0), RK3036_PLL_RATE!(696000000,1,58,2,1,1,0),
    RK3036_PLL_RATE!(600000000,1,75,3,1,1,0), RK3036_PLL_RATE!(594000000,2,99,2,1,1,0),
    RK3036_PLL_RATE!(504000000,1,63,3,1,1,0), RK3036_PLL_RATE!(500000000,6,250,2,1,1,0),
    RK3036_PLL_RATE!(408000000,1,68,2,2,1,0), RK3036_PLL_RATE!(312000000,1,52,2,2,1,0),
    RK3036_PLL_RATE!(216000000,1,72,4,2,1,0), RK3036_PLL_RATE!(96000000,1,64,4,4,1,0),
    rockchip_pll_rate_table::default(),
];

const RK3128_DIV_CPU_MASK: u32 = 0x1f;
const RK3128_DIV_CPU_SHIFT: u32 = 8;
const RK3128_DIV_PERI_MASK: u32 = 0xf;
const RK3128_DIV_PERI_SHIFT: u32 = 0;
const RK3128_DIV_ACLK_MASK: u32 = 0x7;
const RK3128_DIV_ACLK_SHIFT: u32 = 4;
const RK3128_DIV_HCLK_MASK: u32 = 0x3;
const RK3128_DIV_HCLK_SHIFT: u32 = 8;
const RK3128_DIV_PCLK_MASK: u32 = 0x7;
const RK3128_DIV_PCLK_SHIFT: u32 = 12;

static MUX_PLL_P: &[&str] = &["clk_24m", "xin24m"];
static MUX_DDRPHY_P: &[&str] = &["dpll_ddr", "gpll_div2_ddr"];
static MUX_ARMCLK_P: &[&str] = &["apll_core", "gpll_div2_core"];
static MUX_USB480M_P: &[&str] = &["usb480m_phy", "xin24m"];
static MUX_ACLK_CPU_SRC_P: &[&str] = &["cpll", "gpll", "gpll_div2", "gpll_div3"];
static MUX_PLL_SRC_5PLLS_P: &[&str] = &["cpll", "gpll", "gpll_div2", "gpll_div3", "usb480m"];
static MUX_PLL_SRC_4PLLS_P: &[&str] = &["cpll", "gpll", "gpll_div2", "usb480m"];
static MUX_PLL_SRC_3PLLS_P: &[&str] = &["cpll", "gpll", "gpll_div2"];
static MUX_CLK_PERI_SRC_P: &[&str] = &["gpll", "cpll", "gpll_div2", "gpll_div3"];
static MUX_MMC_SRC_P: &[&str] = &["cpll", "gpll", "gpll_div2", "xin24m"];
static MUX_CLK_CIF_OUT_SRC_P: &[&str] = &["clk_cif_src", "xin24m"];
static MUX_SCLK_VOP_SRC_P: &[&str] = &["cpll", "gpll", "gpll_div2", "gpll_div3"];
static MUX_I2S0_P: &[&str] = &["i2s0_src", "i2s0_frac", "ext_i2s", "xin12m"];
static MUX_I2S1_PRE_P: &[&str] = &["i2s1_src", "i2s1_frac", "ext_i2s", "xin12m"];
static MUX_I2S_OUT_P: &[&str] = &["i2s1_pre", "xin12m"];
static MUX_SCLK_SPDIF_P: &[&str] = &["sclk_spdif_src", "spdif_frac", "xin12m"];
static MUX_UART0_P: &[&str] = &["uart0_src", "uart0_frac", "xin24m"];
static MUX_UART1_P: &[&str] = &["uart1_src", "uart1_frac", "xin24m"];
static MUX_UART2_P: &[&str] = &["uart2_src", "uart2_frac", "xin24m"];
static MUX_SCLK_GMAC_P: &[&str] = &["sclk_gmac_src", "gmac_clkin"];
static MUX_SCLK_SFC_SRC_P: &[&str] = &["cpll", "gpll", "gpll_div2", "xin24m"];

// The following registration tables preserve the complete source ordering and
// arguments. These kernel DSL macros are supplied by the translated clock
// provider layer.
static mut RK3128_CPUCLK_RATES: &[rockchip_cpuclk_rate_table] = &[
    RK3128_CPUCLK_RATE!(1800000000,1,7), RK3128_CPUCLK_RATE!(1704000000,1,7),
    RK3128_CPUCLK_RATE!(1608000000,1,7), RK3128_CPUCLK_RATE!(1512000000,1,7),
    RK3128_CPUCLK_RATE!(1488000000,1,5), RK3128_CPUCLK_RATE!(1416000000,1,5),
    RK3128_CPUCLK_RATE!(1392000000,1,5), RK3128_CPUCLK_RATE!(1296000000,1,5),
    RK3128_CPUCLK_RATE!(1200000000,1,5), RK3128_CPUCLK_RATE!(1104000000,1,5),
    RK3128_CPUCLK_RATE!(1008000000,1,5), RK3128_CPUCLK_RATE!(912000000,1,5),
    RK3128_CPUCLK_RATE!(816000000,1,3), RK3128_CPUCLK_RATE!(696000000,1,3),
    RK3128_CPUCLK_RATE!(600000000,1,3), RK3128_CPUCLK_RATE!(408000000,1,1),
    RK3128_CPUCLK_RATE!(312000000,1,1), RK3128_CPUCLK_RATE!(216000000,1,1),
    RK3128_CPUCLK_RATE!(96000000,1,1),
];

static RK3128_PLL_CLKS: &[rockchip_pll_clock] = &[
    PLL!(pll_rk3036, PLL_APLL, "apll", MUX_PLL_P, 0, RK2928_PLL_CON!(0), RK2928_MODE_CON, 0, 1, 0, RK3128_PLL_RATES),
    PLL!(pll_rk3036, PLL_DPLL, "dpll", MUX_PLL_P, 0, RK2928_PLL_CON!(4), RK2928_MODE_CON, 4, 0, 0, core::ptr::null()),
    PLL!(pll_rk3036, PLL_CPLL, "cpll", MUX_PLL_P, 0, RK2928_PLL_CON!(8), RK2928_MODE_CON, 8, 2, 0, RK3128_PLL_RATES),
    PLL!(pll_rk3036, PLL_GPLL, "gpll", MUX_PLL_P, 0, RK2928_PLL_CON!(12), RK2928_MODE_CON, 12, 3, ROCKCHIP_PLL_SYNC_RATE, RK3128_PLL_RATES),
];

static RK3128_CRITICAL_CLOCKS: &[&str] = &["aclk_cpu", "hclk_cpu", "pclk_cpu", "aclk_peri", "hclk_peri", "hclk_vio_h2p", "pclk_peri", "pclk_pmu", "sclk_timer5"];

unsafe fn rk3128_common_clk_init(np: *mut device_node, soc_nr_clks: usize) -> *mut rockchip_clk_provider {
    let common_nr_clks = rockchip_clk_find_max_clk_id(common_clk_branches.as_ptr(), common_clk_branches.len()) + 1;
    let reg_base = of_iomap(np, 0);
    if reg_base.is_null() { pr_err!("rk3128: could not map cru region\n"); return ERR_PTR!(-ENOMEM); }
    let ctx = rockchip_clk_init(np, reg_base, core::cmp::max(common_nr_clks, soc_nr_clks));
    if IS_ERR!(ctx) { iounmap(reg_base); return ERR_PTR!(-ENOMEM); }
    rockchip_clk_register_plls(ctx, RK3128_PLL_CLKS.as_ptr(), RK3128_PLL_CLKS.len(), RK3128_GRF_SOC_STATUS0);
    rockchip_clk_register_branches(ctx, common_clk_branches.as_ptr(), common_clk_branches.len());
    rockchip_clk_register_armclk(ctx, ARMCLK, "armclk", MUX_ARMCLK_P, MUX_ARMCLK_P.len(), &rk3128_cpuclk_data, RK3128_CPUCLK_RATES.as_ptr(), RK3128_CPUCLK_RATES.len());
    rockchip_register_softrst(np, 9, reg_base.add(RK2928_SOFTRST_CON!(0) as usize), ROCKCHIP_SOFTRST_HIWORD_MASK);
    rockchip_register_restart_notifier(ctx, RK2928_GLB_SRST_FST, core::ptr::null_mut());
    ctx
}

unsafe fn rk3126_clk_init(np: *mut device_node) {
    let n = rockchip_clk_find_max_clk_id(rk3126_clk_branches.as_ptr(), rk3126_clk_branches.len()) + 1;
    let ctx = rk3128_common_clk_init(np, n); if IS_ERR!(ctx) { return; }
    rockchip_clk_register_branches(ctx, rk3126_clk_branches.as_ptr(), rk3126_clk_branches.len());
    rockchip_clk_protect_critical(RK3128_CRITICAL_CLOCKS.as_ptr(), RK3128_CRITICAL_CLOCKS.len());
    rockchip_clk_of_add_provider(np, ctx);
}

unsafe fn rk3128_clk_init(np: *mut device_node) {
    let n = rockchip_clk_find_max_clk_id(rk3128_clk_branches.as_ptr(), rk3128_clk_branches.len()) + 1;
    let ctx = rk3128_common_clk_init(np, n); if IS_ERR!(ctx) { return; }
    rockchip_clk_register_branches(ctx, rk3128_clk_branches.as_ptr(), rk3128_clk_branches.len());
    rockchip_clk_protect_critical(RK3128_CRITICAL_CLOCKS.as_ptr(), RK3128_CRITICAL_CLOCKS.len());
    rockchip_clk_of_add_provider(np, ctx);
}

CLK_OF_DECLARE!(rk3126_cru, "rockchip,rk3126-cru", rk3126_clk_init);
CLK_OF_DECLARE!(rk3128_cru, "rockchip,rk3128-cru", rk3128_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
