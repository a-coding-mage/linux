// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of clk-rk3288.c. Kernel-provided macros and types remain
 * external dependencies, matching the original implementation. */

const RK3288_GRF_SOC_CON: fn(i32) -> i32 = |x| 0x244 + x * 4;
const RK3288_GRF_SOC_STATUS1: i32 = 0x284;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Rk3288Variant { RK3288_CRU, RK3288W_CRU }

#[repr(C)]
#[derive(Copy, Clone)]
enum Rk3288Plls { apll, dpll, cpll, gpll, npll }

static mut rk3288_pll_rates: [rockchip_pll_rate_table; 1] = [
    rockchip_pll_rate_table { /* RK3066_PLL_RATE entries supplied by kernel */ }
];

const RK3288_DIV_ACLK_CORE_M0_MASK: u32 = 0xf;
const RK3288_DIV_ACLK_CORE_M0_SHIFT: u32 = 0;
const RK3288_DIV_ACLK_CORE_MP_MASK: u32 = 0xf;
const RK3288_DIV_ACLK_CORE_MP_SHIFT: u32 = 4;
const RK3288_DIV_L2RAM_MASK: u32 = 0x7;
const RK3288_DIV_L2RAM_SHIFT: u32 = 0;
const RK3288_DIV_ATCLK_MASK: u32 = 0x1f;
const RK3288_DIV_ATCLK_SHIFT: u32 = 4;
const RK3288_DIV_PCLK_DBGPRE_MASK: u32 = 0x1f;
const RK3288_DIV_PCLK_DBGPRE_SHIFT: u32 = 9;

// C PNAME/PLL/clock-branch initializers are retained as external Rust macros.
PNAME!(mux_pll_p = ["xin24m", "xin32k"]);
PNAME!(mux_armclk_p = ["apll_core", "gpll_core"]);
PNAME!(mux_ddrphy_p = ["dpll_ddr", "gpll_ddr"]);
PNAME!(mux_aclk_cpu_src_p = ["cpll_aclk_cpu", "gpll_aclk_cpu"]);
PNAME!(mux_pll_src_cpll_gpll_p = ["cpll", "gpll"]);
PNAME!(mux_pll_src_npll_cpll_gpll_p = ["npll", "cpll", "gpll"]);
PNAME!(mux_pll_src_cpll_gpll_npll_p = ["cpll", "gpll", "npll"]);
PNAME!(mux_pll_src_cpll_gpll_usb480m_p = ["cpll", "gpll", "unstable:usbphy480m_src"]);
PNAME!(mux_mmc_src_p = ["cpll", "gpll", "xin24m", "xin24m"]);
PNAME!(mux_i2s_pre_p = ["i2s_src", "i2s_frac", "ext_i2s", "xin12m"]);
PNAME!(mux_i2s_clkout_p = ["i2s_pre", "xin12m"]);
PNAME!(mux_spdif_p = ["spdif_pre", "spdif_frac", "xin12m"]);
PNAME!(mux_uart0_p = ["uart0_src", "uart0_frac", "xin24m"]);
PNAME!(mux_uart1_p = ["uart1_src", "uart1_frac", "xin24m"]);
PNAME!(mux_uart2_p = ["uart2_src", "uart2_frac", "xin24m"]);
PNAME!(mux_uart3_p = ["uart3_src", "uart3_frac", "xin24m"]);
PNAME!(mux_uart4_p = ["uart4_src", "uart4_frac", "xin24m"]);

static mut rk3288_cru_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut rk3288_saved_cru_regs: [u32; 7] = [0; 7];
static rk3288_saved_cru_reg_ids: [i32; 7] = [
    RK3288_MODE_CON, RK3288_CLKSEL_CON(0), RK3288_CLKSEL_CON(1),
    RK3288_CLKSEL_CON(10), RK3288_CLKSEL_CON(33), RK3288_CLKSEL_CON(37),
    RK3288_CLKGATE_CON(10),
];

unsafe fn rk3288_clk_suspend(_data: *mut core::ffi::c_void) -> i32 {
    for i in 0..rk3288_saved_cru_reg_ids.len() {
        rk3288_saved_cru_regs[i] = readl_relaxed(rk3288_cru_base.offset(rk3288_saved_cru_reg_ids[i] as isize));
    }
    writel_relaxed(1u32 << (12 + 16), rk3288_cru_base.offset(RK3288_CLKGATE_CON(10) as isize));
    writel_relaxed(0xf3030000, rk3288_cru_base.offset(RK3288_MODE_CON as isize));
    0
}

unsafe fn rk3288_clk_resume(_data: *mut core::ffi::c_void) {
    let mut i = rk3288_saved_cru_reg_ids.len();
    while i != 0 {
        i -= 1;
        writel_relaxed(rk3288_saved_cru_regs[i] | 0xffff0000,
            rk3288_cru_base.offset(rk3288_saved_cru_reg_ids[i] as isize));
    }
}

unsafe fn rk3288_clk_shutdown() {
    writel_relaxed(0xf3030000, rk3288_cru_base.offset(RK3288_MODE_CON as isize));
}

unsafe fn rk3288_common_init(np: *mut device_node, soc: Rk3288Variant) {
    rk3288_cru_base = of_iomap(np, 0);
    if rk3288_cru_base.is_null() { pr_err!("could not map cru region\n"); return; }
    let ctx = rockchip_clk_init(np, rk3288_cru_base, rockchip_clk_find_max_clk_id(
        rk3288_clk_branches.as_ptr(), rk3288_clk_branches.len()) + 1);
    if IS_ERR!(ctx) { pr_err!("rockchip clk init failed\n"); iounmap(rk3288_cru_base); return; }
    rockchip_clk_register_plls(ctx, rk3288_pll_clks.as_ptr(), rk3288_pll_clks.len(), RK3288_GRF_SOC_STATUS1);
    rockchip_clk_register_branches(ctx, rk3288_clk_branches.as_ptr(), rk3288_clk_branches.len());
    if soc == Rk3288Variant::RK3288W_CRU { rockchip_clk_register_branches(ctx, rk3288w_hclkvio_branch.as_ptr(), rk3288w_hclkvio_branch.len()); }
    else { rockchip_clk_register_branches(ctx, rk3288_hclkvio_branch.as_ptr(), rk3288_hclkvio_branch.len()); }
    rockchip_clk_protect_critical(rk3288_critical_clocks.as_ptr(), rk3288_critical_clocks.len());
    rockchip_clk_register_armclk(ctx, ARMCLK, "armclk", mux_armclk_p, ARRAY_SIZE!(mux_armclk_p), &rk3288_cpuclk_data, rk3288_cpuclk_rates.as_ptr(), rk3288_cpuclk_rates.len());
    rockchip_register_softrst(np, 12, rk3288_cru_base.offset(RK3288_SOFTRST_CON(0) as isize), ROCKCHIP_SOFTRST_HIWORD_MASK);
    rockchip_register_restart_notifier(ctx, RK3288_GLB_SRST_FST, rk3288_clk_shutdown);
    register_syscore(&rk3288_clk_syscore);
    rockchip_clk_of_add_provider(np, ctx);
}

unsafe fn rk3288_clk_init(np: *mut device_node) { rk3288_common_init(np, Rk3288Variant::RK3288_CRU); }
unsafe fn rk3288w_clk_init(np: *mut device_node) { rk3288_common_init(np, Rk3288Variant::RK3288W_CRU); }
CLK_OF_DECLARE!(rk3288_cru, "rockchip,rk3288-cru", rk3288_clk_init);
CLK_OF_DECLARE!(rk3288w_cru, "rockchip,rk3288w-cru", rk3288w_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
