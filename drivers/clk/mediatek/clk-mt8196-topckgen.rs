// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 MediaTek Inc.
 *                    Guangjie Song <guangjie.song@mediatek.com>
 * Copyright (c) 2025 Collabora Ltd.
 *                    Laura Nao <laura.nao@collabora.com>
 */



/* MUX SEL REG */
const CLK_CFG_UPDATE: u32 = 0x0004;
const CLK_CFG_UPDATE1: u32 = 0x0008;
const CLK_CFG_UPDATE2: u32 = 0x000c;
const CLK_CFG_0: u32 = 0x0010;
const CLK_CFG_0_SET: u32 = 0x0014;
const CLK_CFG_0_CLR: u32 = 0x0018;
const CLK_CFG_1: u32 = 0x0020;
const CLK_CFG_1_SET: u32 = 0x0024;
const CLK_CFG_1_CLR: u32 = 0x0028;
const CLK_CFG_2: u32 = 0x0030;
const CLK_CFG_2_SET: u32 = 0x0034;
const CLK_CFG_2_CLR: u32 = 0x0038;
const CLK_CFG_3: u32 = 0x0040;
const CLK_CFG_3_SET: u32 = 0x0044;
const CLK_CFG_3_CLR: u32 = 0x0048;
const CLK_CFG_4: u32 = 0x0050;
const CLK_CFG_4_SET: u32 = 0x0054;
const CLK_CFG_4_CLR: u32 = 0x0058;
const CLK_CFG_5: u32 = 0x0060;
const CLK_CFG_5_SET: u32 = 0x0064;
const CLK_CFG_5_CLR: u32 = 0x0068;
const CLK_CFG_6: u32 = 0x0070;
const CLK_CFG_6_SET: u32 = 0x0074;
const CLK_CFG_6_CLR: u32 = 0x0078;
const CLK_CFG_7: u32 = 0x0080;
const CLK_CFG_7_SET: u32 = 0x0084;
const CLK_CFG_7_CLR: u32 = 0x0088;
const CLK_CFG_8: u32 = 0x0090;
const CLK_CFG_8_SET: u32 = 0x0094;
const CLK_CFG_8_CLR: u32 = 0x0098;
const CLK_CFG_9: u32 = 0x00a0;
const CLK_CFG_9_SET: u32 = 0x00a4;
const CLK_CFG_9_CLR: u32 = 0x00a8;
const CLK_CFG_10: u32 = 0x00b0;
const CLK_CFG_10_SET: u32 = 0x00b4;
const CLK_CFG_10_CLR: u32 = 0x00b8;
const CLK_CFG_11: u32 = 0x00c0;
const CLK_CFG_11_SET: u32 = 0x00c4;
const CLK_CFG_11_CLR: u32 = 0x00c8;
const CLK_CFG_12: u32 = 0x00d0;
const CLK_CFG_12_SET: u32 = 0x00d4;
const CLK_CFG_12_CLR: u32 = 0x00d8;
const CLK_CFG_13: u32 = 0x00e0;
const CLK_CFG_13_SET: u32 = 0x00e4;
const CLK_CFG_13_CLR: u32 = 0x00e8;
const CLK_CFG_14: u32 = 0x00f0;
const CLK_CFG_14_SET: u32 = 0x00f4;
const CLK_CFG_14_CLR: u32 = 0x00f8;
const CLK_CFG_15: u32 = 0x0100;
const CLK_CFG_15_SET: u32 = 0x0104;
const CLK_CFG_15_CLR: u32 = 0x0108;
const CLK_CFG_16: u32 = 0x0110;
const CLK_CFG_16_SET: u32 = 0x0114;
const CLK_CFG_16_CLR: u32 = 0x0118;
const CLK_CFG_17: u32 = 0x0120;
const CLK_CFG_17_SET: u32 = 0x0124;
const CLK_CFG_17_CLR: u32 = 0x0128;
const CLK_CFG_18: u32 = 0x0130;
const CLK_CFG_18_SET: u32 = 0x0134;
const CLK_CFG_18_CLR: u32 = 0x0138;
const CLK_CFG_19: u32 = 0x0140;
const CLK_CFG_19_SET: u32 = 0x0144;
const CLK_CFG_19_CLR: u32 = 0x0148;
const CLK_AUDDIV_0: u32 = 0x020c;
const CLK_FENC_STATUS_MON_0: u32 = 0x0270;
const CLK_FENC_STATUS_MON_1: u32 = 0x0274;
const CLK_FENC_STATUS_MON_2: u32 = 0x0278;

/* MUX SHIFT */
const TOP_MUX_AXI_SHIFT: u32 = 0;
const TOP_MUX_MEM_SUB_SHIFT: u32 = 1;
const TOP_MUX_IO_NOC_SHIFT: u32 = 2;
const TOP_MUX_PERI_AXI_SHIFT: u32 = 3;
const TOP_MUX_UFS_PEXTP0_AXI_SHIFT: u32 = 4;
const TOP_MUX_PEXTP1_USB_AXI_SHIFT: u32 = 5;
const TOP_MUX_PERI_FMEM_SUB_SHIFT: u32 = 6;
const TOP_MUX_UFS_PEXPT0_MEM_SUB_SHIFT: u32 = 7;
const TOP_MUX_PEXTP1_USB_MEM_SUB_SHIFT: u32 = 8;
const TOP_MUX_PERI_NOC_SHIFT: u32 = 9;
const TOP_MUX_EMI_N_SHIFT: u32 = 10;
const TOP_MUX_EMI_S_SHIFT: u32 = 11;
const TOP_MUX_AP2CONN_HOST_SHIFT: u32 = 14;
const TOP_MUX_ATB_SHIFT: u32 = 15;
const TOP_MUX_CIRQ_SHIFT: u32 = 16;
const TOP_MUX_PBUS_156M_SHIFT: u32 = 17;
const TOP_MUX_EFUSE_SHIFT: u32 = 20;
const TOP_MUX_MCU_L3GIC_SHIFT: u32 = 21;
const TOP_MUX_MCU_INFRA_SHIFT: u32 = 22;
const TOP_MUX_DSP_SHIFT: u32 = 23;
const TOP_MUX_MFG_REF_SHIFT: u32 = 24;
const TOP_MUX_MFG_EB_SHIFT: u32 = 26;
const TOP_MUX_UART_SHIFT: u32 = 27;
const TOP_MUX_SPI0_BCLK_SHIFT: u32 = 28;
const TOP_MUX_SPI1_BCLK_SHIFT: u32 = 29;
const TOP_MUX_SPI2_BCLK_SHIFT: u32 = 30;
const TOP_MUX_SPI3_BCLK_SHIFT: u32 = 0;
const TOP_MUX_SPI4_BCLK_SHIFT: u32 = 1;
const TOP_MUX_SPI5_BCLK_SHIFT: u32 = 2;
const TOP_MUX_SPI6_BCLK_SHIFT: u32 = 3;
const TOP_MUX_SPI7_BCLK_SHIFT: u32 = 4;
const TOP_MUX_MSDC30_1_SHIFT: u32 = 7;
const TOP_MUX_MSDC30_2_SHIFT: u32 = 8;
const TOP_MUX_DISP_PWM_SHIFT: u32 = 9;
const TOP_MUX_USB_TOP_1P_SHIFT: u32 = 10;
const TOP_MUX_SSUSB_XHCI_1P_SHIFT: u32 = 11;
const TOP_MUX_SSUSB_FMCNT_P1_SHIFT: u32 = 12;
const TOP_MUX_I2C_PERI_SHIFT: u32 = 13;
const TOP_MUX_I2C_EAST_SHIFT: u32 = 14;
const TOP_MUX_I2C_WEST_SHIFT: u32 = 15;
const TOP_MUX_I2C_NORTH_SHIFT: u32 = 16;
const TOP_MUX_AES_UFSFDE_SHIFT: u32 = 17;
const TOP_MUX_UFS_SHIFT: u32 = 18;
const TOP_MUX_AUD_1_SHIFT: u32 = 21;
const TOP_MUX_AUD_2_SHIFT: u32 = 22;
const TOP_MUX_ADSP_SHIFT: u32 = 23;
const TOP_MUX_ADSP_UARTHUB_B_SHIFT: u32 = 24;
const TOP_MUX_DPMAIF_MAIN_SHIFT: u32 = 25;
const TOP_MUX_PWM_SHIFT: u32 = 26;
const TOP_MUX_MCUPM_SHIFT: u32 = 27;
const TOP_MUX_SFLASH_SHIFT: u32 = 28;
const TOP_MUX_IPSEAST_SHIFT: u32 = 29;
const TOP_MUX_TL_SHIFT: u32 = 0;
const TOP_MUX_TL_P1_SHIFT: u32 = 1;
const TOP_MUX_TL_P2_SHIFT: u32 = 2;
const TOP_MUX_EMI_INTERFACE_546_SHIFT: u32 = 3;
const TOP_MUX_SDF_SHIFT: u32 = 4;
const TOP_MUX_UARTHUB_BCLK_SHIFT: u32 = 5;
const TOP_MUX_DPSW_CMP_26M_SHIFT: u32 = 6;
const TOP_MUX_SMAPCK_SHIFT: u32 = 7;
const TOP_MUX_SSR_PKA_SHIFT: u32 = 8;
const TOP_MUX_SSR_DMA_SHIFT: u32 = 9;
const TOP_MUX_SSR_KDF_SHIFT: u32 = 10;
const TOP_MUX_SSR_RNG_SHIFT: u32 = 11;
const TOP_MUX_SPU0_SHIFT: u32 = 12;
const TOP_MUX_SPU1_SHIFT: u32 = 13;
const TOP_MUX_DXCC_SHIFT: u32 = 14;

/* CKSTA REG */
const CKSTA_REG: u32 = 0x01c8;
const CKSTA_REG1: u32 = 0x01cc;
const CKSTA_REG2: u32 = 0x01d0;

/* DIVIDER REG */
const CLK_AUDDIV_2: u32 = 0x0214;
const CLK_AUDDIV_3: u32 = 0x0220;
const CLK_AUDDIV_4: u32 = 0x0224;
const CLK_AUDDIV_5: u32 = 0x0228;

/* HW Voter REG */
const HWV_CG_0_SET: u32 = 0x0000;
const HWV_CG_0_CLR: u32 = 0x0004;
const HWV_CG_0_DONE: u32 = 0x2c00;
const HWV_CG_1_SET: u32 = 0x0008;
const HWV_CG_1_CLR: u32 = 0x000c;
const HWV_CG_1_DONE: u32 = 0x2c04;
const HWV_CG_2_SET: u32 = 0x0010;
const HWV_CG_2_CLR: u32 = 0x0014;
const HWV_CG_2_DONE: u32 = 0x2c08;
const HWV_CG_3_SET: u32 = 0x0018;
const HWV_CG_3_CLR: u32 = 0x001c;
const HWV_CG_3_DONE: u32 = 0x2c0c;
const HWV_CG_4_SET: u32 = 0x0020;
const HWV_CG_4_CLR: u32 = 0x0024;
const HWV_CG_4_DONE: u32 = 0x2c10;
const HWV_CG_5_SET: u32 = 0x0028;
const HWV_CG_5_CLR: u32 = 0x002c;
const HWV_CG_5_DONE: u32 = 0x2c14;
const HWV_CG_6_SET: u32 = 0x0030;
const HWV_CG_6_CLR: u32 = 0x0034;
const HWV_CG_6_DONE: u32 = 0x2c18;
const HWV_CG_7_SET: u32 = 0x0038;
const HWV_CG_7_CLR: u32 = 0x003c;
const HWV_CG_7_DONE: u32 = 0x2c1c;
const HWV_CG_8_SET: u32 = 0x0040;
const HWV_CG_8_CLR: u32 = 0x0044;
const HWV_CG_8_DONE: u32 = 0x2c20;

const mtk_fixed_factor top_divs : &[&str] = &[
	factor!(CLK_TOP_MAINPLL_D3, "mainpll_d3", "mainpll", 1, 3),
	factor!(CLK_TOP_MAINPLL_D4, "mainpll_d4", "mainpll", 1, 4),
	factor!(CLK_TOP_MAINPLL_D4_D2, "mainpll_d4_d2", "mainpll", 1, 8),
	factor!(CLK_TOP_MAINPLL_D4_D4, "mainpll_d4_d4", "mainpll", 1, 16),
	factor!(CLK_TOP_MAINPLL_D4_D8, "mainpll_d4_d8", "mainpll", 1, 32),
	factor!(CLK_TOP_MAINPLL_D5, "mainpll_d5", "mainpll", 1, 5),
	factor!(CLK_TOP_MAINPLL_D5_D2, "mainpll_d5_d2", "mainpll", 1, 10),
	factor!(CLK_TOP_MAINPLL_D5_D4, "mainpll_d5_d4", "mainpll", 1, 20),
	factor!(CLK_TOP_MAINPLL_D5_D8, "mainpll_d5_d8", "mainpll", 1, 40),
	factor!(CLK_TOP_MAINPLL_D6, "mainpll_d6", "mainpll", 1, 6),
	factor!(CLK_TOP_MAINPLL_D6_D2, "mainpll_d6_d2", "mainpll", 1, 12),
	factor!(CLK_TOP_MAINPLL_D7, "mainpll_d7", "mainpll", 1, 7),
	factor!(CLK_TOP_MAINPLL_D7_D2, "mainpll_d7_d2", "mainpll", 1, 14),
	factor!(CLK_TOP_MAINPLL_D7_D4, "mainpll_d7_d4", "mainpll", 1, 28),
	factor!(CLK_TOP_MAINPLL_D7_D8, "mainpll_d7_d8", "mainpll", 1, 56),
	factor!(CLK_TOP_MAINPLL_D9, "mainpll_d9", "mainpll", 1, 9),
	factor!(CLK_TOP_UNIVPLL_D4, "univpll_d4", "univpll", 1, 4),
	factor!(CLK_TOP_UNIVPLL_D4_D2, "univpll_d4_d2", "univpll", 1, 8),
	factor!(CLK_TOP_UNIVPLL_D4_D4, "univpll_d4_d4", "univpll", 1, 16),
	factor!(CLK_TOP_UNIVPLL_D4_D8, "univpll_d4_d8", "univpll", 1, 32),
	factor!(CLK_TOP_UNIVPLL_D5, "univpll_d5", "univpll", 1, 5),
	factor!(CLK_TOP_UNIVPLL_D5_D2, "univpll_d5_d2", "univpll", 1, 10),
	factor!(CLK_TOP_UNIVPLL_D5_D4, "univpll_d5_d4", "univpll", 1, 20),
	factor!(CLK_TOP_UNIVPLL_D6, "univpll_d6", "univpll", 1, 6),
	factor!(CLK_TOP_UNIVPLL_D6_D2, "univpll_d6_d2", "univpll", 1, 12),
	factor!(CLK_TOP_UNIVPLL_D6_D4, "univpll_d6_d4", "univpll", 1, 24),
	factor!(CLK_TOP_UNIVPLL_D6_D8, "univpll_d6_d8", "univpll", 1, 48),
	factor!(CLK_TOP_UNIVPLL_D6_D16, "univpll_d6_d16", "univpll", 1, 96),
	factor!(CLK_TOP_UNIVPLL_192M, "univpll_192m", "univpll", 1, 13),
	factor!(CLK_TOP_UNIVPLL_192M_D4, "univpll_192m_d4", "univpll", 1, 52),
	factor!(CLK_TOP_UNIVPLL_192M_D8, "univpll_192m_d8", "univpll", 1, 104),
	factor!(CLK_TOP_UNIVPLL_192M_D16, "univpll_192m_d16", "univpll", 1, 208),
	factor!(CLK_TOP_UNIVPLL_192M_D32, "univpll_192m_d32", "univpll", 1, 416),
	factor!(CLK_TOP_UNIVPLL_192M_D10, "univpll_192m_d10", "univpll", 1, 130),
	factor!(CLK_TOP_TVDPLL1_D2, "tvdpll1_d2", "tvdpll1", 1, 2),
	factor!(CLK_TOP_MSDCPLL_D2, "msdcpll_d2", "msdcpll", 1, 2),
	factor!(CLK_TOP_OSC_D2, "osc_d2", "ulposc", 1, 2),
	factor!(CLK_TOP_OSC_D3, "osc_d3", "ulposc", 1, 3),
	factor!(CLK_TOP_OSC_D4, "osc_d4", "ulposc", 1, 4),
	factor!(CLK_TOP_OSC_D5, "osc_d5", "ulposc", 1, 5),
	factor!(CLK_TOP_OSC_D7, "osc_d7", "ulposc", 1, 7),
	factor!(CLK_TOP_OSC_D8, "osc_d8", "ulposc", 1, 8),
	factor!(CLK_TOP_OSC_D10, "osc_d10", "ulposc", 1, 10),
	factor!(CLK_TOP_OSC_D14, "osc_d14", "ulposc", 1, 14),
	factor!(CLK_TOP_OSC_D20, "osc_d20", "ulposc", 1, 20),
	factor!(CLK_TOP_OSC_D32, "osc_d32", "ulposc", 1, 32),
	factor!(CLK_TOP_OSC_D40, "osc_d40", "ulposc", 1, 40),
};

const axi_parents : &[&str] = &[
	"clk26m",
	"osc_d20",
	"osc_d8",
	"osc_d4",
	"mainpll_d4_d4",
	"mainpll_d7_d2"
};

const mem_sub_parents : &[&str] = &[
	"clk26m",
	"osc_d20",
	"osc_d4",
	"univpll_d4_d4",
	"osc_d3",
	"mainpll_d5_d2",
	"mainpll_d4_d2",
	"mainpll_d6",
	"mainpll_d5",
	"univpll_d5",
	"mainpll_d4",
	"mainpll_d3"
};

const io_noc_parents : &[&str] = &[
	"clk26m",
	"osc_d20",
	"osc_d8",
	"osc_d4",
	"mainpll_d6_d2",
	"mainpll_d9"
};

const shared_axi_parents : &[&str] = &[
	"clk26m",
	"mainpll_d7_d8",
	"mainpll_d5_d8",
	"osc_d8",
	"mainpll_d7_d4",
	"mainpll_d5_d4",
	"mainpll_d4_d4",
	"mainpll_d7_d2"
};

const shared_sub_parents : &[&str] = &[
	"clk26m",
	"mainpll_d5_d8",
	"mainpll_d5_d4",
	"osc_d4",
	"univpll_d4_d4",
	"mainpll_d5_d2",
	"mainpll_d4_d2",
	"mainpll_d6",
	"mainpll_d5",
	"univpll_d5",
	"mainpll_d4"
};

const p_noc_parents : &[&str] = &[
	"clk26m",
	"mainpll_d5_d8",
	"mainpll_d5_d4",
	"osc_d4",
	"univpll_d4_d4",
	"mainpll_d5_d2",
	"mainpll_d4_d2",
	"mainpll_d6",
	"mainpll_d5",
	"univpll_d5",
	"mainpll_d4",
	"mainpll_d3"
};

const emi_parents : &[&str] = &[
	"clk26m",
	"osc_d4",
	"mainpll_d5_d8",
	"mainpll_d5_d4",
	"mainpll_d4_d4",
	"emipll1_ck"
};

const ap2conn_host_parents : &[&str] = &[
	"clk26m",
	"mainpll_d7_d4"
};

const atb_parents : &[&str] = &[
	"clk26m",
	"mainpll_d5_d2",
	"mainpll_d4_d2",
	"mainpll_d6"
};

const cirq_parents : &[&str] = &[
	"clk26m",
	"osc_d20",
	"mainpll_d7_d4"
};

const pbus_156m_parents : &[&str] = &[
	"clk26m",
	"mainpll_d7_d2",
	"osc_d2",
	"mainpll_d7"
};

const efuse_parents : &[&str] = &[
	"clk26m",
	"osc_d20"
};

const mcu_l3gic_parents : &[&str] = &[
	"clk26m",
	"osc_d8",
	"mainpll_d4_d4",
	"mainpll_d7_d2"
};

const mcu_infra_parents : &[&str] = &[
	"clk26m",
	"osc_d20",
	"mainpll_d7_d2",
	"mainpll_d5_d2",
	"mainpll_d4_d2",
	"mainpll_d9",
	"mainpll_d6"
};

const dsp_parents : &[&str] = &[
	"clk26m",
	"osc_d5",
	"osc_d4",
	"osc_d3",
	"univpll_d6_d2",
	"osc_d2",
	"univpll_d5",
	"osc"
};

const mfg_ref_parents : &[&str] = &[
	"clk26m",
	"mainpll_d7_d2"
};

const mfg_eb_parents : &[&str] = &[
	"clk26m",
	"mainpll_d7_d2",
	"mainpll_d6_d2",
	"mainpll_d5_d2"
};

const uart_parents : &[&str] = &[
	"clk26m",
	"univpll_d6_d8",
	"univpll_d6_d4",
	"univpll_d6_d2"
};

const spi_b_parents : &[&str] = &[
	"clk26m",
	"univpll_d6_d4",
	"univpll_d5_d4",
	"mainpll_d4_d4",
	"univpll_d4_d4",
	"mainpll_d6_d2",
	"univpll_192m",
	"univpll_d6_d2"
};

const msdc30_parents : &[&str] = &[
	"clk26m",
	"univpll_d6_d4",
	"mainpll_d6_d2",
	"univpll_d6_d2",
	"msdcpll_d2"
};

const disp_pwm_parents : &[&str] = &[
	"clk26m",
	"osc_d32",
	"osc_d8",
	"univpll_d6_d4",
	"univpll_d5_d4",
	"osc_d4",
	"mainpll_d4_d4"
};

const usb_1p_parents : &[&str] = &[
	"clk26m",
	"univpll_d5_d4"
};

const usb_fmcnt_p1_parents : &[&str] = &[
	"clk26m",
	"univpll_192m_d4"
};

const i2c_parents : &[&str] = &[
	"clk26m",
	"mainpll_d4_d8",
	"univpll_d5_d4",
	"mainpll_d4_d4",
	"univpll_d5_d2"
};

const aes_ufsfde_parents : &[&str] = &[
	"clk26m",
	"mainpll_d4_d4",
	"univpll_d6_d2",
	"mainpll_d4_d2",
	"univpll_d6",
	"mainpll_d4"
};

const ufs_parents : &[&str] = &[
	"clk26m",
	"mainpll_d4_d4",
	"univpll_d6_d2",
	"mainpll_d4_d2",
	"univpll_d6",
	"mainpll_d5",
	"univpll_d5"
};

const aud_1_parents : &[&str] = &[
	"clk26m",
	"vlp_apll1"
};

const aud_2_parents : &[&str] = &[
	"clk26m",
	"vlp_apll2"
};

const adsp_parents : &[&str] = &[
	"clk26m",
	"adsppll"
};

const adsp_uarthub_b_parents : &[&str] = &[
	"clk26m",
	"univpll_d6_d4",
	"univpll_d6_d2"
};

const dpmaif_main_parents : &[&str] = &[
	"clk26m",
	"univpll_d4_d4",
	"univpll_d5_d2",
	"mainpll_d4_d2",
	"univpll_d4_d2",
	"mainpll_d6",
	"univpll_d6",
	"mainpll_d5",
	"univpll_d5"
};

const pwm_parents : &[&str] = &[
	"clk26m",
	"mainpll_d7_d4",
	"univpll_d4_d8"
};

const mcupm_parents : &[&str] = &[
	"clk26m",
	"mainpll_d7_d2",
	"mainpll_d6_d2",
	"univpll_d6_d2",
	"mainpll_d5_d2"
};

const ipseast_parents : &[&str] = &[
	"clk26m",
	"mainpll_d6",
	"mainpll_d5",
	"mainpll_d4",
	"mainpll_d3"
};

const tl_parents : &[&str] = &[
	"clk26m",
	"mainpll_d7_d4",
	"mainpll_d4_d4",
	"mainpll_d5_d2"
};

const md_emi_parents : &[&str] = &[
	"clk26m",
	"mainpll_d4"
};

const sdf_parents : &[&str] = &[
	"clk26m",
	"mainpll_d5_d2",
	"mainpll_d4_d2",
	"mainpll_d6",
	"mainpll_d4",
	"univpll_d4"
};

const uarthub_b_parents : &[&str] = &[
	"clk26m",
	"univpll_d6_d4",
	"univpll_d6_d2"
};

const dpsw_cmp_26m_parents : &[&str] = &[
	"clk26m",
	"osc_d20"
};

const smapparents : &[&str] = &[
	"clk26m",
	"mainpll_d4_d8"
};

const ssr_parents : &[&str] = &[
	"clk26m",
	"mainpll_d4_d4",
	"mainpll_d4_d2",
	"mainpll_d7",
	"mainpll_d6",
	"mainpll_d5"
};

const ssr_kdf_parents : &[&str] = &[
	"clk26m",
	"mainpll_d4_d4",
	"mainpll_d4_d2",
	"mainpll_d7"
};

const ssr_rng_parents : &[&str] = &[
	"clk26m",
	"mainpll_d4_d4",
	"mainpll_d5_d2",
	"mainpll_d4_d2"
};

const spu_parents : &[&str] = &[
	"clk26m",
	"mainpll_d4_d4",
	"mainpll_d4_d2",
	"mainpll_d7",
	"mainpll_d6",
	"mainpll_d5"
};

const dxcc_parents : &[&str] = &[
	"clk26m",
	"mainpll_d4_d8",
	"mainpll_d4_d4",
	"mainpll_d4_d2"
};

const apll_m_parents : &[&str] = &[
	"aud_1",
	"aud_2"
};

const sflash_parents : &[&str] = &[
	"clk26m",
	"mainpll_d7_d8",
	"univpll_d6_d8"
};

const mtk_mux top_muxes : &[&str] = &[
	/* CLK_CFG_0 */
	mux_clr_set_upd!(CLK_TOP_AXI, "axi",
		axi_parents, CLK_CFG_0, CLK_CFG_0_SET,
		CLK_CFG_0_CLR, 0, 3,
		CLK_CFG_UPDATE, TOP_MUX_AXI_SHIFT),
	mux_clr_set_upd!(CLK_TOP_MEM_SUB, "mem_sub",
		mem_sub_parents, CLK_CFG_0, CLK_CFG_0_SET,
		CLK_CFG_0_CLR, 8, 4,
		CLK_CFG_UPDATE, TOP_MUX_MEM_SUB_SHIFT),
	mux_clr_set_upd!(CLK_TOP_IO_NOC, "io_noc",
		io_noc_parents, CLK_CFG_0, CLK_CFG_0_SET,
		CLK_CFG_0_CLR, 16, 3,
		CLK_CFG_UPDATE, TOP_MUX_IO_NOC_SHIFT),
	mux_clr_set_upd!(CLK_TOP_P_AXI, "p_axi",
		shared_axi_parents, CLK_CFG_0, CLK_CFG_0_SET,
		CLK_CFG_0_CLR, 24, 3,
		CLK_CFG_UPDATE, TOP_MUX_PERI_AXI_SHIFT),
	/* CLK_CFG_1 */
	mux_clr_set_upd!(CLK_TOP_UFS_PEXTP0_AXI, "ufs_pextp0_axi",
		shared_axi_parents, CLK_CFG_1, CLK_CFG_1_SET,
		CLK_CFG_1_CLR, 0, 3,
		CLK_CFG_UPDATE, TOP_MUX_UFS_PEXTP0_AXI_SHIFT),
	mux_clr_set_upd!(CLK_TOP_PEXTP1_USB_AXI, "pextp1_usb_axi",
		shared_axi_parents, CLK_CFG_1, CLK_CFG_1_SET,
		CLK_CFG_1_CLR, 8, 3,
		CLK_CFG_UPDATE, TOP_MUX_PEXTP1_USB_AXI_SHIFT),
	mux_clr_set_upd!(CLK_TOP_P_FMEM_SUB, "p_fmem_sub",
		shared_sub_parents, CLK_CFG_1, CLK_CFG_1_SET,
		CLK_CFG_1_CLR, 16, 4,
		CLK_CFG_UPDATE, TOP_MUX_PERI_FMEM_SUB_SHIFT),
	mux_clr_set_upd!(CLK_TOP_PEXPT0_MEM_SUB, "ufs_pexpt0_mem_sub",
		shared_sub_parents, CLK_CFG_1, CLK_CFG_1_SET,
		CLK_CFG_1_CLR, 24, 4,
		CLK_CFG_UPDATE, TOP_MUX_UFS_PEXPT0_MEM_SUB_SHIFT),
	/* CLK_CFG_2 */
	mux_clr_set_upd!(CLK_TOP_PEXTP1_USB_MEM_SUB, "pextp1_usb_mem_sub",
		shared_sub_parents, CLK_CFG_2, CLK_CFG_2_SET,
		CLK_CFG_2_CLR, 0, 4,
		CLK_CFG_UPDATE, TOP_MUX_PEXTP1_USB_MEM_SUB_SHIFT),
	mux_clr_set_upd!(CLK_TOP_P_NOC, "p_noc",
		p_noc_parents, CLK_CFG_2, CLK_CFG_2_SET,
		CLK_CFG_2_CLR, 8, 4,
		CLK_CFG_UPDATE, TOP_MUX_PERI_NOC_SHIFT),
	mux_clr_set_upd!(CLK_TOP_EMI_N, "emi_n",
		emi_parents, CLK_CFG_2, CLK_CFG_2_SET,
		CLK_CFG_2_CLR, 16, 3,
		CLK_CFG_UPDATE, TOP_MUX_EMI_N_SHIFT),
	mux_clr_set_upd!(CLK_TOP_EMI_S, "emi_s",
		emi_parents, CLK_CFG_2, CLK_CFG_2_SET,
		CLK_CFG_2_CLR, 24, 3,
		CLK_CFG_UPDATE, TOP_MUX_EMI_S_SHIFT),
	/* CLK_CFG_3 */
	mux_clr_set_upd!(CLK_TOP_AP2CONN_HOST, "ap2conn_host",
		ap2conn_host_parents, CLK_CFG_3, CLK_CFG_3_SET,
		CLK_CFG_3_CLR, 16, 1,
		CLK_CFG_UPDATE, TOP_MUX_AP2CONN_HOST_SHIFT),
	mux_clr_set_upd!(CLK_TOP_ATB, "atb",
		atb_parents, CLK_CFG_3, CLK_CFG_3_SET,
		CLK_CFG_3_CLR, 24, 2,
		CLK_CFG_UPDATE, TOP_MUX_ATB_SHIFT),
	/* CLK_CFG_4 */
	mux_clr_set_upd!(CLK_TOP_CIRQ, "cirq",
		cirq_parents, CLK_CFG_4, CLK_CFG_4_SET,
		CLK_CFG_4_CLR, 0, 2,
		CLK_CFG_UPDATE, TOP_MUX_CIRQ_SHIFT),
	mux_clr_set_upd!(CLK_TOP_PBUS_156M, "pbus_156m",
		pbus_156m_parents, CLK_CFG_4, CLK_CFG_4_SET,
		CLK_CFG_4_CLR, 8, 2,
		CLK_CFG_UPDATE, TOP_MUX_PBUS_156M_SHIFT),
	/* CLK_CFG_5 */
	mux_clr_set_upd!(CLK_TOP_EFUSE, "efuse",
		efuse_parents, CLK_CFG_5, CLK_CFG_5_SET,
		CLK_CFG_5_CLR, 0, 1,
		CLK_CFG_UPDATE, TOP_MUX_EFUSE_SHIFT),
	mux_clr_set_upd!(CLK_TOP_MCL3GIC, "mcu_l3gic",
		mcu_l3gic_parents, CLK_CFG_5, CLK_CFG_5_SET,
		CLK_CFG_5_CLR, 8, 2,
		CLK_CFG_UPDATE, TOP_MUX_MCU_L3GIC_SHIFT),
	mux_clr_set_upd!(CLK_TOP_MCINFRA, "mcu_infra",
		mcu_infra_parents, CLK_CFG_5, CLK_CFG_5_SET,
		CLK_CFG_5_CLR, 16, 3,
		CLK_CFG_UPDATE, TOP_MUX_MCU_INFRA_SHIFT),
	mux_clr_set_upd!(CLK_TOP_DSP, "dsp",
		dsp_parents, CLK_CFG_5, CLK_CFG_5_SET,
		CLK_CFG_5_CLR, 24, 3,
		CLK_CFG_UPDATE, TOP_MUX_DSP_SHIFT),
	/* CLK_CFG_6 */
	mux_gate_fenc_clr_set_upd_flags!(CLK_TOP_MFG_REF, "mfg_ref", mfg_ref_parents,
		NULL, array_size!(mfg_ref_parents),
		CLK_CFG_6, CLK_CFG_6_SET, CLK_CFG_6_CLR,
		0, 1, 7, CLK_CFG_UPDATE, TOP_MUX_MFG_REF_SHIFT,
		CLK_FENC_STATUS_MON_0, 7, CLK_IGNORE_UNUSED),
	mux_gate_clr_set_upd!(CLK_TOP_MFG_EB, "mfg_eb",
		mfg_eb_parents, CLK_CFG_6, CLK_CFG_6_SET,
		CLK_CFG_6_CLR, 16, 2,
		23, CLK_CFG_UPDATE, TOP_MUX_MFG_EB_SHIFT),
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_UART, "uart", uart_parents,
		CLK_CFG_6, CLK_CFG_6_SET, CLK_CFG_6_CLR,
		HWV_CG_3_DONE, HWV_CG_3_SET, HWV_CG_3_CLR,
		24, 2, 31, CLK_CFG_UPDATE, TOP_MUX_UART_SHIFT,
		CLK_FENC_STATUS_MON_0, 4),
	/* CLK_CFG_7 */
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_SPI0_BCLK, "spi0_b", spi_b_parents,
		CLK_CFG_7, CLK_CFG_7_SET, CLK_CFG_7_CLR,
		HWV_CG_4_DONE, HWV_CG_4_SET, HWV_CG_4_CLR,
		0, 3, 7, CLK_CFG_UPDATE, TOP_MUX_SPI0_BCLK_SHIFT,
		CLK_FENC_STATUS_MON_0, 3),
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_SPI1_BCLK, "spi1_b", spi_b_parents,
		CLK_CFG_7, CLK_CFG_7_SET, CLK_CFG_7_CLR,
		HWV_CG_4_DONE, HWV_CG_4_SET, HWV_CG_4_CLR,
		8, 3, 15, CLK_CFG_UPDATE, TOP_MUX_SPI1_BCLK_SHIFT,
		CLK_FENC_STATUS_MON_0, 2),
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_SPI2_BCLK, "spi2_b", spi_b_parents,
		CLK_CFG_7, CLK_CFG_7_SET, CLK_CFG_7_CLR,
		HWV_CG_4_DONE, HWV_CG_4_SET, HWV_CG_4_CLR,
		16, 3, 23, CLK_CFG_UPDATE, TOP_MUX_SPI2_BCLK_SHIFT,
		CLK_FENC_STATUS_MON_0, 1),
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_SPI3_BCLK, "spi3_b", spi_b_parents,
		CLK_CFG_7, CLK_CFG_7_SET, CLK_CFG_7_CLR,
		HWV_CG_4_DONE, HWV_CG_4_SET, HWV_CG_4_CLR,
		24, 3, 31, CLK_CFG_UPDATE1, TOP_MUX_SPI3_BCLK_SHIFT,
		CLK_FENC_STATUS_MON_0, 0),
	/* CLK_CFG_8 */
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_SPI4_BCLK, "spi4_b", spi_b_parents,
		CLK_CFG_8, CLK_CFG_8_SET, CLK_CFG_8_CLR,
		HWV_CG_5_DONE, HWV_CG_5_SET, HWV_CG_5_CLR,
		0, 3, 7, CLK_CFG_UPDATE1, TOP_MUX_SPI4_BCLK_SHIFT,
		CLK_FENC_STATUS_MON_1, 31),
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_SPI5_BCLK, "spi5_b", spi_b_parents,
		CLK_CFG_8, CLK_CFG_8_SET, CLK_CFG_8_CLR,
		HWV_CG_5_DONE, HWV_CG_5_SET, HWV_CG_5_CLR,
		8, 3, 15, CLK_CFG_UPDATE1, TOP_MUX_SPI5_BCLK_SHIFT,
		CLK_FENC_STATUS_MON_1, 30),
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_SPI6_BCLK, "spi6_b", spi_b_parents,
		CLK_CFG_8, CLK_CFG_8_SET, CLK_CFG_8_CLR,
		HWV_CG_5_DONE, HWV_CG_5_SET, HWV_CG_5_CLR,
		16, 3, 23, CLK_CFG_UPDATE1, TOP_MUX_SPI6_BCLK_SHIFT,
		CLK_FENC_STATUS_MON_1, 29),
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_SPI7_BCLK, "spi7_b", spi_b_parents,
		CLK_CFG_8, CLK_CFG_8_SET, CLK_CFG_8_CLR,
		HWV_CG_5_DONE, HWV_CG_5_SET, HWV_CG_5_CLR,
		24, 3, 31, CLK_CFG_UPDATE1, TOP_MUX_SPI7_BCLK_SHIFT,
		CLK_FENC_STATUS_MON_1, 28),
	/* CLK_CFG_9 */
	mux_gate_fenc_clr_set_upd!(CLK_TOP_MSDC30_1, "msdc30_1", msdc30_parents,
		CLK_CFG_9, CLK_CFG_9_SET, CLK_CFG_9_CLR,
		16, 3, 23, CLK_CFG_UPDATE1, TOP_MUX_MSDC30_1_SHIFT,
		CLK_FENC_STATUS_MON_1, 25),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_MSDC30_2, "msdc30_2", msdc30_parents,
		CLK_CFG_9, CLK_CFG_9_SET, CLK_CFG_9_CLR,
		24, 3, 31, CLK_CFG_UPDATE1, TOP_MUX_MSDC30_2_SHIFT,
		CLK_FENC_STATUS_MON_1, 24),
	/* CLK_CFG_10 */
	mux_gate_fenc_clr_set_upd!(CLK_TOP_DISP_PWM, "disp_pwm", disp_pwm_parents,
		CLK_CFG_10, CLK_CFG_10_SET, CLK_CFG_10_CLR,
		0, 3, 7, CLK_CFG_UPDATE1, TOP_MUX_DISP_PWM_SHIFT,
		CLK_FENC_STATUS_MON_1, 23),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_USB_TOP_1P, "usb_1p", usb_1p_parents,
		CLK_CFG_10, CLK_CFG_10_SET, CLK_CFG_10_CLR,
		8, 1, 15, CLK_CFG_UPDATE1, TOP_MUX_USB_TOP_1P_SHIFT,
		CLK_FENC_STATUS_MON_1, 22),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_USB_XHCI_1P, "usb_xhci_1p", usb_1p_parents,
		CLK_CFG_10, CLK_CFG_10_SET, CLK_CFG_10_CLR,
		16, 1, 23, CLK_CFG_UPDATE1, TOP_MUX_SSUSB_XHCI_1P_SHIFT,
		CLK_FENC_STATUS_MON_1, 21),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_USB_FMCNT_P1, "usb_fmcnt_p1", usb_fmcnt_p1_parents,
		CLK_CFG_10, CLK_CFG_10_SET, CLK_CFG_10_CLR,
		24, 1, 31, CLK_CFG_UPDATE1, TOP_MUX_SSUSB_FMCNT_P1_SHIFT,
		CLK_FENC_STATUS_MON_1, 20),
	/* CLK_CFG_11 */
	mux_gate_fenc_clr_set_upd!(CLK_TOP_I2C_P, "i2c_p", i2c_parents,
		CLK_CFG_11, CLK_CFG_11_SET, CLK_CFG_11_CLR,
		0, 3, 7, CLK_CFG_UPDATE1, TOP_MUX_I2C_PERI_SHIFT,
		CLK_FENC_STATUS_MON_1, 19),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_I2C_EAST, "i2c_east", i2c_parents,
		CLK_CFG_11, CLK_CFG_11_SET, CLK_CFG_11_CLR,
		8, 3, 15, CLK_CFG_UPDATE1, TOP_MUX_I2C_EAST_SHIFT,
		CLK_FENC_STATUS_MON_1, 18),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_I2C_WEST, "i2c_west", i2c_parents,
		CLK_CFG_11, CLK_CFG_11_SET, CLK_CFG_11_CLR,
		16, 3, 23, CLK_CFG_UPDATE1, TOP_MUX_I2C_WEST_SHIFT,
		CLK_FENC_STATUS_MON_1, 17),
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_I2C_NORTH, "i2c_north", i2c_parents,
		CLK_CFG_11, CLK_CFG_11_SET, CLK_CFG_11_CLR,
		HWV_CG_6_DONE, HWV_CG_6_SET, HWV_CG_6_CLR,
		24, 3, 31, CLK_CFG_UPDATE1, TOP_MUX_I2C_NORTH_SHIFT,
		CLK_FENC_STATUS_MON_1, 16),
	/* CLK_CFG_12 */
	mux_gate_fenc_clr_set_upd!(CLK_TOP_AES_UFSFDE, "aes_ufsfde", aes_ufsfde_parents,
		CLK_CFG_12, CLK_CFG_12_SET, CLK_CFG_12_CLR,
		0, 3, 7, CLK_CFG_UPDATE1, TOP_MUX_AES_UFSFDE_SHIFT,
		CLK_FENC_STATUS_MON_1, 15),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_UFS, "ufs", ufs_parents,
		CLK_CFG_12, CLK_CFG_12_SET, CLK_CFG_12_CLR,
		8, 3, 15, CLK_CFG_UPDATE1, TOP_MUX_UFS_SHIFT,
		CLK_FENC_STATUS_MON_1, 14),
	/* CLK_CFG_13 */
	mux_gate_fenc_clr_set_upd!(CLK_TOP_AUD_1, "aud_1", aud_1_parents,
		CLK_CFG_13, CLK_CFG_13_SET, CLK_CFG_13_CLR,
		0, 1, 7, CLK_CFG_UPDATE1, TOP_MUX_AUD_1_SHIFT,
		CLK_FENC_STATUS_MON_1, 11),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_AUD_2, "aud_2", aud_2_parents,
		CLK_CFG_13, CLK_CFG_13_SET, CLK_CFG_13_CLR,
		8, 1, 15, CLK_CFG_UPDATE1, TOP_MUX_AUD_2_SHIFT,
		CLK_FENC_STATUS_MON_1, 10),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_ADSP, "adsp", adsp_parents,
		CLK_CFG_13, CLK_CFG_13_SET, CLK_CFG_13_CLR,
		16, 1, 23, CLK_CFG_UPDATE1, TOP_MUX_ADSP_SHIFT,
		CLK_FENC_STATUS_MON_1, 9),
	mux_gate_clr_set_upd!(CLK_TOP_ADSP_UARTHUB_B, "adsp_uarthub_b",
		adsp_uarthub_b_parents, CLK_CFG_13, CLK_CFG_13_SET,
		CLK_CFG_13_CLR, 24, 2, 31,
		CLK_CFG_UPDATE1, TOP_MUX_ADSP_UARTHUB_B_SHIFT),
	/* CLK_CFG_14 */
	mux_gate_fenc_clr_set_upd!(CLK_TOP_DPMAIF_MAIN, "dpmaif_main", dpmaif_main_parents,
		CLK_CFG_14, CLK_CFG_14_SET, CLK_CFG_14_CLR,
		0, 4, 7, CLK_CFG_UPDATE1, TOP_MUX_DPMAIF_MAIN_SHIFT,
		CLK_FENC_STATUS_MON_1, 7),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_PWM, "pwm", pwm_parents,
		CLK_CFG_14, CLK_CFG_14_SET, CLK_CFG_14_CLR,
		8, 2, 15, CLK_CFG_UPDATE1, TOP_MUX_PWM_SHIFT,
		CLK_FENC_STATUS_MON_1, 6),
	mux_clr_set_upd!(CLK_TOP_MCUPM, "mcupm",
		mcupm_parents, CLK_CFG_14, CLK_CFG_14_SET,
		CLK_CFG_14_CLR, 16, 3,
		CLK_CFG_UPDATE1, TOP_MUX_MCUPM_SHIFT),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_SFLASH, "sflash", sflash_parents,
		CLK_CFG_14, CLK_CFG_14_SET, CLK_CFG_14_CLR,
		24, 2, 31, CLK_CFG_UPDATE1, TOP_MUX_SFLASH_SHIFT,
		CLK_FENC_STATUS_MON_1, 4),
	/* CLK_CFG_15 */
	mux_gate_fenc_clr_set_upd!(CLK_TOP_IPSEAST, "ipseast", ipseast_parents,
		CLK_CFG_15, CLK_CFG_15_SET, CLK_CFG_15_CLR,
		0, 3, 7, CLK_CFG_UPDATE1, TOP_MUX_IPSEAST_SHIFT,
		CLK_FENC_STATUS_MON_1, 3),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_TL, "tl", tl_parents,
		CLK_CFG_15, CLK_CFG_15_SET, CLK_CFG_15_CLR,
		16, 2, 23, CLK_CFG_UPDATE2, TOP_MUX_TL_SHIFT,
		CLK_FENC_STATUS_MON_1, 1),
	mux_gate_fenc_clr_set_upd!(CLK_TOP_TL_P1, "tl_p1", tl_parents,
		CLK_CFG_15, CLK_CFG_15_SET, CLK_CFG_15_CLR,
		24, 2, 31, CLK_CFG_UPDATE2, TOP_MUX_TL_P1_SHIFT,
		CLK_FENC_STATUS_MON_1, 0),
	/* CLK_CFG_16 */
	mux_gate_fenc_clr_set_upd!(CLK_TOP_TL_P2, "tl_p2", tl_parents,
		CLK_CFG_16, CLK_CFG_16_SET, CLK_CFG_16_CLR,
		0, 2, 7, CLK_CFG_UPDATE2, TOP_MUX_TL_P2_SHIFT,
		CLK_FENC_STATUS_MON_2, 31),
	mux_clr_set_upd!(CLK_TOP_EMI_INTERFACE_546, "emi_interface_546",
		md_emi_parents, CLK_CFG_16, CLK_CFG_16_SET,
		CLK_CFG_16_CLR, 8, 1,
		CLK_CFG_UPDATE2, TOP_MUX_EMI_INTERFACE_546_SHIFT),
	mux_clr_set_upd!(CLK_TOP_SDF, "sdf",
		sdf_parents, CLK_CFG_16, CLK_CFG_16_SET,
		CLK_CFG_16_CLR, 16, 3,
		CLK_CFG_UPDATE2, TOP_MUX_SDF_SHIFT),
	mux_gate_hwv_fenc_clr_set_upd!(CLK_TOP_UARTHUB_BCLK, "uarthub_b", uarthub_b_parents,
		CLK_CFG_16, CLK_CFG_16_SET, CLK_CFG_16_CLR,
		HWV_CG_7_DONE, HWV_CG_7_SET, HWV_CG_7_CLR,
		24, 2, 31, CLK_CFG_UPDATE2, TOP_MUX_UARTHUB_BCLK_SHIFT,
		CLK_FENC_STATUS_MON_2, 28),
	/* CLK_CFG_17 */
	mux_clr_set_upd!(CLK_TOP_DPSW_CMP_26M, "dpsw_cmp_26m",
		dpsw_cmp_26m_parents, CLK_CFG_17, CLK_CFG_17_SET,
		CLK_CFG_17_CLR, 0, 1,
		CLK_CFG_UPDATE2, TOP_MUX_DPSW_CMP_26M_SHIFT),
	mux_clr_set_upd!(CLK_TOP_SMAP, "smap",
		smapparents, CLK_CFG_17, CLK_CFG_17_SET,
		CLK_CFG_17_CLR, 8, 1,
		CLK_CFG_UPDATE2, TOP_MUX_SMAPCK_SHIFT),
	mux_clr_set_upd!(CLK_TOP_SSR_PKA, "ssr_pka",
		ssr_parents, CLK_CFG_17, CLK_CFG_17_SET,
		CLK_CFG_17_CLR, 16, 3,
		CLK_CFG_UPDATE2, TOP_MUX_SSR_PKA_SHIFT),
	mux_clr_set_upd!(CLK_TOP_SSR_DMA, "ssr_dma",
		ssr_parents, CLK_CFG_17, CLK_CFG_17_SET,
		CLK_CFG_17_CLR, 24, 3,
		CLK_CFG_UPDATE2, TOP_MUX_SSR_DMA_SHIFT),
	/* CLK_CFG_18 */
	mux_clr_set_upd!(CLK_TOP_SSR_KDF, "ssr_kdf",
		ssr_kdf_parents, CLK_CFG_18, CLK_CFG_18_SET,
		CLK_CFG_18_CLR, 0, 2,
		CLK_CFG_UPDATE2, TOP_MUX_SSR_KDF_SHIFT),
	mux_clr_set_upd!(CLK_TOP_SSR_RNG, "ssr_rng",
		ssr_rng_parents, CLK_CFG_18, CLK_CFG_18_SET,
		CLK_CFG_18_CLR, 8, 2,
		CLK_CFG_UPDATE2, TOP_MUX_SSR_RNG_SHIFT),
	mux_clr_set_upd!(CLK_TOP_SPU0, "spu0",
		spu_parents, CLK_CFG_18, CLK_CFG_18_SET,
		CLK_CFG_18_CLR, 16, 3,
		CLK_CFG_UPDATE2, TOP_MUX_SPU0_SHIFT),
	mux_clr_set_upd!(CLK_TOP_SPU1, "spu1",
		spu_parents, CLK_CFG_18, CLK_CFG_18_SET,
		CLK_CFG_18_CLR, 24, 3,
		CLK_CFG_UPDATE2, TOP_MUX_SPU1_SHIFT),
	/* CLK_CFG_19 */
	mux_clr_set_upd!(CLK_TOP_DXCC, "dxcc",
		dxcc_parents, CLK_CFG_19, CLK_CFG_19_SET,
		CLK_CFG_19_CLR, 0, 2,
		CLK_CFG_UPDATE2, TOP_MUX_DXCC_SHIFT),
};

const mtk_composite top_aud_divs : &[&str] = &[
	/* CLK_AUDDIV_2 */
	mux_div_gate!(CLK_TOP_APLL_I2SIN0, "apll_i2sin0_m", apll_m_parents,
		CLK_AUDDIV_0, 16, 1, CLK_AUDDIV_2, 0, 8, CLK_AUDDIV_0, 0),
	mux_div_gate!(CLK_TOP_APLL_I2SIN1, "apll_i2sin1_m", apll_m_parents,
		CLK_AUDDIV_0, 17, 1, CLK_AUDDIV_2, 8, 8, CLK_AUDDIV_0, 1),
	mux_div_gate!(CLK_TOP_APLL_I2SIN2, "apll_i2sin2_m", apll_m_parents,
		CLK_AUDDIV_0, 18, 1, CLK_AUDDIV_2, 16, 8, CLK_AUDDIV_0, 2),
	mux_div_gate!(CLK_TOP_APLL_I2SIN3, "apll_i2sin3_m", apll_m_parents,
		CLK_AUDDIV_0, 19, 1, CLK_AUDDIV_2, 24, 8, CLK_AUDDIV_0, 3),
	/* CLK_AUDDIV_3 */
	mux_div_gate!(CLK_TOP_APLL_I2SIN4, "apll_i2sin4_m", apll_m_parents,
		CLK_AUDDIV_0, 20, 1, CLK_AUDDIV_3, 0, 8, CLK_AUDDIV_0, 4),
	mux_div_gate!(CLK_TOP_APLL_I2SIN6, "apll_i2sin6_m", apll_m_parents,
		CLK_AUDDIV_0, 21, 1, CLK_AUDDIV_3, 8, 8, CLK_AUDDIV_0, 5),
	mux_div_gate!(CLK_TOP_APLL_I2SOUT0, "apll_i2sout0_m", apll_m_parents,
		CLK_AUDDIV_0, 22, 1, CLK_AUDDIV_3, 16, 8, CLK_AUDDIV_0, 6),
	mux_div_gate!(CLK_TOP_APLL_I2SOUT1, "apll_i2sout1_m", apll_m_parents,
		CLK_AUDDIV_0, 23, 1, CLK_AUDDIV_3, 24, 8, CLK_AUDDIV_0, 7),
	/* CLK_AUDDIV_4 */
	mux_div_gate!(CLK_TOP_APLL_I2SOUT2, "apll_i2sout2_m", apll_m_parents,
		CLK_AUDDIV_0, 24, 1, CLK_AUDDIV_4, 0, 8, CLK_AUDDIV_0, 8),
	mux_div_gate!(CLK_TOP_APLL_I2SOUT3, "apll_i2sout3_m", apll_m_parents,
		CLK_AUDDIV_0, 25, 1, CLK_AUDDIV_4, 8, 8, CLK_AUDDIV_0, 9),
	mux_div_gate!(CLK_TOP_APLL_I2SOUT4, "apll_i2sout4_m", apll_m_parents,
		CLK_AUDDIV_0, 26, 1, CLK_AUDDIV_4, 16, 8, CLK_AUDDIV_0, 10),
	mux_div_gate!(CLK_TOP_APLL_I2SOUT6, "apll_i2sout6_m", apll_m_parents,
		CLK_AUDDIV_0, 27, 1, CLK_AUDDIV_4, 24, 8, CLK_AUDDIV_0, 11),
	/* CLK_AUDDIV_5 */
	mux_div_gate!(CLK_TOP_APLL_FMI2S, "apll_fmi2s_m", apll_m_parents,
		CLK_AUDDIV_0, 28, 1, CLK_AUDDIV_5, 0, 8, CLK_AUDDIV_0, 12),
	mux!(CLK_TOP_APLL_TDMOUT, "apll_tdmout_m",
	    apll_m_parents, CLK_AUDDIV_0, 29, 1),
	div_gate!(CLK_TOP_APLL12_DIV_TDMOUT_M, "apll12_div_tdmout_m",
		"apll_tdmout_m", CLK_AUDDIV_0,
		13, CLK_AUDDIV_5, 8, 8),
	div_gate!(CLK_TOP_APLL12_DIV_TDMOUT_B, "apll12_div_tdmout_b",
		"apll_tdmout_m", CLK_AUDDIV_0,
		14, CLK_AUDDIV_5, 8, 16),
};

const mtk_clk_desc topck_desc = {
	.factor_clks = top_divs,
	.num_factor_clks = array_size!(top_divs),
	.mux_clks = top_muxes,
	.num_mux_clks = array_size!(top_muxes),
	.composite_clks = top_aud_divs,
	.num_composite_clks = array_size!(top_aud_divs)
};

const of_device_id of_match_clk_mt8196_ck : &[&str] = &[
	{ .compatible = "mediatek,mt8196-topckgen", .data = &topck_desc },
	{ /* sentinel */ }
};
// MODULE_DEVICE_TABLE(of, of_match_clk_mt8196_ck);

static mut platform_driver clk_mt8196_topck_drv = {
	.probe = mtk_clk_simple_probe,
	.remove = mtk_clk_simple_remove,
	.driver = {
		.name = "clk-mt8196-topck",
		.of_match_table = of_match_clk_mt8196_ck,
	},
};

// MODULE_DESCRIPTION("MediaTek MT8196 top clock generators driver");
// module_platform_driver(clk_mt8196_topck_drv);
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
