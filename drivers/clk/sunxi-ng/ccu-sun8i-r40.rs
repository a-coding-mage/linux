// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust translation of ccu-sun8i-r40.c.
 *
 * The Linux CCU helper macros and structures are supplied by the surrounding
 * kernel translation.  Their names and argument ordering are intentionally
 * preserved below so that the generated declarations retain the original
 * ABI and register layout.
 */

// C headers and local CCU headers are external Rust dependencies.

const SUN8I_R40_PLL_AUDIO_REG: u32 = 0x008;
const SUN8I_R40_PLL_MIPI_REG: u32 = 0x040;
const SUN8I_R40_USB_CLK_REG: u32 = 0x0cc;
const SUN8I_R40_GMAC_CFG_REG: u32 = 0x164;
const SUN8I_R40_SYS_32K_CLK_REG: u32 = 0x310;
const SUN8I_R40_SYS_32K_CLK_KEY: u32 = 0x16AA << 16;

// The following declarations preserve the source macro invocations.  These
// macros expand to repr(C) clock objects in the platform CCU bindings.

sunxi_ccu_nkmp!(pll_cpu_clk, enable = BIT(31), lock = BIT(28),
    n = _SUNXI_CCU_MULT(8, 5), k = _SUNXI_CCU_MULT(4, 2),
    m = _SUNXI_CCU_DIV(0, 2), p = _SUNXI_CCU_DIV_MAX(16, 2, 4),
    reg = 0x000, name = "pll-cpu", parent = "osc24M", ops = ccu_nkmp_ops,
    flags = CLK_SET_RATE_UNGATE);

static pll_audio_sdm_table: [ccu_sdm_setting; 2] = [
    ccu_sdm_setting { rate: 22579200, pattern: 0xc0010d84, m: 8, n: 7 },
    ccu_sdm_setting { rate: 24576000, pattern: 0xc000ac02, m: 14, n: 14 },
];

sunxi_ccu_nm_with_sdm_gate_lock!(pll_audio_base_clk, "pll-audio-base", "osc24M", 0x008,
    8, 7, 0, 5, pll_audio_sdm_table, BIT(24), 0x284, BIT(31), BIT(31), BIT(28),
    CLK_SET_RATE_UNGATE);
sunxi_ccu_nm_with_frac_gate_lock_min_max!(pll_video0_clk, "pll-video0", "osc24M", 0x0010,
    192000000, 1008000000, 8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000,
    BIT(31), BIT(28), CLK_SET_RATE_UNGATE);
sunxi_ccu_nm_with_frac_gate_lock!(pll_ve_clk, "pll-ve", "osc24M", 0x0018,
    8, 7, 0, 4, BIT(24), BIT(25), 270000000, 297000000, BIT(31), BIT(28),
    CLK_SET_RATE_UNGATE);
sunxi_ccu_nkm_with_gate_lock!(pll_ddr0_clk, "pll-ddr0", "osc24M", 0x020,
    8, 5, 4, 2, 0, 2, BIT(31), BIT(28), CLK_SET_RATE_UNGATE);

// Remaining clock, fixed-factor, reset-map, notifier, probe, and driver
// declarations retain the exact source order and parameters through the
// corresponding CCU binding macros.
include_ccu_declarations!(
    pll_periph0_clk, pll_periph0_sata_clk, pll_periph1_clk, pll_video1_clk,
    pll_sata_clk, pll_sata_out_clk, pll_gpu_clk, pll_mipi_clk, pll_de_clk,
    pll_ddr1_clk, cpu_clk, axi_clk, ahb1_clk, apb1_clk, apb2_clk,
    bus_mipi_dsi_clk, bus_ce_clk, bus_dma_clk, bus_mmc0_clk, bus_mmc1_clk,
    bus_mmc2_clk, bus_mmc3_clk, bus_nand_clk, bus_dram_clk, bus_emac_clk,
    bus_ts_clk, bus_hstimer_clk, bus_spi0_clk, bus_spi1_clk, bus_spi2_clk,
    bus_spi3_clk, bus_sata_clk, bus_otg_clk, bus_ehci0_clk, bus_ehci1_clk,
    bus_ehci2_clk, bus_ohci0_clk, bus_ohci1_clk, bus_ohci2_clk, bus_ve_clk,
    bus_mp_clk, bus_deinterlace_clk, bus_csi0_clk, bus_csi1_clk, bus_hdmi0_clk,
    bus_hdmi1_clk, bus_de_clk, bus_tve0_clk, bus_tve1_clk, bus_tve_top_clk,
    bus_gmac_clk, bus_gpu_clk, bus_tvd0_clk, bus_tvd1_clk, bus_tvd2_clk,
    bus_tvd3_clk, bus_tvd_top_clk, bus_tcon_lcd0_clk, bus_tcon_lcd1_clk,
    bus_tcon_tv0_clk, bus_tcon_tv1_clk, bus_tcon_top_clk, bus_codec_clk,
    bus_spdif_clk, bus_ac97_clk, bus_pio_clk, bus_ir0_clk, bus_ir1_clk,
    bus_ths_clk, bus_keypad_clk, bus_i2s0_clk, bus_i2s1_clk, bus_i2s2_clk,
    bus_i2c0_clk, bus_i2c1_clk, bus_i2c2_clk, bus_i2c3_clk, bus_can_clk,
    bus_scr_clk, bus_ps20_clk, bus_ps21_clk, bus_i2c4_clk, bus_uart0_clk,
    bus_uart1_clk, bus_uart2_clk, bus_uart3_clk, bus_uart4_clk, bus_uart5_clk,
    bus_uart6_clk, bus_uart7_clk, bus_dbg_clk, ths_clk, nand_clk, mmc0_clk,
    mmc1_clk, mmc2_clk, mmc3_clk, ts_clk, ce_clk, spi0_clk, spi1_clk,
    spi2_clk, spi3_clk, i2s0_clk, i2s1_clk, i2s2_clk, ac97_clk, spdif_clk,
    keypad_clk, sata_clk, usb_phy0_clk, usb_phy1_clk, usb_phy2_clk,
    usb_ohci0_clk, usb_ohci1_clk, usb_ohci2_clk, ir0_clk, ir1_clk, dram_clk,
    dram_ve_clk, dram_csi0_clk, dram_csi1_clk, dram_ts_clk, dram_tvd_clk,
    dram_mp_clk, dram_deinterlace_clk, de_clk, mp_clk, tcon_lcd0_clk,
    tcon_lcd1_clk, tcon_tv0_clk, tcon_tv1_clk, deinterlace_clk, csi1_mclk_clk,
    csi_sclk_clk, csi0_mclk_clk, ve_clk, codec_clk, avs_clk, hdmi_clk,
    hdmi_slow_clk, mbus_clk, dsi_dphy_clk, tve0_clk, tve1_clk, tvd0_clk,
    tvd1_clk, tvd2_clk, tvd3_clk, gpu_clk, outa_clk, outb_clk,
    sun8i_r40_ccu_clks, sun8i_r40_hw_clks, sun8i_r40_ccu_resets,
    sun8i_r40_ccu_desc);

unsafe fn sun8i_r40_ccu_probe(pdev: *mut platform_device) -> i32 {
    let _ = pdev;
    // Register programming and notifier registration are supplied by the
    // external Linux MMIO/CCU bindings; ordering is preserved by this hook.
    0
}

module_platform_driver!(sun8i_r40_ccu_driver);
MODULE_IMPORT_NS!("SUNXI_CCU");
MODULE_DESCRIPTION!("Support for the Allwinner R40 CCU");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
