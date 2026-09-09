/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OMAP GPIO handling defines and functions
 *
 * Copyright (C) 2003-2005 Nokia Corporation
 *
 * Written by Juha Yrjölä <juha.yrjola@nokia.com>
 */

// C dependencies: linux/io.h and linux/platform_device.h.

pub const OMAP1_MPUIO_BASE: u32 = 0xfffb5000;

/* These are the omap15xx/16xx offsets. The omap7xx offsets are
 * OMAP_MPUIO_ / 2 offsets below. */
pub const OMAP_MPUIO_INPUT_LATCH: u32 = 0x00;
pub const OMAP_MPUIO_OUTPUT: u32 = 0x04;
pub const OMAP_MPUIO_IO_CNTL: u32 = 0x08;
pub const OMAP_MPUIO_KBR_LATCH: u32 = 0x10;
pub const OMAP_MPUIO_KBC: u32 = 0x14;
pub const OMAP_MPUIO_GPIO_EVENT_MODE: u32 = 0x18;
pub const OMAP_MPUIO_GPIO_INT_EDGE: u32 = 0x1c;
pub const OMAP_MPUIO_KBD_INT: u32 = 0x20;
pub const OMAP_MPUIO_GPIO_INT: u32 = 0x24;
pub const OMAP_MPUIO_KBD_MASKIT: u32 = 0x28;
pub const OMAP_MPUIO_GPIO_MASKIT: u32 = 0x2c;
pub const OMAP_MPUIO_GPIO_DEBOUNCING: u32 = 0x30;
pub const OMAP_MPUIO_LATCH: u32 = 0x34;

pub const OMAP34XX_NR_GPIOS: u32 = 6;

pub const OMAP1510_GPIO_DATA_INPUT: u32 = 0x00;
pub const OMAP1510_GPIO_DATA_OUTPUT: u32 = 0x04;
pub const OMAP1510_GPIO_DIR_CONTROL: u32 = 0x08;
pub const OMAP1510_GPIO_INT_CONTROL: u32 = 0x0c;
pub const OMAP1510_GPIO_INT_MASK: u32 = 0x10;
pub const OMAP1510_GPIO_INT_STATUS: u32 = 0x14;
pub const OMAP1510_GPIO_PIN_CONTROL: u32 = 0x18;
pub const OMAP1510_IH_GPIO_BASE: u32 = 64;

pub const OMAP1610_GPIO_REVISION: u32 = 0x0000;
pub const OMAP1610_GPIO_SYSCONFIG: u32 = 0x0010;
pub const OMAP1610_GPIO_SYSSTATUS: u32 = 0x0014;
pub const OMAP1610_GPIO_IRQSTATUS1: u32 = 0x0018;
pub const OMAP1610_GPIO_IRQENABLE1: u32 = 0x001c;
pub const OMAP1610_GPIO_WAKEUPENABLE: u32 = 0x0028;
pub const OMAP1610_GPIO_DATAIN: u32 = 0x002c;
pub const OMAP1610_GPIO_DATAOUT: u32 = 0x0030;
pub const OMAP1610_GPIO_DIRECTION: u32 = 0x0034;
pub const OMAP1610_GPIO_EDGE_CTRL1: u32 = 0x0038;
pub const OMAP1610_GPIO_EDGE_CTRL2: u32 = 0x003c;
pub const OMAP1610_GPIO_CLEAR_IRQENABLE1: u32 = 0x009c;
pub const OMAP1610_GPIO_CLEAR_WAKEUPENA: u32 = 0x00a8;
pub const OMAP1610_GPIO_CLEAR_DATAOUT: u32 = 0x00b0;
pub const OMAP1610_GPIO_SET_IRQENABLE1: u32 = 0x00dc;
pub const OMAP1610_GPIO_SET_WAKEUPENA: u32 = 0x00e8;
pub const OMAP1610_GPIO_SET_DATAOUT: u32 = 0x00f0;

pub const OMAP7XX_GPIO_DATA_INPUT: u32 = 0x00;
pub const OMAP7XX_GPIO_DATA_OUTPUT: u32 = 0x04;
pub const OMAP7XX_GPIO_DIR_CONTROL: u32 = 0x08;
pub const OMAP7XX_GPIO_INT_CONTROL: u32 = 0x0c;
pub const OMAP7XX_GPIO_INT_MASK: u32 = 0x10;
pub const OMAP7XX_GPIO_INT_STATUS: u32 = 0x14;

pub const OMAP24XX_GPIO_REVISION: u32 = 0x0000;
pub const OMAP24XX_GPIO_SYSCONFIG: u32 = 0x0010;
pub const OMAP24XX_GPIO_IRQSTATUS1: u32 = 0x0018;
pub const OMAP24XX_GPIO_IRQSTATUS2: u32 = 0x0028;
pub const OMAP24XX_GPIO_IRQENABLE2: u32 = 0x002c;
pub const OMAP24XX_GPIO_IRQENABLE1: u32 = 0x001c;
pub const OMAP24XX_GPIO_WAKE_EN: u32 = 0x0020;
pub const OMAP24XX_GPIO_CTRL: u32 = 0x0030;
pub const OMAP24XX_GPIO_OE: u32 = 0x0034;
pub const OMAP24XX_GPIO_DATAIN: u32 = 0x0038;
pub const OMAP24XX_GPIO_DATAOUT: u32 = 0x003c;
pub const OMAP24XX_GPIO_LEVELDETECT0: u32 = 0x0040;
pub const OMAP24XX_GPIO_LEVELDETECT1: u32 = 0x0044;
pub const OMAP24XX_GPIO_RISINGDETECT: u32 = 0x0048;
pub const OMAP24XX_GPIO_FALLINGDETECT: u32 = 0x004c;
pub const OMAP24XX_GPIO_DEBOUNCE_EN: u32 = 0x0050;
pub const OMAP24XX_GPIO_DEBOUNCE_VAL: u32 = 0x0054;
pub const OMAP24XX_GPIO_CLEARIRQENABLE1: u32 = 0x0060;
pub const OMAP24XX_GPIO_SETIRQENABLE1: u32 = 0x0064;
pub const OMAP24XX_GPIO_CLEARWKUENA: u32 = 0x0080;
pub const OMAP24XX_GPIO_SETWKUENA: u32 = 0x0084;
pub const OMAP24XX_GPIO_CLEARDATAOUT: u32 = 0x0090;
pub const OMAP24XX_GPIO_SETDATAOUT: u32 = 0x0094;

pub const OMAP4_GPIO_REVISION: u32 = 0x0000;
pub const OMAP4_GPIO_SYSCONFIG: u32 = 0x0010;
pub const OMAP4_GPIO_EOI: u32 = 0x0020;
pub const OMAP4_GPIO_IRQSTATUSRAW0: u32 = 0x0024;
pub const OMAP4_GPIO_IRQSTATUSRAW1: u32 = 0x0028;
pub const OMAP4_GPIO_IRQSTATUS0: u32 = 0x002c;
pub const OMAP4_GPIO_IRQSTATUS1: u32 = 0x0030;
pub const OMAP4_GPIO_IRQSTATUSSET0: u32 = 0x0034;
pub const OMAP4_GPIO_IRQSTATUSSET1: u32 = 0x0038;
pub const OMAP4_GPIO_IRQSTATUSCLR0: u32 = 0x003c;
pub const OMAP4_GPIO_IRQSTATUSCLR1: u32 = 0x0040;
pub const OMAP4_GPIO_IRQWAKEN0: u32 = 0x0044;
pub const OMAP4_GPIO_IRQWAKEN1: u32 = 0x0048;
pub const OMAP4_GPIO_IRQENABLE1: u32 = 0x011c;
pub const OMAP4_GPIO_WAKE_EN: u32 = 0x0120;
pub const OMAP4_GPIO_IRQSTATUS2: u32 = 0x0128;
pub const OMAP4_GPIO_IRQENABLE2: u32 = 0x012c;
pub const OMAP4_GPIO_CTRL: u32 = 0x0130;
pub const OMAP4_GPIO_OE: u32 = 0x0134;
pub const OMAP4_GPIO_DATAIN: u32 = 0x0138;
pub const OMAP4_GPIO_DATAOUT: u32 = 0x013c;
pub const OMAP4_GPIO_LEVELDETECT0: u32 = 0x0140;
pub const OMAP4_GPIO_LEVELDETECT1: u32 = 0x0144;
pub const OMAP4_GPIO_RISINGDETECT: u32 = 0x0148;
pub const OMAP4_GPIO_FALLINGDETECT: u32 = 0x014c;
pub const OMAP4_GPIO_DEBOUNCENABLE: u32 = 0x0150;
pub const OMAP4_GPIO_DEBOUNCINGTIME: u32 = 0x0154;
pub const OMAP4_GPIO_CLEARIRQENABLE1: u32 = 0x0160;
pub const OMAP4_GPIO_SETIRQENABLE1: u32 = 0x0164;
pub const OMAP4_GPIO_CLEARWKUENA: u32 = 0x0180;
pub const OMAP4_GPIO_SETWKUENA: u32 = 0x0184;
pub const OMAP4_GPIO_CLEARDATAOUT: u32 = 0x0190;
pub const OMAP4_GPIO_SETDATAOUT: u32 = 0x0194;

pub const OMAP_MAX_GPIO_LINES: u32 = 192;

#[repr(C)]
pub struct omap_gpio_reg_offs {
    pub revision: u16,
    pub sysconfig: u16,
    pub direction: u16,
    pub datain: u16,
    pub dataout: u16,
    pub set_dataout: u16,
    pub clr_dataout: u16,
    pub irqstatus: u16,
    pub irqstatus2: u16,
    pub irqstatus_raw0: u16,
    pub irqstatus_raw1: u16,
    pub irqenable: u16,
    pub irqenable2: u16,
    pub set_irqenable: u16,
    pub clr_irqenable: u16,
    pub debounce: u16,
    pub debounce_en: u16,
    pub ctrl: u16,
    pub wkup_en: u16,
    pub leveldetect0: u16,
    pub leveldetect1: u16,
    pub risingdetect: u16,
    pub fallingdetect: u16,
    pub irqctrl: u16,
    pub edgectrl1: u16,
    pub edgectrl2: u16,
    pub pinctrl: u16,
    pub irqenable_inv: bool,
}

#[repr(C)]
pub struct omap_gpio_platform_data {
    pub bank_type: i32,
    pub bank_width: i32, // GPIO bank width
    pub bank_stride: i32, // Only needed for omap1 MPUIO
    pub dbck_flag: bool, // dbck required or not - True for OMAP3&4
    pub loses_context: bool, // whether the bank would ever lose context
    pub is_mpuio: bool, // whether the bank is of type MPUIO
    pub non_wakeup_gpios: u32,
    pub regs: *const omap_gpio_reg_offs,
    // Return context loss count due to PM states changing
    pub get_context_loss_count: Option<unsafe extern "C" fn(dev: *mut device) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
