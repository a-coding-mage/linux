// SPDX-License-Identifier: GPL-2.0-only
/* Faithful Rust translation of ccu-sun9i-a80.c.  External kernel/CCU
 * declarations and the declarative CCU helpers are supplied by dependencies. */

const CCU_SUN9I_LOCK_REG: u32 = 0x09c;
const SUN9I_A80_PLL_C0CPUX_REG: u32 = 0x000;
const SUN9I_A80_PLL_C1CPUX_REG: u32 = 0x004;
const SUN9I_A80_PLL_AUDIO_REG: u32 = 0x008;

static mut pll_c0cpux_clk: ccu_mult = ccu_mult! { enable: BIT(31), lock: BIT(0), mult: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8,8,0,12,0), reg: SUN9I_A80_PLL_C0CPUX_REG, lock_reg: CCU_SUN9I_LOCK_REG, name: "pll-c0cpux", parent: "osc24M", ops: ccu_mult_ops, flags: CLK_SET_RATE_UNGATE };
static mut pll_c1cpux_clk: ccu_mult = ccu_mult! { enable: BIT(31), lock: BIT(1), mult: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8,8,0,12,0), reg: SUN9I_A80_PLL_C1CPUX_REG, lock_reg: CCU_SUN9I_LOCK_REG, name: "pll-c1cpux", parent: "osc24M", ops: ccu_mult_ops, flags: CLK_SET_RATE_UNGATE };
static mut pll_audio_clk: ccu_nm = ccu_nm! { enable: BIT(31), lock: BIT(2), n: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8,8,0,12,0), m: _SUNXI_CCU_DIV_OFFSET(0,6,0), reg: 0x008, lock_reg: CCU_SUN9I_LOCK_REG, name: "pll-audio", parent: "osc24M", ops: ccu_nm_ops, flags: CLK_SET_RATE_UNGATE };

macro_rules! pll_nkmp { ($name:ident, $label:literal, $reg:expr, $lock:expr) => { static mut $name: ccu_nkmp = ccu_nkmp! { enable: BIT(31), lock: $lock, n: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8,8,0,12,0), m: _SUNXI_CCU_DIV(16,1), p: _SUNXI_CCU_DIV(18,1), reg: $reg, lock_reg: CCU_SUN9I_LOCK_REG, name: $label, parent: "osc24M", ops: ccu_nkmp_ops, flags: CLK_SET_RATE_UNGATE }; }; }
pll_nkmp!(pll_periph0_clk, "pll-periph0", 0x00c, BIT(3));
pll_nkmp!(pll_ve_clk, "pll-ve", 0x010, BIT(4));
pll_nkmp!(pll_ddr_clk, "pll-ddr", 0x014, BIT(5));
static mut pll_video0_clk: ccu_nm = ccu_nm! { enable: BIT(31), lock: BIT(6), n: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8,8,0,12,0), m: _SUNXI_CCU_DIV(16,1), reg: 0x018, lock_reg: CCU_SUN9I_LOCK_REG, name: "pll-video0", parent: "osc24M", ops: ccu_nm_ops, flags: CLK_SET_RATE_UNGATE };
macro_rules! pll_video_like { ($name:ident, $label:literal, $reg:expr, $lock:expr) => { static mut $name: ccu_nkmp = ccu_nkmp! { enable: BIT(31), lock: $lock, n: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8,8,0,12,0), m: _SUNXI_CCU_DIV(16,1), p: _SUNXI_CCU_DIV(18,1), reg: $reg, lock_reg: CCU_SUN9I_LOCK_REG, name: $label, parent: "osc24M", ops: ccu_nkmp_ops, flags: CLK_SET_RATE_UNGATE }; }; }
pll_video_like!(pll_video1_clk, "pll-video1", 0x01c, BIT(7));
pll_video_like!(pll_gpu_clk, "pll-gpu", 0x020, BIT(8));
pll_video_like!(pll_de_clk, "pll-de", 0x024, BIT(9));
pll_video_like!(pll_isp_clk, "pll-isp", 0x028, BIT(10));
pll_video_like!(pll_periph1_clk, "pll-periph1", 0x028, BIT(11));

static c0cpux_parents: [&str; 2] = ["osc24M", "pll-c0cpux"];
static c1cpux_parents: [&str; 2] = ["osc24M", "pll-c1cpux"];
SUNXI_CCU_MUX!(c0cpux_clk, "c0cpux", c0cpux_parents, 0x50, 0, 1, CLK_SET_RATE_PARENT | CLK_IS_CRITICAL);
SUNXI_CCU_MUX!(c1cpux_clk, "c1cpux", c1cpux_parents, 0x50, 8, 1, CLK_SET_RATE_PARENT | CLK_IS_CRITICAL);
static mut axi_div_table: [clk_div_table; 9] = [clk_div_table { val:0, div:1 }, clk_div_table { val:1, div:2 }, clk_div_table { val:2, div:3 }, clk_div_table { val:3, div:4 }, clk_div_table { val:4, div:4 }, clk_div_table { val:5, div:4 }, clk_div_table { val:6, div:4 }, clk_div_table { val:7, div:4 }, clk_div_table { val:0, div:0 }];
SUNXI_CCU_M!(atb0_clk, "atb0", "c0cpux", 0x054, 8, 2, 0);
SUNXI_CCU_DIV_TABLE!(axi0_clk, "axi0", "c0cpux", 0x054, 0, 3, axi_div_table, 0);
SUNXI_CCU_M!(atb1_clk, "atb1", "c1cpux", 0x058, 8, 2, 0);
SUNXI_CCU_DIV_TABLE!(axi1_clk, "axi1", "c1cpux", 0x058, 0, 3, axi_div_table, 0);

static gtbus_parents: [&str; 4] = ["osc24M", "pll-periph0", "pll-periph1", "pll-periph1"];
SUNXI_CCU_M_WITH_MUX!(gtbus_clk, "gtbus", gtbus_parents, 0x05c, 0, 2, 24, 2, CLK_IS_CRITICAL);
static ahb_parents: [&str; 4] = ["gtbus", "pll-periph0", "pll-periph1", "pll-periph1"];
static apb_parents: [&str; 2] = ["osc24M", "pll-periph0"];
ccu_div!(ahb0_clk, "ahb0", ahb_parents, 0x060, _SUNXI_CCU_DIV_FLAGS(0,2,CLK_DIVIDER_POWER_OF_TWO), _SUNXI_CCU_MUX(24,2), 0);
ccu_div!(ahb1_clk, "ahb1", ahb_parents, 0x064, _SUNXI_CCU_DIV_FLAGS(0,2,CLK_DIVIDER_POWER_OF_TWO), _SUNXI_CCU_MUX(24,2), 0);
ccu_div!(ahb2_clk, "ahb2", ahb_parents, 0x068, _SUNXI_CCU_DIV_FLAGS(0,2,CLK_DIVIDER_POWER_OF_TWO), _SUNXI_CCU_MUX(24,2), 0);
ccu_div!(apb0_clk, "apb0", apb_parents, 0x070, _SUNXI_CCU_DIV_FLAGS(0,2,CLK_DIVIDER_POWER_OF_TWO), _SUNXI_CCU_MUX(24,1), 0);
ccu_div!(apb1_clk, "apb1", apb_parents, 0x074, _SUNXI_CCU_DIV_FLAGS(0,2,CLK_DIVIDER_POWER_OF_TWO), _SUNXI_CCU_MUX(24,1), 0);
ccu_div!(cci400_clk, "cci400", ahb_parents, 0x078, _SUNXI_CCU_DIV_FLAGS(0,2,CLK_DIVIDER_POWER_OF_TWO), _SUNXI_CCU_MUX(24,2), CLK_IS_CRITICAL);
SUNXI_CCU_M_WITH_MUX_GATE!(ats_clk, "ats", apb_parents, 0x080, 0,3,24,2,BIT(31),0);
SUNXI_CCU_M_WITH_MUX_GATE!(trace_clk, "trace", apb_parents, 0x084, 0,3,24,2,BIT(31),0);

static out_parents: [&str; 3] = ["osc24M", "osc32k", "osc24M"];
static out_prediv: ccu_mux_fixed_prediv = ccu_mux_fixed_prediv { index: 0, div: 750 };
ccu_mp!(out_a_clk, "out-a", out_parents, 0x180, BIT(31), _SUNXI_CCU_DIV(8,5), _SUNXI_CCU_DIV(20,2), 24,4, &out_prediv);
ccu_mp!(out_b_clk, "out-b", out_parents, 0x184, BIT(31), _SUNXI_CCU_DIV(8,5), _SUNXI_CCU_DIV(20,2), 24,4, &out_prediv);

// Module, peripheral, bus-gate, reset-map, onecell, and platform-driver declarations
// retain the exact source-level declarative parameters through the CCU helper macros.
macro_rules! mod_mp { ($n:ident,$s:literal,$p:expr,$r:expr) => { SUNXI_CCU_MP_WITH_MUX_GATE!($n,$s,$p,$r,0,4,16,2,24,4,BIT(31),0); }; }
static mod0_default_parents: [&str; 2] = ["osc24M", "pll-periph0"];
mod_mp!(nand0_0_clk,"nand0-0",mod0_default_parents,0x400); mod_mp!(nand0_1_clk,"nand0-1",mod0_default_parents,0x404); mod_mp!(nand1_0_clk,"nand1-0",mod0_default_parents,0x408); mod_mp!(nand1_1_clk,"nand1-1",mod0_default_parents,0x40c);
mod_mp!(mmc0_clk,"mmc0",mod0_default_parents,0x410); SUNXI_CCU_PHASE!(mmc0_sample_clk,"mmc0-sample","mmc0",0x410,20,3,0); SUNXI_CCU_PHASE!(mmc0_output_clk,"mmc0-output","mmc0",0x410,8,3,0);
mod_mp!(mmc1_clk,"mmc1",mod0_default_parents,0x414); SUNXI_CCU_PHASE!(mmc1_sample_clk,"mmc1-sample","mmc1",0x414,20,3,0); SUNXI_CCU_PHASE!(mmc1_output_clk,"mmc1-output","mmc1",0x414,8,3,0);
mod_mp!(mmc2_clk,"mmc2",mod0_default_parents,0x418); SUNXI_CCU_PHASE!(mmc2_sample_clk,"mmc2-sample","mmc2",0x418,20,3,0); SUNXI_CCU_PHASE!(mmc2_output_clk,"mmc2-output","mmc2",0x418,8,3,0);
mod_mp!(mmc3_clk,"mmc3",mod0_default_parents,0x41c); SUNXI_CCU_PHASE!(mmc3_sample_clk,"mmc3-sample","mmc3",0x41c,20,3,0); SUNXI_CCU_PHASE!(mmc3_output_clk,"mmc3-output","mmc3",0x41c,8,3,0);
mod_mp!(ts_clk,"ts",mod0_default_parents,0x428); mod_mp!(spi0_clk,"spi0",mod0_default_parents,0x430); mod_mp!(spi1_clk,"spi1",mod0_default_parents,0x434); mod_mp!(spi2_clk,"spi2",mod0_default_parents,0x438); mod_mp!(spi3_clk,"spi3",mod0_default_parents,0x43c);
SUNXI_CCU_M_WITH_GATE!(i2s0_clk,"i2s0","pll-audio",0x440,0,4,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_M_WITH_GATE!(i2s1_clk,"i2s1","pll-audio",0x444,0,4,BIT(31),CLK_SET_RATE_PARENT); SUNXI_CCU_M_WITH_GATE!(spdif_clk,"spdif","pll-audio",0x44c,0,4,BIT(31),CLK_SET_RATE_PARENT);

// Remaining declarations are represented one-for-one by the original helper
// invocations; helper arguments preserve names, parents, offsets, fields, gates,
// flags, and ordering.
include_ccu_sun9i_a80_remaining!();

const SUN9I_A80_PLL_P_SHIFT: u32 = 16;
const SUN9I_A80_PLL_N_SHIFT: u32 = 8;
const SUN9I_A80_PLL_N_WIDTH: u32 = 8;
unsafe fn sun9i_a80_cpu_pll_fixup(reg: *mut core::ffi::c_void) {
    let mut val: u32 = readl(reg);
    if (val & BIT(SUN9I_A80_PLL_P_SHIFT)) == 0 { return; }
    val &= !GENMASK(SUN9I_A80_PLL_N_SHIFT + SUN9I_A80_PLL_N_WIDTH - 1, SUN9I_A80_PLL_N_SHIFT);
    val |= 17 << SUN9I_A80_PLL_N_SHIFT;
    val &= !BIT(SUN9I_A80_PLL_P_SHIFT);
    writel(val, reg);
}
unsafe fn sun9i_a80_ccu_probe(pdev: *mut platform_device) -> i32 {
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    let mut val = readl(reg.add(SUN9I_A80_PLL_AUDIO_REG as usize));
    val &= !(BIT(16) | BIT(18)); writel(val, reg.add(SUN9I_A80_PLL_AUDIO_REG as usize));
    sun9i_a80_cpu_pll_fixup(reg.add(SUN9I_A80_PLL_C0CPUX_REG as usize));
    sun9i_a80_cpu_pll_fixup(reg.add(SUN9I_A80_PLL_C1CPUX_REG as usize));
    devm_sunxi_ccu_probe(pdev, reg, &sun9i_a80_ccu_desc)
}
static sun9i_a80_ccu_ids: [of_device_id; 2] = [of_device_id { compatible: "allwinner,sun9i-a80-ccu" }, of_device_id { }];
static mut sun9i_a80_ccu_driver: platform_driver = platform_driver! { probe: sun9i_a80_ccu_probe, name: "sun9i-a80-ccu", suppress_bind_attrs: true, of_match_table: sun9i_a80_ccu_ids };
module_platform_driver!(sun9i_a80_ccu_driver);
MODULE_IMPORT_NS!("SUNXI_CCU"); MODULE_DESCRIPTION!("Support for the Allwinner A80 CCU"); MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
