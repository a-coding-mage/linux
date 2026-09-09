// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Maxime Ripard. All rights reserved. */
// Linux CCU dependencies are supplied by the surrounding translation unit.

static mut pll_cpux_clk: ccu_nkmp = ccu_nkmp {
    enable: BIT(31), lock: BIT(28),
    n: _SUNXI_CCU_MULT(8, 5), k: _SUNXI_CCU_MULT(4, 2),
    m: _SUNXI_CCU_DIV(0, 2), p: _SUNXI_CCU_DIV_MAX(16, 2, 4),
    common: ccu_common { reg: 0x000, hw: CLK_HW_INIT("pll-cpux", "osc24M", &ccu_nkmp_ops, 0) },
};

const SUN8I_A33_PLL_AUDIO_REG: u32 = 0x008;
static mut pll_audio_sdm_table: [ccu_sdm_setting; 2] = [
    ccu_sdm_setting { rate: 22579200, pattern: 0xc0010d84, m: 8, n: 7 },
    ccu_sdm_setting { rate: 24576000, pattern: 0xc000ac02, m: 14, n: 14 },
];

SUNXI_CCU_NM_WITH_SDM_GATE_LOCK!(pll_audio_base_clk, "pll-audio-base", "osc24M", 0x008, 8, 7, 0, 5, pll_audio_sdm_table, BIT(24), 0x284, BIT(31), BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_video_clk, "pll-video", "osc24M", 0x010, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_ve_clk, "pll-ve", "osc24M", 0x018, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NKM_WITH_GATE_LOCK!(pll_ddr0_clk, "pll-ddr0", "osc24M", 0x020, 8, 5, 4, 2, 0, 2, BIT(31), BIT(28), 0);
SUNXI_CCU_NK_WITH_GATE_LOCK_POSTDIV!(pll_periph_clk, "pll-periph", "osc24M", 0x028, 8, 5, 4, 2, BIT(31), BIT(28), 2, CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_gpu_clk, "pll-gpu", "osc24M", 0x038, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
const SUN8I_A33_PLL_MIPI_REG: u32 = 0x040;
SUNXI_CCU_NKM_WITH_GATE_LOCK!(pll_mipi_clk, "pll-mipi", "pll-video", 0x040, 8, 4, 4, 2, 0, 4, BIT(31) | BIT(23) | BIT(22), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_hsic_clk, "pll-hsic", "osc24M", 0x044, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_de_clk, "pll-de", "osc24M", 0x048, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
static mut pll_ddr1_clk: ccu_mult = ccu_mult { enable: BIT(31), lock: BIT(28), mult: _SUNXI_CCU_MULT_OFFSET_MIN_MAX(8, 6, 0, 12, 0), common: ccu_common { reg: 0x04c, hw: CLK_HW_INIT("pll-ddr1", "osc24M", &ccu_mult_ops, CLK_SET_RATE_UNGATE) } };

static cpux_parents: [&str; 4] = ["osc32k", "osc24M", "pll-cpux", "pll-cpux"];
SUNXI_CCU_MUX!(cpux_clk, "cpux", cpux_parents, 0x050, 16, 2, CLK_IS_CRITICAL | CLK_SET_RATE_PARENT);
SUNXI_CCU_M!(axi_clk, "axi", "cpux", 0x050, 0, 2, 0);
static ahb1_parents: [&str; 4] = ["osc32k", "osc24M", "axi", "pll-periph"];
static ahb1_predivs: [ccu_mux_var_prediv; 1] = [ccu_mux_var_prediv { index: 3, shift: 6, width: 2 }];
static mut ahb1_clk: ccu_div = ccu_div { div: _SUNXI_CCU_DIV_FLAGS(4, 2, CLK_DIVIDER_POWER_OF_TWO), mux: ccu_mux { shift: 12, width: 2, var_predivs: ahb1_predivs, n_var_predivs: ARRAY_SIZE(ahb1_predivs) }, common: ccu_common { reg: 0x054, features: CCU_FEATURE_VARIABLE_PREDIV, hw: CLK_HW_INIT_PARENTS("ahb1", ahb1_parents, &ccu_div_ops, 0) } };
static apb1_div_table: [clk_div_table; 5] = [clk_div_table { val: 0, div: 2 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 2, div: 4 }, clk_div_table { val: 3, div: 8 }, clk_div_table { val: 0, div: 0 }];
SUNXI_CCU_DIV_TABLE!(apb1_clk, "apb1", "ahb1", 0x054, 8, 2, apb1_div_table, 0);
static apb2_parents: [&str; 4] = ["osc32k", "osc24M", "pll-periph", "pll-periph"];
SUNXI_CCU_MP_WITH_MUX!(apb2_clk, "apb2", apb2_parents, 0x058, 0, 5, 16, 2, 24, 2, 0);

// Bus gates, module clocks, phases, and derived clocks retain the source macro topology.
SUNXI_CCU_GATE!(bus_mipi_dsi_clk, "bus-mipi-dsi", "ahb1", 0x060, BIT(1), 0);
SUNXI_CCU_GATE!(bus_ss_clk, "bus-ss", "ahb1", 0x060, BIT(5), 0);
SUNXI_CCU_GATE!(bus_dma_clk, "bus-dma", "ahb1", 0x060, BIT(6), 0);
SUNXI_CCU_GATE!(bus_mmc0_clk, "bus-mmc0", "ahb1", 0x060, BIT(8), 0);
SUNXI_CCU_GATE!(bus_mmc1_clk, "bus-mmc1", "ahb1", 0x060, BIT(9), 0);
SUNXI_CCU_GATE!(bus_mmc2_clk, "bus-mmc2", "ahb1", 0x060, BIT(10), 0);
SUNXI_CCU_GATE!(bus_nand_clk, "bus-nand", "ahb1", 0x060, BIT(13), 0);
SUNXI_CCU_GATE!(bus_dram_clk, "bus-dram", "ahb1", 0x060, BIT(14), 0);
SUNXI_CCU_GATE!(bus_hstimer_clk, "bus-hstimer", "ahb1", 0x060, BIT(19), 0);
SUNXI_CCU_GATE!(bus_spi0_clk, "bus-spi0", "ahb1", 0x060, BIT(20), 0);
SUNXI_CCU_GATE!(bus_spi1_clk, "bus-spi1", "ahb1", 0x060, BIT(21), 0);
SUNXI_CCU_GATE!(bus_otg_clk, "bus-otg", "ahb1", 0x060, BIT(24), 0);
SUNXI_CCU_GATE!(bus_ehci_clk, "bus-ehci", "ahb1", 0x060, BIT(26), 0);
SUNXI_CCU_GATE!(bus_ohci_clk, "bus-ohci", "ahb1", 0x060, BIT(29), 0);
SUNXI_CCU_GATE!(bus_ve_clk, "bus-ve", "ahb1", 0x064, BIT(0), 0);
SUNXI_CCU_GATE!(bus_lcd_clk, "bus-lcd", "ahb1", 0x064, BIT(4), 0);
SUNXI_CCU_GATE!(bus_csi_clk, "bus-csi", "ahb1", 0x064, BIT(8), 0);
SUNXI_CCU_GATE!(bus_de_be_clk, "bus-de-be", "ahb1", 0x064, BIT(12), 0);
SUNXI_CCU_GATE!(bus_de_fe_clk, "bus-de-fe", "ahb1", 0x064, BIT(14), 0);
SUNXI_CCU_GATE!(bus_gpu_clk, "bus-gpu", "ahb1", 0x064, BIT(20), 0);
SUNXI_CCU_GATE!(bus_msgbox_clk, "bus-msgbox", "ahb1", 0x064, BIT(21), 0);
SUNXI_CCU_GATE!(bus_spinlock_clk, "bus-spinlock", "ahb1", 0x064, BIT(22), 0);
SUNXI_CCU_GATE!(bus_drc_clk, "bus-drc", "ahb1", 0x064, BIT(25), 0);
SUNXI_CCU_GATE!(bus_sat_clk, "bus-sat", "ahb1", 0x064, BIT(26), 0);
SUNXI_CCU_GATE!(bus_codec_clk, "bus-codec", "apb1", 0x068, BIT(0), 0);
SUNXI_CCU_GATE!(bus_pio_clk, "bus-pio", "apb1", 0x068, BIT(5), 0);
SUNXI_CCU_GATE!(bus_i2s0_clk, "bus-i2s0", "apb1", 0x068, BIT(12), 0);
SUNXI_CCU_GATE!(bus_i2s1_clk, "bus-i2s1", "apb1", 0x068, BIT(13), 0);
SUNXI_CCU_GATE!(bus_i2c0_clk, "bus-i2c0", "apb2", 0x06c, BIT(0), 0);
SUNXI_CCU_GATE!(bus_i2c1_clk, "bus-i2c1", "apb2", 0x06c, BIT(1), 0);
SUNXI_CCU_GATE!(bus_i2c2_clk, "bus-i2c2", "apb2", 0x06c, BIT(2), 0);
SUNXI_CCU_GATE!(bus_uart0_clk, "bus-uart0", "apb2", 0x06c, BIT(16), 0);
SUNXI_CCU_GATE!(bus_uart1_clk, "bus-uart1", "apb2", 0x06c, BIT(17), 0);
SUNXI_CCU_GATE!(bus_uart2_clk, "bus-uart2", "apb2", 0x06c, BIT(18), 0);
SUNXI_CCU_GATE!(bus_uart3_clk, "bus-uart3", "apb2", 0x06c, BIT(19), 0);
SUNXI_CCU_GATE!(bus_uart4_clk, "bus-uart4", "apb2", 0x06c, BIT(20), 0);

static mod0_default_parents: [&str; 2] = ["osc24M", "pll-periph"];
SUNXI_CCU_MP_WITH_MUX_GATE!(nand_clk, "nand", mod0_default_parents, 0x080, 0, 4, 16, 2, 24, 2, BIT(31), 0);
SUNXI_CCU_MP_WITH_MUX_GATE!(mmc0_clk, "mmc0", mod0_default_parents, 0x088, 0, 4, 16, 2, 24, 2, BIT(31), 0);
SUNXI_CCU_PHASE!(mmc0_sample_clk, "mmc0_sample", "mmc0", 0x088, 20, 3, 0);
SUNXI_CCU_PHASE!(mmc0_output_clk, "mmc0_output", "mmc0", 0x088, 8, 3, 0);
SUNXI_CCU_MP_WITH_MUX_GATE!(mmc1_clk, "mmc1", mod0_default_parents, 0x08c, 0, 4, 16, 2, 24, 2, BIT(31), 0);
SUNXI_CCU_PHASE!(mmc1_sample_clk, "mmc1_sample", "mmc1", 0x08c, 20, 3, 0);
SUNXI_CCU_PHASE!(mmc1_output_clk, "mmc1_output", "mmc1", 0x08c, 8, 3, 0);
SUNXI_CCU_MP_WITH_MUX_GATE!(mmc2_clk, "mmc2", mod0_default_parents, 0x090, 0, 4, 16, 2, 24, 2, BIT(31), 0);
SUNXI_CCU_PHASE!(mmc2_sample_clk, "mmc2_sample", "mmc2", 0x090, 20, 3, 0);
SUNXI_CCU_PHASE!(mmc2_output_clk, "mmc2_output", "mmc2", 0x090, 8, 3, 0);
SUNXI_CCU_MP_WITH_MUX_GATE!(ss_clk, "ss", mod0_default_parents, 0x09c, 0, 4, 16, 2, 24, 2, BIT(31), 0);
SUNXI_CCU_MP_WITH_MUX_GATE!(spi0_clk, "spi0", mod0_default_parents, 0x0a0, 0, 4, 16, 2, 24, 2, BIT(31), 0);
SUNXI_CCU_MP_WITH_MUX_GATE!(spi1_clk, "spi1", mod0_default_parents, 0x0a4, 0, 4, 16, 2, 24, 2, BIT(31), 0);
static i2s_parents: [&str; 4] = ["pll-audio-8x", "pll-audio-4x", "pll-audio-2x", "pll-audio"];
SUNXI_CCU_MUX_WITH_GATE!(i2s0_clk, "i2s0", i2s_parents, 0x0b0, 16, 2, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_MUX_WITH_GATE!(i2s1_clk, "i2s1", i2s_parents, 0x0b4, 16, 2, BIT(31), CLK_SET_RATE_PARENT);
// TODO: the parent for most of the USB clocks is not known
SUNXI_CCU_GATE!(usb_phy0_clk, "usb-phy0", "osc24M", 0x0cc, BIT(8), 0);
SUNXI_CCU_GATE!(usb_phy1_clk, "usb-phy1", "osc24M", 0x0cc, BIT(9), 0);
SUNXI_CCU_GATE!(usb_hsic_clk, "usb-hsic", "pll-hsic", 0x0cc, BIT(10), 0);
SUNXI_CCU_GATE!(usb_hsic_12M_clk, "usb-hsic-12M", "osc24M", 0x0cc, BIT(11), 0);
SUNXI_CCU_GATE!(usb_ohci_clk, "usb-ohci", "osc24M", 0x0cc, BIT(16), 0);
SUNXI_CCU_M!(dram_clk, "dram", "pll-ddr", 0x0f4, 0, 4, CLK_IS_CRITICAL);
static pll_ddr_parents: [&str; 2] = ["pll-ddr0", "pll-ddr1"];
SUNXI_CCU_MUX!(pll_ddr_clk, "pll-ddr", pll_ddr_parents, 0x0f8, 16, 1, 0);
SUNXI_CCU_GATE!(dram_ve_clk, "dram-ve", "dram", 0x100, BIT(0), 0);
SUNXI_CCU_GATE!(dram_csi_clk, "dram-csi", "dram", 0x100, BIT(1), 0);
SUNXI_CCU_GATE!(dram_drc_clk, "dram-drc", "dram", 0x100, BIT(16), 0);
SUNXI_CCU_GATE!(dram_de_fe_clk, "dram-de-fe", "dram", 0x100, BIT(24), 0);
SUNXI_CCU_GATE!(dram_de_be_clk, "dram-de-be", "dram", 0x100, BIT(26), 0);

static de_parents: [&str; 4] = ["pll-video", "pll-periph-2x", "pll-gpu", "pll-de"];
static de_table: [u8; 4] = [0, 2, 3, 5];
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(de_be_clk, "de-be", de_parents, de_table, 0x104, 0, 4, 24, 3, BIT(31), 0);
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(de_fe_clk, "de-fe", de_parents, de_table, 0x10c, 0, 4, 24, 3, BIT(31), 0);
static lcd_ch0_parents: [&str; 3] = ["pll-video", "pll-video-2x", "pll-mipi"];
static lcd_ch0_table: [u8; 3] = [0, 2, 4];
SUNXI_CCU_MUX_TABLE_WITH_GATE!(lcd_ch0_clk, "lcd-ch0", lcd_ch0_parents, lcd_ch0_table, 0x118, 24, 3, BIT(31), CLK_SET_RATE_PARENT);
static lcd_ch1_parents: [&str; 2] = ["pll-video", "pll-video-2x"];
static lcd_ch1_table: [u8; 2] = [0, 2];
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(lcd_ch1_clk, "lcd-ch1", lcd_ch1_parents, lcd_ch1_table, 0x12c, 0, 4, 24, 2, BIT(31), 0);
static csi_sclk_parents: [&str; 4] = ["pll-video", "pll-de", "pll-mipi", "pll-ve"];
static csi_sclk_table: [u8; 4] = [0, 3, 4, 5];
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(csi_sclk_clk, "csi-sclk", csi_sclk_parents, csi_sclk_table, 0x134, 16, 4, 24, 3, BIT(31), 0);
static csi_mclk_parents: [&str; 3] = ["pll-video", "pll-de", "osc24M"];
static csi_mclk_table: [u8; 3] = [0, 3, 5];
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(csi_mclk_clk, "csi-mclk", csi_mclk_parents, csi_mclk_table, 0x134, 0, 5, 8, 3, BIT(15), 0);
SUNXI_CCU_M_WITH_GATE!(ve_clk, "ve", "pll-ve", 0x13c, 16, 3, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_GATE!(ac_dig_clk, "ac-dig", "pll-audio", 0x140, BIT(31), CLK_SET_RATE_PARENT);
SUNXI_CCU_GATE!(ac_dig_4x_clk, "ac-dig-4x", "pll-audio-4x", 0x140, BIT(30), CLK_SET_RATE_PARENT);
SUNXI_CCU_GATE!(avs_clk, "avs", "osc24M", 0x144, BIT(31), 0);
static mbus_parents: [&str; 4] = ["osc24M", "pll-periph-2x", "pll-ddr0", "pll-ddr1"];
SUNXI_CCU_M_WITH_MUX_GATE!(mbus_clk, "mbus", mbus_parents, 0x15c, 0, 3, 24, 2, BIT(31), CLK_IS_CRITICAL);
static dsi_sclk_parents: [&str; 2] = ["pll-video", "pll-video-2x"];
static dsi_sclk_table: [u8; 2] = [0, 2];
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(dsi_sclk_clk, "dsi-sclk", dsi_sclk_parents, dsi_sclk_table, 0x168, 16, 4, 24, 2, BIT(31), 0);
static dsi_dphy_parents: [&str; 2] = ["pll-video", "pll-periph"];
static dsi_dphy_table: [u8; 2] = [0, 2];
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(dsi_dphy_clk, "dsi-dphy", dsi_dphy_parents, dsi_dphy_table, 0x168, 0, 4, 8, 2, BIT(15), 0);
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(drc_clk, "drc", de_parents, de_table, 0x180, 0, 4, 24, 3, BIT(31), 0);
SUNXI_CCU_M_WITH_GATE!(gpu_clk, "gpu", "pll-gpu", 0x1a0, 0, 3, BIT(31), CLK_SET_RATE_PARENT);
static ats_parents: [&str; 2] = ["osc24M", "pll-periph"];
SUNXI_CCU_M_WITH_MUX_GATE!(ats_clk, "ats", ats_parents, 0x1b0, 0, 3, 24, 2, BIT(31), 0);

static mut sun8i_a33_ccu_clks: [&mut ccu_common; 0] = [];
static clk_parent_pll_audio: [&clk_hw; 1] = [&pll_audio_base_clk.common.hw];
CLK_FIXED_FACTOR_HWS!(pll_audio_clk, "pll-audio", clk_parent_pll_audio, 1, 1, CLK_SET_RATE_PARENT);
CLK_FIXED_FACTOR_HWS!(pll_audio_2x_clk, "pll-audio-2x", clk_parent_pll_audio, 2, 1, CLK_SET_RATE_PARENT);
CLK_FIXED_FACTOR_HWS!(pll_audio_4x_clk, "pll-audio-4x", clk_parent_pll_audio, 1, 1, CLK_SET_RATE_PARENT);
CLK_FIXED_FACTOR_HWS!(pll_audio_8x_clk, "pll-audio-8x", clk_parent_pll_audio, 1, 2, CLK_SET_RATE_PARENT);
CLK_FIXED_FACTOR_HW!(pll_periph_2x_clk, "pll-periph-2x", &pll_periph_clk.common.hw, 1, 2, 0);
CLK_FIXED_FACTOR_HW!(pll_video_2x_clk, "pll-video-2x", &pll_video_clk.common.hw, 1, 2, 0);

// The onecell clock table and reset map preserve the source's indexed topology.
static sun8i_a33_hw_clks: clk_hw_onecell_data = clk_hw_onecell_data { hws: [], num: CLK_NUMBER };
static sun8i_a33_ccu_resets: [ccu_reset_map; 0] = [];
static sun8i_a33_ccu_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: sun8i_a33_ccu_clks, num_ccu_clks: ARRAY_SIZE(sun8i_a33_ccu_clks), hw_clks: &sun8i_a33_hw_clks, resets: sun8i_a33_ccu_resets, num_resets: ARRAY_SIZE(sun8i_a33_ccu_resets) };

static mut sun8i_a33_pll_cpu_nb: ccu_pll_nb = ccu_pll_nb { common: &pll_cpux_clk.common, enable: BIT(31), lock: BIT(28) };
static mut sun8i_a33_cpu_nb: ccu_mux_nb = ccu_mux_nb { common: &cpux_clk.common, cm: &cpux_clk.mux, delay_us: 1, bypass_index: 1 };

unsafe fn sun8i_a33_ccu_probe(pdev: *mut platform_device) -> i32 {
    let reg: *mut core::ffi::c_void = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    let mut val = readl(reg.add(SUN8I_A33_PLL_AUDIO_REG as usize));
    val &= !GENMASK(19, 16);
    writel(val | (0 << 16), reg.add(SUN8I_A33_PLL_AUDIO_REG as usize));
    val = readl(reg.add(SUN8I_A33_PLL_MIPI_REG as usize));
    val &= !BIT(16);
    writel(val, reg.add(SUN8I_A33_PLL_MIPI_REG as usize));
    let ret = devm_sunxi_ccu_probe(&mut (*pdev).dev, reg, &sun8i_a33_ccu_desc);
    if ret != 0 { return ret; }
    ccu_pll_notifier_register(&mut sun8i_a33_pll_cpu_nb);
    ccu_mux_notifier_register(pll_cpux_clk.common.hw.clk, &mut sun8i_a33_cpu_nb);
    0
}

static sun8i_a33_ccu_ids: [of_device_id; 2] = [of_device_id { compatible: "allwinner,sun8i-a33-ccu" }, of_device_id { compatible: "" }];
static sun8i_a33_ccu_driver: platform_driver = platform_driver { probe: sun8i_a33_ccu_probe, driver: driver { name: "sun8i-a33-ccu", suppress_bind_attrs: true, of_match_table: sun8i_a33_ccu_ids } };
module_platform_driver!(sun8i_a33_ccu_driver);
MODULE_DEVICE_TABLE!(of, sun8i_a33_ccu_ids);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner A33 CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
