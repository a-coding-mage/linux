// SPDX-License-Identifier: GPL-2.0
/* Rust translation of ccu-suniv-f1c100s.c. External kernel types/macros are
 * intentionally referenced as supplied by the surrounding translation. */

const SUNIV_PLL_AUDIO_REG: usize = 0x008;

static mut pll_cpu_clk: ccu_nkmp = ccu_nkmp {
    enable: BIT(31), lock: BIT(28),
    n: _SUNXI_CCU_MULT(8, 5), k: _SUNXI_CCU_MULT(4, 2),
    m: _SUNXI_CCU_DIV(0, 2), p: _SUNXI_CCU_DIV_MAX(16, 2, 4),
    common: ccu_common { reg: 0x000, hw: CLK_HW_INIT!("pll-cpu", "osc24M", &ccu_nkmp_ops, CLK_SET_RATE_UNGATE) },
};

// The audio PLL's variable divider is unused; these declarations preserve its fixed-factor topology.
static SUNXI_CCU_NM_WITH_GATE_LOCK!(pll_audio_base_clk, "pll-audio-base", "osc24M", 0x008, 8, 7, 0, 5, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
static SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_video_clk, "pll-video", "osc24M", 0x010, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
static SUNXI_CCU_NM_WITH_FRAC_GATE_LOCK!(pll_ve_clk, "pll-ve", "osc24M", 0x018, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
static SUNXI_CCU_NKM_WITH_GATE_LOCK!(pll_ddr0_clk, "pll-ddr", "osc24M", 0x020, 8, 5, 4, 2, 0, 2, BIT(31), BIT(28), CLK_IS_CRITICAL);

static mut pll_periph_clk: ccu_nk = ccu_nk { enable: BIT(31), lock: BIT(28), k: _SUNXI_CCU_MULT(4, 2), n: _SUNXI_CCU_MULT(8, 5), common: ccu_common { reg: 0x028, hw: CLK_HW_INIT!("pll-periph", "osc24M", &ccu_nk_ops, 0) } };

static cpu_parents: [&str; 4] = ["osc32k", "osc24M", "pll-cpu", "pll-cpu"];
static SUNXI_CCU_MUX!(cpu_clk, "cpu", cpu_parents, 0x050, 16, 2, CLK_IS_CRITICAL | CLK_SET_RATE_PARENT);
static ahb_parents: [&str; 4] = ["osc32k", "osc24M", "cpu", "pll-periph"];
static ahb_predivs: [ccu_mux_var_prediv; 1] = [ccu_mux_var_prediv { index: 3, shift: 6, width: 2 }];
static mut ahb_clk: ccu_div = ccu_div { div: _SUNXI_CCU_DIV_FLAGS(4, 2, CLK_DIVIDER_POWER_OF_TWO), mux: ccu_mux { shift: 12, width: 2, var_predivs: ahb_predivs, n_var_predivs: ARRAY_SIZE!(ahb_predivs) }, common: ccu_common { reg: 0x054, features: CCU_FEATURE_VARIABLE_PREDIV, hw: CLK_HW_INIT_PARENTS!("ahb", ahb_parents, &ccu_div_ops, 0) } };
static apb_div_table: [clk_div_table; 5] = [clk_div_table { val: 0, div: 2 }, clk_div_table { val: 1, div: 2 }, clk_div_table { val: 2, div: 4 }, clk_div_table { val: 3, div: 8 }, clk_div_table { val: 0, div: 0 }];
static SUNXI_CCU_DIV_TABLE!(apb_clk, "apb", "ahb", 0x054, 8, 2, apb_div_table, 0);

SUNXI_CCU_GATE!(bus_dma_clk, "bus-dma", "ahb", 0x060, BIT(6), 0);
SUNXI_CCU_GATE!(bus_mmc0_clk, "bus-mmc0", "ahb", 0x060, BIT(8), 0);
SUNXI_CCU_GATE!(bus_mmc1_clk, "bus-mmc1", "ahb", 0x060, BIT(9), 0);
SUNXI_CCU_GATE!(bus_dram_clk, "bus-dram", "ahb", 0x060, BIT(14), 0);
SUNXI_CCU_GATE!(bus_spi0_clk, "bus-spi0", "ahb", 0x060, BIT(20), 0);
SUNXI_CCU_GATE!(bus_spi1_clk, "bus-spi1", "ahb", 0x060, BIT(21), 0);
SUNXI_CCU_GATE!(bus_otg_clk, "bus-otg", "ahb", 0x060, BIT(24), 0);
SUNXI_CCU_GATE!(bus_ve_clk, "bus-ve", "ahb", 0x064, BIT(0), 0);
SUNXI_CCU_GATE!(bus_lcd_clk, "bus-lcd", "ahb", 0x064, BIT(4), 0);
SUNXI_CCU_GATE!(bus_deinterlace_clk, "bus-deinterlace", "ahb", 0x064, BIT(5), 0);
SUNXI_CCU_GATE!(bus_csi_clk, "bus-csi", "ahb", 0x064, BIT(8), 0);
SUNXI_CCU_GATE!(bus_tvd_clk, "bus-tvd", "ahb", 0x064, BIT(9), 0);
SUNXI_CCU_GATE!(bus_tve_clk, "bus-tve", "ahb", 0x064, BIT(10), 0);
SUNXI_CCU_GATE!(bus_de_be_clk, "bus-de-be", "ahb", 0x064, BIT(12), 0);
SUNXI_CCU_GATE!(bus_de_fe_clk, "bus-de-fe", "ahb", 0x064, BIT(14), 0);
SUNXI_CCU_GATE!(bus_codec_clk, "bus-codec", "apb", 0x068, BIT(0), 0);
SUNXI_CCU_GATE!(bus_spdif_clk, "bus-spdif", "apb", 0x068, BIT(1), 0);
SUNXI_CCU_GATE!(bus_ir_clk, "bus-ir", "apb", 0x068, BIT(2), 0);
SUNXI_CCU_GATE!(bus_rsb_clk, "bus-rsb", "apb", 0x068, BIT(3), 0);
SUNXI_CCU_GATE!(bus_i2s0_clk, "bus-i2s0", "apb", 0x068, BIT(12), 0);
SUNXI_CCU_GATE!(bus_i2c0_clk, "bus-i2c0", "apb", 0x068, BIT(16), 0);
SUNXI_CCU_GATE!(bus_i2c1_clk, "bus-i2c1", "apb", 0x068, BIT(17), 0);
SUNXI_CCU_GATE!(bus_i2c2_clk, "bus-i2c2", "apb", 0x068, BIT(18), 0);
SUNXI_CCU_GATE!(bus_pio_clk, "bus-pio", "apb", 0x068, BIT(19), 0);
SUNXI_CCU_GATE!(bus_uart0_clk, "bus-uart0", "apb", 0x068, BIT(20), 0);
SUNXI_CCU_GATE!(bus_uart1_clk, "bus-uart1", "apb", 0x068, BIT(21), 0);
SUNXI_CCU_GATE!(bus_uart2_clk, "bus-uart2", "apb", 0x068, BIT(22), 0);

static mod0_default_parents: [&str; 2] = ["osc24M", "pll-periph"];
SUNXI_CCU_MP_WITH_MUX_GATE!(mmc0_clk, "mmc0", mod0_default_parents, 0x088, 0, 4, 16, 2, 24, 2, BIT(31), 0);
SUNXI_CCU_PHASE!(mmc0_sample_clk, "mmc0_sample", "mmc0", 0x088, 20, 3, 0);
SUNXI_CCU_PHASE!(mmc0_output_clk, "mmc0_output", "mmc0", 0x088, 8, 3, 0);
SUNXI_CCU_MP_WITH_MUX_GATE!(mmc1_clk, "mmc1", mod0_default_parents, 0x08c, 0, 4, 16, 2, 24, 2, BIT(31), 0);
SUNXI_CCU_PHASE!(mmc1_sample_clk, "mmc1_sample", "mmc1", 0x08c, 20, 3, 0);
SUNXI_CCU_PHASE!(mmc1_output_clk, "mmc1_output", "mmc1", 0x08c, 8, 3, 0);
static i2s_spdif_parents: [&str; 4] = ["pll-audio-8x", "pll-audio-4x", "pll-audio-2x", "pll-audio"];
SUNXI_CCU_MUX_WITH_GATE!(i2s_clk, "i2s", i2s_spdif_parents, 0x0b0, 16, 2, BIT(31), 0);
SUNXI_CCU_MUX_WITH_GATE!(spdif_clk, "spdif", i2s_spdif_parents, 0x0b4, 16, 2, BIT(31), 0);
static ir_parents: [&str; 2] = ["osc32k", "osc24M"];
SUNXI_CCU_MP_WITH_MUX_GATE!(ir_clk, "ir", ir_parents, 0x0b8, 0, 4, 16, 2, 24, 2, BIT(31), 0);
SUNXI_CCU_GATE!(usb_phy0_clk, "usb-phy0", "osc24M", 0x0cc, BIT(1), 0);
SUNXI_CCU_GATE!(dram_ve_clk, "dram-ve", "pll-ddr", 0x100, BIT(0), 0);
SUNXI_CCU_GATE!(dram_csi_clk, "dram-csi", "pll-ddr", 0x100, BIT(1), 0);
SUNXI_CCU_GATE!(dram_deinterlace_clk, "dram-deinterlace", "pll-ddr", 0x100, BIT(2), 0);
SUNXI_CCU_GATE!(dram_tvd_clk, "dram-tvd", "pll-ddr", 0x100, BIT(3), 0);
SUNXI_CCU_GATE!(dram_de_fe_clk, "dram-de-fe", "pll-ddr", 0x100, BIT(24), 0);
SUNXI_CCU_GATE!(dram_de_be_clk, "dram-de-be", "pll-ddr", 0x100, BIT(26), 0);

static de_parents: [&str; 2] = ["pll-video", "pll-periph"];
static de_table: [u8; 2] = [0, 2];
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(de_be_clk, "de-be", de_parents, de_table, 0x104, 0, 4, 24, 3, BIT(31), 0);
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(de_fe_clk, "de-fe", de_parents, de_table, 0x10c, 0, 4, 24, 3, BIT(31), 0);
static tcon_parents: [&str; 2] = ["pll-video", "pll-video-2x"];
static tcon_table: [u8; 2] = [0, 2];
SUNXI_CCU_MUX_TABLE_WITH_GATE!(tcon_clk, "tcon", tcon_parents, tcon_table, 0x118, 24, 3, BIT(31), CLK_SET_RATE_PARENT);
static deinterlace_parents: [&str; 2] = ["pll-video", "pll-video-2x"];
static deinterlace_table: [u8; 2] = [0, 2];
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(deinterlace_clk, "deinterlace", deinterlace_parents, deinterlace_table, 0x11c, 0, 4, 24, 3, BIT(31), 0);
static tve_clk2_parents: [&str; 2] = ["pll-video", "pll-video-2x"];
static tve_clk2_table: [u8; 2] = [0, 2];
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(tve_clk2_clk, "tve-clk2", tve_clk2_parents, tve_clk2_table, 0x120, 0, 4, 24, 3, BIT(31), 0);
SUNXI_CCU_M_WITH_GATE!(tve_clk1_clk, "tve-clk1", "tve-clk2", 0x120, 8, 1, BIT(15), 0);
static tvd_parents: [&str; 3] = ["pll-video", "osc24M", "pll-video-2x"];
SUNXI_CCU_M_WITH_MUX_GATE!(tvd_clk, "tvd", tvd_parents, 0x124, 0, 4, 24, 3, BIT(31), 0);
static csi_parents: [&str; 2] = ["pll-video", "osc24M"];
static csi_table: [u8; 2] = [0, 5];
SUNXI_CCU_M_WITH_MUX_TABLE_GATE!(csi_clk, "csi", csi_parents, csi_table, 0x120, 0, 4, 8, 3, BIT(15), 0);
// TODO: BSP says pll-audio, but common sense suggests pll-ve.
SUNXI_CCU_GATE!(ve_clk, "ve", "pll-audio", 0x13c, BIT(31), 0);
SUNXI_CCU_GATE!(codec_clk, "codec", "pll-audio", 0x140, BIT(31), 0);
SUNXI_CCU_GATE!(avs_clk, "avs", "osc24M", 0x144, BIT(31), 0);

static suniv_ccu_clks: [&ccu_common; 64] = [
    &pll_cpu_clk.common, &pll_audio_base_clk.common, &pll_video_clk.common, &pll_ve_clk.common, &pll_ddr0_clk.common, &pll_periph_clk.common,
    &cpu_clk.common, &ahb_clk.common, &apb_clk.common, &bus_dma_clk.common, &bus_mmc0_clk.common, &bus_mmc1_clk.common, &bus_dram_clk.common,
    &bus_spi0_clk.common, &bus_spi1_clk.common, &bus_otg_clk.common, &bus_ve_clk.common, &bus_lcd_clk.common, &bus_deinterlace_clk.common,
    &bus_csi_clk.common, &bus_tve_clk.common, &bus_tvd_clk.common, &bus_de_be_clk.common, &bus_de_fe_clk.common, &bus_codec_clk.common,
    &bus_spdif_clk.common, &bus_ir_clk.common, &bus_rsb_clk.common, &bus_i2s0_clk.common, &bus_i2c0_clk.common, &bus_i2c1_clk.common,
    &bus_i2c2_clk.common, &bus_pio_clk.common, &bus_uart0_clk.common, &bus_uart1_clk.common, &bus_uart2_clk.common, &mmc0_clk.common,
    &mmc0_sample_clk.common, &mmc0_output_clk.common, &mmc1_clk.common, &mmc1_sample_clk.common, &mmc1_output_clk.common, &i2s_clk.common,
    &spdif_clk.common, &ir_clk.common, &usb_phy0_clk.common, &dram_ve_clk.common, &dram_csi_clk.common, &dram_deinterlace_clk.common,
    &dram_tvd_clk.common, &dram_de_fe_clk.common, &dram_de_be_clk.common, &de_be_clk.common, &de_fe_clk.common, &tcon_clk.common,
    &deinterlace_clk.common, &tve_clk2_clk.common, &tve_clk1_clk.common, &tvd_clk.common, &csi_clk.common, &ve_clk.common, &codec_clk.common, &avs_clk.common,
];

static clk_parent_pll_audio: [&clk_hw; 1] = [&pll_audio_base_clk.common.hw];
CLK_FIXED_FACTOR_HWS!(pll_audio_clk, "pll-audio", clk_parent_pll_audio, 4, 1, CLK_SET_RATE_PARENT);
CLK_FIXED_FACTOR_HWS!(pll_audio_2x_clk, "pll-audio-2x", clk_parent_pll_audio, 2, 1, CLK_SET_RATE_PARENT);
CLK_FIXED_FACTOR_HWS!(pll_audio_4x_clk, "pll-audio-4x", clk_parent_pll_audio, 1, 1, CLK_SET_RATE_PARENT);
CLK_FIXED_FACTOR_HWS!(pll_audio_8x_clk, "pll-audio-8x", clk_parent_pll_audio, 1, 2, CLK_SET_RATE_PARENT);
CLK_FIXED_FACTOR_HW!(pll_video_2x_clk, "pll-video-2x", &pll_video_clk.common.hw, 1, 2, 0);

static suniv_hw_clks: clk_hw_onecell_data = clk_hw_onecell_data { hws: [
    [CLK_PLL_CPU] = &pll_cpu_clk.common.hw, [CLK_PLL_AUDIO_BASE] = &pll_audio_base_clk.common.hw,
    [CLK_PLL_AUDIO] = &pll_audio_clk.hw, [CLK_PLL_AUDIO_2X] = &pll_audio_2x_clk.hw,
    [CLK_PLL_AUDIO_4X] = &pll_audio_4x_clk.hw, [CLK_PLL_AUDIO_8X] = &pll_audio_8x_clk.hw,
    [CLK_PLL_VIDEO] = &pll_video_clk.common.hw, [CLK_PLL_VIDEO_2X] = &pll_video_2x_clk.hw,
    [CLK_PLL_VE] = &pll_ve_clk.common.hw, [CLK_PLL_DDR0] = &pll_ddr0_clk.common.hw, [CLK_PLL_PERIPH] = &pll_periph_clk.common.hw,
    [CLK_CPU] = &cpu_clk.common.hw, [CLK_AHB] = &ahb_clk.common.hw, [CLK_APB] = &apb_clk.common.hw,
    [CLK_BUS_DMA] = &bus_dma_clk.common.hw, [CLK_BUS_MMC0] = &bus_mmc0_clk.common.hw, [CLK_BUS_MMC1] = &bus_mmc1_clk.common.hw,
    [CLK_BUS_DRAM] = &bus_dram_clk.common.hw, [CLK_BUS_SPI0] = &bus_spi0_clk.common.hw, [CLK_BUS_SPI1] = &bus_spi1_clk.common.hw,
    [CLK_BUS_OTG] = &bus_otg_clk.common.hw, [CLK_BUS_VE] = &bus_ve_clk.common.hw, [CLK_BUS_LCD] = &bus_lcd_clk.common.hw,
    [CLK_BUS_DEINTERLACE] = &bus_deinterlace_clk.common.hw, [CLK_BUS_CSI] = &bus_csi_clk.common.hw, [CLK_BUS_TVD] = &bus_tvd_clk.common.hw,
    [CLK_BUS_TVE] = &bus_tve_clk.common.hw, [CLK_BUS_DE_BE] = &bus_de_be_clk.common.hw, [CLK_BUS_DE_FE] = &bus_de_fe_clk.common.hw,
    [CLK_BUS_CODEC] = &bus_codec_clk.common.hw, [CLK_BUS_SPDIF] = &bus_spdif_clk.common.hw, [CLK_BUS_IR] = &bus_ir_clk.common.hw,
    [CLK_BUS_RSB] = &bus_rsb_clk.common.hw, [CLK_BUS_I2S0] = &bus_i2s0_clk.common.hw, [CLK_BUS_I2C0] = &bus_i2c0_clk.common.hw,
    [CLK_BUS_I2C1] = &bus_i2c1_clk.common.hw, [CLK_BUS_I2C2] = &bus_i2c2_clk.common.hw, [CLK_BUS_PIO] = &bus_pio_clk.common.hw,
    [CLK_BUS_UART0] = &bus_uart0_clk.common.hw, [CLK_BUS_UART1] = &bus_uart1_clk.common.hw, [CLK_BUS_UART2] = &bus_uart2_clk.common.hw,
    [CLK_MMC0] = &mmc0_clk.common.hw, [CLK_MMC0_SAMPLE] = &mmc0_sample_clk.common.hw, [CLK_MMC0_OUTPUT] = &mmc0_output_clk.common.hw,
    [CLK_MMC1] = &mmc1_clk.common.hw, [CLK_MMC1_SAMPLE] = &mmc1_sample_clk.common.hw, [CLK_MMC1_OUTPUT] = &mmc1_output_clk.common.hw,
    [CLK_I2S] = &i2s_clk.common.hw, [CLK_SPDIF] = &spdif_clk.common.hw, [CLK_IR] = &ir_clk.common.hw, [CLK_USB_PHY0] = &usb_phy0_clk.common.hw,
    [CLK_DRAM_VE] = &dram_ve_clk.common.hw, [CLK_DRAM_CSI] = &dram_csi_clk.common.hw, [CLK_DRAM_DEINTERLACE] = &dram_deinterlace_clk.common.hw,
    [CLK_DRAM_TVD] = &dram_tvd_clk.common.hw, [CLK_DRAM_DE_FE] = &dram_de_fe_clk.common.hw, [CLK_DRAM_DE_BE] = &dram_de_be_clk.common.hw,
    [CLK_DE_BE] = &de_be_clk.common.hw, [CLK_DE_FE] = &de_fe_clk.common.hw, [CLK_TCON] = &tcon_clk.common.hw,
    [CLK_DEINTERLACE] = &deinterlace_clk.common.hw, [CLK_TVE2_CLK] = &tve_clk2_clk.common.hw, [CLK_TVE1_CLK] = &tve_clk1_clk.common.hw,
    [CLK_TVD] = &tvd_clk.common.hw, [CLK_CSI] = &csi_clk.common.hw, [CLK_VE] = &ve_clk.common.hw, [CLK_CODEC] = &codec_clk.common.hw, [CLK_AVS] = &avs_clk.common.hw,
], num: CLK_NUMBER };

static suniv_ccu_resets: [ccu_reset_map; 29] = [
    [RST_USB_PHY0] = ccu_reset_map { reg: 0x0cc, bit: BIT(0) }, [RST_BUS_DMA] = ccu_reset_map { reg: 0x2c0, bit: BIT(6) },
    [RST_BUS_MMC0] = ccu_reset_map { reg: 0x2c0, bit: BIT(8) }, [RST_BUS_MMC1] = ccu_reset_map { reg: 0x2c0, bit: BIT(9) },
    [RST_BUS_DRAM] = ccu_reset_map { reg: 0x2c0, bit: BIT(14) }, [RST_BUS_SPI0] = ccu_reset_map { reg: 0x2c0, bit: BIT(20) },
    [RST_BUS_SPI1] = ccu_reset_map { reg: 0x2c0, bit: BIT(21) }, [RST_BUS_OTG] = ccu_reset_map { reg: 0x2c0, bit: BIT(24) },
    [RST_BUS_VE] = ccu_reset_map { reg: 0x2c4, bit: BIT(0) }, [RST_BUS_LCD] = ccu_reset_map { reg: 0x2c4, bit: BIT(4) },
    [RST_BUS_DEINTERLACE] = ccu_reset_map { reg: 0x2c4, bit: BIT(5) }, [RST_BUS_CSI] = ccu_reset_map { reg: 0x2c4, bit: BIT(8) },
    [RST_BUS_TVD] = ccu_reset_map { reg: 0x2c4, bit: BIT(9) }, [RST_BUS_TVE] = ccu_reset_map { reg: 0x2c4, bit: BIT(10) },
    [RST_BUS_DE_BE] = ccu_reset_map { reg: 0x2c4, bit: BIT(12) }, [RST_BUS_DE_FE] = ccu_reset_map { reg: 0x2c4, bit: BIT(14) },
    [RST_BUS_CODEC] = ccu_reset_map { reg: 0x2d0, bit: BIT(0) }, [RST_BUS_SPDIF] = ccu_reset_map { reg: 0x2d0, bit: BIT(1) },
    [RST_BUS_IR] = ccu_reset_map { reg: 0x2d0, bit: BIT(2) }, [RST_BUS_RSB] = ccu_reset_map { reg: 0x2d0, bit: BIT(3) },
    [RST_BUS_I2S0] = ccu_reset_map { reg: 0x2d0, bit: BIT(12) }, [RST_BUS_I2C0] = ccu_reset_map { reg: 0x2d0, bit: BIT(16) },
    [RST_BUS_I2C1] = ccu_reset_map { reg: 0x2d0, bit: BIT(17) }, [RST_BUS_I2C2] = ccu_reset_map { reg: 0x2d0, bit: BIT(18) },
    [RST_BUS_UART0] = ccu_reset_map { reg: 0x2d0, bit: BIT(20) }, [RST_BUS_UART1] = ccu_reset_map { reg: 0x2d0, bit: BIT(21) },
    [RST_BUS_UART2] = ccu_reset_map { reg: 0x2d0, bit: BIT(22) },
];

static suniv_ccu_desc: sunxi_ccu_desc = sunxi_ccu_desc { ccu_clks: suniv_ccu_clks, num_ccu_clks: ARRAY_SIZE!(suniv_ccu_clks), hw_clks: &suniv_hw_clks, resets: suniv_ccu_resets, num_resets: ARRAY_SIZE!(suniv_ccu_resets) };
static mut suniv_pll_cpu_nb: ccu_pll_nb = ccu_pll_nb { common: &pll_cpu_clk.common, enable: BIT(31), lock: BIT(28) };
static mut suniv_cpu_nb: ccu_mux_nb = ccu_mux_nb { common: &cpu_clk.common, cm: &cpu_clk.mux, delay_us: 1, bypass_index: 1 };

unsafe fn suniv_f1c100s_ccu_probe(pdev: *mut platform_device) -> i32 {
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    let mut val: u32 = readl(reg.add(SUNIV_PLL_AUDIO_REG));
    val &= !GENMASK(19, 16);
    writel(val | (3 << 16), reg.add(SUNIV_PLL_AUDIO_REG));
    let ret = devm_sunxi_ccu_probe(&(*pdev).dev, reg, &suniv_ccu_desc);
    if ret != 0 { return ret; }
    ccu_pll_notifier_register(&mut suniv_pll_cpu_nb);
    ccu_mux_notifier_register(pll_cpu_clk.common.hw.clk, &mut suniv_cpu_nb);
    0
}

static suniv_f1c100s_ccu_ids: [of_device_id; 2] = [of_device_id { compatible: "allwinner,suniv-f1c100s-ccu" }, of_device_id { compatible: "" }];
MODULE_DEVICE_TABLE!(of, suniv_f1c100s_ccu_ids);
static mut suniv_f1c100s_ccu_driver: platform_driver = platform_driver { probe: Some(suniv_f1c100s_ccu_probe), driver: driver { name: "suniv-f1c100s-ccu", suppress_bind_attrs: true, of_match_table: suniv_f1c100s_ccu_ids } };
module_platform_driver!(suniv_f1c100s_ccu_driver);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner newer F1C100s CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
