/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * DA9150 MFD Driver - Core Data
 *
 * Copyright (c) 2014 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

/* I2C address paging */
pub const DA9150_REG_PAGE_SHIFT: u32 = 8;
pub const DA9150_REG_PAGE_MASK: u32 = 0xFF;

/* IRQs */
pub const DA9150_NUM_IRQ_REGS: u32 = 4;
pub const DA9150_IRQ_VBUS: u32 = 0;
pub const DA9150_IRQ_CHG: u32 = 1;
pub const DA9150_IRQ_TCLASS: u32 = 2;
pub const DA9150_IRQ_TJUNC: u32 = 3;
pub const DA9150_IRQ_VFAULT: u32 = 4;
pub const DA9150_IRQ_CONF: u32 = 5;
pub const DA9150_IRQ_DAT: u32 = 6;
pub const DA9150_IRQ_DTYPE: u32 = 7;
pub const DA9150_IRQ_ID: u32 = 8;
pub const DA9150_IRQ_ADP: u32 = 9;
pub const DA9150_IRQ_SESS_END: u32 = 10;
pub const DA9150_IRQ_SESS_VLD: u32 = 11;
pub const DA9150_IRQ_FG: u32 = 12;
pub const DA9150_IRQ_GP: u32 = 13;
pub const DA9150_IRQ_TBAT: u32 = 14;
pub const DA9150_IRQ_GPIOA: u32 = 15;
pub const DA9150_IRQ_GPIOB: u32 = 16;
pub const DA9150_IRQ_GPIOC: u32 = 17;
pub const DA9150_IRQ_GPIOD: u32 = 18;
pub const DA9150_IRQ_GPADC: u32 = 19;
pub const DA9150_IRQ_WKUP: u32 = 20;

/* I2C sub-device address */
pub const DA9150_QIF_I2C_ADDR_LSB: u32 = 0x5;

#[repr(C)]
pub struct da9150_fg_pdata {
    pub update_interval: u32, /* msecs */
    pub warn_soc_lvl: u8,     /* % value */
    pub crit_soc_lvl: u8,     /* % value */
}

#[repr(C)]
pub struct da9150_pdata {
    pub irq_base: i32,
    pub fg_pdata: *mut da9150_fg_pdata,
}

#[repr(C)]
pub struct da9150 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub core_qif: *mut i2c_client,

    pub regmap_irq_data: *mut regmap_irq_chip_data,
    pub irq: i32,
    pub irq_base: i32,
}

/* Device I/O - Query Interface for FG and standard register access */
extern "C" {
    pub fn da9150_read_qif(da9150: *mut da9150, addr: u8, count: i32, buf: *mut u8);
    pub fn da9150_write_qif(
        da9150: *mut da9150,
        addr: u8,
        count: i32,
        buf: *const u8,
    );

    pub fn da9150_reg_read(da9150: *mut da9150, reg: u16) -> u8;
    pub fn da9150_reg_write(da9150: *mut da9150, reg: u16, val: u8);
    pub fn da9150_set_bits(da9150: *mut da9150, reg: u16, mask: u8, val: u8);

    pub fn da9150_bulk_read(da9150: *mut da9150, reg: u16, count: i32, buf: *mut u8);
    pub fn da9150_bulk_write(da9150: *mut da9150, reg: u16, count: i32, buf: *const u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
