/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2023-2025 Rockchip Electronics Co., Ltd.
 * Author: Finley Xiao <finley.xiao@rock-chips.com>
 */

/* Translated from the C device-tree binding header. */

/* CRU-->SOFTRST_CON00 */
pub const SRST_NCOREPORESET0_AC: u32 = 0;
pub const SRST_NCOREPORESET1_AC: u32 = 1;
pub const SRST_NCOREPORESET2_AC: u32 = 2;
pub const SRST_NCORESET0_AC: u32 = 3;
pub const SRST_NCORESET1_AC: u32 = 4;
pub const SRST_NCORESET2_AC: u32 = 5;
pub const SRST_NL2RESET_AC: u32 = 6;
pub const SRST_A_CORE_BIU_AC: u32 = 7;
pub const SRST_H_M0: u32 = 8;

/* CRU-->SOFTRST_CON02 */
pub const SRST_NDBGRESET: u32 = 9;
pub const SRST_P_CORE_BIU: u32 = 10;
pub const SRST_PMU: u32 = 11;

/* CRU-->SOFTRST_CON03 */
pub const SRST_P_DBG: u32 = 12;
pub const SRST_POT_DBG: u32 = 13;
pub const SRST_P_CORE_GRF: u32 = 14;
pub const SRST_CORE_EMA_DETECT: u32 = 15;
pub const SRST_REF_PVTPLL_CORE: u32 = 16;
pub const SRST_P_GPIO1: u32 = 17;
pub const SRST_DB_GPIO1: u32 = 18;

/* CRU-->SOFTRST_CON04 */
pub const SRST_A_CORE_PERI_BIU: u32 = 19;
pub const SRST_A_DSMC: u32 = 20;
pub const SRST_P_DSMC: u32 = 21;
pub const SRST_FLEXBUS: u32 = 22;
pub const SRST_A_FLEXBUS: u32 = 23;
pub const SRST_H_FLEXBUS: u32 = 24;
pub const SRST_A_DSMC_SLV: u32 = 25;
pub const SRST_H_DSMC_SLV: u32 = 26;
pub const SRST_DSMC_SLV: u32 = 27;

/* CRU-->SOFTRST_CON05 */
pub const SRST_A_BUS_BIU: u32 = 28;
pub const SRST_H_BUS_BIU: u32 = 29;
pub const SRST_P_BUS_BIU: u32 = 30;
pub const SRST_A_SYSRAM: u32 = 31;
pub const SRST_H_SYSRAM: u32 = 32;
pub const SRST_A_DMAC0: u32 = 33;
pub const SRST_A_DMAC1: u32 = 34;
/* The C header repeats SRST_H_M0 with value 35; Rust cannot redeclare it. */
pub const SRST_H_M0_35: u32 = 35;
pub const SRST_M0_JTAG: u32 = 36;
pub const SRST_H_CRYPTO: u32 = 37;

/* CRU-->SOFTRST_CON06 */
pub const SRST_H_RNG: u32 = 38;
pub const SRST_P_BUS_GRF: u32 = 39;
pub const SRST_P_TIMER0: u32 = 40;
pub const SRST_TIMER0_CH0: u32 = 41;
pub const SRST_TIMER0_CH1: u32 = 42;
pub const SRST_TIMER0_CH2: u32 = 43;
pub const SRST_TIMER0_CH3: u32 = 44;
pub const SRST_TIMER0_CH4: u32 = 45;
pub const SRST_TIMER0_CH5: u32 = 46;
pub const SRST_P_WDT0: u32 = 47;
pub const SRST_T_WDT0: u32 = 48;
pub const SRST_P_WDT1: u32 = 49;
pub const SRST_T_WDT1: u32 = 50;
pub const SRST_P_MAILBOX: u32 = 51;
pub const SRST_P_INTMUX: u32 = 52;
pub const SRST_P_SPINLOCK: u32 = 53;

/* CRU-->SOFTRST_CON07 */
pub const SRST_P_DDRC: u32 = 54;
pub const SRST_H_DDRPHY: u32 = 55;
pub const SRST_P_DDRMON: u32 = 56;
pub const SRST_DDRMON_OSC: u32 = 57;
pub const SRST_P_DDR_LPC: u32 = 58;
pub const SRST_H_USBOTG0: u32 = 59;
pub const SRST_USBOTG0_ADP: u32 = 60;
pub const SRST_H_USBOTG1: u32 = 61;
pub const SRST_USBOTG1_ADP: u32 = 62;
pub const SRST_P_USBPHY: u32 = 63;
pub const SRST_USBPHY_POR: u32 = 64;
pub const SRST_USBPHY_OTG0: u32 = 65;
pub const SRST_USBPHY_OTG1: u32 = 66;

/* CRU-->SOFTRST_CON08 */
pub const SRST_A_DMA2DDR: u32 = 67;
pub const SRST_P_DMA2DDR: u32 = 68;
/* CRU-->SOFTRST_CON09 */
pub const SRST_USBOTG0_UTMI: u32 = 69;
pub const SRST_USBOTG1_UTMI: u32 = 70;
/* CRU-->SOFTRST_CON10 */
pub const SRST_A_DDRC_0: u32 = 71;
pub const SRST_A_DDRC_1: u32 = 72;
pub const SRST_A_DDR_BIU: u32 = 73;
pub const SRST_DDRC: u32 = 74;
pub const SRST_DDRMON: u32 = 75;
/* CRU-->SOFTRST_CON11 */
pub const SRST_H_LSPERI_BIU: u32 = 76;
pub const SRST_P_UART0: u32 = 77;
pub const SRST_P_UART1: u32 = 78;
pub const SRST_P_UART2: u32 = 79;
pub const SRST_P_UART3: u32 = 80;
pub const SRST_P_UART4: u32 = 81;
pub const SRST_UART0: u32 = 82;
pub const SRST_UART1: u32 = 83;
pub const SRST_UART2: u32 = 84;
pub const SRST_UART3: u32 = 85;
pub const SRST_UART4: u32 = 86;
pub const SRST_P_I2C0: u32 = 87;
pub const SRST_I2C0: u32 = 88;
/* CRU-->SOFTRST_CON12 */
pub const SRST_P_I2C1: u32 = 89;
pub const SRST_I2C1: u32 = 90;
pub const SRST_P_I2C2: u32 = 91;
pub const SRST_I2C2: u32 = 92;
pub const SRST_P_PWM1: u32 = 93;
pub const SRST_PWM1: u32 = 94;
pub const SRST_P_SPI0: u32 = 95;
pub const SRST_SPI0: u32 = 96;
pub const SRST_P_SPI1: u32 = 97;
pub const SRST_SPI1: u32 = 98;
pub const SRST_P_GPIO2: u32 = 99;
pub const SRST_DB_GPIO2: u32 = 100;
/* CRU-->SOFTRST_CON13 */
pub const SRST_P_GPIO3: u32 = 101;
pub const SRST_DB_GPIO3: u32 = 102;
pub const SRST_P_GPIO4: u32 = 103;
pub const SRST_DB_GPIO4: u32 = 104;
pub const SRST_H_CAN0: u32 = 105;
pub const SRST_CAN0: u32 = 106;
pub const SRST_H_CAN1: u32 = 107;
pub const SRST_CAN1: u32 = 108;
pub const SRST_H_PDM: u32 = 109;
pub const SRST_M_PDM: u32 = 110;
pub const SRST_PDM: u32 = 111;
pub const SRST_SPDIFTX: u32 = 112;
pub const SRST_H_SPDIFTX: u32 = 113;
pub const SRST_H_SPDIFRX: u32 = 114;
pub const SRST_SPDIFRX: u32 = 115;
pub const SRST_M_SAI0: u32 = 116;
/* CRU-->SOFTRST_CON14 */
pub const SRST_H_SAI0: u32 = 117;
pub const SRST_M_SAI1: u32 = 118;
pub const SRST_H_SAI1: u32 = 119;
pub const SRST_H_ASRC0: u32 = 120;
pub const SRST_ASRC0: u32 = 121;
pub const SRST_H_ASRC1: u32 = 122;
pub const SRST_ASRC1: u32 = 123;
/* CRU-->SOFTRST_CON17 */
pub const SRST_H_HSPERI_BIU: u32 = 124;
pub const SRST_H_SDMMC: u32 = 125;
pub const SRST_H_FSPI: u32 = 126;
pub const SRST_S_FSPI: u32 = 127;
pub const SRST_P_SPI2: u32 = 128;
pub const SRST_A_MAC0: u32 = 129;
pub const SRST_A_MAC1: u32 = 130;
/* CRU-->SOFTRST_CON18 */
pub const SRST_M_SAI2: u32 = 131;
pub const SRST_H_SAI2: u32 = 132;
pub const SRST_H_SAI3: u32 = 133;
pub const SRST_M_SAI3: u32 = 134;
pub const SRST_H_SAI4: u32 = 135;
pub const SRST_M_SAI4: u32 = 136;
pub const SRST_H_DSM: u32 = 137;
pub const SRST_M_DSM: u32 = 138;
pub const SRST_P_AUDIO_ADC: u32 = 139;
pub const SRST_M_AUDIO_ADC: u32 = 140;
/* CRU-->SOFTRST_CON19 */
pub const SRST_P_SARADC: u32 = 141;
pub const SRST_SARADC: u32 = 142;
pub const SRST_SARADC_PHY: u32 = 143;
pub const SRST_P_OTPC_NS: u32 = 144;
pub const SRST_SBPI_OTPC_NS: u32 = 145;
pub const SRST_USER_OTPC_NS: u32 = 146;
pub const SRST_P_UART5: u32 = 147;
pub const SRST_UART5: u32 = 148;
pub const SRST_P_GPIO234_IOC: u32 = 149;
/* CRU-->SOFTRST_CON21 */
pub const SRST_A_VIO_BIU: u32 = 150;
pub const SRST_H_VIO_BIU: u32 = 151;
pub const SRST_H_RGA: u32 = 152;
pub const SRST_A_RGA: u32 = 153;
pub const SRST_CORE_RGA: u32 = 154;
pub const SRST_A_VOP: u32 = 155;
pub const SRST_H_VOP: u32 = 156;
pub const SRST_VOP: u32 = 157;
pub const SRST_P_DPHY: u32 = 158;
pub const SRST_P_DSI_HOST: u32 = 159;
pub const SRST_P_TSADC: u32 = 160;
pub const SRST_TSADC: u32 = 161;
/* CRU-->SOFTRST_CON22 */
pub const SRST_P_GPIO1_IOC: u32 = 162;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
