/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) 2019 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 *
 */

/* RESET0 */
pub const RESET_HIU: u32 = 0;
/* 1 */
pub const RESET_DOS: u32 = 2;
/* 3-4 */
pub const RESET_VIU: u32 = 5;
pub const RESET_AFIFO: u32 = 6;
pub const RESET_VID_PLL_DIV: u32 = 7;
/* 8-9 */
pub const RESET_VENC: u32 = 10;
pub const RESET_ASSIST: u32 = 11;
pub const RESET_PCIE_CTRL_A: u32 = 12;
pub const RESET_VCBUS: u32 = 13;
pub const RESET_PCIE_PHY: u32 = 14;
pub const RESET_PCIE_APB: u32 = 15;
pub const RESET_GIC: u32 = 16;
pub const RESET_CAPB3_DECODE: u32 = 17;
/* 18 */
pub const RESET_HDMITX_CAPB3: u32 = 19;
pub const RESET_DVALIN_CAPB3: u32 = 20;
pub const RESET_DOS_CAPB3: u32 = 21;
/* 22 */
pub const RESET_CBUS_CAPB3: u32 = 23;
pub const RESET_AHB_CNTL: u32 = 24;
pub const RESET_AHB_DATA: u32 = 25;
pub const RESET_VCBUS_CLK81: u32 = 26;
/* 27-31 */
/* RESET1 */
/* 32 */
pub const RESET_DEMUX: u32 = 33;
pub const RESET_USB: u32 = 34;
pub const RESET_DDR: u32 = 35;
/* 36 */
pub const RESET_BT656: u32 = 37;
pub const RESET_AHB_SRAM: u32 = 38;
/* 39 */
pub const RESET_PARSER: u32 = 40;
/* 41 */
pub const RESET_ISA: u32 = 42;
pub const RESET_ETHERNET: u32 = 43;
pub const RESET_SD_EMMC_A: u32 = 44;
pub const RESET_SD_EMMC_B: u32 = 45;
pub const RESET_SD_EMMC_C: u32 = 46;
/* 47 */
pub const RESET_USB_PHY20: u32 = 48;
pub const RESET_USB_PHY21: u32 = 49;
/* 50-60 */
pub const RESET_AUDIO_CODEC: u32 = 61;
/* 62-63 */
/* RESET2 */
/* 64 */
pub const RESET_AUDIO: u32 = 65;
pub const RESET_HDMITX_PHY: u32 = 66;
/* 67 */
pub const RESET_MIPI_DSI_HOST: u32 = 68;
pub const RESET_ALOCKER: u32 = 69;
pub const RESET_GE2D: u32 = 70;
pub const RESET_PARSER_REG: u32 = 71;
pub const RESET_PARSER_FETCH: u32 = 72;
pub const RESET_CTL: u32 = 73;
pub const RESET_PARSER_TOP: u32 = 74;
/* 75 */
pub const RESET_NNA: u32 = 76;
/* 77 */
pub const RESET_DVALIN: u32 = 78;
pub const RESET_HDMITX: u32 = 79;
/* 80-95 */
/* RESET3 */
/* 96-95 */
pub const RESET_DEMUX_TOP: u32 = 105;
pub const RESET_DEMUX_DES_PL: u32 = 106;
pub const RESET_DEMUX_S2P_0: u32 = 107;
pub const RESET_DEMUX_S2P_1: u32 = 108;
pub const RESET_DEMUX_0: u32 = 109;
pub const RESET_DEMUX_1: u32 = 110;
pub const RESET_DEMUX_2: u32 = 111;
/* 112-127 */
/* RESET4 */
/* 128-129 */
pub const RESET_MIPI_DSI_PHY: u32 = 130;
/* 131-132 */
pub const RESET_RDMA: u32 = 133;
pub const RESET_VENCI: u32 = 134;
pub const RESET_VENCP: u32 = 135;
/* 136 */
pub const RESET_VDAC: u32 = 137;
/* 138-139 */
pub const RESET_VDI6: u32 = 140;
pub const RESET_VENCL: u32 = 141;
pub const RESET_I2C_M1: u32 = 142;
pub const RESET_I2C_M2: u32 = 143;
/* 144-159 */
/* RESET5 */
/* 160-191 */
/* RESET6 */
pub const RESET_GEN: u32 = 192;
pub const RESET_SPICC0: u32 = 193;
pub const RESET_SC: u32 = 194;
pub const RESET_SANA_3: u32 = 195;
pub const RESET_I2C_M0: u32 = 196;
pub const RESET_TS_PLL: u32 = 197;
pub const RESET_SPICC1: u32 = 198;
pub const RESET_STREAM: u32 = 199;
pub const RESET_TS_CPU: u32 = 200;
pub const RESET_UART0: u32 = 201;
pub const RESET_UART1_2: u32 = 202;
pub const RESET_ASYNC0: u32 = 203;
pub const RESET_ASYNC1: u32 = 204;
pub const RESET_SPIFC0: u32 = 205;
pub const RESET_I2C_M3: u32 = 206;
/* 207-223 */
/* RESET7 */
pub const RESET_USB_DDR_0: u32 = 224;
pub const RESET_USB_DDR_1: u32 = 225;
pub const RESET_USB_DDR_2: u32 = 226;
pub const RESET_USB_DDR_3: u32 = 227;
pub const RESET_TS_GPU: u32 = 228;
pub const RESET_DEVICE_MMC_ARB: u32 = 229;
pub const RESET_DVALIN_DMC_PIPL: u32 = 230;
pub const RESET_VID_LOCK: u32 = 231;
pub const RESET_NIC_DMC_PIPL: u32 = 232;
pub const RESET_DMC_VPU_PIPL: u32 = 233;
pub const RESET_GE2D_DMC_PIPL: u32 = 234;
pub const RESET_HCODEC_DMC_PIPL: u32 = 235;
pub const RESET_WAVE420_DMC_PIPL: u32 = 236;
pub const RESET_HEVCF_DMC_PIPL: u32 = 237;
/* 238-255 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
