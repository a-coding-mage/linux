/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) 2016 BayLibre, SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 *
 * Copyright (c) 2017 Amlogic, inc.
 * Author: Yixun Lan <yixun.lan@amlogic.com>
 *
 */

/* RESET0 */
pub const RESET_HIU: u32 = 0;
pub const RESET_PCIE_A: u32 = 1;
pub const RESET_PCIE_B: u32 = 2;
pub const RESET_DDR_TOP: u32 = 3;
/* 4 */
pub const RESET_VIU: u32 = 5;
pub const RESET_PCIE_PHY: u32 = 6;
pub const RESET_PCIE_APB: u32 = 7;
/* 8 */
/* 9 */
pub const RESET_VENC: u32 = 10;
pub const RESET_ASSIST: u32 = 11;
/* 12 */
pub const RESET_VCBUS: u32 = 13;
/* 14 */
/* 15 */
pub const RESET_GIC: u32 = 16;
pub const RESET_CAPB3_DECODE: u32 = 17;
/* 18-21 */
pub const RESET_SYS_CPU_CAPB3: u32 = 22;
pub const RESET_CBUS_CAPB3: u32 = 23;
pub const RESET_AHB_CNTL: u32 = 24;
pub const RESET_AHB_DATA: u32 = 25;
pub const RESET_VCBUS_CLK81: u32 = 26;
pub const RESET_MMC: u32 = 27;
/* 28-31 */
/* RESET1 */
/* 32 */
/* 33 */
pub const RESET_USB_OTG: u32 = 34;
pub const RESET_DDR: u32 = 35;
pub const RESET_AO_RESET: u32 = 36;
/* 37 */
pub const RESET_AHB_SRAM: u32 = 38;
/* 39 */
/* 40 */
pub const RESET_DMA: u32 = 41;
pub const RESET_ISA: u32 = 42;
pub const RESET_ETHERNET: u32 = 43;
/* 44 */
pub const RESET_SD_EMMC_B: u32 = 45;
pub const RESET_SD_EMMC_C: u32 = 46;
pub const RESET_ROM_BOOT: u32 = 47;
pub const RESET_SYS_CPU_0: u32 = 48;
pub const RESET_SYS_CPU_1: u32 = 49;
pub const RESET_SYS_CPU_2: u32 = 50;
pub const RESET_SYS_CPU_3: u32 = 51;
pub const RESET_SYS_CPU_CORE_0: u32 = 52;
pub const RESET_SYS_CPU_CORE_1: u32 = 53;
pub const RESET_SYS_CPU_CORE_2: u32 = 54;
pub const RESET_SYS_CPU_CORE_3: u32 = 55;
pub const RESET_SYS_PLL_DIV: u32 = 56;
pub const RESET_SYS_CPU_AXI: u32 = 57;
pub const RESET_SYS_CPU_L2: u32 = 58;
pub const RESET_SYS_CPU_P: u32 = 59;
pub const RESET_SYS_CPU_MBIST: u32 = 60;
/* 61-63 */
/* RESET2 */
/* 64 */
/* 65 */
pub const RESET_AUDIO: u32 = 66;
/* 67 */
pub const RESET_MIPI_HOST: u32 = 68;
pub const RESET_AUDIO_LOCKER: u32 = 69;
pub const RESET_GE2D: u32 = 70;
/* 71-76 */
pub const RESET_AO_CPU_RESET: u32 = 77;
/* 78-95 */
/* RESET3 */
pub const RESET_RING_OSCILLATOR: u32 = 96;
/* 97-127 */
/* RESET4 */
/* 128 */
/* 129 */
pub const RESET_MIPI_PHY: u32 = 130;
/* 131-140 */
pub const RESET_VENCL: u32 = 141;
pub const RESET_I2C_MASTER_2: u32 = 142;
pub const RESET_I2C_MASTER_1: u32 = 143;
/* 144-159 */
/* RESET5 */
/* 160-191 */
/* RESET6 */
pub const RESET_PERIPHS_GENERAL: u32 = 192;
pub const RESET_PERIPHS_SPICC: u32 = 193;
/* 194 */
/* 195 */
pub const RESET_PERIPHS_I2C_MASTER_0: u32 = 196;
/* 197-200 */
pub const RESET_PERIPHS_UART_0: u32 = 201;
pub const RESET_PERIPHS_UART_1: u32 = 202;
/* 203-204 */
pub const RESET_PERIPHS_SPI_0: u32 = 205;
pub const RESET_PERIPHS_I2C_MASTER_3: u32 = 206;
/* 207-223 */
/* RESET7 */
pub const RESET_USB_DDR_0: u32 = 224;
pub const RESET_USB_DDR_1: u32 = 225;
pub const RESET_USB_DDR_2: u32 = 226;
pub const RESET_USB_DDR_3: u32 = 227;
/* 228 */
pub const RESET_DEVICE_MMC_ARB: u32 = 229;
/* 230 */
pub const RESET_VID_LOCK: u32 = 231;
pub const RESET_A9_DMC_PIPEL: u32 = 232;
pub const RESET_DMC_VPU_PIPEL: u32 = 233;
/* 234-255 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
