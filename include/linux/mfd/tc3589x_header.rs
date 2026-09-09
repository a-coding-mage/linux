/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) ST-Ericsson SA 2010
 */

// C header guard omitted.

#[repr(C)]
pub enum tx3589x_block {
    TC3589x_BLOCK_GPIO = 1 << 0,
    TC3589x_BLOCK_KEYPAD = 1 << 1,
}

pub const TC3589x_RSTCTRL_IRQRST: u32 = 1 << 4;
pub const TC3589x_RSTCTRL_TIMRST: u32 = 1 << 3;
pub const TC3589x_RSTCTRL_ROTRST: u32 = 1 << 2;
pub const TC3589x_RSTCTRL_KBDRST: u32 = 1 << 1;
pub const TC3589x_RSTCTRL_GPIRST: u32 = 1 << 0;

pub const TC3589x_DKBDMSK_ELINT: u32 = 1 << 1;
pub const TC3589x_DKBDMSK_EINT: u32 = 1 << 0;

/* Keyboard Configuration Registers */
pub const TC3589x_KBDSETTLE_REG: u32 = 0x01;
pub const TC3589x_KBDBOUNCE: u32 = 0x02;
pub const TC3589x_KBDSIZE: u32 = 0x03;
pub const TC3589x_KBCFG_LSB: u32 = 0x04;
pub const TC3589x_KBCFG_MSB: u32 = 0x05;
pub const TC3589x_KBDIC: u32 = 0x08;
pub const TC3589x_KBDMSK: u32 = 0x09;
pub const TC3589x_EVTCODE_FIFO: u32 = 0x10;
pub const TC3589x_KBDMFS: u32 = 0x8F;

pub const TC3589x_IRQST: u32 = 0x91;

pub const TC3589x_MANFCODE_MAGIC: u32 = 0x03;
pub const TC3589x_MANFCODE: u32 = 0x80;
pub const TC3589x_VERSION: u32 = 0x81;
pub const TC3589x_IOCFG: u32 = 0xA7;

pub const TC3589x_CLKMODE: u32 = 0x88;
pub const TC3589x_CLKCFG: u32 = 0x89;
pub const TC3589x_CLKEN: u32 = 0x8A;

pub const TC3589x_RSTCTRL: u32 = 0x82;
pub const TC3589x_EXTRSTN: u32 = 0x83;
pub const TC3589x_RSTINTCLR: u32 = 0x84;

/* Pull up/down configuration registers */
// TC3589x_IOCFG is defined above as well in the C header.
pub const TC3589x_IOPULLCFG0_LSB: u32 = 0xAA;
pub const TC3589x_IOPULLCFG0_MSB: u32 = 0xAB;
pub const TC3589x_IOPULLCFG1_LSB: u32 = 0xAC;
pub const TC3589x_IOPULLCFG1_MSB: u32 = 0xAD;
pub const TC3589x_IOPULLCFG2_LSB: u32 = 0xAE;

pub const TC3589x_GPIOIS0: u32 = 0xC9;
pub const TC3589x_GPIOIS1: u32 = 0xCA;
pub const TC3589x_GPIOIS2: u32 = 0xCB;
pub const TC3589x_GPIOIBE0: u32 = 0xCC;
pub const TC3589x_GPIOIBE1: u32 = 0xCD;
pub const TC3589x_GPIOIBE2: u32 = 0xCE;
pub const TC3589x_GPIOIEV0: u32 = 0xCF;
pub const TC3589x_GPIOIEV1: u32 = 0xD0;
pub const TC3589x_GPIOIEV2: u32 = 0xD1;
pub const TC3589x_GPIOIE0: u32 = 0xD2;
pub const TC3589x_GPIOIE1: u32 = 0xD3;
pub const TC3589x_GPIOIE2: u32 = 0xD4;
pub const TC3589x_GPIORIS0: u32 = 0xD6;
pub const TC3589x_GPIORIS1: u32 = 0xD7;
pub const TC3589x_GPIORIS2: u32 = 0xD8;
pub const TC3589x_GPIOMIS0: u32 = 0xD9;
pub const TC3589x_GPIOMIS1: u32 = 0xDA;
pub const TC3589x_GPIOMIS2: u32 = 0xDB;
pub const TC3589x_GPIOIC0: u32 = 0xDC;
pub const TC3589x_GPIOIC1: u32 = 0xDD;
pub const TC3589x_GPIOIC2: u32 = 0xDE;

pub const TC3589x_GPIODATA0: u32 = 0xC0;
pub const TC3589x_GPIOMASK0: u32 = 0xc1;
pub const TC3589x_GPIODATA1: u32 = 0xC2;
pub const TC3589x_GPIOMASK1: u32 = 0xc3;
pub const TC3589x_GPIODATA2: u32 = 0xC4;
pub const TC3589x_GPIOMASK2: u32 = 0xC5;

pub const TC3589x_GPIODIR0: u32 = 0xC6;
pub const TC3589x_GPIODIR1: u32 = 0xC7;
pub const TC3589x_GPIODIR2: u32 = 0xC8;

pub const TC3589x_GPIOSYNC0: u32 = 0xE6;
pub const TC3589x_GPIOSYNC1: u32 = 0xE7;
pub const TC3589x_GPIOSYNC2: u32 = 0xE8;

pub const TC3589x_GPIOWAKE0: u32 = 0xE9;
pub const TC3589x_GPIOWAKE1: u32 = 0xEA;
pub const TC3589x_GPIOWAKE2: u32 = 0xEB;

pub const TC3589x_GPIOODM0: u32 = 0xE0;
pub const TC3589x_GPIOODE0: u32 = 0xE1;
pub const TC3589x_GPIOODM1: u32 = 0xE2;
pub const TC3589x_GPIOODE1: u32 = 0xE3;
pub const TC3589x_GPIOODM2: u32 = 0xE4;
pub const TC3589x_GPIOODE2: u32 = 0xE5;

pub const TC3589x_DIRECT0: u32 = 0xEC;
pub const TC3589x_DKBDMSK: u32 = 0xF3;

pub const TC3589x_INT_GPIIRQ: i32 = 0;
pub const TC3589x_INT_TI0IRQ: i32 = 1;
pub const TC3589x_INT_TI1IRQ: i32 = 2;
pub const TC3589x_INT_TI2IRQ: i32 = 3;
pub const TC3589x_INT_ROTIRQ: i32 = 5;
pub const TC3589x_INT_KBDIRQ: i32 = 6;
pub const TC3589x_INT_PORIRQ: i32 = 7;

pub const TC3589x_NR_INTERNAL_IRQS: i32 = 8;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tc3589x {
    pub lock: mutex,
    pub dev: *mut device,
    pub i2c: *mut i2c_client,
    pub domain: *mut irq_domain,
    pub irq_base: i32,
    pub num_gpio: i32,
    pub pdata: *mut tc3589x_platform_data,
}

extern "C" {
    pub fn tc3589x_reg_write(tc3589x: *mut tc3589x, reg: u8, data: u8) -> i32;
    pub fn tc3589x_reg_read(tc3589x: *mut tc3589x, reg: u8) -> i32;
    pub fn tc3589x_block_read(
        tc3589x: *mut tc3589x,
        reg: u8,
        length: u8,
        values: *mut u8,
    ) -> i32;
    pub fn tc3589x_block_write(
        tc3589x: *mut tc3589x,
        reg: u8,
        length: u8,
        values: *const u8,
    ) -> i32;
    pub fn tc3589x_set_bits(
        tc3589x: *mut tc3589x,
        reg: u8,
        mask: u8,
        val: u8,
    ) -> i32;
}

/*
 * Keypad related platform specific constants
 * These values may be modified for fine tuning
 */
pub const TC_KPD_ROWS: u32 = 0x8;
pub const TC_KPD_COLUMNS: u32 = 0x8;
pub const TC_KPD_DEBOUNCE_PERIOD: u32 = 0xA3;
pub const TC_KPD_SETTLE_TIME: u32 = 0xA3;

/**
 * struct tc3589x_platform_data - TC3589x platform data
 * @block: bitmask of blocks to enable (use TC3589x_BLOCK_*)
 */
#[repr(C)]
pub struct tc3589x_platform_data {
    pub block: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
