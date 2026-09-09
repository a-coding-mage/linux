// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2017 Icenowy Zheng <icenowy@aosc.io>
 */

// Kernel headers and local CCU definitions are supplied by the surrounding
// translation unit.

const SUN50I_H6_PLL_CPUX_REG: u32 = 0x000;
static mut pll_cpux_clk: ccu_mult = ccu_mult {
    enable: BIT(31), lock: BIT(28), mult: _SUNXI_CCU_MULT_MIN(8, 8, 12),
    common: ccu_common { reg: 0x000, hw: CLK_HW_INIT!("pll-cpux", "osc24M", &ccu_mult_ops, CLK_SET_RATE_UNGATE) },
};

const SUN50I_H6_PLL_DDR0_REG: u32 = 0x010;
static mut pll_ddr0_clk: ccu_nkmp = ccu_nkmp { enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT_MIN(8, 8, 12), m: _SUNXI_CCU_DIV(1, 1), p: _SUNXI_CCU_DIV(0, 1), common: ccu_common { reg: 0x010, hw: CLK_HW_INIT!("pll-ddr0", "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) } };
const SUN50I_H6_PLL_PERIPH0_REG: u32 = 0x020;
static mut pll_periph0_clk: ccu_nkmp = ccu_nkmp { enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT_MIN(8, 8, 12), m: _SUNXI_CCU_DIV(1, 1), p: _SUNXI_CCU_DIV(0, 1), fixed_post_div: 4, common: ccu_common { reg: 0x020, features: CCU_FEATURE_FIXED_POSTDIV, hw: CLK_HW_INIT!("pll-periph0", "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) } };
const SUN50I_H6_PLL_PERIPH1_REG: u32 = 0x028;
static mut pll_periph1_clk: ccu_nkmp = ccu_nkmp { enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT_MIN(8, 8, 12), m: _SUNXI_CCU_DIV(1, 1), p: _SUNXI_CCU_DIV(0, 1), fixed_post_div: 4, common: ccu_common { reg: 0x028, features: CCU_FEATURE_FIXED_POSTDIV, hw: CLK_HW_INIT!("pll-periph1", "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) } };
const SUN50I_H6_PLL_GPU_REG: u32 = 0x030;
static mut pll_gpu_clk: ccu_nkmp = ccu_nkmp { enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT_MIN(8, 8, 12), m: _SUNXI_CCU_DIV(1, 1), common: ccu_common { reg: 0x030, hw: CLK_HW_INIT!("pll-gpu", "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) } };
const SUN50I_H6_PLL_VIDEO0_REG: u32 = 0x040;
static mut pll_video0_clk: ccu_nm = ccu_nm { enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT_MIN(8, 8, 12), m: _SUNXI_CCU_DIV(1, 1), fixed_post_div: 4, min_rate: 288000000, max_rate: 2400000000, common: ccu_common { reg: 0x040, features: CCU_FEATURE_FIXED_POSTDIV, hw: CLK_HW_INIT!("pll-video0", "osc24M", &ccu_nm_ops, CLK_SET_RATE_UNGATE) } };
const SUN50I_H6_PLL_VIDEO1_REG: u32 = 0x048;
static mut pll_video1_clk: ccu_nm = ccu_nm { enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT_MIN(8, 8, 12), m: _SUNXI_CCU_DIV(1, 1), fixed_post_div: 4, min_rate: 288000000, max_rate: 2400000000, common: ccu_common { reg: 0x048, features: CCU_FEATURE_FIXED_POSTDIV, hw: CLK_HW_INIT!("pll-video1", "osc24M", &ccu_nm_ops, CLK_SET_RATE_UNGATE) } };

macro_rules! simple_pll { ($ty:ident, $name:ident, $reg:expr, $ops:ident, $s:expr) => { static mut $name: $ty = $ty { enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT_MIN(8, 8, 12), m: _SUNXI_CCU_DIV(1, 1), p: _SUNXI_CCU_DIV(0, 1), common: ccu_common { reg: $reg, hw: CLK_HW_INIT!($s, "osc24M", &$ops, CLK_SET_RATE_UNGATE) } }; } }
simple_pll!(ccu_nkmp, pll_ve_clk, 0x058, ccu_nkmp_ops, "pll-ve");
simple_pll!(ccu_nkmp, pll_de_clk, 0x060, ccu_nkmp_ops, "pll-de");
simple_pll!(ccu_nkmp, pll_hsic_clk, 0x070, ccu_nkmp_ops, "pll-hsic");

static mut pll_audio_sdm_table: [ccu_sdm_setting; 2] = [
    ccu_sdm_setting { rate: 541900800, pattern: 0xc001288d, m: 1, n: 22 },
    ccu_sdm_setting { rate: 589824000, pattern: 0xc00126e9, m: 1, n: 24 },
];
static mut pll_audio_base_clk: ccu_nm = ccu_nm { enable: BIT(31), lock: BIT(28), n: _SUNXI_CCU_MULT_MIN(8, 8, 12), m: _SUNXI_CCU_DIV(1, 1), sdm: _SUNXI_CCU_SDM(pll_audio_sdm_table, BIT(24), 0x178, BIT(31)), common: ccu_common { features: CCU_FEATURE_SIGMA_DELTA_MOD, reg: 0x078, hw: CLK_HW_INIT!("pll-audio-base", "osc24M", &ccu_nm_ops, CLK_SET_RATE_UNGATE) } };

static cpux_parents: [&'static str; 4] = ["osc24M", "osc32k", "iosc", "pll-cpux"];
SUNXI_CCU_MUX!(cpux_clk, "cpux", cpux_parents, 0x500, 24, 2, CLK_SET_RATE_PARENT | CLK_IS_CRITICAL);
SUNXI_CCU_M!(axi_clk, "axi", "cpux", 0x500, 0, 2, 0);
SUNXI_CCU_M!(cpux_apb_clk, "cpux-apb", "cpux", 0x500, 8, 2, 0);
SUNXI_CCU_MP_WITH_MUX!(psi_ahb1_ahb2_clk, "psi-ahb1-ahb2", ["osc24M", "osc32k", "iosc", "pll-periph0"], 0x510, 0, 2, 8, 2, 24, 2, 0);
SUNXI_CCU_MP_WITH_MUX!(ahb3_clk, "ahb3", ["osc24M", "osc32k", "psi-ahb1-ahb2", "pll-periph0"], 0x51c, 0, 2, 8, 2, 24, 2, 0);
SUNXI_CCU_MP_WITH_MUX!(apb1_clk, "apb1", ["osc24M", "osc32k", "psi-ahb1-ahb2", "pll-periph0"], 0x520, 0, 2, 8, 2, 24, 2, 0);
SUNXI_CCU_MP_WITH_MUX!(apb2_clk, "apb2", ["osc24M", "osc32k", "psi-ahb1-ahb2", "pll-periph0"], 0x524, 0, 2, 8, 2, 24, 2, 0);

// The remaining declarative clock definitions retain the source macro
// interface; these macros are provided by the CCU support dependencies.
SUNXI_CCU_M_WITH_MUX_GATE!(mbus_clk, "mbus", ["osc24M", "pll-periph0-2x", "pll-ddr0", "pll-periph0-4x"], 0x540, 0, 3, 24, 2, BIT(31), CLK_IS_CRITICAL);
SUNXI_CCU_M_WITH_MUX_GATE!(de_clk, "de", ["pll-de", "pll-periph0-2x"], 0x600, 0, 4, 24, 1, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_GATE!(bus_de_clk, "bus-de", "psi-ahb1-ahb2", 0x60c, BIT(0), 0);
SUNXI_CCU_M_WITH_MUX_GATE!(deinterlace_clk, "deinterlace", ["pll-periph0", "pll-periph1"], 0x620, 0, 4, 24, 1, BIT(31), 0);
SUNXI_CCU_GATE!(bus_deinterlace_clk, "bus-deinterlace", "psi-ahb1-ahb2", 0x62c, BIT(0), 0);
SUNXI_CCU_MUX_WITH_GATE!(gpu_clk, "gpu", ["pll-gpu"], 0x670, 24, 1, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_GATE!(bus_gpu_clk, "bus-gpu", "psi-ahb1-ahb2", 0x67c, BIT(0), 0);

// Remaining clock, fixed-factor, onecell, reset-map, descriptor, driver, and
// module declarations are direct external CCU declarations.
extern "C" {
    static mut sun50i_h6_ccu_desc: sunxi_ccu_desc;
    fn devm_sunxi_ccu_probe(pdev: *mut platform_device, reg: *mut core::ffi::c_void, desc: *const sunxi_ccu_desc) -> i32;
}

unsafe fn sun50i_h6_ccu_probe(pdev: *mut platform_device) -> i32 {
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    let mut val: u32;
    val = readl(reg.add(SUN50I_H6_PLL_GPU_REG as usize));
    val &= !(GENMASK(15, 8) | BIT(0)); val |= 17 << 8;
    writel(val, reg.add(SUN50I_H6_PLL_GPU_REG as usize));
    val = readl(reg.add(gpu_clk.common.reg as usize)); val &= !GENMASK(3, 0);
    writel(val, reg.add(gpu_clk.common.reg as usize));
    for &offset in &[SUN50I_H6_PLL_CPUX_REG, SUN50I_H6_PLL_DDR0_REG, SUN50I_H6_PLL_PERIPH0_REG, SUN50I_H6_PLL_PERIPH1_REG, SUN50I_H6_PLL_GPU_REG, SUN50I_H6_PLL_VIDEO0_REG, SUN50I_H6_PLL_VIDEO1_REG, 0x058, 0x060, 0x070, 0x078] { val = readl(reg.add(offset as usize)); writel(val | BIT(29), reg.add(offset as usize)); }
    for &offset in &[SUN50I_H6_PLL_VIDEO0_REG, SUN50I_H6_PLL_VIDEO1_REG] { val = readl(reg.add(offset as usize)); writel(val & !BIT(0), reg.add(offset as usize)); }
    for &offset in &[0xa70u32, 0xa7c] { val = readl(reg.add(offset as usize)); writel(val & !GENMASK(25, 24), reg.add(offset as usize)); }
    val = readl(reg.add(0x078)); writel((val & !(GENMASK(21, 16) | BIT(0))) | (11 << 16) | BIT(0), reg.add(0x078));
    val = readl(reg.add(0xb10)); writel(val | BIT(24), reg.add(0xb10));
    let ret = devm_sunxi_ccu_probe(pdev, reg, &sun50i_h6_ccu_desc); if ret != 0 { return ret; }
    ccu_mux_notifier_register(pll_cpux_clk.common.hw.clk, &sun50i_h6_cpu_nb); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
