/* SPDX-License-Identifier: GPL-2.0-only */
/* sm501-regs.h
 *
 * Copyright 2006 Simtec Electronics
 *
 * Silicon Motion SM501 register definitions
*/

/* System Configuration area */
/* System config base */
pub const SM501_SYS_CONFIG: u32 = (0x000000);

/* config 1 */
pub const SM501_SYSTEM_CONTROL: u32 = (0x000000);

pub const SM501_SYSCTRL_PANEL_TRISTATE: u32 = (1<<0);
pub const SM501_SYSCTRL_MEM_TRISTATE: u32 = (1<<1);
pub const SM501_SYSCTRL_CRT_TRISTATE: u32 = (1<<2);

pub const SM501_SYSCTRL_PCI_SLAVE_BURST_MASK: u32 = (3<<4);
pub const SM501_SYSCTRL_PCI_SLAVE_BURST_1: u32 = (0<<4);
pub const SM501_SYSCTRL_PCI_SLAVE_BURST_2: u32 = (1<<4);
pub const SM501_SYSCTRL_PCI_SLAVE_BURST_4: u32 = (2<<4);
pub const SM501_SYSCTRL_PCI_SLAVE_BURST_8: u32 = (3<<4);

pub const SM501_SYSCTRL_PCI_CLOCK_RUN_EN: u32 = (1<<6);
pub const SM501_SYSCTRL_PCI_RETRY_DISABLE: u32 = (1<<7);
pub const SM501_SYSCTRL_PCI_SUBSYS_LOCK: u32 = (1<<11);
pub const SM501_SYSCTRL_PCI_BURST_READ_EN: u32 = (1<<15);

pub const SM501_SYSCTRL_2D_ENGINE_STATUS: u32 = (1<<19);

/* miscellaneous control */

pub const SM501_MISC_CONTROL: u32 = (0x000004);

pub const SM501_MISC_BUS_SH: u32 = (0x0);
pub const SM501_MISC_BUS_PCI: u32 = (0x1);
pub const SM501_MISC_BUS_XSCALE: u32 = (0x2);
pub const SM501_MISC_BUS_NEC: u32 = (0x6);
pub const SM501_MISC_BUS_MASK: u32 = (0x7);

pub const SM501_MISC_VR_62MB: u32 = (1<<3);
pub const SM501_MISC_CDR_RESET: u32 = (1<<7);
pub const SM501_MISC_USB_LB: u32 = (1<<8);
pub const SM501_MISC_USB_SLAVE: u32 = (1<<9);
pub const SM501_MISC_BL_1: u32 = (1<<10);
pub const SM501_MISC_MC: u32 = (1<<11);
pub const SM501_MISC_DAC_POWER: u32 = (1<<12);
pub const SM501_MISC_IRQ_INVERT: u32 = (1<<16);
pub const SM501_MISC_SH: u32 = (1<<17);

pub const SM501_MISC_HOLD_EMPTY: u32 = (0<<18);
pub const SM501_MISC_HOLD_8: u32 = (1<<18);
pub const SM501_MISC_HOLD_16: u32 = (2<<18);
pub const SM501_MISC_HOLD_24: u32 = (3<<18);
pub const SM501_MISC_HOLD_32: u32 = (4<<18);
pub const SM501_MISC_HOLD_MASK: u32 = (7<<18);

pub const SM501_MISC_FREQ_12: u32 = (1<<24);
pub const SM501_MISC_PNL_24BIT: u32 = (1<<25);
pub const SM501_MISC_8051_LE: u32 = (1<<26);



pub const SM501_GPIO31_0_CONTROL: u32 = (0x000008);
pub const SM501_GPIO63_32_CONTROL: u32 = (0x00000C);
pub const SM501_DRAM_CONTROL: u32 = (0x000010);

/* command list */
pub const SM501_ARBTRTN_CONTROL: u32 = (0x000014);

/* command list */
pub const SM501_COMMAND_LIST_STATUS: u32 = (0x000024);

/* interrupt debug */
pub const SM501_RAW_IRQ_STATUS: u32 = (0x000028);
pub const SM501_RAW_IRQ_CLEAR: u32 = (0x000028);
pub const SM501_IRQ_STATUS: u32 = (0x00002C);
pub const SM501_IRQ_MASK: u32 = (0x000030);
pub const SM501_DEBUG_CONTROL: u32 = (0x000034);

/* power management */
pub const SM501_POWERMODE_P2X_SRC: u32 = (1<<29);
pub const SM501_POWERMODE_V2X_SRC: u32 = (1<<20);
pub const SM501_POWERMODE_M_SRC: u32 = (1<<12);
pub const SM501_POWERMODE_M1_SRC: u32 = (1<<4);

pub const SM501_CURRENT_GATE: u32 = (0x000038);
pub const SM501_CURRENT_CLOCK: u32 = (0x00003C);
pub const SM501_POWER_MODE_0_GATE: u32 = (0x000040);
pub const SM501_POWER_MODE_0_CLOCK: u32 = (0x000044);
pub const SM501_POWER_MODE_1_GATE: u32 = (0x000048);
pub const SM501_POWER_MODE_1_CLOCK: u32 = (0x00004C);
pub const SM501_SLEEP_MODE_GATE: u32 = (0x000050);
pub const SM501_POWER_MODE_CONTROL: u32 = (0x000054);

/* power gates for units within the 501 */
pub const SM501_GATE_HOST: u32 = (0);
pub const SM501_GATE_MEMORY: u32 = (1);
pub const SM501_GATE_DISPLAY: u32 = (2);
pub const SM501_GATE_2D_ENGINE: u32 = (3);
pub const SM501_GATE_CSC: u32 = (4);
pub const SM501_GATE_ZVPORT: u32 = (5);
pub const SM501_GATE_GPIO: u32 = (6);
pub const SM501_GATE_UART0: u32 = (7);
pub const SM501_GATE_UART1: u32 = (8);
pub const SM501_GATE_SSP: u32 = (10);
pub const SM501_GATE_USB_HOST: u32 = (11);
pub const SM501_GATE_USB_GADGET: u32 = (12);
pub const SM501_GATE_UCONTROLLER: u32 = (17);
pub const SM501_GATE_AC97: u32 = (18);

/* panel clock */
pub const SM501_CLOCK_P2XCLK: u32 = (24);
/* crt clock */
pub const SM501_CLOCK_V2XCLK: u32 = (16);
/* main clock */
pub const SM501_CLOCK_MCLK: u32 = (8);
/* SDRAM controller clock */
pub const SM501_CLOCK_M1XCLK: u32 = (0);

/* config 2 */
pub const SM501_PCI_MASTER_BASE: u32 = (0x000058);
pub const SM501_ENDIAN_CONTROL: u32 = (0x00005C);
pub const SM501_DEVICEID: u32 = (0x000060);
/* 0x050100A0 */

pub const SM501_DEVICEID_SM501: u32 = (0x05010000);
pub const SM501_DEVICEID_IDMASK: u32 = (0xffff0000);
pub const SM501_DEVICEID_REVMASK: u32 = (0x000000ff);

pub const SM501_PLLCLOCK_COUNT: u32 = (0x000064);
pub const SM501_MISC_TIMING: u32 = (0x000068);
pub const SM501_CURRENT_SDRAM_CLOCK: u32 = (0x00006C);

pub const SM501_PROGRAMMABLE_PLL_CONTROL: u32 = (0x000074);

/* GPIO base */
pub const SM501_GPIO: u32 = (0x010000);
pub const SM501_GPIO_DATA_LOW: u32 = (0x00);
pub const SM501_GPIO_DATA_HIGH: u32 = (0x04);
pub const SM501_GPIO_DDR_LOW: u32 = (0x08);
pub const SM501_GPIO_DDR_HIGH: u32 = (0x0C);
pub const SM501_GPIO_IRQ_SETUP: u32 = (0x10);
pub const SM501_GPIO_IRQ_STATUS: u32 = (0x14);
pub const SM501_GPIO_IRQ_RESET: u32 = (0x14);

/* I2C controller base */
pub const SM501_I2C: u32 = (0x010040);
pub const SM501_I2C_BYTE_COUNT: u32 = (0x00);
pub const SM501_I2C_CONTROL: u32 = (0x01);
pub const SM501_I2C_STATUS: u32 = (0x02);
pub const SM501_I2C_RESET: u32 = (0x02);
pub const SM501_I2C_SLAVE_ADDRESS: u32 = (0x03);
pub const SM501_I2C_DATA: u32 = (0x04);

/* SSP base */
pub const SM501_SSP: u32 = (0x020000);

/* Uart 0 base */
pub const SM501_UART0: u32 = (0x030000);

/* Uart 1 base */
pub const SM501_UART1: u32 = (0x030020);

/* USB host port base */
pub const SM501_USB_HOST: u32 = (0x040000);

/* USB slave/gadget base */
pub const SM501_USB_GADGET: u32 = (0x060000);

/* USB slave/gadget data port base */
pub const SM501_USB_GADGET_DATA: u32 = (0x070000);

/* Display controller/video engine base */
pub const SM501_DC: u32 = (0x080000);

/* common defines for the SM501 address registers */
pub const SM501_ADDR_FLIP: u32 = (1<<31);
pub const SM501_ADDR_EXT: u32 = (1<<27);
pub const SM501_ADDR_CS1: u32 = (1<<26);
pub const SM501_ADDR_MASK: u32 = (0x3f << 26);

pub const SM501_FIFO_MASK: u32 = (0x3 << 16);
pub const SM501_FIFO_1: u32 = (0x0 << 16);
pub const SM501_FIFO_3: u32 = (0x1 << 16);
pub const SM501_FIFO_7: u32 = (0x2 << 16);
pub const SM501_FIFO_11: u32 = (0x3 << 16);

/* common registers for panel and the crt */
pub const SM501_OFF_DC_H_TOT: u32 = (0x000);
pub const SM501_OFF_DC_V_TOT: u32 = (0x008);
pub const SM501_OFF_DC_H_SYNC: u32 = (0x004);
pub const SM501_OFF_DC_V_SYNC: u32 = (0x00C);

pub const SM501_DC_PANEL_CONTROL: u32 = (0x000);

pub const SM501_DC_PANEL_CONTROL_FPEN: u32 = (1<<27);
pub const SM501_DC_PANEL_CONTROL_BIAS: u32 = (1<<26);
pub const SM501_DC_PANEL_CONTROL_DATA: u32 = (1<<25);
pub const SM501_DC_PANEL_CONTROL_VDD: u32 = (1<<24);
pub const SM501_DC_PANEL_CONTROL_DP: u32 = (1<<23);

pub const SM501_DC_PANEL_CONTROL_TFT_888: u32 = (0<<21);
pub const SM501_DC_PANEL_CONTROL_TFT_333: u32 = (1<<21);
pub const SM501_DC_PANEL_CONTROL_TFT_444: u32 = (2<<21);

pub const SM501_DC_PANEL_CONTROL_DE: u32 = (1<<20);

pub const SM501_DC_PANEL_CONTROL_LCD_TFT: u32 = (0<<18);
pub const SM501_DC_PANEL_CONTROL_LCD_STN8: u32 = (1<<18);
pub const SM501_DC_PANEL_CONTROL_LCD_STN12: u32 = (2<<18);

pub const SM501_DC_PANEL_CONTROL_CP: u32 = (1<<14);
pub const SM501_DC_PANEL_CONTROL_VSP: u32 = (1<<13);
pub const SM501_DC_PANEL_CONTROL_HSP: u32 = (1<<12);
pub const SM501_DC_PANEL_CONTROL_CK: u32 = (1<<9);
pub const SM501_DC_PANEL_CONTROL_TE: u32 = (1<<8);
pub const SM501_DC_PANEL_CONTROL_VPD: u32 = (1<<7);
pub const SM501_DC_PANEL_CONTROL_VP: u32 = (1<<6);
pub const SM501_DC_PANEL_CONTROL_HPD: u32 = (1<<5);
pub const SM501_DC_PANEL_CONTROL_HP: u32 = (1<<4);
pub const SM501_DC_PANEL_CONTROL_GAMMA: u32 = (1<<3);
pub const SM501_DC_PANEL_CONTROL_EN: u32 = (1<<2);

pub const SM501_DC_PANEL_CONTROL_8BPP: u32 = (0<<0);
pub const SM501_DC_PANEL_CONTROL_16BPP: u32 = (1<<0);
pub const SM501_DC_PANEL_CONTROL_32BPP: u32 = (2<<0);


pub const SM501_DC_PANEL_PANNING_CONTROL: u32 = (0x004);
pub const SM501_DC_PANEL_COLOR_KEY: u32 = (0x008);
pub const SM501_DC_PANEL_FB_ADDR: u32 = (0x00C);
pub const SM501_DC_PANEL_FB_OFFSET: u32 = (0x010);
pub const SM501_DC_PANEL_FB_WIDTH: u32 = (0x014);
pub const SM501_DC_PANEL_FB_HEIGHT: u32 = (0x018);
pub const SM501_DC_PANEL_TL_LOC: u32 = (0x01C);
pub const SM501_DC_PANEL_BR_LOC: u32 = (0x020);
pub const SM501_DC_PANEL_H_TOT: u32 = (0x024);
pub const SM501_DC_PANEL_H_SYNC: u32 = (0x028);
pub const SM501_DC_PANEL_V_TOT: u32 = (0x02C);
pub const SM501_DC_PANEL_V_SYNC: u32 = (0x030);
pub const SM501_DC_PANEL_CUR_LINE: u32 = (0x034);

pub const SM501_DC_VIDEO_CONTROL: u32 = (0x040);
pub const SM501_DC_VIDEO_FB0_ADDR: u32 = (0x044);
pub const SM501_DC_VIDEO_FB_WIDTH: u32 = (0x048);
pub const SM501_DC_VIDEO_FB0_LAST_ADDR: u32 = (0x04C);
pub const SM501_DC_VIDEO_TL_LOC: u32 = (0x050);
pub const SM501_DC_VIDEO_BR_LOC: u32 = (0x054);
pub const SM501_DC_VIDEO_SCALE: u32 = (0x058);
pub const SM501_DC_VIDEO_INIT_SCALE: u32 = (0x05C);
pub const SM501_DC_VIDEO_YUV_CONSTANTS: u32 = (0x060);
pub const SM501_DC_VIDEO_FB1_ADDR: u32 = (0x064);
pub const SM501_DC_VIDEO_FB1_LAST_ADDR: u32 = (0x068);

pub const SM501_DC_VIDEO_ALPHA_CONTROL: u32 = (0x080);
pub const SM501_DC_VIDEO_ALPHA_FB_ADDR: u32 = (0x084);
pub const SM501_DC_VIDEO_ALPHA_FB_OFFSET: u32 = (0x088);
pub const SM501_DC_VIDEO_ALPHA_FB_LAST_ADDR: u32 = (0x08C);
pub const SM501_DC_VIDEO_ALPHA_TL_LOC: u32 = (0x090);
pub const SM501_DC_VIDEO_ALPHA_BR_LOC: u32 = (0x094);
pub const SM501_DC_VIDEO_ALPHA_SCALE: u32 = (0x098);
pub const SM501_DC_VIDEO_ALPHA_INIT_SCALE: u32 = (0x09C);
pub const SM501_DC_VIDEO_ALPHA_CHROMA_KEY: u32 = (0x0A0);
pub const SM501_DC_VIDEO_ALPHA_COLOR_LOOKUP: u32 = (0x0A4);

pub const SM501_DC_PANEL_HWC_BASE: u32 = (0x0F0);
pub const SM501_DC_PANEL_HWC_ADDR: u32 = (0x0F0);
pub const SM501_DC_PANEL_HWC_LOC: u32 = (0x0F4);
pub const SM501_DC_PANEL_HWC_COLOR_1_2: u32 = (0x0F8);
pub const SM501_DC_PANEL_HWC_COLOR_3: u32 = (0x0FC);

pub const SM501_HWC_EN: u32 = (1<<31);

pub const SM501_OFF_HWC_ADDR: u32 = (0x00);
pub const SM501_OFF_HWC_LOC: u32 = (0x04);
pub const SM501_OFF_HWC_COLOR_1_2: u32 = (0x08);
pub const SM501_OFF_HWC_COLOR_3: u32 = (0x0C);

pub const SM501_DC_ALPHA_CONTROL: u32 = (0x100);
pub const SM501_DC_ALPHA_FB_ADDR: u32 = (0x104);
pub const SM501_DC_ALPHA_FB_OFFSET: u32 = (0x108);
pub const SM501_DC_ALPHA_TL_LOC: u32 = (0x10C);
pub const SM501_DC_ALPHA_BR_LOC: u32 = (0x110);
pub const SM501_DC_ALPHA_CHROMA_KEY: u32 = (0x114);
pub const SM501_DC_ALPHA_COLOR_LOOKUP: u32 = (0x118);

pub const SM501_DC_CRT_CONTROL: u32 = (0x200);

pub const SM501_DC_CRT_CONTROL_TVP: u32 = (1<<15);
pub const SM501_DC_CRT_CONTROL_CP: u32 = (1<<14);
pub const SM501_DC_CRT_CONTROL_VSP: u32 = (1<<13);
pub const SM501_DC_CRT_CONTROL_HSP: u32 = (1<<12);
pub const SM501_DC_CRT_CONTROL_VS: u32 = (1<<11);
pub const SM501_DC_CRT_CONTROL_BLANK: u32 = (1<<10);
pub const SM501_DC_CRT_CONTROL_SEL: u32 = (1<<9);
pub const SM501_DC_CRT_CONTROL_TE: u32 = (1<<8);
pub const SM501_DC_CRT_CONTROL_PIXEL_MASK: u32 = (0xF << 4);
pub const SM501_DC_CRT_CONTROL_GAMMA: u32 = (1<<3);
pub const SM501_DC_CRT_CONTROL_ENABLE: u32 = (1<<2);

pub const SM501_DC_CRT_CONTROL_8BPP: u32 = (0<<0);
pub const SM501_DC_CRT_CONTROL_16BPP: u32 = (1<<0);
pub const SM501_DC_CRT_CONTROL_32BPP: u32 = (2<<0);

pub const SM501_DC_CRT_FB_ADDR: u32 = (0x204);
pub const SM501_DC_CRT_FB_OFFSET: u32 = (0x208);
pub const SM501_DC_CRT_H_TOT: u32 = (0x20C);
pub const SM501_DC_CRT_H_SYNC: u32 = (0x210);
pub const SM501_DC_CRT_V_TOT: u32 = (0x214);
pub const SM501_DC_CRT_V_SYNC: u32 = (0x218);
pub const SM501_DC_CRT_SIGNATURE_ANALYZER: u32 = (0x21C);
pub const SM501_DC_CRT_CUR_LINE: u32 = (0x220);
pub const SM501_DC_CRT_MONITOR_DETECT: u32 = (0x224);

pub const SM501_DC_CRT_HWC_BASE: u32 = (0x230);
pub const SM501_DC_CRT_HWC_ADDR: u32 = (0x230);
pub const SM501_DC_CRT_HWC_LOC: u32 = (0x234);
pub const SM501_DC_CRT_HWC_COLOR_1_2: u32 = (0x238);
pub const SM501_DC_CRT_HWC_COLOR_3: u32 = (0x23C);

pub const SM501_DC_PANEL_PALETTE: u32 = (0x400);

pub const SM501_DC_VIDEO_PALETTE: u32 = (0x800);

pub const SM501_DC_CRT_PALETTE: u32 = (0xC00);

/* Zoom Video port base */
pub const SM501_ZVPORT: u32 = (0x090000);

/* AC97/I2S base */
pub const SM501_AC97: u32 = (0x0A0000);

/* 8051 micro controller base */
pub const SM501_UCONTROLLER: u32 = (0x0B0000);

/* 8051 micro controller SRAM base */
pub const SM501_UCONTROLLER_SRAM: u32 = (0x0C0000);

/* DMA base */
pub const SM501_DMA: u32 = (0x0D0000);

/* 2d engine base */
pub const SM501_2D_ENGINE: u32 = (0x100000);
pub const SM501_2D_SOURCE: u32 = (0x00);
pub const SM501_2D_DESTINATION: u32 = (0x04);
pub const SM501_2D_DIMENSION: u32 = (0x08);
pub const SM501_2D_CONTROL: u32 = (0x0C);
pub const SM501_2D_PITCH: u32 = (0x10);
pub const SM501_2D_FOREGROUND: u32 = (0x14);
pub const SM501_2D_BACKGROUND: u32 = (0x18);
pub const SM501_2D_STRETCH: u32 = (0x1C);
pub const SM501_2D_COLOR_COMPARE: u32 = (0x20);
pub const SM501_2D_COLOR_COMPARE_MASK: u32 = (0x24);
pub const SM501_2D_MASK: u32 = (0x28);
pub const SM501_2D_CLIP_TL: u32 = (0x2C);
pub const SM501_2D_CLIP_BR: u32 = (0x30);
pub const SM501_2D_MONO_PATTERN_LOW: u32 = (0x34);
pub const SM501_2D_MONO_PATTERN_HIGH: u32 = (0x38);
pub const SM501_2D_WINDOW_WIDTH: u32 = (0x3C);
pub const SM501_2D_SOURCE_BASE: u32 = (0x40);
pub const SM501_2D_DESTINATION_BASE: u32 = (0x44);
pub const SM501_2D_ALPHA: u32 = (0x48);
pub const SM501_2D_WRAP: u32 = (0x4C);
pub const SM501_2D_STATUS: u32 = (0x50);

pub const SM501_CSC_Y_SOURCE_BASE: u32 = (0xC8);
pub const SM501_CSC_CONSTANTS: u32 = (0xCC);
pub const SM501_CSC_Y_SOURCE_X: u32 = (0xD0);
pub const SM501_CSC_Y_SOURCE_Y: u32 = (0xD4);
pub const SM501_CSC_U_SOURCE_BASE: u32 = (0xD8);
pub const SM501_CSC_V_SOURCE_BASE: u32 = (0xDC);
pub const SM501_CSC_SOURCE_DIMENSION: u32 = (0xE0);
pub const SM501_CSC_SOURCE_PITCH: u32 = (0xE4);
pub const SM501_CSC_DESTINATION: u32 = (0xE8);
pub const SM501_CSC_DESTINATION_DIMENSION: u32 = (0xEC);
pub const SM501_CSC_DESTINATION_PITCH: u32 = (0xF0);
pub const SM501_CSC_SCALE_FACTOR: u32 = (0xF4);
pub const SM501_CSC_DESTINATION_BASE: u32 = (0xF8);
pub const SM501_CSC_CONTROL: u32 = (0xFC);

/* 2d engine data port base */
pub const SM501_2D_ENGINE_DATA: u32 = (0x110000);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
