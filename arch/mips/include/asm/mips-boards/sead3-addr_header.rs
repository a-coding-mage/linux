/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2015 Imagination Technologies, Inc.
 *   written by Ralf Baechle <ralf@linux-mips.org>
 */

/*
 * Target #0 Register Decode
 */
pub const SEAD3_SD_SPDCNF: u32 = 0xbb000040;
pub const SEAD3_SD_SPADDR: u32 = 0xbb000048;
pub const SEAD3_SD_DATA: u32 = 0xbb000050;

/*
 * Target #1 Register Decode
 */
pub const SEAD3_CFG: u32 = 0xbb100110;
pub const SEAD3_GIC_BASE_ADDRESS: u32 = 0xbb1c0000;
pub const SEAD3_SHARED_SECTION: u32 = 0xbb1c0000;
pub const SEAD3_VPE_LOCAL_SECTION: u32 = 0xbb1c8000;
pub const SEAD3_VPE_OTHER_SECTION: u32 = 0xbb1cc000;
pub const SEAD3_USER_MODE_VISIBLE_SECTION: u32 = 0xbb1d0000;

/*
 * Target #3 Register Decode
 */
pub const SEAD3_USB_HS_BASE: u32 = 0xbb200000;
pub const SEAD3_USB_HS_IDENTIFICATION_REGS: u32 = 0xbb200000;
pub const SEAD3_USB_HS_CAPABILITY_REGS: u32 = 0xbb200100;
pub const SEAD3_USB_HS_OPERATIONAL_REGS: u32 = 0xbb200140;
pub const SEAD3_RESERVED: u32 = 0xbe800000;

/*
 * Target #3 Register Decode
 */
pub const SEAD3_SRAM: u32 = 0xbe000000;
pub const SEAD3_OPTIONAL_SRAM: u32 = 0xbe400000;
pub const SEAD3_FPGA: u32 = 0xbf000000;

pub const SEAD3_PI_PIC32_USB_STATUS: u32 = 0xbf000060;
pub const SEAD3_PI_PIC32_USB_STATUS_IO_RDY: u32 = 1 << 0;
pub const SEAD3_PI_PIC32_USB_STATUS_SPL_INT: u32 = 1 << 1;
pub const SEAD3_PI_PIC32_USB_STATUS_GPIOA_INT: u32 = 1 << 2;
pub const SEAD3_PI_PIC32_USB_STATUS_GPIOB_INT: u32 = 1 << 3;

pub const SEAD3_PI_SOFT_ENDIAN: u32 = 0xbf000070;

pub const SEAD3_CPLD_P_SWITCH: u32 = 0xbf000200;
pub const SEAD3_CPLD_F_SWITCH: u32 = 0xbf000208;
pub const SEAD3_CPLD_P_LED: u32 = 0xbf000210;
pub const SEAD3_CPLD_F_LED: u32 = 0xbf000218;
pub const SEAD3_NEWSC_LIVE: u32 = 0xbf000220;
pub const SEAD3_NEWSC_REG: u32 = 0xbf000228;
pub const SEAD3_NEWSC_CTRL: u32 = 0xbf000230;

pub const SEAD3_LCD_CONTROL: u32 = 0xbf000400;
pub const SEAD3_LCD_DATA: u32 = 0xbf000408;
pub const SEAD3_CPLD_LCD_STATUS: u32 = 0xbf000410;
pub const SEAD3_CPLD_LCD_DATA: u32 = 0xbf000418;

pub const SEAD3_CPLD_PI_DEVRST: u32 = 0xbf000480;
pub const SEAD3_CPLD_PI_DEVRST_IC32_RST: u32 = 1 << 0;
pub const SEAD3_RESERVED_0: u32 = 0xbf000500;

pub const SEAD3_PIC32_REGISTERS: u32 = 0xbf000600;
pub const SEAD3_RESERVED_1: u32 = 0xbf000700;
pub const SEAD3_UART_CH_0: u32 = 0xbf000800;
pub const SEAD3_UART_CH_1: u32 = 0xbf000900;
pub const SEAD3_RESERVED_2: u32 = 0xbf000a00;
pub const SEAD3_ETHERNET: u32 = 0xbf010000;
pub const SEAD3_RESERVED_3: u32 = 0xbf020000;
pub const SEAD3_USER_EXPANSION: u32 = 0xbf400000;
pub const SEAD3_RESERVED_4: u32 = 0xbf800000;
pub const SEAD3_BOOT_FLASH_EXTENSION: u32 = 0xbfa00000;
pub const SEAD3_BOOT_FLASH: u32 = 0xbfc00000;
pub const SEAD3_REVISION_REGISTER: u32 = 0xbfc00010;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
