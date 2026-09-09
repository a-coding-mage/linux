/* SPDX-License-Identifier: GPL-2.0-only OR BSD-3-Clause */
/*
 * Copyright (C) STMicroelectronics 2020 - All Rights Reserved
 * Author: Gabriel Fernandez <gabriel.fernandez@foss.st.com> for STMicroelectronics.
 */

/* OSCILLATOR clocks */
pub const CK_HSE: u32 = 0;
pub const CK_CSI: u32 = 1;
pub const CK_LSI: u32 = 2;
pub const CK_LSE: u32 = 3;
pub const CK_HSI: u32 = 4;
pub const CK_HSE_DIV2: u32 = 5;

/* PLL */
pub const PLL1: u32 = 6;
pub const PLL2: u32 = 7;
pub const PLL3: u32 = 8;
pub const PLL4: u32 = 9;

/* ODF */
pub const PLL1_P: u32 = 10;
pub const PLL1_Q: u32 = 11;
pub const PLL1_R: u32 = 12;
pub const PLL2_P: u32 = 13;
pub const PLL2_Q: u32 = 14;
pub const PLL2_R: u32 = 15;
pub const PLL3_P: u32 = 16;
pub const PLL3_Q: u32 = 17;
pub const PLL3_R: u32 = 18;
pub const PLL4_P: u32 = 19;
pub const PLL4_Q: u32 = 20;
pub const PLL4_R: u32 = 21;

pub const PCLK1: u32 = 22;
pub const PCLK2: u32 = 23;
pub const PCLK3: u32 = 24;
pub const PCLK4: u32 = 25;
pub const PCLK5: u32 = 26;
pub const PCLK6: u32 = 27;

/* SYSTEM CLOCK */
pub const CK_PER: u32 = 28;
pub const CK_MPU: u32 = 29;
pub const CK_AXI: u32 = 30;
pub const CK_MLAHB: u32 = 31;

/* BASE TIMER */
pub const CK_TIMG1: u32 = 32;
pub const CK_TIMG2: u32 = 33;
pub const CK_TIMG3: u32 = 34;

/* AUX */
pub const RTC: u32 = 35;

/* TRACE & DEBUG clocks */
pub const CK_DBG: u32 = 36;
pub const CK_TRACE: u32 = 37;

/* MCO clocks */
pub const CK_MCO1: u32 = 38;
pub const CK_MCO2: u32 = 39;

/* IP clocks */
pub const SYSCFG: u32 = 40;
pub const VREF: u32 = 41;
pub const DTS: u32 = 42;
pub const PMBCTRL: u32 = 43;
pub const HDP: u32 = 44;
pub const IWDG2: u32 = 45;
pub const STGENRO: u32 = 46;
pub const USART1: u32 = 47;
pub const RTCAPB: u32 = 48;
pub const TZC: u32 = 49;
pub const TZPC: u32 = 50;
pub const IWDG1: u32 = 51;
pub const BSEC: u32 = 52;
pub const DMA1: u32 = 53;
pub const DMA2: u32 = 54;
pub const DMAMUX1: u32 = 55;
pub const DMAMUX2: u32 = 56;
pub const GPIOA: u32 = 57;
pub const GPIOB: u32 = 58;
pub const GPIOC: u32 = 59;
pub const GPIOD: u32 = 60;
pub const GPIOE: u32 = 61;
pub const GPIOF: u32 = 62;
pub const GPIOG: u32 = 63;
pub const GPIOH: u32 = 64;
pub const GPIOI: u32 = 65;
pub const CRYP1: u32 = 66;
pub const HASH1: u32 = 67;
pub const BKPSRAM: u32 = 68;
pub const MDMA: u32 = 69;
pub const CRC1: u32 = 70;
pub const USBH: u32 = 71;
pub const DMA3: u32 = 72;
pub const TSC: u32 = 73;
pub const PKA: u32 = 74;
pub const AXIMC: u32 = 75;
pub const MCE: u32 = 76;
pub const ETH1TX: u32 = 77;
pub const ETH2TX: u32 = 78;
pub const ETH1RX: u32 = 79;
pub const ETH2RX: u32 = 80;
pub const ETH1MAC: u32 = 81;
pub const ETH2MAC: u32 = 82;
pub const ETH1STP: u32 = 83;
pub const ETH2STP: u32 = 84;

/* IP clocks with parents */
pub const SDMMC1_K: u32 = 85;
pub const SDMMC2_K: u32 = 86;
pub const ADC1_K: u32 = 87;
pub const ADC2_K: u32 = 88;
pub const FMC_K: u32 = 89;
pub const QSPI_K: u32 = 90;
pub const RNG1_K: u32 = 91;
pub const USBPHY_K: u32 = 92;
pub const STGEN_K: u32 = 93;
pub const SPDIF_K: u32 = 94;
pub const SPI1_K: u32 = 95;
pub const SPI2_K: u32 = 96;
pub const SPI3_K: u32 = 97;
pub const SPI4_K: u32 = 98;
pub const SPI5_K: u32 = 99;
pub const I2C1_K: u32 = 100;
pub const I2C2_K: u32 = 101;
pub const I2C3_K: u32 = 102;
pub const I2C4_K: u32 = 103;
pub const I2C5_K: u32 = 104;
pub const TIM2_K: u32 = 105;
pub const TIM3_K: u32 = 106;
pub const TIM4_K: u32 = 107;
pub const TIM5_K: u32 = 108;
pub const TIM6_K: u32 = 109;
pub const TIM7_K: u32 = 110;
pub const TIM12_K: u32 = 111;
pub const TIM13_K: u32 = 112;
pub const TIM14_K: u32 = 113;
pub const TIM1_K: u32 = 114;
pub const TIM8_K: u32 = 115;
pub const TIM15_K: u32 = 116;
pub const TIM16_K: u32 = 117;
pub const TIM17_K: u32 = 118;
pub const LPTIM1_K: u32 = 119;
pub const LPTIM2_K: u32 = 120;
pub const LPTIM3_K: u32 = 121;
pub const LPTIM4_K: u32 = 122;
pub const LPTIM5_K: u32 = 123;
pub const USART1_K: u32 = 124;
pub const USART2_K: u32 = 125;
pub const USART3_K: u32 = 126;
pub const UART4_K: u32 = 127;
pub const UART5_K: u32 = 128;
pub const USART6_K: u32 = 129;
pub const UART7_K: u32 = 130;
pub const UART8_K: u32 = 131;
pub const DFSDM_K: u32 = 132;
pub const FDCAN_K: u32 = 133;
pub const SAI1_K: u32 = 134;
pub const SAI2_K: u32 = 135;
pub const ADFSDM_K: u32 = 136;
pub const USBO_K: u32 = 137;
pub const LTDC_PX: u32 = 138;
pub const ETH1CK_K: u32 = 139;
pub const ETH1PTP_K: u32 = 140;
pub const ETH2CK_K: u32 = 141;
pub const ETH2PTP_K: u32 = 142;
pub const DCMIPP_K: u32 = 143;
pub const SAES_K: u32 = 144;
pub const DTS_K: u32 = 145;

/* DDR */
pub const DDRC1: u32 = 146;
pub const DDRC1LP: u32 = 147;
pub const DDRC2: u32 = 148;
pub const DDRC2LP: u32 = 149;
pub const DDRPHYC: u32 = 150;
pub const DDRPHYCLP: u32 = 151;
pub const DDRCAPB: u32 = 152;
pub const DDRCAPBLP: u32 = 153;
pub const AXIDCG: u32 = 154;
pub const DDRPHYCAPB: u32 = 155;
pub const DDRPHYCAPBLP: u32 = 156;
pub const DDRPERFM: u32 = 157;

pub const ADC1: u32 = 158;
pub const ADC2: u32 = 159;
pub const SAI1: u32 = 160;
pub const SAI2: u32 = 161;

pub const STM32MP1_LAST_CLK: u32 = 162;

/* SCMI clock identifiers */
pub const CK_SCMI_HSE: u32 = 0;
pub const CK_SCMI_HSI: u32 = 1;
pub const CK_SCMI_CSI: u32 = 2;
pub const CK_SCMI_LSE: u32 = 3;
pub const CK_SCMI_LSI: u32 = 4;
pub const CK_SCMI_HSE_DIV2: u32 = 5;
pub const CK_SCMI_PLL2_Q: u32 = 6;
pub const CK_SCMI_PLL2_R: u32 = 7;
pub const CK_SCMI_PLL3_P: u32 = 8;
pub const CK_SCMI_PLL3_Q: u32 = 9;
pub const CK_SCMI_PLL3_R: u32 = 10;
pub const CK_SCMI_PLL4_P: u32 = 11;
pub const CK_SCMI_PLL4_Q: u32 = 12;
pub const CK_SCMI_PLL4_R: u32 = 13;
pub const CK_SCMI_MPU: u32 = 14;
pub const CK_SCMI_AXI: u32 = 15;
pub const CK_SCMI_MLAHB: u32 = 16;
pub const CK_SCMI_CKPER: u32 = 17;
pub const CK_SCMI_PCLK1: u32 = 18;
pub const CK_SCMI_PCLK2: u32 = 19;
pub const CK_SCMI_PCLK3: u32 = 20;
pub const CK_SCMI_PCLK4: u32 = 21;
pub const CK_SCMI_PCLK5: u32 = 22;
pub const CK_SCMI_PCLK6: u32 = 23;
pub const CK_SCMI_CKTIMG1: u32 = 24;
pub const CK_SCMI_CKTIMG2: u32 = 25;
pub const CK_SCMI_CKTIMG3: u32 = 26;
pub const CK_SCMI_RTC: u32 = 27;
pub const CK_SCMI_RTCAPB: u32 = 28;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
