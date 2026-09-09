/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/*
 * Copyright (c) 2016 BayLibre, SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 */

/*\tRESET0\t\t\t\t*/
pub const RESET_HIU: u32 = 0;
/*\t\t\t\t\t1\t*/
pub const RESET_DOS_RESET: u32 = 2;
pub const RESET_DDR_TOP: u32 = 3;
pub const RESET_DCU_RESET: u32 = 4;
pub const RESET_VIU: u32 = 5;
pub const RESET_AIU: u32 = 6;
pub const RESET_VID_PLL_DIV: u32 = 7;
/*\t\t\t\t\t8\t*/
pub const RESET_PMUX: u32 = 9;
pub const RESET_VENC: u32 = 10;
pub const RESET_ASSIST: u32 = 11;
pub const RESET_AFIFO2: u32 = 12;
pub const RESET_VCBUS: u32 = 13;
/*\t\t\t\t\t14\t*/
/*\t\t\t\t\t15\t*/
pub const RESET_GIC: u32 = 16;
pub const RESET_CAPB3_DECODE: u32 = 17;
pub const RESET_NAND_CAPB3: u32 = 18;
pub const RESET_HDMITX_CAPB3: u32 = 19;
pub const RESET_MALI_CAPB3: u32 = 20;
pub const RESET_DOS_CAPB3: u32 = 21;
pub const RESET_SYS_CPU_CAPB3: u32 = 22;
pub const RESET_CBUS_CAPB3: u32 = 23;
pub const RESET_AHB_CNTL: u32 = 24;
pub const RESET_AHB_DATA: u32 = 25;
pub const RESET_VCBUS_CLK81: u32 = 26;
pub const RESET_MMC: u32 = 27;
pub const RESET_MIPI_0: u32 = 28;
pub const RESET_MIPI_1: u32 = 29;
pub const RESET_MIPI_2: u32 = 30;
pub const RESET_MIPI_3: u32 = 31;
/*\tRESET1\t\t\t\t*/
pub const RESET_CPPM: u32 = 32;
pub const RESET_DEMUX: u32 = 33;
pub const RESET_USB_OTG: u32 = 34;
pub const RESET_DDR: u32 = 35;
pub const RESET_AO_RESET: u32 = 36;
pub const RESET_BT656: u32 = 37;
pub const RESET_AHB_SRAM: u32 = 38;
/*\t\t\t\t\t39\t*/
pub const RESET_PARSER: u32 = 40;
pub const RESET_BLKMV: u32 = 41;
pub const RESET_ISA: u32 = 42;
pub const RESET_ETHERNET: u32 = 43;
pub const RESET_SD_EMMC_A: u32 = 44;
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
pub const RESET_ACODEC: u32 = 61;
/*\t\t\t\t\t62\t*/
/*\t\t\t\t\t63\t*/
/*\tRESET2\t\t\t\t*/
pub const RESET_VD_RMEM: u32 = 64;
pub const RESET_AUDIN: u32 = 65;
pub const RESET_HDMI_TX: u32 = 66;
/*\t\t\t\t\t67\t*/
/*\t\t\t\t\t68\t*/
/*\t\t\t\t\t69\t*/
pub const RESET_GE2D: u32 = 70;
pub const RESET_PARSER_REG: u32 = 71;
pub const RESET_PARSER_FETCH: u32 = 72;
pub const RESET_PARSER_CTL: u32 = 73;
pub const RESET_PARSER_TOP: u32 = 74;
/*\t\t\t\t\t75\t*/
/*\t\t\t\t\t76\t*/
pub const RESET_AO_CPU_RESET: u32 = 77;
pub const RESET_MALI: u32 = 78;
pub const RESET_HDMI_SYSTEM_RESET: u32 = 79;
/*\t\t\t\t\t80-95\t*/
/*\tRESET3\t\t\t\t*/
pub const RESET_RING_OSCILLATOR: u32 = 96;
pub const RESET_SYS_CPU: u32 = 97;
pub const RESET_EFUSE: u32 = 98;
pub const RESET_SYS_CPU_BVCI: u32 = 99;
pub const RESET_AIFIFO: u32 = 100;
pub const RESET_TVFE: u32 = 101;
pub const RESET_AHB_BRIDGE_CNTL: u32 = 102;
/*\t\t\t\t\t103\t*/
pub const RESET_AUDIO_DAC: u32 = 104;
pub const RESET_DEMUX_TOP: u32 = 105;
pub const RESET_DEMUX_DES: u32 = 106;
pub const RESET_DEMUX_S2P_0: u32 = 107;
pub const RESET_DEMUX_S2P_1: u32 = 108;
pub const RESET_DEMUX_RESET_0: u32 = 109;
pub const RESET_DEMUX_RESET_1: u32 = 110;
pub const RESET_DEMUX_RESET_2: u32 = 111;
/*\t\t\t\t\t112-127\t*/
/*\tRESET4\t\t\t\t*/
/*\t\t\t\t\t128\t*/
/*\t\t\t\t\t129\t*/
/*\t\t\t\t\t130\t*/
/*\t\t\t\t\t131\t*/
pub const RESET_DVIN_RESET: u32 = 132;
pub const RESET_RDMA: u32 = 133;
pub const RESET_VENCI: u32 = 134;
pub const RESET_VENCP: u32 = 135;
/*\t\t\t\t\t136\t*/
pub const RESET_VDAC: u32 = 137;
pub const RESET_RTC: u32 = 138;
/*\t\t\t\t\t139\t*/
pub const RESET_VDI6: u32 = 140;
pub const RESET_VENCL: u32 = 141;
pub const RESET_I2C_MASTER_2: u32 = 142;
pub const RESET_I2C_MASTER_1: u32 = 143;
/*\t\t\t\t\t144-159\t*/
/*\tRESET5\t\t\t\t*/
/*\t\t\t\t\t160-191\t*/
/*\tRESET6\t\t\t\t*/
pub const RESET_PERIPHS_GENERAL: u32 = 192;
pub const RESET_PERIPHS_SPICC: u32 = 193;
pub const RESET_PERIPHS_SMART_CARD: u32 = 194;
pub const RESET_PERIPHS_SAR_ADC: u32 = 195;
pub const RESET_PERIPHS_I2C_MASTER_0: u32 = 196;
pub const RESET_SANA: u32 = 197;
/*\t\t\t\t\t198\t*/
pub const RESET_PERIPHS_STREAM_INTERFACE: u32 = 199;
pub const RESET_PERIPHS_SDIO: u32 = 200;
pub const RESET_PERIPHS_UART_0: u32 = 201;
pub const RESET_PERIPHS_UART_1_2: u32 = 202;
pub const RESET_PERIPHS_ASYNC_0: u32 = 203;
pub const RESET_PERIPHS_ASYNC_1: u32 = 204;
pub const RESET_PERIPHS_SPI_0: u32 = 205;
pub const RESET_PERIPHS_SDHC: u32 = 206;
pub const RESET_UART_SLIP: u32 = 207;
/*\t\t\t\t\t208-223\t*/
/*\tRESET7\t\t\t\t*/
pub const RESET_USB_DDR_0: u32 = 224;
pub const RESET_USB_DDR_1: u32 = 225;
pub const RESET_USB_DDR_2: u32 = 226;
pub const RESET_USB_DDR_3: u32 = 227;
/*\t\t\t\t\t228\t*/
pub const RESET_DEVICE_MMC_ARB: u32 = 229;
/*\t\t\t\t\t230\t*/
pub const RESET_VID_LOCK: u32 = 231;
pub const RESET_A9_DMC_PIPEL: u32 = 232;
/*\t\t\t\t\t233-255\t*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
