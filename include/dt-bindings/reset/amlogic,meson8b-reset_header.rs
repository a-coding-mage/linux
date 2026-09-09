/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/*
 * Copyright (c) 2016 BayLibre, SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 */

/*\tRESET0\t\t\t\t\t*/
pub const RESET_HIU: u32 = 0;
pub const RESET_VLD: u32 = 1;
pub const RESET_IQIDCT: u32 = 2;
pub const RESET_MC: u32 = 3;
/*\t\t\t\t\t8\t*/
pub const RESET_VIU: u32 = 5;
pub const RESET_AIU: u32 = 6;
pub const RESET_MCPU: u32 = 7;
pub const RESET_CCPU: u32 = 8;
pub const RESET_PMUX: u32 = 9;
pub const RESET_VENC: u32 = 10;
pub const RESET_ASSIST: u32 = 11;
pub const RESET_AFIFO2: u32 = 12;
pub const RESET_MDEC: u32 = 13;
pub const RESET_VLD_PART: u32 = 14;
pub const RESET_VIFIFO: u32 = 15;
/*\t\t\t\t\t16-31\t*/
/*\tRESET1\t\t\t\t\t*/
/*\t\t\t\t\t32\t*/
pub const RESET_DEMUX: u32 = 33;
pub const RESET_USB_OTG: u32 = 34;
pub const RESET_DDR: u32 = 35;
pub const RESET_VDAC_1: u32 = 36;
pub const RESET_BT656: u32 = 37;
pub const RESET_AHB_SRAM: u32 = 38;
pub const RESET_AHB_BRIDGE: u32 = 39;
pub const RESET_PARSER: u32 = 40;
pub const RESET_BLKMV: u32 = 41;
pub const RESET_ISA: u32 = 42;
pub const RESET_ETHERNET: u32 = 43;
pub const RESET_ABUF: u32 = 44;
pub const RESET_AHB_DATA: u32 = 45;
pub const RESET_AHB_CNTL: u32 = 46;
pub const RESET_ROM_BOOT: u32 = 47;
/*\t\t\t\t\t48-63\t*/
/*\tRESET2\t\t\t\t\t*/
pub const RESET_VD_RMEM: u32 = 64;
pub const RESET_AUDIN: u32 = 65;
pub const RESET_DBLK: u32 = 66;
pub const RESET_PIC_DC: u32 = 67;
pub const RESET_PSC: u32 = 68;
pub const RESET_NAND: u32 = 69;
pub const RESET_GE2D: u32 = 70;
pub const RESET_PARSER_REG: u32 = 71;
pub const RESET_PARSER_FETCH: u32 = 72;
pub const RESET_PARSER_CTL: u32 = 73;
pub const RESET_PARSER_TOP: u32 = 74;
pub const RESET_HDMI_APB: u32 = 75;
pub const RESET_AUDIO_APB: u32 = 76;
pub const RESET_MEDIA_CPU: u32 = 77;
pub const RESET_MALI: u32 = 78;
pub const RESET_HDMI_SYSTEM_RESET: u32 = 79;
/*\t\t\t\t\t80-95\t*/
/*\tRESET3\t\t\t\t\t*/
pub const RESET_RING_OSCILLATOR: u32 = 96;
pub const RESET_SYS_CPU_0: u32 = 97;
pub const RESET_EFUSE: u32 = 98;
pub const RESET_SYS_CPU_BVCI: u32 = 99;
pub const RESET_AIFIFO: u32 = 100;
pub const RESET_AUDIO_PLL_MODULATOR: u32 = 101;
pub const RESET_AHB_BRIDGE_CNTL: u32 = 102;
pub const RESET_SYS_CPU_1: u32 = 103;
pub const RESET_AUDIO_DAC: u32 = 104;
pub const RESET_DEMUX_TOP: u32 = 105;
pub const RESET_DEMUX_DES: u32 = 106;
pub const RESET_DEMUX_S2P_0: u32 = 107;
pub const RESET_DEMUX_S2P_1: u32 = 108;
pub const RESET_DEMUX_RESET_0: u32 = 109;
pub const RESET_DEMUX_RESET_1: u32 = 110;
pub const RESET_DEMUX_RESET_2: u32 = 111;
/*\t\t\t\t\t112-127\t*/
/*\tRESET4\t\t\t\t\t*/
pub const RESET_PL310: u32 = 128;
pub const RESET_A5_APB: u32 = 129;
pub const RESET_A5_AXI: u32 = 130;
pub const RESET_A5: u32 = 131;
pub const RESET_DVIN: u32 = 132;
pub const RESET_RDMA: u32 = 133;
pub const RESET_VENCI: u32 = 134;
pub const RESET_VENCP: u32 = 135;
pub const RESET_VENCT: u32 = 136;
pub const RESET_VDAC_4: u32 = 137;
pub const RESET_RTC: u32 = 138;
pub const RESET_A5_DEBUG: u32 = 139;
pub const RESET_VDI6: u32 = 140;
pub const RESET_VENCL: u32 = 141;
/*\t\t\t\t\t142-159\t*/
/*\tRESET5\t\t\t\t\t*/
pub const RESET_DDR_PLL: u32 = 160;
pub const RESET_MISC_PLL: u32 = 161;
pub const RESET_SYS_PLL: u32 = 162;
pub const RESET_HPLL_PLL: u32 = 163;
pub const RESET_AUDIO_PLL: u32 = 164;
pub const RESET_VID2_PLL: u32 = 165;
/*\t\t\t\t\t166-191\t*/
/*\tRESET6\t\t\t\t\t*/
pub const RESET_PERIPHS_GENERAL: u32 = 192;
pub const RESET_PERIPHS_IR_REMOTE: u32 = 193;
pub const RESET_PERIPHS_SMART_CARD: u32 = 194;
pub const RESET_PERIPHS_SAR_ADC: u32 = 195;
pub const RESET_PERIPHS_I2C_MASTER_0: u32 = 196;
pub const RESET_PERIPHS_I2C_MASTER_1: u32 = 197;
pub const RESET_PERIPHS_I2C_SLAVE: u32 = 198;
pub const RESET_PERIPHS_STREAM_INTERFACE: u32 = 199;
pub const RESET_PERIPHS_SDIO: u32 = 200;
pub const RESET_PERIPHS_UART_0: u32 = 201;
pub const RESET_PERIPHS_UART_1: u32 = 202;
pub const RESET_PERIPHS_ASYNC_0: u32 = 203;
pub const RESET_PERIPHS_ASYNC_1: u32 = 204;
pub const RESET_PERIPHS_SPI_0: u32 = 205;
pub const RESET_PERIPHS_SPI_1: u32 = 206;
pub const RESET_PERIPHS_LED_PWM: u32 = 207;
/*\t\t\t\t\t208-223\t*/
/*\tRESET7\t\t\t\t\t*/
/*\t\t\t\t\t224-255\t*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
