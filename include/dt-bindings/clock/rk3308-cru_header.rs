// SPDX-License-Identifier: GPL-2.0 
/*
 * Copyright (c) 2019 Rockchip Electronics Co. Ltd.
 * Author: Finley Xiao <finley.xiao@rock-chips.com>
 */

// Header guard omitted in Rust.
// Header guard omitted in Rust.

// core clocks 
pub const PLL_APLL: u32 = 1;
pub const PLL_DPLL: u32 = 2;
pub const PLL_VPLL0: u32 = 3;
pub const PLL_VPLL1: u32 = 4;
pub const ARMCLK: u32 = 5;

// sclk (special clocks) 
pub const USB480M: u32 = 14;
pub const SCLK_RTC32K: u32 = 15;
pub const SCLK_PVTM_CORE: u32 = 16;
pub const SCLK_UART0: u32 = 17;
pub const SCLK_UART1: u32 = 18;
pub const SCLK_UART2: u32 = 19;
pub const SCLK_UART3: u32 = 20;
pub const SCLK_UART4: u32 = 21;
pub const SCLK_I2C0: u32 = 22;
pub const SCLK_I2C1: u32 = 23;
pub const SCLK_I2C2: u32 = 24;
pub const SCLK_I2C3: u32 = 25;
pub const SCLK_PWM0: u32 = 26;
pub const SCLK_SPI0: u32 = 27;
pub const SCLK_SPI1: u32 = 28;
pub const SCLK_SPI2: u32 = 29;
pub const SCLK_TIMER0: u32 = 30;
pub const SCLK_TIMER1: u32 = 31;
pub const SCLK_TIMER2: u32 = 32;
pub const SCLK_TIMER3: u32 = 33;
pub const SCLK_TIMER4: u32 = 34;
pub const SCLK_TIMER5: u32 = 35;
pub const SCLK_TSADC: u32 = 36;
pub const SCLK_SARADC: u32 = 37;
pub const SCLK_OTP: u32 = 38;
pub const SCLK_OTP_USR: u32 = 39;
pub const SCLK_CPU_BOOST: u32 = 40;
pub const SCLK_CRYPTO: u32 = 41;
pub const SCLK_CRYPTO_APK: u32 = 42;
pub const SCLK_NANDC_DIV: u32 = 43;
pub const SCLK_NANDC_DIV50: u32 = 44;
pub const SCLK_NANDC: u32 = 45;
pub const SCLK_SDMMC_DIV: u32 = 46;
pub const SCLK_SDMMC_DIV50: u32 = 47;
pub const SCLK_SDMMC: u32 = 48;
pub const SCLK_SDMMC_DRV: u32 = 49;
pub const SCLK_SDMMC_SAMPLE: u32 = 50;
pub const SCLK_SDIO_DIV: u32 = 51;
pub const SCLK_SDIO_DIV50: u32 = 52;
pub const SCLK_SDIO: u32 = 53;
pub const SCLK_SDIO_DRV: u32 = 54;
pub const SCLK_SDIO_SAMPLE: u32 = 55;
pub const SCLK_EMMC_DIV: u32 = 56;
pub const SCLK_EMMC_DIV50: u32 = 57;
pub const SCLK_EMMC: u32 = 58;
pub const SCLK_EMMC_DRV: u32 = 59;
pub const SCLK_EMMC_SAMPLE: u32 = 60;
pub const SCLK_SFC: u32 = 61;
pub const SCLK_OTG_ADP: u32 = 62;
pub const SCLK_MAC_SRC: u32 = 63;
pub const SCLK_MAC: u32 = 64;
pub const SCLK_MAC_REF: u32 = 65;
pub const SCLK_MAC_RX_TX: u32 = 66;
pub const SCLK_MAC_RMII: u32 = 67;
pub const SCLK_DDR_MON_TIMER: u32 = 68;
pub const SCLK_DDR_MON: u32 = 69;
pub const SCLK_DDRCLK: u32 = 70;
pub const SCLK_PMU: u32 = 71;
pub const SCLK_USBPHY_REF: u32 = 72;
pub const SCLK_WIFI: u32 = 73;
pub const SCLK_PVTM_PMU: u32 = 74;
pub const SCLK_PDM: u32 = 75;
pub const SCLK_I2S0_8CH_TX: u32 = 76;
pub const SCLK_I2S0_8CH_TX_OUT: u32 = 77;
pub const SCLK_I2S0_8CH_RX: u32 = 78;
pub const SCLK_I2S0_8CH_RX_OUT: u32 = 79;
pub const SCLK_I2S1_8CH_TX: u32 = 80;
pub const SCLK_I2S1_8CH_TX_OUT: u32 = 81;
pub const SCLK_I2S1_8CH_RX: u32 = 82;
pub const SCLK_I2S1_8CH_RX_OUT: u32 = 83;
pub const SCLK_I2S2_8CH_TX: u32 = 84;
pub const SCLK_I2S2_8CH_TX_OUT: u32 = 85;
pub const SCLK_I2S2_8CH_RX: u32 = 86;
pub const SCLK_I2S2_8CH_RX_OUT: u32 = 87;
pub const SCLK_I2S3_8CH_TX: u32 = 88;
pub const SCLK_I2S3_8CH_TX_OUT: u32 = 89;
pub const SCLK_I2S3_8CH_RX: u32 = 90;
pub const SCLK_I2S3_8CH_RX_OUT: u32 = 91;
pub const SCLK_I2S0_2CH: u32 = 92;
pub const SCLK_I2S0_2CH_OUT: u32 = 93;
pub const SCLK_I2S1_2CH: u32 = 94;
pub const SCLK_I2S1_2CH_OUT: u32 = 95;
pub const SCLK_SPDIF_TX_DIV: u32 = 96;
pub const SCLK_SPDIF_TX_DIV50: u32 = 97;
pub const SCLK_SPDIF_TX: u32 = 98;
pub const SCLK_SPDIF_RX_DIV: u32 = 99;
pub const SCLK_SPDIF_RX_DIV50: u32 = 100;
pub const SCLK_SPDIF_RX: u32 = 101;
pub const SCLK_I2S0_8CH_TX_MUX: u32 = 102;
pub const SCLK_I2S0_8CH_RX_MUX: u32 = 103;
pub const SCLK_I2S1_8CH_TX_MUX: u32 = 104;
pub const SCLK_I2S1_8CH_RX_MUX: u32 = 105;
pub const SCLK_I2S2_8CH_TX_MUX: u32 = 106;
pub const SCLK_I2S2_8CH_RX_MUX: u32 = 107;
pub const SCLK_I2S3_8CH_TX_MUX: u32 = 108;
pub const SCLK_I2S3_8CH_RX_MUX: u32 = 109;
pub const SCLK_I2S0_8CH_TX_SRC: u32 = 110;
pub const SCLK_I2S0_8CH_RX_SRC: u32 = 111;
pub const SCLK_I2S1_8CH_TX_SRC: u32 = 112;
pub const SCLK_I2S1_8CH_RX_SRC: u32 = 113;
pub const SCLK_I2S2_8CH_TX_SRC: u32 = 114;
pub const SCLK_I2S2_8CH_RX_SRC: u32 = 115;
pub const SCLK_I2S3_8CH_TX_SRC: u32 = 116;
pub const SCLK_I2S3_8CH_RX_SRC: u32 = 117;
pub const SCLK_I2S0_2CH_SRC: u32 = 118;
pub const SCLK_I2S1_2CH_SRC: u32 = 119;
pub const SCLK_PWM1: u32 = 120;
pub const SCLK_PWM2: u32 = 121;
pub const SCLK_OWIRE: u32 = 122;

// dclk 
pub const DCLK_VOP: u32 = 125;

// aclk 
pub const ACLK_BUS_SRC: u32 = 130;
pub const ACLK_BUS: u32 = 131;
pub const ACLK_PERI_SRC: u32 = 132;
pub const ACLK_PERI: u32 = 133;
pub const ACLK_MAC: u32 = 134;
pub const ACLK_CRYPTO: u32 = 135;
pub const ACLK_VOP: u32 = 136;
pub const ACLK_GIC: u32 = 137;
pub const ACLK_DMAC0: u32 = 138;
pub const ACLK_DMAC1: u32 = 139;

// hclk 
pub const HCLK_BUS: u32 = 150;
pub const HCLK_PERI: u32 = 151;
pub const HCLK_AUDIO: u32 = 152;
pub const HCLK_NANDC: u32 = 153;
pub const HCLK_SDMMC: u32 = 154;
pub const HCLK_SDIO: u32 = 155;
pub const HCLK_EMMC: u32 = 156;
pub const HCLK_SFC: u32 = 157;
pub const HCLK_OTG: u32 = 158;
pub const HCLK_HOST: u32 = 159;
pub const HCLK_HOST_ARB: u32 = 160;
pub const HCLK_PDM: u32 = 161;
pub const HCLK_SPDIFTX: u32 = 162;
pub const HCLK_SPDIFRX: u32 = 163;
pub const HCLK_I2S0_8CH: u32 = 164;
pub const HCLK_I2S1_8CH: u32 = 165;
pub const HCLK_I2S2_8CH: u32 = 166;
pub const HCLK_I2S3_8CH: u32 = 167;
pub const HCLK_I2S0_2CH: u32 = 168;
pub const HCLK_I2S1_2CH: u32 = 169;
pub const HCLK_VAD: u32 = 170;
pub const HCLK_CRYPTO: u32 = 171;
pub const HCLK_VOP: u32 = 172;

// pclk 
pub const PCLK_BUS: u32 = 190;
pub const PCLK_DDR: u32 = 191;
pub const PCLK_PERI: u32 = 192;
pub const PCLK_PMU: u32 = 193;
pub const PCLK_AUDIO: u32 = 194;
pub const PCLK_MAC: u32 = 195;
pub const PCLK_ACODEC: u32 = 196;
pub const PCLK_UART0: u32 = 197;
pub const PCLK_UART1: u32 = 198;
pub const PCLK_UART2: u32 = 199;
pub const PCLK_UART3: u32 = 200;
pub const PCLK_UART4: u32 = 201;
pub const PCLK_I2C0: u32 = 202;
pub const PCLK_I2C1: u32 = 203;
pub const PCLK_I2C2: u32 = 204;
pub const PCLK_I2C3: u32 = 205;
pub const PCLK_PWM0: u32 = 206;
pub const PCLK_SPI0: u32 = 207;
pub const PCLK_SPI1: u32 = 208;
pub const PCLK_SPI2: u32 = 209;
pub const PCLK_SARADC: u32 = 210;
pub const PCLK_TSADC: u32 = 211;
pub const PCLK_TIMER: u32 = 212;
pub const PCLK_OTP_NS: u32 = 213;
pub const PCLK_WDT: u32 = 214;
pub const PCLK_GPIO0: u32 = 215;
pub const PCLK_GPIO1: u32 = 216;
pub const PCLK_GPIO2: u32 = 217;
pub const PCLK_GPIO3: u32 = 218;
pub const PCLK_GPIO4: u32 = 219;
pub const PCLK_SGRF: u32 = 220;
pub const PCLK_GRF: u32 = 221;
pub const PCLK_USBSD_DET: u32 = 222;
pub const PCLK_DDR_UPCTL: u32 = 223;
pub const PCLK_DDR_MON: u32 = 224;
pub const PCLK_DDRPHY: u32 = 225;
pub const PCLK_DDR_STDBY: u32 = 226;
pub const PCLK_USB_GRF: u32 = 227;
pub const PCLK_CRU: u32 = 228;
pub const PCLK_OTP_PHY: u32 = 229;
pub const PCLK_CPU_BOOST: u32 = 230;
pub const PCLK_PWM1: u32 = 231;
pub const PCLK_PWM2: u32 = 232;
pub const PCLK_CAN: u32 = 233;
pub const PCLK_OWIRE: u32 = 234;

// soft-reset indices 

// cru_softrst_con0 
pub const SRST_CORE0_PO: u32 = 0;
pub const SRST_CORE1_PO: u32 = 1;
pub const SRST_CORE2_PO: u32 = 2;
pub const SRST_CORE3_PO: u32 = 3;
pub const SRST_CORE0: u32 = 4;
pub const SRST_CORE1: u32 = 5;
pub const SRST_CORE2: u32 = 6;
pub const SRST_CORE3: u32 = 7;
pub const SRST_CORE0_DBG: u32 = 8;
pub const SRST_CORE1_DBG: u32 = 9;
pub const SRST_CORE2_DBG: u32 = 10;
pub const SRST_CORE3_DBG: u32 = 11;
pub const SRST_TOPDBG: u32 = 12;
pub const SRST_CORE_NOC: u32 = 13;
pub const SRST_STRC_A: u32 = 14;
pub const SRST_L2C: u32 = 15;

// cru_softrst_con1 
pub const SRST_DAP: u32 = 16;
pub const SRST_CORE_PVTM: u32 = 17;
pub const SRST_CORE_PRF: u32 = 18;
pub const SRST_CORE_GRF: u32 = 19;
pub const SRST_DDRUPCTL: u32 = 20;
pub const SRST_DDRUPCTL_P: u32 = 22;
pub const SRST_MSCH: u32 = 23;
pub const SRST_DDRMON_P: u32 = 25;
pub const SRST_DDRSTDBY_P: u32 = 26;
pub const SRST_DDRSTDBY: u32 = 27;
pub const SRST_DDRPHY: u32 = 28;
pub const SRST_DDRPHY_DIV: u32 = 29;
pub const SRST_DDRPHY_P: u32 = 30;

// cru_softrst_con2 
pub const SRST_BUS_NIU_H: u32 = 32;
pub const SRST_USB_NIU_P: u32 = 33;
pub const SRST_CRYPTO_A: u32 = 34;
pub const SRST_CRYPTO_H: u32 = 35;
pub const SRST_CRYPTO: u32 = 36;
pub const SRST_CRYPTO_APK: u32 = 37;
pub const SRST_VOP_A: u32 = 38;
pub const SRST_VOP_H: u32 = 39;
pub const SRST_VOP_D: u32 = 40;
pub const SRST_INTMEM_A: u32 = 41;
pub const SRST_ROM_H: u32 = 42;
pub const SRST_GIC_A: u32 = 43;
pub const SRST_UART0_P: u32 = 44;
pub const SRST_UART0: u32 = 45;
pub const SRST_UART1_P: u32 = 46;
pub const SRST_UART1: u32 = 47;

// cru_softrst_con3 
pub const SRST_UART2_P: u32 = 48;
pub const SRST_UART2: u32 = 49;
pub const SRST_UART3_P: u32 = 50;
pub const SRST_UART3: u32 = 51;
pub const SRST_UART4_P: u32 = 52;
pub const SRST_UART4: u32 = 53;
pub const SRST_I2C0_P: u32 = 54;
pub const SRST_I2C0: u32 = 55;
pub const SRST_I2C1_P: u32 = 56;
pub const SRST_I2C1: u32 = 57;
pub const SRST_I2C2_P: u32 = 58;
pub const SRST_I2C2: u32 = 59;
pub const SRST_I2C3_P: u32 = 60;
pub const SRST_I2C3: u32 = 61;
pub const SRST_PWM0_P: u32 = 62;
pub const SRST_PWM0: u32 = 63;

// cru_softrst_con4 
pub const SRST_SPI0_P: u32 = 64;
pub const SRST_SPI0: u32 = 65;
pub const SRST_SPI1_P: u32 = 66;
pub const SRST_SPI1: u32 = 67;
pub const SRST_SPI2_P: u32 = 68;
pub const SRST_SPI2: u32 = 69;
pub const SRST_SARADC_P: u32 = 70;
pub const SRST_TSADC_P: u32 = 71;
pub const SRST_TSADC: u32 = 72;
pub const SRST_TIMER0_P: u32 = 73;
pub const SRST_TIMER0: u32 = 74;
pub const SRST_TIMER1: u32 = 75;
pub const SRST_TIMER2: u32 = 76;
pub const SRST_TIMER3: u32 = 77;
pub const SRST_TIMER4: u32 = 78;
pub const SRST_TIMER5: u32 = 79;

// cru_softrst_con5 
pub const SRST_OTP_NS_P: u32 = 80;
pub const SRST_OTP_NS_SBPI: u32 = 81;
pub const SRST_OTP_NS_USR: u32 = 82;
pub const SRST_OTP_PHY_P: u32 = 83;
pub const SRST_OTP_PHY: u32 = 84;
pub const SRST_GPIO0_P: u32 = 86;
pub const SRST_GPIO1_P: u32 = 87;
pub const SRST_GPIO2_P: u32 = 88;
pub const SRST_GPIO3_P: u32 = 89;
pub const SRST_GPIO4_P: u32 = 90;
pub const SRST_GRF_P: u32 = 91;
pub const SRST_USBSD_DET_P: u32 = 92;
pub const SRST_PMU: u32 = 93;
pub const SRST_PMU_PVTM: u32 = 94;
pub const SRST_USB_GRF_P: u32 = 95;

// cru_softrst_con6 
pub const SRST_CPU_BOOST: u32 = 96;
pub const SRST_CPU_BOOST_P: u32 = 97;
pub const SRST_PWM1_P: u32 = 98;
pub const SRST_PWM1: u32 = 99;
pub const SRST_PWM2_P: u32 = 100;
pub const SRST_PWM2: u32 = 101;
pub const SRST_PERI_NIU_A: u32 = 104;
pub const SRST_PERI_NIU_H: u32 = 105;
pub const SRST_PERI_NIU_p: u32 = 106;
pub const SRST_USB2OTG_H: u32 = 107;
pub const SRST_USB2OTG: u32 = 108;
pub const SRST_USB2OTG_ADP: u32 = 109;
pub const SRST_USB2HOST_H: u32 = 110;
pub const SRST_USB2HOST_ARB_H: u32 = 111;

// cru_softrst_con7 
pub const SRST_USB2HOST_AUX_H: u32 = 112;
pub const SRST_USB2HOST_EHCI: u32 = 113;
pub const SRST_USB2HOST: u32 = 114;
pub const SRST_USBPHYPOR: u32 = 115;
pub const SRST_UTMI0: u32 = 116;
pub const SRST_UTMI1: u32 = 117;
pub const SRST_SDIO_H: u32 = 118;
pub const SRST_EMMC_H: u32 = 119;
pub const SRST_SFC_H: u32 = 120;
pub const SRST_SFC: u32 = 121;
pub const SRST_SD_H: u32 = 122;
pub const SRST_NANDC_H: u32 = 123;
pub const SRST_NANDC_N: u32 = 124;
pub const SRST_MAC_A: u32 = 125;
pub const SRST_CAN_P: u32 = 126;
pub const SRST_OWIRE_P: u32 = 127;

// cru_softrst_con8 
pub const SRST_AUDIO_NIU_H: u32 = 128;
pub const SRST_AUDIO_NIU_P: u32 = 129;
pub const SRST_PDM_H: u32 = 130;
pub const SRST_PDM_M: u32 = 131;
pub const SRST_SPDIFTX_H: u32 = 132;
pub const SRST_SPDIFTX_M: u32 = 133;
pub const SRST_SPDIFRX_H: u32 = 134;
pub const SRST_SPDIFRX_M: u32 = 135;
pub const SRST_I2S0_8CH_H: u32 = 136;
pub const SRST_I2S0_8CH_TX_M: u32 = 137;
pub const SRST_I2S0_8CH_RX_M: u32 = 138;
pub const SRST_I2S1_8CH_H: u32 = 139;
pub const SRST_I2S1_8CH_TX_M: u32 = 140;
pub const SRST_I2S1_8CH_RX_M: u32 = 141;
pub const SRST_I2S2_8CH_H: u32 = 142;
pub const SRST_I2S2_8CH_TX_M: u32 = 143;

// cru_softrst_con9 
pub const SRST_I2S2_8CH_RX_M: u32 = 144;
pub const SRST_I2S3_8CH_H: u32 = 145;
pub const SRST_I2S3_8CH_TX_M: u32 = 146;
pub const SRST_I2S3_8CH_RX_M: u32 = 147;
pub const SRST_I2S0_2CH_H: u32 = 148;
pub const SRST_I2S0_2CH_M: u32 = 149;
pub const SRST_I2S1_2CH_H: u32 = 150;
pub const SRST_I2S1_2CH_M: u32 = 151;
pub const SRST_VAD_H: u32 = 152;
pub const SRST_ACODEC_P: u32 = 153;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
