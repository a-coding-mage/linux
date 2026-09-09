/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) ST-Ericsson SA 2010
 *
 * Author: Rabin Vincent <rabin.vincent@stericsson.com> for ST-Ericsson
 */

/* C dependency: #include <linux/mutex.h> */

#[inline]
pub const fn STMPE_SAMPLE_TIME(x: u8) -> u8 { (x & 0xf) << 4 }
#[inline]
pub const fn STMPE_MOD_12B(x: u8) -> u8 { (x & 0x1) << 3 }
#[inline]
pub const fn STMPE_REF_SEL(x: u8) -> u8 { (x & 0x1) << 1 }
#[inline]
pub const fn STMPE_ADC_FREQ(x: u8) -> u8 { x & 0x3 }
#[inline]
pub const fn STMPE_AVE_CTRL(x: u8) -> u8 { (x & 0x3) << 6 }
#[inline]
pub const fn STMPE_DET_DELAY(x: u8) -> u8 { (x & 0x7) << 3 }
#[inline]
pub const fn STMPE_SETTLING(x: u8) -> u8 { x & 0x7 }
#[inline]
pub const fn STMPE_FRACTION_Z(x: u8) -> u8 { x & 0x7 }
#[inline]
pub const fn STMPE_I_DRIVE(x: u8) -> u8 { x & 0x1 }
#[inline]
pub const fn STMPE_OP_MODE(x: u8) -> u8 { (x & 0x7) << 1 }

pub const STMPE811_REG_ADC_CTRL1: u8 = 0x20;
pub const STMPE811_REG_ADC_CTRL2: u8 = 0x21;

pub enum device {}
pub enum regulator {}
pub enum irq_domain {}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stmpe_block {
    STMPE_BLOCK_GPIO = 1 << 0,
    STMPE_BLOCK_KEYPAD = 1 << 1,
    STMPE_BLOCK_TOUCHSCREEN = 1 << 2,
    STMPE_BLOCK_ADC = 1 << 3,
    STMPE_BLOCK_PWM = 1 << 4,
    STMPE_BLOCK_ROTATOR = 1 << 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stmpe_partnum {
    STMPE610,
    STMPE801,
    STMPE811,
    STMPE1600,
    STMPE1601,
    STMPE1801,
    STMPE2401,
    STMPE2403,
    STMPE_NBR_PARTS,
}

/*
 * For registers whose locations differ on variants, the correct address is
 * obtained by indexing stmpe->regs with one of the following.
 */
pub const STMPE_IDX_CHIP_ID: usize = 0;
pub const STMPE_IDX_SYS_CTRL: usize = 1;
pub const STMPE_IDX_SYS_CTRL2: usize = 2;
pub const STMPE_IDX_ICR_LSB: usize = 3;
pub const STMPE_IDX_IER_LSB: usize = 4;
pub const STMPE_IDX_IER_MSB: usize = 5;
pub const STMPE_IDX_ISR_LSB: usize = 6;
pub const STMPE_IDX_ISR_MSB: usize = 7;
pub const STMPE_IDX_GPMR_LSB: usize = 8;
pub const STMPE_IDX_GPMR_CSB: usize = 9;
pub const STMPE_IDX_GPMR_MSB: usize = 10;
pub const STMPE_IDX_GPSR_LSB: usize = 11;
pub const STMPE_IDX_GPSR_CSB: usize = 12;
pub const STMPE_IDX_GPSR_MSB: usize = 13;
pub const STMPE_IDX_GPCR_LSB: usize = 14;
pub const STMPE_IDX_GPCR_CSB: usize = 15;
pub const STMPE_IDX_GPCR_MSB: usize = 16;
pub const STMPE_IDX_GPDR_LSB: usize = 17;
pub const STMPE_IDX_GPDR_CSB: usize = 18;
pub const STMPE_IDX_GPDR_MSB: usize = 19;
pub const STMPE_IDX_GPEDR_LSB: usize = 20;
pub const STMPE_IDX_GPEDR_CSB: usize = 21;
pub const STMPE_IDX_GPEDR_MSB: usize = 22;
pub const STMPE_IDX_GPRER_LSB: usize = 23;
pub const STMPE_IDX_GPRER_CSB: usize = 24;
pub const STMPE_IDX_GPRER_MSB: usize = 25;
pub const STMPE_IDX_GPFER_LSB: usize = 26;
pub const STMPE_IDX_GPFER_CSB: usize = 27;
pub const STMPE_IDX_GPFER_MSB: usize = 28;
pub const STMPE_IDX_GPPUR_LSB: usize = 29;
pub const STMPE_IDX_GPPDR_LSB: usize = 30;
pub const STMPE_IDX_GPAFR_U_MSB: usize = 31;
pub const STMPE_IDX_IEGPIOR_LSB: usize = 32;
pub const STMPE_IDX_IEGPIOR_CSB: usize = 33;
pub const STMPE_IDX_IEGPIOR_MSB: usize = 34;
pub const STMPE_IDX_ISGPIOR_LSB: usize = 35;
pub const STMPE_IDX_ISGPIOR_CSB: usize = 36;
pub const STMPE_IDX_ISGPIOR_MSB: usize = 37;
pub const STMPE_IDX_MAX: usize = 38;

pub enum stmpe_variant_info {}
pub enum stmpe_client_info {}
pub enum stmpe_platform_data {}

/**
 * struct stmpe - STMPE MFD structure
 * @vcc: optional VCC regulator
 * @vio: optional VIO regulator
 * @lock: lock protecting I/O operations
 * @irq_lock: IRQ bus lock
 * @dev: device, mostly for dev_dbg()
 * @irq_domain: IRQ domain
 * @client: client - i2c or spi
 * @ci: client specific information
 * @partnum: part number
 * @variant: the detected STMPE model number
 * @regs: list of addresses of registers which are at different addresses on
 *	  different variants.  Indexed by one of STMPE_IDX_*.
 * @irq: irq number for stmpe
 * @num_gpios: number of gpios, differs for variants
 * @ier: cache of IER registers for bus_lock
 * @oldier: cache of IER registers for bus_lock
 * @pdata: platform data
 */
#[repr(C)]
pub struct stmpe {
    pub vcc: *mut regulator,
    pub vio: *mut regulator,
    pub lock: mutex,
    pub irq_lock: mutex,
    pub dev: *mut device,
    pub domain: *mut irq_domain,
    pub client: *mut core::ffi::c_void,
    pub ci: *mut stmpe_client_info,
    pub partnum: stmpe_partnum,
    pub variant: *mut stmpe_variant_info,
    pub regs: *const u8,
    pub irq: i32,
    pub num_gpios: i32,
    pub ier: [u8; 2],
    pub oldier: [u8; 2],
    pub pdata: *mut stmpe_platform_data,
    /* For devices that use an ADC */
    pub sample_time: u8,
    pub mod_12b: u8,
    pub ref_sel: u8,
    pub adc_freq: u8,
}

unsafe extern "C" {
    pub fn stmpe_reg_write(stmpe: *mut stmpe, reg: u8, data: u8) -> i32;
    pub fn stmpe_reg_read(stmpe: *mut stmpe, reg: u8) -> i32;
    pub fn stmpe_block_read(stmpe: *mut stmpe, reg: u8, length: u8, values: *mut u8) -> i32;
    pub fn stmpe_block_write(stmpe: *mut stmpe, reg: u8, length: u8, values: *const u8) -> i32;
    pub fn stmpe_set_bits(stmpe: *mut stmpe, reg: u8, mask: u8, val: u8) -> i32;
    pub fn stmpe_set_altfunc(stmpe: *mut stmpe, pins: u32, block: stmpe_block) -> i32;
    pub fn stmpe_enable(stmpe: *mut stmpe, blocks: u32) -> i32;
    pub fn stmpe_disable(stmpe: *mut stmpe, blocks: u32) -> i32;
    pub fn stmpe811_adc_common_init(stmpe: *mut stmpe) -> i32;
}

pub const STMPE_GPIO_NOREQ_811_TOUCH: u8 = 0xf0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
