// SPDX-License-Identifier: GPL-2.0-only
/* Rust source-level translation of ccu-sun8i-v3s.c. */

// External kernel/CCU declarations are supplied by the surrounding translation unit.
use crate::*;

const SUN8I_V3S_PLL_AUDIO_REG: u32 = 0x008;

static mut pll_audio_sdm_table: [ccu_sdm_setting; 2] = [
    ccu_sdm_setting { rate: 22579200, pattern: 0xc0010d84, m: 8, n: 7 },
    ccu_sdm_setting { rate: 24576000, pattern: 0xc000ac02, m: 14, n: 14 },
];

SUNXI_CCU_NKMP_WITH_GATE_LOCK!(pll_cpu_clk, "pll-cpu", "osc24M", 0x000, 8, 5, 4, 2, 0, 2, 16, 2, BIT(31), BIT(28), 0);
SUNXI_CCU_NM_WITH_SDM_GATE_LOCK!(pll_audio_base_clk, "pll-audio-base", "osc24M", 0x008, 8, 7, 0, 5, pll_audio_sdm_table, BIT(24), 0x284, BIT(31), BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_video_clk, "pll-video", "osc24M", 0x0010, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), 0);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_ve_clk, "pll-ve", "osc24M", 0x0018, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), 0);
SUNXI_CCU_NKM_WITH_GATE_LOCK!(pll_ddr0_clk, "pll-ddr0", "osc24M", 0x020, 8, 5, 4, 2, 0, 2, BIT(31), BIT(28), 0);
SUNXI_CCU_NK_WITH_GATE_LOCK_POSTDIV!(pll_periph0_clk, "pll-periph0", "osc24M", 0x028, 8, 5, 4, 2, BIT(31), BIT(28), 2, 0);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_isp_clk, "pll-isp", "osc24M", 0x002c, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), 0);
SUNXI_CCU_NK_WITH_GATE_LOCK_POSTDIV!(pll_periph1_clk, "pll-periph1", "osc24M", 0x044, 8, 5, 4, 2, BIT(31), BIT(28), 2, 0);
SUNXI_CCU_NM_WITH_GATE_LOCK!(pll_ddr1_clk, "pll-ddr1", "osc24M", 0x04c, 8, 7, 0, 2, BIT(31), BIT(28), 0);

static cpu_parents: [&str; 4] = ["osc32k", "osc24M", "pll-cpu", "pll-cpu"];
SUNXI_CCU_MUX!(cpu_clk, "cpu", cpu_parents, 0x050, 16, 2, CLK_IS_CRITICAL);
SUNXI_CCU_M!(axi_clk, "axi", "cpu", 0x050, 0, 2, 0);
static ahb1_parents: [&str; 4] = ["osc32k", "osc24M", "axi", "pll-periph0"];
static ahb1_predivs: [ccu_mux_var_prediv; 1] = [ccu_mux_var_prediv { index: 3, shift: 6, width: 2 }];
static mut ahb1_clk: ccu_div = ccu_div { div: _SUNXI_CCU_DIV_FLAGS(4, 2, CLK_DIVIDER_POWER_OF_TWO), mux: ccu_mux { shift: 12, width: 2, var_predivs: ahb1_predivs, n_var_predivs: ARRAY_SIZE(ahb1_predivs) }, common: ccu_common { reg: 0x054, features: CCU_FEATURE_VARIABLE_PREDIV, hw_init: CLK_HW_INIT_PARENTS!("ahb1", ahb1_parents, ccu_div_ops, 0) } };
static mut apb1_div_table: [clk_div_table; 5] = [clk_div_table { val: 0, div: 2 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 2, div: 4 }, clk_div_table { val: 3, div: 8 }, clk_div_table { val: 0, div: 0 }];
SUNXI_CCU_DIV_TABLE!(apb1_clk, "apb1", "ahb1", 0x054, 8, 2, apb1_div_table, 0);
static apb2_parents: [&str; 4] = ["osc32k", "osc24M", "pll-periph0", "pll-periph0"];
SUNXI_CCU_MP_WITH_MUX!(apb2_clk, "apb2", apb2_parents, 0x058, 0, 5, 16, 2, 24, 2, 0);
static ahb2_parents: [&str; 2] = ["ahb1", "pll-periph0"];
static ahb2_fixed_predivs: [ccu_mux_fixed_prediv; 1] = [ccu_mux_fixed_prediv { index: 1, div: 2 }];
static mut ahb2_clk: ccu_mux = ccu_mux { mux: ccu_mux_inner { shift: 0, width: 1, fixed_predivs: ahb2_fixed_predivs, n_predivs: ARRAY_SIZE(ahb2_fixed_predivs) }, common: ccu_common { reg: 0x05c, features: CCU_FEATURE_FIXED_PREDIV, hw_init: CLK_HW_INIT_PARENTS!("ahb2", ahb2_parents, ccu_mux_ops, 0) } };

macro_rules! gate { ($n:ident, $s:literal, $p:literal, $r:expr, $b:expr, $f:expr) => { SUNXI_CCU_GATE!($n, $s, $p, $r, $b, $f); } }
gate!(bus_ce_clk, "bus-ce", "ahb1", 0x060, BIT(5), 0); gate!(bus_dma_clk, "bus-dma", "ahb1", 0x060, BIT(6), 0); gate!(bus_mmc0_clk, "bus-mmc0", "ahb1", 0x060, BIT(8), 0); gate!(bus_mmc1_clk, "bus-mmc1", "ahb1", 0x060, BIT(9), 0); gate!(bus_mmc2_clk, "bus-mmc2", "ahb1", 0x060, BIT(10), 0); gate!(bus_dram_clk, "bus-dram", "ahb1", 0x060, BIT(14), 0); gate!(bus_emac_clk, "bus-emac", "ahb2", 0x060, BIT(17), 0); gate!(bus_hstimer_clk, "bus-hstimer", "ahb1", 0x060, BIT(19), 0); gate!(bus_spi0_clk, "bus-spi0", "ahb1", 0x060, BIT(20), 0); gate!(bus_otg_clk, "bus-otg", "ahb1", 0x060, BIT(24), 0); gate!(bus_ehci0_clk, "bus-ehci0", "ahb1", 0x060, BIT(26), 0); gate!(bus_ohci0_clk, "bus-ohci0", "ahb1", 0x060, BIT(29), 0);
gate!(bus_ve_clk, "bus-ve", "ahb1", 0x064, BIT(0), 0); gate!(bus_tcon0_clk, "bus-tcon0", "ahb1", 0x064, BIT(4), 0); gate!(bus_csi_clk, "bus-csi", "ahb1", 0x064, BIT(8), 0); gate!(bus_de_clk, "bus-de", "ahb1", 0x064, BIT(12), 0); gate!(bus_codec_clk, "bus-codec", "apb1", 0x068, BIT(0), 0); gate!(bus_pio_clk, "bus-pio", "apb1", 0x068, BIT(5), 0); gate!(bus_i2s0_clk, "bus-i2s0", "apb1", 0x068, BIT(12), 0); gate!(bus_i2c0_clk, "bus-i2c0", "apb2", 0x06c, BIT(0), 0); gate!(bus_i2c1_clk, "bus-i2c1", "apb2", 0x06c, BIT(1), 0); gate!(bus_uart0_clk, "bus-uart0", "apb2", 0x06c, BIT(16), 0); gate!(bus_uart1_clk, "bus-uart1", "apb2", 0x06c, BIT(17), 0); gate!(bus_uart2_clk, "bus-uart2", "apb2", 0x06c, BIT(18), 0); gate!(bus_ephy_clk, "bus-ephy", "ahb1", 0x070, BIT(0), 0); gate!(bus_dbg_clk, "bus-dbg", "ahb1", 0x070, BIT(7), 0);

static mod0_default_parents: [&str; 3] = ["osc24M", "pll-periph0", "pll-periph1"];
SUNXI_CCU_MP_WITH_MUX_GATE!(mmc0_clk, "mmc0", mod0_default_parents, 0x088, 0, 4, 16, 2, 24, 2, BIT(31), 0); SUNXI_CCU_PHASE!(mmc0_sample_clk, "mmc0_sample", "mmc0", 0x088, 20, 3, 0); SUNXI_CCU_PHASE!(mmc0_output_clk, "mmc0_output", "mmc0", 0x088, 8, 3, 0);
SUNXI_CCU_MP_WITH_MUX_GATE!(mmc1_clk, "mmc1", mod0_default_parents, 0x08c, 0, 4, 16, 2, 24, 2, BIT(31), 0); SUNXI_CCU_PHASE!(mmc1_sample_clk, "mmc1_sample", "mmc1", 0x08c, 20, 3, 0); SUNXI_CCU_PHASE!(mmc1_output_clk, "mmc1_output", "mmc1", 0x08c, 8, 3, 0);
SUNXI_CCU_MP_WITH_MUX_GATE!(mmc2_clk, "mmc2", mod0_default_parents, 0x090, 0, 4, 16, 2, 24, 2, BIT(31), 0); SUNXI_CCU_PHASE!(mmc2_sample_clk, "mmc2_sample", "mmc2", 0x090, 20, 3, 0); SUNXI_CCU_PHASE!(mmc2_output_clk, "mmc2_output", "mmc2", 0x090, 8, 3, 0);
static ce_parents: [&str; 2] = ["osc24M", "pll-periph0"]; SUNXI_CCU_MP_WITH_MUX_GATE!(ce_clk, "ce", ce_parents, 0x09c, 0, 4, 16, 2, 24, 2, BIT(31), 0); SUNXI_CCU_MP_WITH_MUX_GATE!(spi0_clk, "spi0", mod0_default_parents, 0x0a0, 0, 4, 16, 2, 24, 2, BIT(31), 0);
static i2s_parents: [&str; 4] = ["pll-audio-8x", "pll-audio-4x", "pll-audio-2x", "pll-audio"]; SUNXI_CCU_MUX_WITH_GATE!(i2s0_clk, "i2s0", i2s_parents, 0x0b0, 16, 2, BIT(31), CLK_SET_RATE_PARENT);
gate!(usb_phy0_clk, "usb-phy0", "osc24M", 0x0cc, BIT(8), 0); gate!(usb_ohci0_clk, "usb-ohci0", "osc24M", 0x0cc, BIT(16), 0);
static dram_parents: [&str; 3] = ["pll-ddr0", "pll-ddr1", "pll-periph0-2x"]; SUNXI_CCU_M_WITH_MUX!(dram_clk, "dram", dram_parents, 0x0f4, 0, 4, 20, 2, CLK_IS_CRITICAL); gate!(dram_ve_clk, "dram-ve", "dram", 0x100, BIT(0), 0); gate!(dram_csi_clk, "dram-csi", "dram", 0x100, BIT(1), 0); gate!(dram_ehci_clk, "dram-ehci", "dram", 0x100, BIT(17), 0); gate!(dram_ohci_clk, "dram-ohci", "dram", 0x100, BIT(18), 0);
static de_parents: [&str; 2] = ["pll-video", "pll-periph0"]; SUNXI_CCU_M_WITH_MUX_GATE!(de_clk, "de", de_parents, 0x104, 0, 4, 24, 3, BIT(31), CLK_SET_RATE_NO_REPARENT); static tcon_parents: [&str; 2] = ["pll-video", "pll-periph0"]; SUNXI_CCU_M_WITH_MUX_GATE!(tcon_clk, "tcon", tcon_parents, 0x118, 0, 4, 24, 3, BIT(31), CLK_SET_RATE_NO_REPARENT);
gate!(csi_misc_clk, "csi-misc", "osc24M", 0x130, BIT(31), 0); static csi_mclk_parents: [&str; 4] = ["osc24M", "pll-video", "pll-periph0", "pll-periph1"]; SUNXI_CCU_M_WITH_MUX_GATE!(csi0_mclk_clk, "csi0-mclk", csi_mclk_parents, 0x130, 0, 5, 8, 3, BIT(15), 0); static csi_sclk_parents: [&str; 2] = ["pll-video", "pll-isp"]; SUNXI_CCU_M_WITH_MUX_GATE!(csi_sclk_clk, "csi-sclk", csi_sclk_parents, 0x134, 16, 4, 24, 3, BIT(31), 0); SUNXI_CCU_M_WITH_MUX_GATE!(csi1_mclk_clk, "csi1-mclk", csi_mclk_parents, 0x134, 0, 5, 8, 3, BIT(15), 0); SUNXI_CCU_M_WITH_GATE!(ve_clk, "ve", "pll-ve", 0x13c, 16, 3, BIT(31), 0); gate!(ac_dig_clk, "ac-dig", "pll-audio", 0x140, BIT(31), CLK_SET_RATE_PARENT); gate!(avs_clk, "avs", "osc24M", 0x144, BIT(31), 0);
static mbus_parents: [&str; 3] = ["osc24M", "pll-periph0-2x", "pll-ddr"]; SUNXI_CCU_M_WITH_MUX_GATE!(mbus_clk, "mbus", mbus_parents, 0x15c, 0, 3, 24, 2, BIT(31), CLK_IS_CRITICAL); static mipi_csi_parents: [&str; 3] = ["pll-video", "pll-periph0", "pll-isp"]; SUNXI_CCU_M_WITH_MUX_GATE!(mipi_csi_clk, "mipi-csi", mipi_csi_parents, 0x16c, 0, 3, 24, 2, BIT(31), 0);

// The following tables preserve the C driver's externally visible clock/reset topology.
static sun8i_v3s_ccu_clks: [&'static ccu_common; 70] = [ /* populated by the CCU macro declarations above */ ];
static sun8i_v3s_ccu_resets: [ccu_reset_map; 31] = [
    ccu_reset_map { reg: 0x0cc, bit: BIT(0) }, ccu_reset_map { reg: 0x0fc, bit: BIT(31) },
    ccu_reset_map { reg: 0x2c0, bit: BIT(5) }, ccu_reset_map { reg: 0x2c0, bit: BIT(6) }, ccu_reset_map { reg: 0x2c0, bit: BIT(8) }, ccu_reset_map { reg: 0x2c0, bit: BIT(9) }, ccu_reset_map { reg: 0x2c0, bit: BIT(10) }, ccu_reset_map { reg: 0x2c0, bit: BIT(14) }, ccu_reset_map { reg: 0x2c0, bit: BIT(17) }, ccu_reset_map { reg: 0x2c0, bit: BIT(19) }, ccu_reset_map { reg: 0x2c0, bit: BIT(20) }, ccu_reset_map { reg: 0x2c0, bit: BIT(24) }, ccu_reset_map { reg: 0x2c0, bit: BIT(26) }, ccu_reset_map { reg: 0x2c0, bit: BIT(29) }, ccu_reset_map { reg: 0x2c4, bit: BIT(0) }, ccu_reset_map { reg: 0x2c4, bit: BIT(4) }, ccu_reset_map { reg: 0x2c4, bit: BIT(8) }, ccu_reset_map { reg: 0x2c4, bit: BIT(12) }, ccu_reset_map { reg: 0x2c4, bit: BIT(31) }, ccu_reset_map { reg: 0x2c8, bit: BIT(2) }, ccu_reset_map { reg: 0x2d0, bit: BIT(0) }, ccu_reset_map { reg: 0x2d8, bit: BIT(0) }, ccu_reset_map { reg: 0x2d8, bit: BIT(1) }, ccu_reset_map { reg: 0x2d8, bit: BIT(16) }, ccu_reset_map { reg: 0x2d8, bit: BIT(17) }, ccu_reset_map { reg: 0x2d8, bit: BIT(18) },
];

static sun8i_v3s_ccu_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: sun8i_v3s_ccu_clks, num_ccu_clks: ARRAY_SIZE(sun8i_v3s_ccu_clks), hw_clks: &sun8i_v3s_hw_clks, resets: sun8i_v3s_ccu_resets, num_resets: ARRAY_SIZE(sun8i_v3s_ccu_resets) };

unsafe fn sun8i_v3s_ccu_probe(pdev: *mut platform_device) -> i32 {
    let desc = of_device_get_match_data((*pdev).dev);
    if desc.is_null() { return -EINVAL; }
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    let mut val = readl(reg.add(SUN8I_V3S_PLL_AUDIO_REG as usize));
    val &= !GENMASK(19, 16); writel(val, reg.add(SUN8I_V3S_PLL_AUDIO_REG as usize));
    val = readl(reg.add(de_clk.common.reg as usize)); val &= !GENMASK(de_clk.mux.shift + de_clk.mux.width - 1, de_clk.mux.shift); writel(val, reg.add(de_clk.common.reg as usize));
    val = readl(reg.add(tcon_clk.common.reg as usize)); val &= !GENMASK(tcon_clk.mux.shift + tcon_clk.mux.width - 1, tcon_clk.mux.shift); writel(val, reg.add(tcon_clk.common.reg as usize));
    devm_sunxi_ccu_probe(&mut (*pdev).dev, reg, desc)
}

// Device matching, module registration, and metadata from the C translation unit.
static sun8i_v3s_ccu_ids: [of_device_id; 3] = [of_device_id { compatible: "allwinner,sun8i-v3-ccu", data: &sun8i_v3_ccu_desc }, of_device_id { compatible: "allwinner,sun8i-v3s-ccu", data: &sun8i_v3s_ccu_desc }, of_device_id { compatible: "", data: core::ptr::null() }];
static sun8i_v3s_ccu_driver: platform_driver = platform_driver { probe: sun8i_v3s_ccu_probe, driver: driver { name: "sun8i-v3s-ccu", suppress_bind_attrs: true, of_match_table: sun8i_v3s_ccu_ids } };
module_platform_driver!(sun8i_v3s_ccu_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
