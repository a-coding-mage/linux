/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2004-2007 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2008 Juergen Beisert, kernel@pengutronix.de
 *
 * This contains hardware definitions that are common between i.MX21 and
 * i.MX27.
 */

// The following addresses are common between i.MX21 and i.MX27.
// Register offsets.
pub const MX2x_AIPI_BASE_ADDR: usize = 0x10000000;
pub const MX2x_AIPI_SIZE: usize = SZ_1M;
pub const MX2x_DMA_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x01000;
pub const MX2x_WDOG_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x02000;
pub const MX2x_GPT1_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x03000;
pub const MX2x_GPT2_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x04000;
pub const MX2x_GPT3_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x05000;
pub const MX2x_PWM_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x06000;
pub const MX2x_RTC_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x07000;
pub const MX2x_KPP_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x08000;
pub const MX2x_OWIRE_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x09000;
pub const MX2x_UART1_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x0a000;
pub const MX2x_UART2_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x0b000;
pub const MX2x_UART3_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x0c000;
pub const MX2x_UART4_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x0d000;
pub const MX2x_CSPI1_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x0e000;
pub const MX2x_CSPI2_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x0f000;
pub const MX2x_SSI1_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x10000;
pub const MX2x_SSI2_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x11000;
pub const MX2x_I2C_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x12000;
pub const MX2x_SDHC1_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x13000;
pub const MX2x_SDHC2_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x14000;
pub const MX2x_GPIO_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x15000;
pub const MX2x_AUDMUX_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x16000;
pub const MX2x_CSPI3_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x17000;
pub const MX2x_LCDC_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x21000;
pub const MX2x_SLCDC_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x22000;
pub const MX2x_USBOTG_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x24000;
pub const MX2x_EMMA_PP_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x26000;
pub const MX2x_EMMA_PRP_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x26400;
pub const MX2x_CCM_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x27000;
pub const MX2x_SYSCTRL_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x27800;
pub const MX2x_JAM_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x3e000;
pub const MX2x_MAX_BASE_ADDR: usize = MX2x_AIPI_BASE_ADDR + 0x3f000;

pub const MX2x_AVIC_BASE_ADDR: usize = 0x10040000;

pub const MX2x_SAHB1_BASE_ADDR: usize = 0x80000000;
pub const MX2x_SAHB1_SIZE: usize = SZ_1M;
pub const MX2x_CSI_BASE_ADDR: usize = MX2x_SAHB1_BASE_ADDR + 0x0000;

// Fixed interrupt numbers. `NR_IRQS_LEGACY` is supplied by the interrupt subsystem.
pub const MX2x_INT_CSPI3: usize = NR_IRQS_LEGACY + 6;
pub const MX2x_INT_GPIO: usize = NR_IRQS_LEGACY + 8;
pub const MX2x_INT_SDHC2: usize = NR_IRQS_LEGACY + 10;
pub const MX2x_INT_SDHC1: usize = NR_IRQS_LEGACY + 11;
pub const MX2x_INT_I2C: usize = NR_IRQS_LEGACY + 12;
pub const MX2x_INT_SSI2: usize = NR_IRQS_LEGACY + 13;
pub const MX2x_INT_SSI1: usize = NR_IRQS_LEGACY + 14;
pub const MX2x_INT_CSPI2: usize = NR_IRQS_LEGACY + 15;
pub const MX2x_INT_CSPI1: usize = NR_IRQS_LEGACY + 16;
pub const MX2x_INT_UART4: usize = NR_IRQS_LEGACY + 17;
pub const MX2x_INT_UART3: usize = NR_IRQS_LEGACY + 18;
pub const MX2x_INT_UART2: usize = NR_IRQS_LEGACY + 19;
pub const MX2x_INT_UART1: usize = NR_IRQS_LEGACY + 20;
pub const MX2x_INT_KPP: usize = NR_IRQS_LEGACY + 21;
pub const MX2x_INT_RTC: usize = NR_IRQS_LEGACY + 22;
pub const MX2x_INT_PWM: usize = NR_IRQS_LEGACY + 23;
pub const MX2x_INT_GPT3: usize = NR_IRQS_LEGACY + 24;
pub const MX2x_INT_GPT2: usize = NR_IRQS_LEGACY + 25;
pub const MX2x_INT_GPT1: usize = NR_IRQS_LEGACY + 26;
pub const MX2x_INT_WDOG: usize = NR_IRQS_LEGACY + 27;
pub const MX2x_INT_PCMCIA: usize = NR_IRQS_LEGACY + 28;
pub const MX2x_INT_NANDFC: usize = NR_IRQS_LEGACY + 29;
pub const MX2x_INT_CSI: usize = NR_IRQS_LEGACY + 31;
pub const MX2x_INT_DMACH0: usize = NR_IRQS_LEGACY + 32;
pub const MX2x_INT_DMACH1: usize = NR_IRQS_LEGACY + 33;
pub const MX2x_INT_DMACH2: usize = NR_IRQS_LEGACY + 34;
pub const MX2x_INT_DMACH3: usize = NR_IRQS_LEGACY + 35;
pub const MX2x_INT_DMACH4: usize = NR_IRQS_LEGACY + 36;
pub const MX2x_INT_DMACH5: usize = NR_IRQS_LEGACY + 37;
pub const MX2x_INT_DMACH6: usize = NR_IRQS_LEGACY + 38;
pub const MX2x_INT_DMACH7: usize = NR_IRQS_LEGACY + 39;
pub const MX2x_INT_DMACH8: usize = NR_IRQS_LEGACY + 40;
pub const MX2x_INT_DMACH9: usize = NR_IRQS_LEGACY + 41;
pub const MX2x_INT_DMACH10: usize = NR_IRQS_LEGACY + 42;
pub const MX2x_INT_DMACH11: usize = NR_IRQS_LEGACY + 43;
pub const MX2x_INT_DMACH12: usize = NR_IRQS_LEGACY + 44;
pub const MX2x_INT_DMACH13: usize = NR_IRQS_LEGACY + 45;
pub const MX2x_INT_DMACH14: usize = NR_IRQS_LEGACY + 46;
pub const MX2x_INT_DMACH15: usize = NR_IRQS_LEGACY + 47;
pub const MX2x_INT_EMMAPRP: usize = NR_IRQS_LEGACY + 51;
pub const MX2x_INT_EMMAPP: usize = NR_IRQS_LEGACY + 52;
pub const MX2x_INT_SLCDC: usize = NR_IRQS_LEGACY + 60;
pub const MX2x_INT_LCDC: usize = NR_IRQS_LEGACY + 61;

// Fixed DMA request numbers.
pub const MX2x_DMA_REQ_CSPI3_RX: usize = 1;
pub const MX2x_DMA_REQ_CSPI3_TX: usize = 2;
pub const MX2x_DMA_REQ_EXT: usize = 3;
pub const MX2x_DMA_REQ_SDHC2: usize = 6;
pub const MX2x_DMA_REQ_SDHC1: usize = 7;
pub const MX2x_DMA_REQ_SSI2_RX0: usize = 8;
pub const MX2x_DMA_REQ_SSI2_TX0: usize = 9;
pub const MX2x_DMA_REQ_SSI2_RX1: usize = 10;
pub const MX2x_DMA_REQ_SSI2_TX1: usize = 11;
pub const MX2x_DMA_REQ_SSI1_RX0: usize = 12;
pub const MX2x_DMA_REQ_SSI1_TX0: usize = 13;
pub const MX2x_DMA_REQ_SSI1_RX1: usize = 14;
pub const MX2x_DMA_REQ_SSI1_TX1: usize = 15;
pub const MX2x_DMA_REQ_CSPI2_RX: usize = 16;
pub const MX2x_DMA_REQ_CSPI2_TX: usize = 17;
pub const MX2x_DMA_REQ_CSPI1_RX: usize = 18;
pub const MX2x_DMA_REQ_CSPI1_TX: usize = 19;
pub const MX2x_DMA_REQ_UART4_RX: usize = 20;
pub const MX2x_DMA_REQ_UART4_TX: usize = 21;
pub const MX2x_DMA_REQ_UART3_RX: usize = 22;
pub const MX2x_DMA_REQ_UART3_TX: usize = 23;
pub const MX2x_DMA_REQ_UART2_RX: usize = 24;
pub const MX2x_DMA_REQ_UART2_TX: usize = 25;
pub const MX2x_DMA_REQ_UART1_RX: usize = 26;
pub const MX2x_DMA_REQ_UART1_TX: usize = 27;
pub const MX2x_DMA_REQ_CSI_STAT: usize = 30;
pub const MX2x_DMA_REQ_CSI_RX: usize = 31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
