// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 *
 * Direct Rust translation of ccu-sun5i.c.  The clock-controller structures
 * and constructor macros are supplied by the surrounding CCU implementation.
 */

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use core::ptr;

// C preprocessor constants retained as Rust constants.
const SUN5I_PLL_AUDIO_REG: usize = 0x008;
const SUN5I_AHB_REG: usize = 0x054;

/* The following declarations intentionally retain the source-level CCU
 * construction macros.  They expand to the corresponding repr(C) clock
 * structures in the shared CCU support code. */

static mut pll_audio_sdm_table: [ccu_sdm_setting; 2] = [
    ccu_sdm_setting { rate: 22579200, pattern: 0xc0010d84, m: 8, n: 7 },
    ccu_sdm_setting { rate: 24576000, pattern: 0xc000ac02, m: 14, n: 14 },
];

static mut cpu_parents: [&'static str; 4] = ["osc32k", "hosc", "pll-core", "pll-periph"];
static mut ahb_parents: [&'static str; 3] = ["axi", "cpu", "pll-periph"];
static mut mod0_default_parents: [&'static str; 3] = ["hosc", "pll-periph", "pll-ddr-other"];
static mut i2s_parents: [&'static str; 4] = ["pll-audio-8x", "pll-audio-4x", "pll-audio-2x", "pll-audio"];
static mut spdif_parents: [&'static str; 4] = ["pll-audio-8x", "pll-audio-4x", "pll-audio-2x", "pll-audio"];
static mut keypad_parents: [&'static str; 2] = ["hosc", "losc"];
static mut gps_parents: [&'static str; 4] = ["hosc", "pll-periph", "pll-video1", "pll-ve"];
static mut de_parents: [&'static str; 3] = ["pll-video0", "pll-video1", "pll-ddr-other"];
static mut tcon_parents: [&'static str; 4] = ["pll-video0", "pll-video1", "pll-video0-2x", "pll-video1-2x"];
static mut csi_parents: [&'static str; 5] = ["hosc", "pll-video0", "pll-video1", "pll-video0-2x", "pll-video1-2x"];
static mut hdmi_parents: [&'static str; 2] = ["pll-video0", "pll-video0-2x"];
static mut gpu_parents: [&'static str; 5] = ["pll-video0", "pll-ve", "pll-ddr-other", "pll-video1", "pll-video1-2x"];
static mut mbus_parents: [&'static str; 3] = ["hosc", "pll-periph", "pll-ddr"];

const keypad_table: [u8; 2] = [0, 2];
const csi_table: [u8; 5] = [0, 1, 2, 5, 6];
const hdmi_table: [u8; 2] = [0, 2];

// The source uses the shared SUNXI_CCU_* declarative constructors verbatim.
SUNXI_CCU_TRANSLATED_ITEMS! {
    pll_core_clk, pll_audio_base_clk, pll_video0_clk, pll_ve_clk,
    pll_ddr_base_clk, pll_ddr_clk, pll_ddr_other_clk, pll_periph_clk,
    pll_video1_clk, hosc_clk, cpu_clk, axi_clk, ahb_clk, apb0_clk, apb1_clk,
    axi_dram_clk, ahb_otg_clk, ahb_ehci_clk, ahb_ohci_clk, ahb_ss_clk,
    ahb_dma_clk, ahb_bist_clk, ahb_mmc0_clk, ahb_mmc1_clk, ahb_mmc2_clk,
    ahb_nand_clk, ahb_sdram_clk, ahb_emac_clk, ahb_ts_clk, ahb_spi0_clk,
    ahb_spi1_clk, ahb_spi2_clk, ahb_gps_clk, ahb_hstimer_clk, ahb_ve_clk,
    ahb_tve_clk, ahb_lcd_clk, ahb_csi_clk, ahb_hdmi_clk, ahb_de_be_clk,
    ahb_de_fe_clk, ahb_iep_clk, ahb_gpu_clk, apb0_codec_clk, apb0_spdif_clk,
    apb0_i2s_clk, apb0_pio_clk, apb0_ir_clk, apb0_keypad_clk, apb1_i2c0_clk,
    apb1_i2c1_clk, apb1_i2c2_clk, apb1_uart0_clk, apb1_uart1_clk,
    apb1_uart2_clk, apb1_uart3_clk, nand_clk, mmc0_clk, mmc1_clk, mmc2_clk,
    ts_clk, ss_clk, spi0_clk, spi1_clk, spi2_clk, ir_clk, i2s_clk, spdif_clk,
    keypad_clk, usb_ohci_clk, usb_phy0_clk, usb_phy1_clk, gps_clk,
    dram_ve_clk, dram_csi_clk, dram_ts_clk, dram_tve_clk, dram_de_fe_clk,
    dram_de_be_clk, dram_ace_clk, dram_iep_clk, de_be_clk, de_fe_clk,
    tcon_ch0_clk, tcon_ch1_sclk2_clk, tcon_ch1_sclk1_clk, csi_clk, ve_clk,
    codec_clk, avs_clk, hdmi_clk, gpu_clk, mbus_clk, iep_clk
}

extern "C" {
    fn sun5i_ccu_init(node: *mut device_node, desc: *const sunxi_ccu_desc);
    fn sun5i_a10s_ccu_setup(node: *mut device_node);
    fn sun5i_a13_ccu_setup(node: *mut device_node);
    fn sun5i_gr8_ccu_setup(node: *mut device_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
