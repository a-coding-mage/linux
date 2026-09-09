/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/*
 * Copyright (C) STMicroelectronics 2018 - All Rights Reserved
 * Author: Gabriel Fernandez <gabriel.fernandez@st.com> for STMicroelectronics.
 */

/* OSCILLATOR clocks */
pub const CK_HSE: i32 = 0;
pub const CK_CSI: i32 = 1;
pub const CK_LSI: i32 = 2;
pub const CK_LSE: i32 = 3;
pub const CK_HSI: i32 = 4;
pub const CK_HSE_DIV2: i32 = 5;

/* Bus clocks */
pub const TIM2: i32 = 6;
pub const TIM3: i32 = 7;
pub const TIM4: i32 = 8;
pub const TIM5: i32 = 9;
pub const TIM6: i32 = 10;
pub const TIM7: i32 = 11;
pub const TIM12: i32 = 12;
pub const TIM13: i32 = 13;
pub const TIM14: i32 = 14;
pub const LPTIM1: i32 = 15;
pub const SPI2: i32 = 16;
pub const SPI3: i32 = 17;
pub const USART2: i32 = 18;
pub const USART3: i32 = 19;
pub const UART4: i32 = 20;
pub const UART5: i32 = 21;
pub const UART7: i32 = 22;
pub const UART8: i32 = 23;
pub const I2C1: i32 = 24;
pub const I2C2: i32 = 25;
pub const I2C3: i32 = 26;
pub const I2C5: i32 = 27;
pub const SPDIF: i32 = 28;
pub const CEC: i32 = 29;
pub const DAC12: i32 = 30;
pub const MDIO: i32 = 31;
pub const TIM1: i32 = 32;
pub const TIM8: i32 = 33;
pub const TIM15: i32 = 34;
pub const TIM16: i32 = 35;
pub const TIM17: i32 = 36;
pub const SPI1: i32 = 37;
pub const SPI4: i32 = 38;
pub const SPI5: i32 = 39;
pub const USART6: i32 = 40;
pub const SAI1: i32 = 41;
pub const SAI2: i32 = 42;
pub const SAI3: i32 = 43;
pub const DFSDM: i32 = 44;
pub const FDCAN: i32 = 45;
pub const LPTIM2: i32 = 46;
pub const LPTIM3: i32 = 47;
pub const LPTIM4: i32 = 48;
pub const LPTIM5: i32 = 49;
pub const SAI4: i32 = 50;
pub const SYSCFG: i32 = 51;
pub const VREF: i32 = 52;
pub const TMPSENS: i32 = 53;
pub const PMBCTRL: i32 = 54;
pub const HDP: i32 = 55;
pub const LTDC: i32 = 56;
pub const DSI: i32 = 57;
pub const IWDG2: i32 = 58;
pub const USBPHY: i32 = 59;
pub const STGENRO: i32 = 60;
pub const SPI6: i32 = 61;
pub const I2C4: i32 = 62;
pub const I2C6: i32 = 63;
pub const USART1: i32 = 64;
pub const RTCAPB: i32 = 65;
pub const TZC1: i32 = 66;
pub const TZPC: i32 = 67;
pub const IWDG1: i32 = 68;
pub const BSEC: i32 = 69;
pub const STGEN: i32 = 70;
pub const DMA1: i32 = 71;
pub const DMA2: i32 = 72;
pub const DMAMUX: i32 = 73;
pub const ADC12: i32 = 74;
pub const USBO: i32 = 75;
pub const SDMMC3: i32 = 76;
pub const DCMI: i32 = 77;
pub const CRYP2: i32 = 78;
pub const HASH2: i32 = 79;
pub const RNG2: i32 = 80;
pub const CRC2: i32 = 81;
pub const HSEM: i32 = 82;
pub const IPCC: i32 = 83;
pub const GPIOA: i32 = 84;
pub const GPIOB: i32 = 85;
pub const GPIOC: i32 = 86;
pub const GPIOD: i32 = 87;
pub const GPIOE: i32 = 88;
pub const GPIOF: i32 = 89;
pub const GPIOG: i32 = 90;
pub const GPIOH: i32 = 91;
pub const GPIOI: i32 = 92;
pub const GPIOJ: i32 = 93;
pub const GPIOK: i32 = 94;
pub const GPIOZ: i32 = 95;
pub const CRYP1: i32 = 96;
pub const HASH1: i32 = 97;
pub const RNG1: i32 = 98;
pub const BKPSRAM: i32 = 99;
pub const MDMA: i32 = 100;
pub const GPU: i32 = 101;
pub const ETHCK: i32 = 102;
pub const ETHTX: i32 = 103;
pub const ETHRX: i32 = 104;
pub const ETHMAC: i32 = 105;
pub const FMC: i32 = 106;
pub const QSPI: i32 = 107;
pub const SDMMC1: i32 = 108;
pub const SDMMC2: i32 = 109;
pub const CRC1: i32 = 110;
pub const USBH: i32 = 111;
pub const ETHSTP: i32 = 112;
pub const TZC2: i32 = 113;

/* Kernel clocks */
pub const SDMMC1_K: i32 = 118;
pub const SDMMC2_K: i32 = 119;
pub const SDMMC3_K: i32 = 120;
pub const FMC_K: i32 = 121;
pub const QSPI_K: i32 = 122;
pub const ETHCK_K: i32 = 123;
pub const RNG1_K: i32 = 124;
pub const RNG2_K: i32 = 125;
pub const GPU_K: i32 = 126;
pub const USBPHY_K: i32 = 127;
pub const STGEN_K: i32 = 128;
pub const SPDIF_K: i32 = 129;
pub const SPI1_K: i32 = 130;
pub const SPI2_K: i32 = 131;
pub const SPI3_K: i32 = 132;
pub const SPI4_K: i32 = 133;
pub const SPI5_K: i32 = 134;
pub const SPI6_K: i32 = 135;
pub const CEC_K: i32 = 136;
pub const I2C1_K: i32 = 137;
pub const I2C2_K: i32 = 138;
pub const I2C3_K: i32 = 139;
pub const I2C4_K: i32 = 140;
pub const I2C5_K: i32 = 141;
pub const I2C6_K: i32 = 142;
pub const LPTIM1_K: i32 = 143;
pub const LPTIM2_K: i32 = 144;
pub const LPTIM3_K: i32 = 145;
pub const LPTIM4_K: i32 = 146;
pub const LPTIM5_K: i32 = 147;
pub const USART1_K: i32 = 148;
pub const USART2_K: i32 = 149;
pub const USART3_K: i32 = 150;
pub const UART4_K: i32 = 151;
pub const UART5_K: i32 = 152;
pub const USART6_K: i32 = 153;
pub const UART7_K: i32 = 154;
pub const UART8_K: i32 = 155;
pub const DFSDM_K: i32 = 156;
pub const FDCAN_K: i32 = 157;
pub const SAI1_K: i32 = 158;
pub const SAI2_K: i32 = 159;
pub const SAI3_K: i32 = 160;
pub const SAI4_K: i32 = 161;
pub const ADC12_K: i32 = 162;
pub const DSI_K: i32 = 163;
pub const DSI_PX: i32 = 164;
pub const ADFSDM_K: i32 = 165;
pub const USBO_K: i32 = 166;
pub const LTDC_PX: i32 = 167;
pub const DAC12_K: i32 = 168;
pub const ETHPTP_K: i32 = 169;

/* PLL */
pub const PLL1: i32 = 176;
pub const PLL2: i32 = 177;
pub const PLL3: i32 = 178;
pub const PLL4: i32 = 179;

/* ODF */
pub const PLL1_P: i32 = 180;
pub const PLL1_Q: i32 = 181;
pub const PLL1_R: i32 = 182;
pub const PLL2_P: i32 = 183;
pub const PLL2_Q: i32 = 184;
pub const PLL2_R: i32 = 185;
pub const PLL3_P: i32 = 186;
pub const PLL3_Q: i32 = 187;
pub const PLL3_R: i32 = 188;
pub const PLL4_P: i32 = 189;
pub const PLL4_Q: i32 = 190;
pub const PLL4_R: i32 = 191;

/* AUX */
pub const RTC: i32 = 192;

/* MCLK */
pub const CK_PER: i32 = 193;
pub const CK_MPU: i32 = 194;
pub const CK_AXI: i32 = 195;
pub const CK_MCU: i32 = 196;

/* Time base */
pub const TIM2_K: i32 = 197;
pub const TIM3_K: i32 = 198;
pub const TIM4_K: i32 = 199;
pub const TIM5_K: i32 = 200;
pub const TIM6_K: i32 = 201;
pub const TIM7_K: i32 = 202;
pub const TIM12_K: i32 = 203;
pub const TIM13_K: i32 = 204;
pub const TIM14_K: i32 = 205;
pub const TIM1_K: i32 = 206;
pub const TIM8_K: i32 = 207;
pub const TIM15_K: i32 = 208;
pub const TIM16_K: i32 = 209;
pub const TIM17_K: i32 = 210;

/* MCO clocks */
pub const CK_MCO1: i32 = 211;
pub const CK_MCO2: i32 = 212;

/* TRACE & DEBUG clocks */
pub const CK_DBG: i32 = 214;
pub const CK_TRACE: i32 = 215;

/* DDR */
pub const DDRC1: i32 = 220;
pub const DDRC1LP: i32 = 221;
pub const DDRC2: i32 = 222;
pub const DDRC2LP: i32 = 223;
pub const DDRPHYC: i32 = 224;
pub const DDRPHYCLP: i32 = 225;
pub const DDRCAPB: i32 = 226;
pub const DDRCAPBLP: i32 = 227;
pub const AXIDCG: i32 = 228;
pub const DDRPHYCAPB: i32 = 229;
pub const DDRPHYCAPBLP: i32 = 230;
pub const DDRPERFM: i32 = 231;

pub const STM32MP1_LAST_CLK: i32 = 232;

/* SCMI clock identifiers */
pub const CK_SCMI_HSE: i32 = 0;
pub const CK_SCMI_HSI: i32 = 1;
pub const CK_SCMI_CSI: i32 = 2;
pub const CK_SCMI_LSE: i32 = 3;
pub const CK_SCMI_LSI: i32 = 4;
pub const CK_SCMI_PLL2_Q: i32 = 5;
pub const CK_SCMI_PLL2_R: i32 = 6;
pub const CK_SCMI_MPU: i32 = 7;
pub const CK_SCMI_AXI: i32 = 8;
pub const CK_SCMI_BSEC: i32 = 9;
pub const CK_SCMI_CRYP1: i32 = 10;
pub const CK_SCMI_GPIOZ: i32 = 11;
pub const CK_SCMI_HASH1: i32 = 12;
pub const CK_SCMI_I2C4: i32 = 13;
pub const CK_SCMI_I2C6: i32 = 14;
pub const CK_SCMI_IWDG1: i32 = 15;
pub const CK_SCMI_RNG1: i32 = 16;
pub const CK_SCMI_RTC: i32 = 17;
pub const CK_SCMI_RTCAPB: i32 = 18;
pub const CK_SCMI_SPI6: i32 = 19;
pub const CK_SCMI_USART1: i32 = 20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
