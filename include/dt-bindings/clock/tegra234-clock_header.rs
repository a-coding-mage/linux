/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2018-2022, NVIDIA CORPORATION. All rights reserved. */


/**
 * @file
 * @defgroup bpmp_clock_ids Clock ID's
 * @{
 */
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_ACTMON */
pub const TEGRA234_CLK_ACTMON: u32 = 1;
/** @brief output of gate CLK_ENB_ADSP */
pub const TEGRA234_CLK_ADSP: u32 = 2;
/** @brief output of gate CLK_ENB_ADSPNEON */
pub const TEGRA234_CLK_ADSPNEON: u32 = 3;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AHUB */
pub const TEGRA234_CLK_AHUB: u32 = 4;
/** @brief output of gate CLK_ENB_APB2APE */
pub const TEGRA234_CLK_APB2APE: u32 = 5;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_APE */
pub const TEGRA234_CLK_APE: u32 = 6;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AUD_MCLK */
pub const TEGRA234_CLK_AUD_MCLK: u32 = 7;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AXI_CBB */
pub const TEGRA234_CLK_AXI_CBB: u32 = 8;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_CAN1 */
pub const TEGRA234_CLK_CAN1: u32 = 9;
/** @brief output of gate CLK_ENB_CAN1_HOST */
pub const TEGRA234_CLK_CAN1_HOST: u32 = 10;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_CAN2 */
pub const TEGRA234_CLK_CAN2: u32 = 11;
/** @brief output of gate CLK_ENB_CAN2_HOST */
pub const TEGRA234_CLK_CAN2_HOST: u32 = 12;
/** @brief output of divider CLK_RST_CONTROLLER_CLK_M_DIVIDE */
pub const TEGRA234_CLK_CLK_M: u32 = 14;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC1 */
pub const TEGRA234_CLK_DMIC1: u32 = 15;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC2 */
pub const TEGRA234_CLK_DMIC2: u32 = 16;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC3 */
pub const TEGRA234_CLK_DMIC3: u32 = 17;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC4 */
pub const TEGRA234_CLK_DMIC4: u32 = 18;
/** @brief output of gate CLK_ENB_DPAUX */
pub const TEGRA234_CLK_DPAUX: u32 = 19;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVJPG1 */
pub const TEGRA234_CLK_NVJPG1: u32 = 20;
/**
 * @brief output of mux controlled by CLK_RST_CONTROLLER_ACLK_BURST_POLICY
 * divided by the divider controlled by ACLK_CLK_DIVISOR in
 * CLK_RST_CONTROLLER_SUPER_ACLK_DIVIDER
 */
pub const TEGRA234_CLK_ACLK: u32 = 21;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_MSS_ENCRYPT switch divider output */
pub const TEGRA234_CLK_MSS_ENCRYPT: u32 = 22;
/** @brief clock recovered from EAVB input */
pub const TEGRA234_CLK_EQOS_RX_INPUT: u32 = 23;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_AON_APB switch divider output */
pub const TEGRA234_CLK_AON_APB: u32 = 25;
/** @brief CLK_RST_CONTROLLER_AON_NIC_RATE divider output */
pub const TEGRA234_CLK_AON_NIC: u32 = 26;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_AON_CPU_NIC switch divider output */
pub const TEGRA234_CLK_AON_CPU_NIC: u32 = 27;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLA1_BASE for use by audio clocks */
pub const TEGRA234_CLK_PLLA1: u32 = 28;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSPK1 */
pub const TEGRA234_CLK_DSPK1: u32 = 29;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSPK2 */
pub const TEGRA234_CLK_DSPK2: u32 = 30;
/**
 * @brief controls the EMC clock frequency.
 * @details Doing a clk_set_rate on this clock will select the
 * appropriate clock source, program the source rate and execute a
 * specific sequence to switch to the new clock source for both memory
 * controllers. This can be used to control the balance between memory
 * throughput and memory controller power.
 */
pub const TEGRA234_CLK_EMC: u32 = 31;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_AXI_CLK_0 divider gated output */
pub const TEGRA234_CLK_EQOS_AXI: u32 = 32;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_PTP_REF_CLK_0 divider gated output */
pub const TEGRA234_CLK_EQOS_PTP_REF: u32 = 33;
/** @brief output of gate CLK_ENB_EQOS_RX */
pub const TEGRA234_CLK_EQOS_RX: u32 = 34;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_TX_CLK divider gated output */
pub const TEGRA234_CLK_EQOS_TX: u32 = 35;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH1 */
pub const TEGRA234_CLK_EXTPERIPH1: u32 = 36;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH2 */
pub const TEGRA234_CLK_EXTPERIPH2: u32 = 37;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH3 */
pub const TEGRA234_CLK_EXTPERIPH3: u32 = 38;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH4 */
pub const TEGRA234_CLK_EXTPERIPH4: u32 = 39;
/** @brief output of gate CLK_ENB_FUSE */
pub const TEGRA234_CLK_FUSE: u32 = 40;
/** @brief output of GPU GPC0 clkGen (in 1x mode same rate as GPC0 MUX2 out) */
pub const TEGRA234_CLK_GPC0CLK: u32 = 41;
/** @brief TODO */
pub const TEGRA234_CLK_GPU_PWR: u32 = 42;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_HDA2CODEC_2X */
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X */
pub const TEGRA234_CLK_HOST1X: u32 = 46;
/** @brief xusb_hs_hsicp_clk */
pub const TEGRA234_CLK_XUSB_HS_HSICP: u32 = 47;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C1 */
pub const TEGRA234_CLK_I2C1: u32 = 48;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C2 */
pub const TEGRA234_CLK_I2C2: u32 = 49;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C3 */
pub const TEGRA234_CLK_I2C3: u32 = 50;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C4 */
pub const TEGRA234_CLK_I2C4: u32 = 51;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C6 */
pub const TEGRA234_CLK_I2C6: u32 = 52;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C7 */
pub const TEGRA234_CLK_I2C7: u32 = 53;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C8 */
pub const TEGRA234_CLK_I2C8: u32 = 54;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C9 */
pub const TEGRA234_CLK_I2C9: u32 = 55;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S1 */
pub const TEGRA234_CLK_I2S1: u32 = 56;
/** @brief clock recovered from I2S1 input */
pub const TEGRA234_CLK_I2S1_SYNC_INPUT: u32 = 57;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S2 */
pub const TEGRA234_CLK_I2S2: u32 = 58;
/** @brief clock recovered from I2S2 input */
pub const TEGRA234_CLK_I2S2_SYNC_INPUT: u32 = 59;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S3 */
pub const TEGRA234_CLK_I2S3: u32 = 60;
/** @brief clock recovered from I2S3 input */
pub const TEGRA234_CLK_I2S3_SYNC_INPUT: u32 = 61;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S4 */
pub const TEGRA234_CLK_I2S4: u32 = 62;
/** @brief clock recovered from I2S4 input */
pub const TEGRA234_CLK_I2S4_SYNC_INPUT: u32 = 63;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S5 */
pub const TEGRA234_CLK_I2S5: u32 = 64;
/** @brief clock recovered from I2S5 input */
pub const TEGRA234_CLK_I2S5_SYNC_INPUT: u32 = 65;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S6 */
pub const TEGRA234_CLK_I2S6: u32 = 66;
/** @brief clock recovered from I2S6 input */
pub const TEGRA234_CLK_I2S6_SYNC_INPUT: u32 = 67;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_ISP */
pub const TEGRA234_CLK_ISP: u32 = 69;
/** @brief Monitored branch of EQOS_RX clock */
pub const TEGRA234_CLK_EQOS_RX_M: u32 = 70;
/** @brief CLK_RST_CONTROLLER_MAUDCLK_OUT_SWITCH_DIVIDER switch divider output (maudclk) */
pub const TEGRA234_CLK_MAUD: u32 = 71;
/** @brief output of gate CLK_ENB_MIPI_CAL */
pub const TEGRA234_CLK_MIPI_CAL: u32 = 72;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_CORE_PLL_FIXED */
pub const TEGRA234_CLK_MPHY_CORE_PLL_FIXED: u32 = 73;
/** @brief output of gate CLK_ENB_MPHY_L0_RX_ANA */
pub const TEGRA234_CLK_MPHY_L0_RX_ANA: u32 = 74;
/** @brief output of gate CLK_ENB_MPHY_L0_RX_LS_BIT */
pub const TEGRA234_CLK_MPHY_L0_RX_LS_BIT: u32 = 75;
/** @brief output of gate CLK_ENB_MPHY_L0_RX_SYMB */
pub const TEGRA234_CLK_MPHY_L0_RX_SYMB: u32 = 76;
/** @brief output of gate CLK_ENB_MPHY_L0_TX_LS_3XBIT */
pub const TEGRA234_CLK_MPHY_L0_TX_LS_3XBIT: u32 = 77;
/** @brief output of gate CLK_ENB_MPHY_L0_TX_SYMB */
pub const TEGRA234_CLK_MPHY_L0_TX_SYMB: u32 = 78;
/** @brief output of gate CLK_ENB_MPHY_L1_RX_ANA */
pub const TEGRA234_CLK_MPHY_L1_RX_ANA: u32 = 79;
/** @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_TX_1MHZ_REF */
pub const TEGRA234_CLK_MPHY_TX_1MHZ_REF: u32 = 80;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVCSI */
pub const TEGRA234_CLK_NVCSI: u32 = 81;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVCSILP */
pub const TEGRA234_CLK_NVCSILP: u32 = 82;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDEC */
pub const TEGRA234_CLK_NVDEC: u32 = 83;
/** @brief CLK_RST_CONTROLLER_HUBCLK_OUT_SWITCH_DIVIDER switch divider output (hubclk) */
pub const TEGRA234_CLK_HUB: u32 = 84;
/** @brief CLK_RST_CONTROLLER_DISPCLK_SWITCH_DIVIDER switch divider output (dispclk) */
pub const TEGRA234_CLK_DISP: u32 = 85;
/** @brief RG_CLK_CTRL__0_DIV divider output (nvdisplay_p0_clk) */
pub const TEGRA234_CLK_NVDISPLAY_P0: u32 = 86;
/** @brief RG_CLK_CTRL__1_DIV divider output (nvdisplay_p1_clk) */
pub const TEGRA234_CLK_NVDISPLAY_P1: u32 = 87;
/** @brief DSC_CLK (DISPCLK ÷ 3) */
pub const TEGRA234_CLK_DSC: u32 = 88;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVENC */
pub const TEGRA234_CLK_NVENC: u32 = 89;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVJPG */
pub const TEGRA234_CLK_NVJPG: u32 = 90;
/** @brief input from Tegra's XTAL_IN */
pub const TEGRA234_CLK_OSC: u32 = 91;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_AON_TOUCH switch divider output */
pub const TEGRA234_CLK_AON_TOUCH: u32 = 92;
/** PLL controlled by CLK_RST_CONTROLLER_PLLA_BASE for use by audio clocks */
pub const TEGRA234_CLK_PLLA: u32 = 93;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLAON_BASE for use by IP blocks in the AON domain */
pub const TEGRA234_CLK_PLLAON: u32 = 94;
/** Fixed 100MHz PLL for PCIe, SATA and superspeed USB */
pub const TEGRA234_CLK_PLLE: u32 = 100;
/** @brief PLLP vco output */
pub const TEGRA234_CLK_PLLP: u32 = 101;
/** @brief PLLP clk output */
pub const TEGRA234_CLK_PLLP_OUT0: u32 = 102;
/** Fixed frequency 960MHz PLL for USB and EAVB */
pub const TEGRA234_CLK_UTMIP_PLL: u32 = 103;
/** @brief output of the divider CLK_RST_CONTROLLER_PLLA_OUT */
pub const TEGRA234_CLK_PLLA_OUT0: u32 = 104;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM1 */
pub const TEGRA234_CLK_PWM1: u32 = 105;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM2 */
pub const TEGRA234_CLK_PWM2: u32 = 106;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM3 */
pub const TEGRA234_CLK_PWM3: u32 = 107;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM4 */
pub const TEGRA234_CLK_PWM4: u32 = 108;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM5 */
pub const TEGRA234_CLK_PWM5: u32 = 109;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM6 */
pub const TEGRA234_CLK_PWM6: u32 = 110;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM7 */
pub const TEGRA234_CLK_PWM7: u32 = 111;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM8 */
pub const TEGRA234_CLK_PWM8: u32 = 112;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_RCE_CPU_NIC output */
pub const TEGRA234_CLK_RCE_CPU_NIC: u32 = 113;
/** @brief CLK_RST_CONTROLLER_RCE_NIC_RATE divider output */
pub const TEGRA234_CLK_RCE_NIC: u32 = 114;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_AON_I2C_SLOW switch divider output */
pub const TEGRA234_CLK_AON_I2C_SLOW: u32 = 117;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SCE_CPU_NIC */
pub const TEGRA234_CLK_SCE_CPU_NIC: u32 = 118;
/** @brief output of divider CLK_RST_CONTROLLER_SCE_NIC_RATE */
pub const TEGRA234_CLK_SCE_NIC: u32 = 119;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC1 */
pub const TEGRA234_CLK_SDMMC1: u32 = 120;
/** @brief Logical clk for setting the UPHY PLL3 rate */
pub const TEGRA234_CLK_UPHY_PLL3: u32 = 121;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC4 */
pub const TEGRA234_CLK_SDMMC4: u32 = 123;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_SE switch divider gated output */
pub const TEGRA234_CLK_SE: u32 = 124;
/** @brief VPLL select for sor0_ref clk driven by disp_2clk_sor0_head_sel signal */
pub const TEGRA234_CLK_SOR0_PLL_REF: u32 = 125;
/** @brief Output of mux controlled by disp_2clk_sor0_pll_ref_clk_safe signal (sor0_ref_clk) */
pub const TEGRA234_CLK_SOR0_REF: u32 = 126;
/** @brief VPLL select for sor1_ref clk driven by disp_2clk_sor0_head_sel signal */
pub const TEGRA234_CLK_SOR1_PLL_REF: u32 = 127;
/** @brief SOR_PLL_REF_CLK_CTRL__0_DIV divider output */
pub const TEGRA234_CLK_PRE_SOR0_REF: u32 = 128;
/** @brief Output of mux controlled by disp_2clk_sor1_pll_ref_clk_safe signal (sor1_ref_clk) */
pub const TEGRA234_CLK_SOR1_REF: u32 = 129;
/** @brief SOR_PLL_REF_CLK_CTRL__1_DIV divider output */
pub const TEGRA234_CLK_PRE_SOR1_REF: u32 = 130;
/** @brief output of gate CLK_ENB_SOR_SAFE */
pub const TEGRA234_CLK_SOR_SAFE: u32 = 131;
/** @brief SOR_CLK_CTRL__0_DIV divider output */
pub const TEGRA234_CLK_SOR0_DIV: u32 = 132;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC5 */
pub const TEGRA234_CLK_DMIC5: u32 = 134;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI1 */
pub const TEGRA234_CLK_SPI1: u32 = 135;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI2 */
pub const TEGRA234_CLK_SPI2: u32 = 136;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI3 */
pub const TEGRA234_CLK_SPI3: u32 = 137;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C_SLOW */
pub const TEGRA234_CLK_I2C_SLOW: u32 = 138;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC1 */
pub const TEGRA234_CLK_SYNC_DMIC1: u32 = 139;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC2 */
pub const TEGRA234_CLK_SYNC_DMIC2: u32 = 140;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC3 */
pub const TEGRA234_CLK_SYNC_DMIC3: u32 = 141;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC4 */
pub const TEGRA234_CLK_SYNC_DMIC4: u32 = 142;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DSPK1 */
pub const TEGRA234_CLK_SYNC_DSPK1: u32 = 143;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DSPK2 */
pub const TEGRA234_CLK_SYNC_DSPK2: u32 = 144;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S1 */
pub const TEGRA234_CLK_SYNC_I2S1: u32 = 145;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S2 */
pub const TEGRA234_CLK_SYNC_I2S2: u32 = 146;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S3 */
pub const TEGRA234_CLK_SYNC_I2S3: u32 = 147;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S4 */
pub const TEGRA234_CLK_SYNC_I2S4: u32 = 148;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S5 */
pub const TEGRA234_CLK_SYNC_I2S5: u32 = 149;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S6 */
pub const TEGRA234_CLK_SYNC_I2S6: u32 = 150;
/** @brief controls MPHY_FORCE_LS_MODE upon enable & disable */
pub const TEGRA234_CLK_MPHY_FORCE_LS_MODE: u32 = 151;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TACH0 */
pub const TEGRA234_CLK_TACH0: u32 = 152;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TSEC */
pub const TEGRA234_CLK_TSEC: u32 = 153;
/** output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PKA */
pub const TEGRA234_CLK_TSEC_PKA: u32 = 154;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTA */
pub const TEGRA234_CLK_UARTA: u32 = 155;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTB */
pub const TEGRA234_CLK_UARTB: u32 = 156;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTC */
pub const TEGRA234_CLK_UARTC: u32 = 157;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTD */
pub const TEGRA234_CLK_UARTD: u32 = 158;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTE */
pub const TEGRA234_CLK_UARTE: u32 = 159;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTF */
pub const TEGRA234_CLK_UARTF: u32 = 160;
/** @brief output of gate CLK_ENB_PEX1_CORE_6 */
pub const TEGRA234_CLK_PEX1_C6_CORE: u32 = 161;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UART_FST_MIPI_CAL */
pub const TEGRA234_CLK_UART_FST_MIPI_CAL: u32 = 162;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UFSDEV_REF */
pub const TEGRA234_CLK_UFSDEV_REF: u32 = 163;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UFSHC_CG_SYS */
pub const TEGRA234_CLK_UFSHC: u32 = 164;
/** @brief output of gate CLK_ENB_USB2_TRK */
pub const TEGRA234_CLK_USB2_TRK: u32 = 165;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_VI */
pub const TEGRA234_CLK_VI: u32 = 166;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_VIC */
pub const TEGRA234_CLK_VIC: u32 = 167;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_CSITE switch divider output */
pub const TEGRA234_CLK_CSITE: u32 = 168;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_IST switch divider output */
pub const TEGRA234_CLK_IST: u32 = 169;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_IST_JTAG_REG_CLK_SEL */
pub const TEGRA234_CLK_JTAG_INTFC_PRE_CG: u32 = 170;
/** @brief output of gate CLK_ENB_PEX2_CORE_7 */
pub const TEGRA234_CLK_PEX2_C7_CORE: u32 = 171;
/** @brief output of gate CLK_ENB_PEX2_CORE_8 */
pub const TEGRA234_CLK_PEX2_C8_CORE: u32 = 172;
/** @brief output of gate CLK_ENB_PEX2_CORE_9 */
pub const TEGRA234_CLK_PEX2_C9_CORE: u32 = 173;
/** @brief dla0_falcon_clk */
pub const TEGRA234_CLK_DLA0_FALCON: u32 = 174;
/** @brief dla0_core_clk */
pub const TEGRA234_CLK_DLA0_CORE: u32 = 175;
/** @brief dla1_falcon_clk */
pub const TEGRA234_CLK_DLA1_FALCON: u32 = 176;
/** @brief dla1_core_clk */
pub const TEGRA234_CLK_DLA1_CORE: u32 = 177;
/** @brief Output of mux controlled by disp_2clk_sor0_clk_safe signal (sor0_clk) */
pub const TEGRA234_CLK_SOR0: u32 = 178;
/** @brief Output of mux controlled by disp_2clk_sor1_clk_safe signal (sor1_clk) */
pub const TEGRA234_CLK_SOR1: u32 = 179;
/** @brief DP macro feedback clock (same as LINKA_SYM CLKOUT) */
pub const TEGRA234_CLK_SOR_PAD_INPUT: u32 = 180;
/** @brief Output of mux controlled by disp_2clk_h0_dsi_sel signal in sf0_clk path */
pub const TEGRA234_CLK_PRE_SF0: u32 = 181;
/** @brief Output of mux controlled by disp_2clk_sf0_clk_safe signal (sf0_clk) */
pub const TEGRA234_CLK_SF0: u32 = 182;
/** @brief Output of mux controlled by disp_2clk_sf1_clk_safe signal (sf1_clk) */
pub const TEGRA234_CLK_SF1: u32 = 183;
/** @brief CLKOUT_AB output from DSI BRICK A (dsi_clkout_ab) */
pub const TEGRA234_CLK_DSI_PAD_INPUT: u32 = 184;
/** @brief output of gate CLK_ENB_PEX2_CORE_10 */
pub const TEGRA234_CLK_PEX2_C10_CORE: u32 = 187;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_UARTI switch divider output (uarti_r_clk) */
pub const TEGRA234_CLK_UARTI: u32 = 188;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_UARTJ switch divider output (uartj_r_clk) */
pub const TEGRA234_CLK_UARTJ: u32 = 189;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_UARTH switch divider output */
pub const TEGRA234_CLK_UARTH: u32 = 190;
/** @brief ungated version of fuse clk */
pub const TEGRA234_CLK_FUSE_SERIAL: u32 = 191;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_QSPI0 switch divider output (qspi0_2x_pm_clk) */
pub const TEGRA234_CLK_QSPI0_2X_PM: u32 = 192;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_QSPI1 switch divider output (qspi1_2x_pm_clk) */
pub const TEGRA234_CLK_QSPI1_2X_PM: u32 = 193;
/** @brief output of the divider QSPI_CLK_DIV2_SEL in CLK_RST_CONTROLLER_CLK_SOURCE_QSPI0 (qspi0_pm_clk) */
pub const TEGRA234_CLK_QSPI0_PM: u32 = 194;
/** @brief output of the divider QSPI_CLK_DIV2_SEL in CLK_RST_CONTROLLER_CLK_SOURCE_QSPI1 (qspi1_pm_clk) */
pub const TEGRA234_CLK_QSPI1_PM: u32 = 195;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_VI_CONST switch divider output */
pub const TEGRA234_CLK_VI_CONST: u32 = 196;
/** @brief NAFLL clock source for BPMP */
pub const TEGRA234_CLK_NAFLL_BPMP: u32 = 197;
/** @brief NAFLL clock source for SCE */
pub const TEGRA234_CLK_NAFLL_SCE: u32 = 198;
/** @brief NAFLL clock source for NVDEC */
pub const TEGRA234_CLK_NAFLL_NVDEC: u32 = 199;
/** @brief NAFLL clock source for NVJPG */
pub const TEGRA234_CLK_NAFLL_NVJPG: u32 = 200;
/** @brief NAFLL clock source for TSEC */
pub const TEGRA234_CLK_NAFLL_TSEC: u32 = 201;
/** @brief NAFLL clock source for VI */
pub const TEGRA234_CLK_NAFLL_VI: u32 = 203;
/** @brief NAFLL clock source for SE */
pub const TEGRA234_CLK_NAFLL_SE: u32 = 204;
/** @brief NAFLL clock source for NVENC */
pub const TEGRA234_CLK_NAFLL_NVENC: u32 = 205;
/** @brief NAFLL clock source for ISP */
pub const TEGRA234_CLK_NAFLL_ISP: u32 = 206;
/** @brief NAFLL clock source for VIC */
pub const TEGRA234_CLK_NAFLL_VIC: u32 = 207;
/** @brief NAFLL clock source for AXICBB */
pub const TEGRA234_CLK_NAFLL_AXICBB: u32 = 209;
/** @brief NAFLL clock source for NVJPG1 */
pub const TEGRA234_CLK_NAFLL_NVJPG1: u32 = 210;
/** @brief NAFLL clock source for PVA core */
pub const TEGRA234_CLK_NAFLL_PVA0_CORE: u32 = 211;
/** @brief NAFLL clock source for PVA VPS */
pub const TEGRA234_CLK_NAFLL_PVA0_VPS: u32 = 212;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_DBGAPB_0 switch divider output (dbgapb_clk) */
pub const TEGRA234_CLK_DBGAPB: u32 = 213;
/** @brief NAFLL clock source for RCE */
pub const TEGRA234_CLK_NAFLL_RCE: u32 = 214;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_LA switch divider output (la_r_clk) */
pub const TEGRA234_CLK_LA: u32 = 215;
/** @brief output of the divider CLK_RST_CONTROLLER_PLLP_OUTD */
pub const TEGRA234_CLK_PLLP_OUT_JTAG: u32 = 216;
/** @brief AXI_CBB branch sharing gate control with SDMMC4 */
pub const TEGRA234_CLK_SDMMC4_AXICIF: u32 = 217;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC_LEGACY_TM switch divider output */
pub const TEGRA234_CLK_SDMMC_LEGACY_TM: u32 = 219;
/** @brief output of gate CLK_ENB_PEX0_CORE_0 */
pub const TEGRA234_CLK_PEX0_C0_CORE: u32 = 220;
/** @brief output of gate CLK_ENB_PEX0_CORE_1 */
pub const TEGRA234_CLK_PEX0_C1_CORE: u32 = 221;
/** @brief output of gate CLK_ENB_PEX0_CORE_2 */
pub const TEGRA234_CLK_PEX0_C2_CORE: u32 = 222;
/** @brief output of gate CLK_ENB_PEX0_CORE_3 */
pub const TEGRA234_CLK_PEX0_C3_CORE: u32 = 223;
/** @brief output of gate CLK_ENB_PEX0_CORE_4 */
pub const TEGRA234_CLK_PEX0_C4_CORE: u32 = 224;
/** @brief output of gate CLK_ENB_PEX1_CORE_5 */
pub const TEGRA234_CLK_PEX1_C5_CORE: u32 = 225;
/** @brief Monitored branch of PEX0_C0_CORE clock */
pub const TEGRA234_CLK_PEX0_C0_CORE_M: u32 = 229;
/** @brief Monitored branch of PEX0_C1_CORE clock */
pub const TEGRA234_CLK_PEX0_C1_CORE_M: u32 = 230;
/** @brief Monitored branch of PEX0_C2_CORE clock */
pub const TEGRA234_CLK_PEX0_C2_CORE_M: u32 = 231;
/** @brief Monitored branch of PEX0_C3_CORE clock */
pub const TEGRA234_CLK_PEX0_C3_CORE_M: u32 = 232;
/** @brief Monitored branch of PEX0_C4_CORE clock */
pub const TEGRA234_CLK_PEX0_C4_CORE_M: u32 = 233;
/** @brief Monitored branch of PEX1_C5_CORE clock */
pub const TEGRA234_CLK_PEX1_C5_CORE_M: u32 = 234;
/** @brief Monitored branch of PEX1_C6_CORE clock */
pub const TEGRA234_CLK_PEX1_C6_CORE_M: u32 = 235;
/** @brief output of GPU GPC1 clkGen (in 1x mode same rate as GPC1 MUX2 out) */
pub const TEGRA234_CLK_GPC1CLK: u32 = 236;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLC4_BASE */
pub const TEGRA234_CLK_PLLC4: u32 = 237;
/** @brief PLLC4 VCO followed by DIV3 path */
pub const TEGRA234_CLK_PLLC4_OUT1: u32 = 239;
/** @brief PLLC4 VCO followed by DIV5 path */
pub const TEGRA234_CLK_PLLC4_OUT2: u32 = 240;
/** @brief output of the mux controlled by PLLC4_CLK_SEL */
pub const TEGRA234_CLK_PLLC4_MUXED: u32 = 241;
/** @brief PLLC4 VCO followed by DIV2 path */
pub const TEGRA234_CLK_PLLC4_VCO_DIV2: u32 = 242;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLNVHS_BASE */
pub const TEGRA234_CLK_PLLNVHS: u32 = 243;
/** @brief Monitored branch of PEX2_C7_CORE clock */
pub const TEGRA234_CLK_PEX2_C7_CORE_M: u32 = 244;
/** @brief Monitored branch of PEX2_C8_CORE clock */
pub const TEGRA234_CLK_PEX2_C8_CORE_M: u32 = 245;
/** @brief Monitored branch of PEX2_C9_CORE clock */
pub const TEGRA234_CLK_PEX2_C9_CORE_M: u32 = 246;
/** @brief Monitored branch of PEX2_C10_CORE clock */
pub const TEGRA234_CLK_PEX2_C10_CORE_M: u32 = 247;
/** @brief RX clock recovered from MGBE0 lane input */
pub const TEGRA234_CLK_MGBE0_RX_INPUT: u32 = 248;
/** @brief RX clock recovered from MGBE1 lane input */
pub const TEGRA234_CLK_MGBE1_RX_INPUT: u32 = 249;
/** @brief RX clock recovered from MGBE2 lane input */
pub const TEGRA234_CLK_MGBE2_RX_INPUT: u32 = 250;
/** @brief RX clock recovered from MGBE3 lane input */
pub const TEGRA234_CLK_MGBE3_RX_INPUT: u32 = 251;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_PEX_SATA_USB_RX_BYP switch divider output */
pub const TEGRA234_CLK_PEX_SATA_USB_RX_BYP: u32 = 254;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL0_MGMT switch divider output */
pub const TEGRA234_CLK_PEX_USB_PAD_PLL0_MGMT: u32 = 255;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL1_MGMT switch divider output */
pub const TEGRA234_CLK_PEX_USB_PAD_PLL1_MGMT: u32 = 256;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL2_MGMT switch divider output */
pub const TEGRA234_CLK_PEX_USB_PAD_PLL2_MGMT: u32 = 257;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL3_MGMT switch divider output */
pub const TEGRA234_CLK_PEX_USB_PAD_PLL3_MGMT: u32 = 258;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_NVHS_RX_BYP switch divider output */
pub const TEGRA234_CLK_NVHS_RX_BYP_REF: u32 = 263;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_NVHS_PLL0_MGMT switch divider output */
pub const TEGRA234_CLK_NVHS_PLL0_MGMT: u32 = 264;
/** @brief xusb_core_dev_clk */
pub const TEGRA234_CLK_XUSB_CORE_DEV: u32 = 265;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_XUSB_CORE_HOST switch divider output  */
pub const TEGRA234_CLK_XUSB_CORE_MUX: u32 = 266;
/** @brief xusb_core_host_clk */
pub const TEGRA234_CLK_XUSB_CORE_HOST: u32 = 267;
/** @brief xusb_core_superspeed_clk */
pub const TEGRA234_CLK_XUSB_CORE_SS: u32 = 268;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_XUSB_FALCON switch divider output */
pub const TEGRA234_CLK_XUSB_FALCON: u32 = 269;
/** @brief xusb_falcon_host_clk */
pub const TEGRA234_CLK_XUSB_FALCON_HOST: u32 = 270;
/** @brief xusb_falcon_superspeed_clk */
pub const TEGRA234_CLK_XUSB_FALCON_SS: u32 = 271;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_XUSB_FS switch divider output */
pub const TEGRA234_CLK_XUSB_FS: u32 = 272;
/** @brief xusb_fs_host_clk */
pub const TEGRA234_CLK_XUSB_FS_HOST: u32 = 273;
/** @brief xusb_fs_dev_clk */
pub const TEGRA234_CLK_XUSB_FS_DEV: u32 = 274;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_XUSB_SS switch divider output */
pub const TEGRA234_CLK_XUSB_SS: u32 = 275;
/** @brief xusb_ss_dev_clk */
pub const TEGRA234_CLK_XUSB_SS_DEV: u32 = 276;
/** @brief xusb_ss_superspeed_clk */
pub const TEGRA234_CLK_XUSB_SS_SUPERSPEED: u32 = 277;
/** @brief NAFLL clock source for CPU cluster 0 */
pub const TEGRA234_CLK_NAFLL_CLUSTER0: u32 = 280;
pub const TEGRA234_CLK_NAFLL_CLUSTER0_CORE: u32 = 280;
/** @brief NAFLL clock source for CPU cluster 1 */
pub const TEGRA234_CLK_NAFLL_CLUSTER1: u32 = 281;
pub const TEGRA234_CLK_NAFLL_CLUSTER1_CORE: u32 = 281;
/** @brief NAFLL clock source for CPU cluster 2 */
pub const TEGRA234_CLK_NAFLL_CLUSTER2: u32 = 282;
pub const TEGRA234_CLK_NAFLL_CLUSTER2_CORE: u32 = 282;
/** @brief CLK_RST_CONTROLLER_CAN1_CORE_RATE divider output */
pub const TEGRA234_CLK_CAN1_CORE: u32 = 284;
/** @brief CLK_RST_CONTROLLER_CAN2_CORE_RATE divider outputt */
pub const TEGRA234_CLK_CAN2_CORE: u32 = 285;
/** @brief CLK_RST_CONTROLLER_PLLA1_OUT1 switch divider output */
pub const TEGRA234_CLK_PLLA1_OUT1: u32 = 286;
/** @brief NVHS PLL hardware power sequencer (overrides 'manual' programming of PLL) */
pub const TEGRA234_CLK_PLLNVHS_HPS: u32 = 287;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLREFE_BASE */
pub const TEGRA234_CLK_PLLREFE_VCOOUT: u32 = 288;
/** @brief 32K input clock provided by PMIC */
pub const TEGRA234_CLK_CLK_32K: u32 = 289;
/** @brief Fixed 48MHz clock divided down from utmipll */
pub const TEGRA234_CLK_UTMIPLL_CLKOUT48: u32 = 291;
/** @brief Fixed 480MHz clock divided down from utmipll */
pub const TEGRA234_CLK_UTMIPLL_CLKOUT480: u32 = 292;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLNVCSI_BASE  */
pub const TEGRA234_CLK_PLLNVCSI: u32 = 294;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_PVA0_CPU_AXI switch divider output */
pub const TEGRA234_CLK_PVA0_CPU_AXI: u32 = 295;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_PVA0_VPS switch divider output */
pub const TEGRA234_CLK_PVA0_VPS: u32 = 297;
/** @brief DLA0_CORE_NAFLL */
pub const TEGRA234_CLK_NAFLL_DLA0_CORE: u32 = 299;
/** @brief DLA0_FALCON_NAFLL */
pub const TEGRA234_CLK_NAFLL_DLA0_FALCON: u32 = 300;
/** @brief DLA1_CORE_NAFLL */
pub const TEGRA234_CLK_NAFLL_DLA1_CORE: u32 = 301;
/** @brief DLA1_FALCON_NAFLL */
pub const TEGRA234_CLK_NAFLL_DLA1_FALCON: u32 = 302;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_UART_FST_MIPI_CAL */
pub const TEGRA234_CLK_AON_UART_FST_MIPI_CAL: u32 = 303;
/** @brief GPU system clock */
pub const TEGRA234_CLK_GPUSYS: u32 = 304;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C5 */
pub const TEGRA234_CLK_I2C5: u32 = 305;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_SE switch divider free running clk */
pub const TEGRA234_CLK_FR_SE: u32 = 306;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_BPMP_CPU_NIC switch divider output */
pub const TEGRA234_CLK_BPMP_CPU_NIC: u32 = 307;
/** @brief output of gate CLK_ENB_BPMP_CPU */
pub const TEGRA234_CLK_BPMP_CPU: u32 = 308;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_TSC switch divider output */
pub const TEGRA234_CLK_TSC: u32 = 309;
/** @brief output of mem pll A sync mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EMC */
pub const TEGRA234_CLK_EMCSA_MPLL: u32 = 310;
/** @brief output of mem pll B sync mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EMCSB */
pub const TEGRA234_CLK_EMCSB_MPLL: u32 = 311;
/** @brief output of mem pll C sync mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EMCSC */
pub const TEGRA234_CLK_EMCSC_MPLL: u32 = 312;
/** @brief output of mem pll D sync mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EMCSD */
pub const TEGRA234_CLK_EMCSD_MPLL: u32 = 313;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLC_BASE */
pub const TEGRA234_CLK_PLLC: u32 = 314;
/** @brief PLL controlled by CLK_RST_CONTROLLER_PLLC2_BASE */
pub const TEGRA234_CLK_PLLC2: u32 = 315;
/** @brief CLK_RST_CONTROLLER_TSC_HS_SUPER_CLK_DIVIDER skip divider output */
pub const TEGRA234_CLK_TSC_REF: u32 = 317;
/** @brief Dummy clock to ensure minimum SoC voltage for fuse burning */
pub const TEGRA234_CLK_FUSE_BURN: u32 = 318;
/** @brief GBE PLL */
pub const TEGRA234_CLK_PLLGBE: u32 = 319;
/** @brief GBE PLL hardware power sequencer */
pub const TEGRA234_CLK_PLLGBE_HPS: u32 = 320;
/** @brief output of EMC CDB side A fixed (DIV4)  divider */
pub const TEGRA234_CLK_EMCSA_EMC: u32 = 321;
/** @brief output of EMC CDB side B fixed (DIV4)  divider */
pub const TEGRA234_CLK_EMCSB_EMC: u32 = 322;
/** @brief output of EMC CDB side C fixed (DIV4)  divider */
pub const TEGRA234_CLK_EMCSC_EMC: u32 = 323;
/** @brief output of EMC CDB side D fixed (DIV4)  divider */
pub const TEGRA234_CLK_EMCSD_EMC: u32 = 324;
/** @brief PLLE hardware power sequencer (overrides 'manual' programming of PLL) */
pub const TEGRA234_CLK_PLLE_HPS: u32 = 326;
/** @brief CLK_ENB_PLLREFE_OUT gate output */
pub const TEGRA234_CLK_PLLREFE_VCOOUT_GATED: u32 = 327;
/** @brief TEGRA234_CLK_SOR_SAFE clk source (PLLP_OUT0 divided by 17) */
pub const TEGRA234_CLK_PLLP_DIV17: u32 = 328;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_SOC_THERM switch divider output */
pub const TEGRA234_CLK_SOC_THERM: u32 = 329;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_TSENSE switch divider output */
pub const TEGRA234_CLK_TSENSE: u32 = 330;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_SEU1 switch divider free running clk */
pub const TEGRA234_CLK_FR_SEU1: u32 = 331;
/** @brief NAFLL clock source for OFA */
pub const TEGRA234_CLK_NAFLL_OFA: u32 = 333;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_OFA switch divider output */
pub const TEGRA234_CLK_OFA: u32 = 334;
/** @brief NAFLL clock source for SEU1 */
pub const TEGRA234_CLK_NAFLL_SEU1: u32 = 335;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_SEU1 switch divider gated output */
pub const TEGRA234_CLK_SEU1: u32 = 336;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI4 */
pub const TEGRA234_CLK_SPI4: u32 = 337;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI5 */
pub const TEGRA234_CLK_SPI5: u32 = 338;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DCE_CPU_NIC */
pub const TEGRA234_CLK_DCE_CPU_NIC: u32 = 339;
/** @brief output of divider CLK_RST_CONTROLLER_DCE_NIC_RATE */
pub const TEGRA234_CLK_DCE_NIC: u32 = 340;
/** @brief NAFLL clock source for DCE */
pub const TEGRA234_CLK_NAFLL_DCE: u32 = 341;
/** @brief Monitored branch of MPHY_L0_RX_ANA clock */
pub const TEGRA234_CLK_MPHY_L0_RX_ANA_M: u32 = 342;
/** @brief Monitored branch of MPHY_L1_RX_ANA clock */
pub const TEGRA234_CLK_MPHY_L1_RX_ANA_M: u32 = 343;
/** @brief ungated version of TX symbol clock after fixed 1/2 divider */
pub const TEGRA234_CLK_MPHY_L0_TX_PRE_SYMB: u32 = 344;
/** @brief output of divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_L0_TX_LS_SYMB */
pub const TEGRA234_CLK_MPHY_L0_TX_LS_SYMB_DIV: u32 = 345;
/** @brief output of gate CLK_ENB_MPHY_L0_TX_2X_SYMB */
pub const TEGRA234_CLK_MPHY_L0_TX_2X_SYMB: u32 = 346;
/** @brief output of SW_MPHY_L0_TX_HS_SYMB divider in CLK_RST_CONTROLLER_MPHY_L0_TX_CLK_CTRL_0 */
pub const TEGRA234_CLK_MPHY_L0_TX_HS_SYMB_DIV: u32 = 347;
/** @brief output of SW_MPHY_L0_TX_LS_3XBIT divider in CLK_RST_CONTROLLER_MPHY_L0_TX_CLK_CTRL_0 */
pub const TEGRA234_CLK_MPHY_L0_TX_LS_3XBIT_DIV: u32 = 348;
/** @brief LS/HS divider mux SW_MPHY_L0_TX_LS_HS_SEL in CLK_RST_CONTROLLER_MPHY_L0_TX_CLK_CTRL_0 */
pub const TEGRA234_CLK_MPHY_L0_TX_MUX_SYMB_DIV: u32 = 349;
/** @brief Monitored branch of MPHY_L0_TX_SYMB clock */
pub const TEGRA234_CLK_MPHY_L0_TX_SYMB_M: u32 = 350;
/** @brief output of divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_L0_RX_LS_SYMB */
pub const TEGRA234_CLK_MPHY_L0_RX_LS_SYMB_DIV: u32 = 351;
/** @brief output of SW_MPHY_L0_RX_HS_SYMB divider in CLK_RST_CONTROLLER_MPHY_L0_RX_CLK_CTRL_0 */
pub const TEGRA234_CLK_MPHY_L0_RX_HS_SYMB_DIV: u32 = 352;
/** @brief output of SW_MPHY_L0_RX_LS_BIT divider in  CLK_RST_CONTROLLER_MPHY_L0_RX_CLK_CTRL_0 */
pub const TEGRA234_CLK_MPHY_L0_RX_LS_BIT_DIV: u32 = 353;
/** @brief LS/HS divider mux SW_MPHY_L0_RX_LS_HS_SEL in CLK_RST_CONTROLLER_MPHY_L0_RX_CLK_CTRL_0 */
pub const TEGRA234_CLK_MPHY_L0_RX_MUX_SYMB_DIV: u32 = 354;
/** @brief Monitored branch of MPHY_L0_RX_SYMB clock */
pub const TEGRA234_CLK_MPHY_L0_RX_SYMB_M: u32 = 355;
/** @brief Monitored branch of MBGE0 RX input clock */
pub const TEGRA234_CLK_MGBE0_RX_INPUT_M: u32 = 357;
/** @brief Monitored branch of MBGE1 RX input clock */
pub const TEGRA234_CLK_MGBE1_RX_INPUT_M: u32 = 358;
/** @brief Monitored branch of MBGE2 RX input clock */
pub const TEGRA234_CLK_MGBE2_RX_INPUT_M: u32 = 359;
/** @brief Monitored branch of MBGE3 RX input clock */
pub const TEGRA234_CLK_MGBE3_RX_INPUT_M: u32 = 360;
/** @brief Monitored branch of MGBE0 RX PCS mux output */
pub const TEGRA234_CLK_MGBE0_RX_PCS_M: u32 = 361;
/** @brief Monitored branch of MGBE1 RX PCS mux output */
pub const TEGRA234_CLK_MGBE1_RX_PCS_M: u32 = 362;
/** @brief Monitored branch of MGBE2 RX PCS mux output */
pub const TEGRA234_CLK_MGBE2_RX_PCS_M: u32 = 363;
/** @brief Monitored branch of MGBE3 RX PCS mux output */
pub const TEGRA234_CLK_MGBE3_RX_PCS_M: u32 = 364;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TACH1 */
pub const TEGRA234_CLK_TACH1: u32 = 365;
/** @brief GBE_UPHY_MGBES_APP_CLK switch divider gated output */
pub const TEGRA234_CLK_MGBES_APP: u32 = 366;
/** @brief Logical clk for setting GBE UPHY PLL2 TX_REF rate */
pub const TEGRA234_CLK_UPHY_GBE_PLL2_TX_REF: u32 = 367;
/** @brief Logical clk for setting GBE UPHY PLL2 XDIG rate */
pub const TEGRA234_CLK_UPHY_GBE_PLL2_XDIG: u32 = 368;
/** @brief RX PCS clock recovered from MGBE0 lane input */
pub const TEGRA234_CLK_MGBE0_RX_PCS_INPUT: u32 = 369;
/** @brief RX PCS clock recovered from MGBE1 lane input */
pub const TEGRA234_CLK_MGBE1_RX_PCS_INPUT: u32 = 370;
/** @brief RX PCS clock recovered from MGBE2 lane input */
pub const TEGRA234_CLK_MGBE2_RX_PCS_INPUT: u32 = 371;
/** @brief RX PCS clock recovered from MGBE3 lane input */
pub const TEGRA234_CLK_MGBE3_RX_PCS_INPUT: u32 = 372;
/** @brief output of mux controlled by GBE_UPHY_MGBE0_RX_PCS_CLK_SRC_SEL */
pub const TEGRA234_CLK_MGBE0_RX_PCS: u32 = 373;
/** @brief GBE_UPHY_MGBE0_TX_CLK divider gated output */
pub const TEGRA234_CLK_MGBE0_TX: u32 = 374;
/** @brief GBE_UPHY_MGBE0_TX_PCS_CLK divider gated output */
pub const TEGRA234_CLK_MGBE0_TX_PCS: u32 = 375;
/** @brief GBE_UPHY_MGBE0_MAC_CLK divider output */
pub const TEGRA234_CLK_MGBE0_MAC_DIVIDER: u32 = 376;
/** @brief GBE_UPHY_MGBE0_MAC_CLK gate output */
pub const TEGRA234_CLK_MGBE0_MAC: u32 = 377;
/** @brief GBE_UPHY_MGBE0_MACSEC_CLK gate output */
pub const TEGRA234_CLK_MGBE0_MACSEC: u32 = 378;
/** @brief GBE_UPHY_MGBE0_EEE_PCS_CLK gate output */
pub const TEGRA234_CLK_MGBE0_EEE_PCS: u32 = 379;
/** @brief GBE_UPHY_MGBE0_APP_CLK gate output */
pub const TEGRA234_CLK_MGBE0_APP: u32 = 380;
/** @brief GBE_UPHY_MGBE0_PTP_REF_CLK divider gated output */
pub const TEGRA234_CLK_MGBE0_PTP_REF: u32 = 381;
/** @brief output of mux controlled by GBE_UPHY_MGBE1_RX_PCS_CLK_SRC_SEL */
pub const TEGRA234_CLK_MGBE1_RX_PCS: u32 = 382;
/** @brief GBE_UPHY_MGBE1_TX_CLK divider gated output */
pub const TEGRA234_CLK_MGBE1_TX: u32 = 383;
/** @brief GBE_UPHY_MGBE1_TX_PCS_CLK divider gated output */
pub const TEGRA234_CLK_MGBE1_TX_PCS: u32 = 384;
/** @brief GBE_UPHY_MGBE1_MAC_CLK divider output */
pub const TEGRA234_CLK_MGBE1_MAC_DIVIDER: u32 = 385;
/** @brief GBE_UPHY_MGBE1_MAC_CLK gate output */
pub const TEGRA234_CLK_MGBE1_MAC: u32 = 386;
/** @brief GBE_UPHY_MGBE1_MACSEC_CLK gate output */
pub const TEGRA234_CLK_MGBE1_MACSEC: u32 = 387;
/** @brief GBE_UPHY_MGBE1_EEE_PCS_CLK gate output */
pub const TEGRA234_CLK_MGBE1_EEE_PCS: u32 = 388;
/** @brief GBE_UPHY_MGBE1_APP_CLK gate output */
pub const TEGRA234_CLK_MGBE1_APP: u32 = 389;
/** @brief GBE_UPHY_MGBE1_PTP_REF_CLK divider gated output */
pub const TEGRA234_CLK_MGBE1_PTP_REF: u32 = 390;
/** @brief output of mux controlled by GBE_UPHY_MGBE2_RX_PCS_CLK_SRC_SEL */
pub const TEGRA234_CLK_MGBE2_RX_PCS: u32 = 391;
/** @brief GBE_UPHY_MGBE2_TX_CLK divider gated output */
pub const TEGRA234_CLK_MGBE2_TX: u32 = 392;
/** @brief GBE_UPHY_MGBE2_TX_PCS_CLK divider gated output */
pub const TEGRA234_CLK_MGBE2_TX_PCS: u32 = 393;
/** @brief GBE_UPHY_MGBE2_MAC_CLK divider output */
pub const TEGRA234_CLK_MGBE2_MAC_DIVIDER: u32 = 394;
/** @brief GBE_UPHY_MGBE2_MAC_CLK gate output */
pub const TEGRA234_CLK_MGBE2_MAC: u32 = 395;
/** @brief GBE_UPHY_MGBE2_MACSEC_CLK gate output */
pub const TEGRA234_CLK_MGBE2_MACSEC: u32 = 396;
/** @brief GBE_UPHY_MGBE2_EEE_PCS_CLK gate output */
pub const TEGRA234_CLK_MGBE2_EEE_PCS: u32 = 397;
/** @brief GBE_UPHY_MGBE2_APP_CLK gate output */
pub const TEGRA234_CLK_MGBE2_APP: u32 = 398;
/** @brief GBE_UPHY_MGBE2_PTP_REF_CLK divider gated output */
pub const TEGRA234_CLK_MGBE2_PTP_REF: u32 = 399;
/** @brief output of mux controlled by GBE_UPHY_MGBE3_RX_PCS_CLK_SRC_SEL */
pub const TEGRA234_CLK_MGBE3_RX_PCS: u32 = 400;
/** @brief GBE_UPHY_MGBE3_TX_CLK divider gated output */
pub const TEGRA234_CLK_MGBE3_TX: u32 = 401;
/** @brief GBE_UPHY_MGBE3_TX_PCS_CLK divider gated output */
pub const TEGRA234_CLK_MGBE3_TX_PCS: u32 = 402;
/** @brief GBE_UPHY_MGBE3_MAC_CLK divider output */
pub const TEGRA234_CLK_MGBE3_MAC_DIVIDER: u32 = 403;
/** @brief GBE_UPHY_MGBE3_MAC_CLK gate output */
pub const TEGRA234_CLK_MGBE3_MAC: u32 = 404;
/** @brief GBE_UPHY_MGBE3_MACSEC_CLK gate output */
pub const TEGRA234_CLK_MGBE3_MACSEC: u32 = 405;
/** @brief GBE_UPHY_MGBE3_EEE_PCS_CLK gate output */
pub const TEGRA234_CLK_MGBE3_EEE_PCS: u32 = 406;
/** @brief GBE_UPHY_MGBE3_APP_CLK gate output */
pub const TEGRA234_CLK_MGBE3_APP: u32 = 407;
/** @brief GBE_UPHY_MGBE3_PTP_REF_CLK divider gated output */
pub const TEGRA234_CLK_MGBE3_PTP_REF: u32 = 408;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_GBE_RX_BYP switch divider output */
pub const TEGRA234_CLK_GBE_RX_BYP_REF: u32 = 409;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_GBE_PLL0_MGMT switch divider output */
pub const TEGRA234_CLK_GBE_PLL0_MGMT: u32 = 410;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_GBE_PLL1_MGMT switch divider output */
pub const TEGRA234_CLK_GBE_PLL1_MGMT: u32 = 411;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_GBE_PLL2_MGMT switch divider output */
pub const TEGRA234_CLK_GBE_PLL2_MGMT: u32 = 412;
/** @brief output of gate CLK_ENB_EQOS_MACSEC_RX */
pub const TEGRA234_CLK_EQOS_MACSEC_RX: u32 = 413;
/** @brief output of gate CLK_ENB_EQOS_MACSEC_TX */
pub const TEGRA234_CLK_EQOS_MACSEC_TX: u32 = 414;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_TX_CLK divider ungated output */
pub const TEGRA234_CLK_EQOS_TX_DIVIDER: u32 = 415;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_NVHS_PLL1_MGMT switch divider output */
pub const TEGRA234_CLK_NVHS_PLL1_MGMT: u32 = 416;
/** @brief CLK_RST_CONTROLLER_CLK_SOURCE_EMCHUB mux output */
pub const TEGRA234_CLK_EMCHUB: u32 = 417;
/** @brief clock recovered from I2S7 input */
pub const TEGRA234_CLK_I2S7_SYNC_INPUT: u32 = 418;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S7 */
pub const TEGRA234_CLK_SYNC_I2S7: u32 = 419;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S7 */
pub const TEGRA234_CLK_I2S7: u32 = 420;
/** @brief Monitored output of I2S7 pad macro mux */
pub const TEGRA234_CLK_I2S7_PAD_M: u32 = 421;
/** @brief clock recovered from I2S8 input */
pub const TEGRA234_CLK_I2S8_SYNC_INPUT: u32 = 422;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S8 */
pub const TEGRA234_CLK_SYNC_I2S8: u32 = 423;
/** @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S8 */
pub const TEGRA234_CLK_I2S8: u32 = 424;
/** @brief Monitored output of I2S8 pad macro mux */
pub const TEGRA234_CLK_I2S8_PAD_M: u32 = 425;
/** @brief NAFLL clock source for GPU GPC0 */
pub const TEGRA234_CLK_NAFLL_GPC0: u32 = 426;
/** @brief NAFLL clock source for GPU GPC1 */
pub const TEGRA234_CLK_NAFLL_GPC1: u32 = 427;
/** @brief NAFLL clock source for GPU SYSCLK */
pub const TEGRA234_CLK_NAFLL_GPUSYS: u32 = 428;
/** @brief NAFLL clock source for CPU cluster 0 DSUCLK */
pub const TEGRA234_CLK_NAFLL_DSU0: u32 = 429;
pub const TEGRA234_CLK_NAFLL_CLUSTER0_DSU: u32 = 429;
/** @brief NAFLL clock source for CPU cluster 1 DSUCLK */
pub const TEGRA234_CLK_NAFLL_DSU1: u32 = 430;
pub const TEGRA234_CLK_NAFLL_CLUSTER1_DSU: u32 = 430;
/** @brief NAFLL clock source for CPU cluster 2 DSUCLK */
pub const TEGRA234_CLK_NAFLL_DSU2: u32 = 431;
pub const TEGRA234_CLK_NAFLL_CLUSTER2_DSU: u32 = 431;
/** @brief output of gate CLK_ENB_SCE_CPU */
pub const TEGRA234_CLK_SCE_CPU: u32 = 432;
/** @brief output of gate CLK_ENB_RCE_CPU */
pub const TEGRA234_CLK_RCE_CPU: u32 = 433;
/** @brief output of gate CLK_ENB_DCE_CPU */
pub const TEGRA234_CLK_DCE_CPU: u32 = 434;
/** @brief DSIPLL VCO output */
pub const TEGRA234_CLK_DSIPLL_VCO: u32 = 435;
/** @brief DSIPLL SYNC_CLKOUTP/N differential output */
pub const TEGRA234_CLK_DSIPLL_CLKOUTPN: u32 = 436;
/** @brief DSIPLL SYNC_CLKOUTA output */
pub const TEGRA234_CLK_DSIPLL_CLKOUTA: u32 = 437;
/** @brief SPPLL0 VCO output */
pub const TEGRA234_CLK_SPPLL0_VCO: u32 = 438;
/** @brief SPPLL0 SYNC_CLKOUTP/N differential output */
pub const TEGRA234_CLK_SPPLL0_CLKOUTPN: u32 = 439;
/** @brief SPPLL0 SYNC_CLKOUTA output */
pub const TEGRA234_CLK_SPPLL0_CLKOUTA: u32 = 440;
/** @brief SPPLL0 SYNC_CLKOUTB output */
pub const TEGRA234_CLK_SPPLL0_CLKOUTB: u32 = 441;
/** @brief SPPLL0 CLKOUT_DIVBY10 output */
pub const TEGRA234_CLK_SPPLL0_DIV10: u32 = 442;
/** @brief SPPLL0 CLKOUT_DIVBY25 output */
pub const TEGRA234_CLK_SPPLL0_DIV25: u32 = 443;
/** @brief SPPLL0 CLKOUT_DIVBY27P/N differential output */
pub const TEGRA234_CLK_SPPLL0_DIV27PN: u32 = 444;
/** @brief SPPLL1 VCO output */
pub const TEGRA234_CLK_SPPLL1_VCO: u32 = 445;
/** @brief SPPLL1 SYNC_CLKOUTP/N differential output */
pub const TEGRA234_CLK_SPPLL1_CLKOUTPN: u32 = 446;
/** @brief SPPLL1 CLKOUT_DIVBY27P/N differential output */
pub const TEGRA234_CLK_SPPLL1_DIV27PN: u32 = 447;
/** @brief VPLL0 reference clock */
pub const TEGRA234_CLK_VPLL0_REF: u32 = 448;
/** @brief VPLL0 */
pub const TEGRA234_CLK_VPLL0: u32 = 449;
/** @brief VPLL1 */
pub const TEGRA234_CLK_VPLL1: u32 = 450;
/** @brief NVDISPLAY_P0_CLK reference select */
pub const TEGRA234_CLK_NVDISPLAY_P0_REF: u32 = 451;
/** @brief RG0_PCLK */
pub const TEGRA234_CLK_RG0: u32 = 452;
/** @brief RG1_PCLK */
pub const TEGRA234_CLK_RG1: u32 = 453;
/** @brief DISPPLL output */
pub const TEGRA234_CLK_DISPPLL: u32 = 454;
/** @brief DISPHUBPLL output */
pub const TEGRA234_CLK_DISPHUBPLL: u32 = 455;
/** @brief CLK_RST_CONTROLLER_DSI_LP_SWITCH_DIVIDER switch divider output (dsi_lp_clk) */
pub const TEGRA234_CLK_DSI_LP: u32 = 456;
/** @brief CLK_RST_CONTROLLER_AZA2XBITCLK_OUT_SWITCH_DIVIDER switch divider output (aza_2xbitclk) */
pub const TEGRA234_CLK_AZA_2XBIT: u32 = 457;
/** @brief aza_2xbitclk / 2 (aza_bitclk) */
pub const TEGRA234_CLK_AZA_BIT: u32 = 458;
/** @brief SWITCH_DSI_CORE_PIXEL_MISC_DSI_CORE_CLK_SRC switch output (dsi_core_clk) */
pub const TEGRA234_CLK_DSI_CORE: u32 = 459;
/** @brief Output of mux controlled by pkt_wr_fifo_signal from dsi (dsi_pixel_clk) */
pub const TEGRA234_CLK_DSI_PIXEL: u32 = 460;
/** @brief Output of mux controlled by disp_2clk_sor0_dp_sel (pre_sor0_clk) */
pub const TEGRA234_CLK_PRE_SOR0: u32 = 461;
/** @brief Output of mux controlled by disp_2clk_sor1_dp_sel (pre_sor1_clk) */
pub const TEGRA234_CLK_PRE_SOR1: u32 = 462;
/** @brief CLK_RST_CONTROLLER_LINK_REFCLK_CFG__0 output */
pub const TEGRA234_CLK_DP_LINK_REF: u32 = 463;
/** @brief Link clock input from DP macro brick PLL */
pub const TEGRA234_CLK_SOR_LINKA_INPUT: u32 = 464;
/** @brief SOR AFIFO clock outut */
pub const TEGRA234_CLK_SOR_LINKA_AFIFO: u32 = 465;
/** @brief Monitored branch of linka_afifo_clk */
pub const TEGRA234_CLK_SOR_LINKA_AFIFO_M: u32 = 466;
/** @brief Monitored branch of rg0_pclk */
pub const TEGRA234_CLK_RG0_M: u32 = 467;
/** @brief Monitored branch of rg1_pclk */
pub const TEGRA234_CLK_RG1_M: u32 = 468;
/** @brief Monitored branch of sor0_clk */
pub const TEGRA234_CLK_SOR0_M: u32 = 469;
/** @brief Monitored branch of sor1_clk */
pub const TEGRA234_CLK_SOR1_M: u32 = 470;
/** @brief EMC PLLHUB output */
pub const TEGRA234_CLK_PLLHUB: u32 = 471;
/** @brief output of fixed (DIV2) MC HUB divider */
pub const TEGRA234_CLK_MCHUB: u32 = 472;
/** @brief output of divider controlled by EMC side A MC_EMC_SAFE_SAME_FREQ */
pub const TEGRA234_CLK_EMCSA_MC: u32 = 473;
/** @brief output of divider controlled by EMC side B MC_EMC_SAFE_SAME_FREQ */
pub const TEGRA234_CLK_EMCSB_MC: u32 = 474;
/** @brief output of divider controlled by EMC side C MC_EMC_SAFE_SAME_FREQ */
pub const TEGRA234_CLK_EMCSC_MC: u32 = 475;
/** @brief output of divider controlled by EMC side D MC_EMC_SAFE_SAME_FREQ */
pub const TEGRA234_CLK_EMCSD_MC: u32 = 476;

/** @} */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
