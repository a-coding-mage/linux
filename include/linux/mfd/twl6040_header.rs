/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MFD driver for twl6040
 *
 * Authors:     Jorge Eduardo Candelaria <jorge.candelaria@ti.com>
 *              Misael Lopez Cruz <misael.lopez@ti.com>
 *
 * Copyright:   (C) 2011 Texas Instruments, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.
pub struct device;
pub struct regmap;
pub struct regmap_irq_chip_data;
pub struct regulator_bulk_data;
pub struct clk;
pub struct mutex;
pub struct mfd_cell;
pub struct completion;
pub struct gpio_desc;

pub const TWL6040_REG_ASICID: u8 = 0x01;
pub const TWL6040_REG_ASICREV: u8 = 0x02;
pub const TWL6040_REG_INTID: u8 = 0x03;
pub const TWL6040_REG_INTMR: u8 = 0x04;
pub const TWL6040_REG_NCPCTL: u8 = 0x05;
pub const TWL6040_REG_LDOCTL: u8 = 0x06;
pub const TWL6040_REG_HPPLLCTL: u8 = 0x07;
pub const TWL6040_REG_LPPLLCTL: u8 = 0x08;
pub const TWL6040_REG_LPPLLDIV: u8 = 0x09;
pub const TWL6040_REG_AMICBCTL: u8 = 0x0A;
pub const TWL6040_REG_DMICBCTL: u8 = 0x0B;
pub const TWL6040_REG_MICLCTL: u8 = 0x0C;
pub const TWL6040_REG_MICRCTL: u8 = 0x0D;
pub const TWL6040_REG_MICGAIN: u8 = 0x0E;
pub const TWL6040_REG_LINEGAIN: u8 = 0x0F;
pub const TWL6040_REG_HSLCTL: u8 = 0x10;
pub const TWL6040_REG_HSRCTL: u8 = 0x11;
pub const TWL6040_REG_HSGAIN: u8 = 0x12;
pub const TWL6040_REG_EARCTL: u8 = 0x13;
pub const TWL6040_REG_HFLCTL: u8 = 0x14;
pub const TWL6040_REG_HFLGAIN: u8 = 0x15;
pub const TWL6040_REG_HFRCTL: u8 = 0x16;
pub const TWL6040_REG_HFRGAIN: u8 = 0x17;
pub const TWL6040_REG_VIBCTLL: u8 = 0x18;
pub const TWL6040_REG_VIBDATL: u8 = 0x19;
pub const TWL6040_REG_VIBCTLR: u8 = 0x1A;
pub const TWL6040_REG_VIBDATR: u8 = 0x1B;
pub const TWL6040_REG_HKCTL1: u8 = 0x1C;
pub const TWL6040_REG_HKCTL2: u8 = 0x1D;
pub const TWL6040_REG_GPOCTL: u8 = 0x1E;
pub const TWL6040_REG_ALB: u8 = 0x1F;
pub const TWL6040_REG_DLB: u8 = 0x20;
pub const TWL6040_REG_TRIM1: u8 = 0x28;
pub const TWL6040_REG_TRIM2: u8 = 0x29;
pub const TWL6040_REG_TRIM3: u8 = 0x2A;
pub const TWL6040_REG_HSOTRIM: u8 = 0x2B;
pub const TWL6040_REG_HFOTRIM: u8 = 0x2C;
pub const TWL6040_REG_ACCCTL: u8 = 0x2D;
pub const TWL6040_REG_STATUS: u8 = 0x2E;

pub const TWL6040_THINT: u8 = 0x01;
pub const TWL6040_PLUGINT: u8 = 0x02;
pub const TWL6040_UNPLUGINT: u8 = 0x04;
pub const TWL6040_HOOKINT: u8 = 0x08;
pub const TWL6040_HFINT: u8 = 0x10;
pub const TWL6040_VIBINT: u8 = 0x20;
pub const TWL6040_READYINT: u8 = 0x40;
pub const TWL6040_THMSK: u8 = 0x01;
pub const TWL6040_PLUGMSK: u8 = 0x02;
pub const TWL6040_HOOKMSK: u8 = 0x08;
pub const TWL6040_HFMSK: u8 = 0x10;
pub const TWL6040_VIBMSK: u8 = 0x20;
pub const TWL6040_READYMSK: u8 = 0x40;
pub const TWL6040_ALLINT_MSK: u8 = 0x7B;
pub const TWL6040_NCPENA: u8 = 0x01;
pub const TWL6040_NCPOPEN: u8 = 0x40;
pub const TWL6040_LSLDOENA: u8 = 0x01;
pub const TWL6040_HSLDOENA: u8 = 0x04;
pub const TWL6040_REFENA: u8 = 0x40;
pub const TWL6040_OSCENA: u8 = 0x80;
pub const TWL6040_HPLLENA: u8 = 0x01;
pub const TWL6040_HPLLRST: u8 = 0x02;
pub const TWL6040_HPLLBP: u8 = 0x04;
pub const TWL6040_HPLLSQRENA: u8 = 0x08;
pub const TWL6040_MCLK_12000KHZ: u8 = 0 << 5;
pub const TWL6040_MCLK_19200KHZ: u8 = 1 << 5;
pub const TWL6040_MCLK_26000KHZ: u8 = 2 << 5;
pub const TWL6040_MCLK_38400KHZ: u8 = 3 << 5;
pub const TWL6040_MCLK_MSK: u8 = 0x60;
pub const TWL6040_LPLLENA: u8 = 0x01;
pub const TWL6040_LPLLRST: u8 = 0x02;
pub const TWL6040_LPLLSEL: u8 = 0x04;
pub const TWL6040_LPLLFIN: u8 = 0x08;
pub const TWL6040_HPLLSEL: u8 = 0x10;
pub const TWL6040_HSDACENA: u8 = 1 << 0;
pub const TWL6040_HSDACMODE: u8 = 1 << 1;
pub const TWL6040_HSDRVENA: u8 = 1 << 2;
pub const TWL6040_HSDRVMODE: u8 = 1 << 3;
pub const TWL6040_HFDACENA: u8 = 1 << 0;
pub const TWL6040_HFPGAENA: u8 = 1 << 1;
pub const TWL6040_HFDRVENA: u8 = 1 << 4;
pub const TWL6040_HFSWENA: u8 = 1 << 6;
pub const TWL6040_VIBENA: u8 = 1 << 0;
pub const TWL6040_VIBSEL: u8 = 1 << 1;
pub const TWL6040_VIBCTRL: u8 = 1 << 2;
pub const TWL6040_VIBCTRL_P: u8 = 1 << 3;
pub const TWL6040_VIBCTRL_N: u8 = 1 << 4;
pub const TWL6040_VIBDAT_MAX: u8 = 0x64;
pub const TWL6040_GPO1: u8 = 0x01;
pub const TWL6040_GPO2: u8 = 0x02;
pub const TWL6040_GPO3: u8 = 0x04;
pub const TWL6040_I2CSEL: u8 = 0x01;
pub const TWL6040_RESETSPLIT: u8 = 0x04;
pub const TWL6040_INTCLRMODE: u8 = 0x08;
#[inline]
pub const fn TWL6040_I2CMODE(x: u8) -> u8 { (x & 0x3) << 4 }
pub const TWL6040_PLUGCOMP: u8 = 0x02;
pub const TWL6040_VIBLOCDET: u8 = 0x10;
pub const TWL6040_VIBROCDET: u8 = 0x20;
pub const TWL6040_TSHUTDET: u8 = 0x40;
pub const TWL6040_CELLS: usize = 4;
pub const TWL6040_REV_ES1_0: u8 = 0x00;
pub const TWL6040_REV_ES1_1: u8 = 0x01; /* Rev ES1.1 and ES1.2 */
pub const TWL6040_REV_ES1_3: u8 = 0x02;
pub const TWL6041_REV_ES2_0: u8 = 0x10;
pub const TWL6040_IRQ_TH: i32 = 0;
pub const TWL6040_IRQ_PLUG: i32 = 1;
pub const TWL6040_IRQ_HOOK: i32 = 2;
pub const TWL6040_IRQ_HF: i32 = 3;
pub const TWL6040_IRQ_VIB: i32 = 4;
pub const TWL6040_IRQ_READY: i32 = 5;
pub const TWL6040_SYSCLK_SEL_LPPLL: i32 = 0;
pub const TWL6040_SYSCLK_SEL_HPPLL: i32 = 1;
pub const TWL6040_GPO_MAX: i32 = 3;

#[repr(C)]
pub struct twl6040 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
    pub supplies: [regulator_bulk_data; 2],
    pub clk32k: *mut clk,
    pub mclk: *mut clk,
    pub mutex: mutex,
    pub irq_mutex: mutex,
    pub cells: [mfd_cell; TWL6040_CELLS],
    pub ready: completion,
    pub audpwron: *mut gpio_desc,
    pub power_count: i32,
    pub rev: i32,
    pub pll: i32,
    pub sysclk_rate: u32,
    pub mclk_rate: u32,
    pub irq: u32,
    pub irq_ready: u32,
    pub irq_th: u32,
}

extern "C" {
    pub fn twl6040_reg_read(twl6040: *mut twl6040, reg: u32) -> i32;
    pub fn twl6040_reg_write(twl6040: *mut twl6040, reg: u32, val: u8) -> i32;
    pub fn twl6040_set_bits(twl6040: *mut twl6040, reg: u32, mask: u8) -> i32;
    pub fn twl6040_clear_bits(twl6040: *mut twl6040, reg: u32, mask: u8) -> i32;
    pub fn twl6040_power(twl6040: *mut twl6040, on: i32) -> i32;
    pub fn twl6040_set_pll(twl6040: *mut twl6040, pll_id: i32, freq_in: u32, freq_out: u32) -> i32;
    pub fn twl6040_get_pll(twl6040: *mut twl6040) -> i32;
    pub fn twl6040_get_sysclk(twl6040: *mut twl6040) -> u32;
    pub fn twl6040_get_vibralr_status(twl6040: *mut twl6040) -> i32;
}

#[inline]
pub unsafe fn twl6040_get_revid(twl6040: *mut twl6040) -> i32 {
    (*twl6040).rev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
