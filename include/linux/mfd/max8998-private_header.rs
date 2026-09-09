/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * max8998-private.h - Voltage regulator driver for the Maxim 8998
 *
 *  Copyright (C) 2009-2010 Samsung Electronics
 *  Kyungmin Park <kyungmin.park@samsung.com>
 *  Marek Szyprowski <m.szyprowski@samsung.com>
 */

pub const MAX8998_NUM_IRQ_REGS: usize = 4;

/* MAX 8998 registers */
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Max8998Reg {
    MAX8998_REG_IRQ1,
    MAX8998_REG_IRQ2,
    MAX8998_REG_IRQ3,
    MAX8998_REG_IRQ4,
    MAX8998_REG_IRQM1,
    MAX8998_REG_IRQM2,
    MAX8998_REG_IRQM3,
    MAX8998_REG_IRQM4,
    MAX8998_REG_STATUS1,
    MAX8998_REG_STATUS2,
    MAX8998_REG_STATUSM1,
    MAX8998_REG_STATUSM2,
    MAX8998_REG_CHGR1,
    MAX8998_REG_CHGR2,
    MAX8998_REG_LDO_ACTIVE_DISCHARGE1,
    MAX8998_REG_LDO_ACTIVE_DISCHARGE2,
    MAX8998_REG_BUCK_ACTIVE_DISCHARGE3,
    MAX8998_REG_ONOFF1,
    MAX8998_REG_ONOFF2,
    MAX8998_REG_ONOFF3,
    MAX8998_REG_ONOFF4,
    MAX8998_REG_BUCK1_VOLTAGE1,
    MAX8998_REG_BUCK1_VOLTAGE2,
    MAX8998_REG_BUCK1_VOLTAGE3,
    MAX8998_REG_BUCK1_VOLTAGE4,
    MAX8998_REG_BUCK2_VOLTAGE1,
    MAX8998_REG_BUCK2_VOLTAGE2,
    MAX8998_REG_BUCK3,
    MAX8998_REG_BUCK4,
    MAX8998_REG_LDO2_LDO3,
    MAX8998_REG_LDO4,
    MAX8998_REG_LDO5,
    MAX8998_REG_LDO6,
    MAX8998_REG_LDO7,
    MAX8998_REG_LDO8_LDO9,
    MAX8998_REG_LDO10_LDO11,
    MAX8998_REG_LDO12,
    MAX8998_REG_LDO13,
    MAX8998_REG_LDO14,
    MAX8998_REG_LDO15,
    MAX8998_REG_LDO16,
    MAX8998_REG_LDO17,
    MAX8998_REG_BKCHR,
    MAX8998_REG_LBCNFG1,
    MAX8998_REG_LBCNFG2,
}

/* IRQ definitions */
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Max8998Irq {
    MAX8998_IRQ_DCINF,
    MAX8998_IRQ_DCINR,
    MAX8998_IRQ_JIGF,
    MAX8998_IRQ_JIGR,
    MAX8998_IRQ_PWRONF,
    MAX8998_IRQ_PWRONR,
    MAX8998_IRQ_WTSREVNT,
    MAX8998_IRQ_SMPLEVNT,
    MAX8998_IRQ_ALARM1,
    MAX8998_IRQ_ALARM0,
    MAX8998_IRQ_ONKEY1S,
    MAX8998_IRQ_TOPOFFR,
    MAX8998_IRQ_DCINOVPR,
    MAX8998_IRQ_CHGRSTF,
    MAX8998_IRQ_DONER,
    MAX8998_IRQ_CHGFAULT,
    MAX8998_IRQ_LOBAT1,
    MAX8998_IRQ_LOBAT2,
    MAX8998_IRQ_NR,
}

/* MAX8998 various variants */
#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Max8998Type {
    TYPE_MAX8998 = 0, /* Default */
    TYPE_LP3974,      /* National version of MAX8998 */
    TYPE_LP3979,      /* Added AVS */
}

pub const MAX8998_IRQ_DCINF_MASK: u32 = 1 << 2;
pub const MAX8998_IRQ_DCINR_MASK: u32 = 1 << 3;
pub const MAX8998_IRQ_JIGF_MASK: u32 = 1 << 4;
pub const MAX8998_IRQ_JIGR_MASK: u32 = 1 << 5;
pub const MAX8998_IRQ_PWRONF_MASK: u32 = 1 << 6;
pub const MAX8998_IRQ_PWRONR_MASK: u32 = 1 << 7;
pub const MAX8998_IRQ_WTSREVNT_MASK: u32 = 1 << 0;
pub const MAX8998_IRQ_SMPLEVNT_MASK: u32 = 1 << 1;
pub const MAX8998_IRQ_ALARM1_MASK: u32 = 1 << 2;
pub const MAX8998_IRQ_ALARM0_MASK: u32 = 1 << 3;
pub const MAX8998_IRQ_ONKEY1S_MASK: u32 = 1 << 0;
pub const MAX8998_IRQ_TOPOFFR_MASK: u32 = 1 << 2;
pub const MAX8998_IRQ_DCINOVPR_MASK: u32 = 1 << 3;
pub const MAX8998_IRQ_CHGRSTF_MASK: u32 = 1 << 4;
pub const MAX8998_IRQ_DONER_MASK: u32 = 1 << 5;
pub const MAX8998_IRQ_CHGFAULT_MASK: u32 = 1 << 7;
pub const MAX8998_IRQ_LOBAT1_MASK: u32 = 1 << 0;
pub const MAX8998_IRQ_LOBAT2_MASK: u32 = 1 << 1;
pub const MAX8998_ENRAMP: u32 = 1 << 4;

/* External types supplied by other translation units. */
#[repr(C)]
pub struct max8998_dev {
    pub dev: *mut device,
    pub pdata: *mut max8998_platform_data,
    pub i2c: *mut i2c_client,
    pub rtc: *mut i2c_client,
    pub iolock: mutex,
    pub irqlock: mutex,
    pub irq_base: u32,
    pub irq_domain: *mut irq_domain,
    pub irq: i32,
    pub ono: i32,
    pub irq_masks_cur: [u8; MAX8998_NUM_IRQ_REGS],
    pub irq_masks_cache: [u8; MAX8998_NUM_IRQ_REGS],
    pub type_: u64,
    pub wakeup: bool,
}

extern "C" {
    pub fn max8998_irq_init(max8998: *mut max8998_dev) -> i32;
    pub fn max8998_irq_exit(max8998: *mut max8998_dev);
    pub fn max8998_irq_resume(max8998: *mut max8998_dev) -> i32;
    pub fn max8998_read_reg(i2c: *mut i2c_client, reg: u8, dest: *mut u8) -> i32;
    pub fn max8998_bulk_read(i2c: *mut i2c_client, reg: u8, count: i32, buf: *mut u8) -> i32;
    pub fn max8998_write_reg(i2c: *mut i2c_client, reg: u8, value: u8) -> i32;
    pub fn max8998_bulk_write(i2c: *mut i2c_client, reg: u8, count: i32, buf: *mut u8) -> i32;
    pub fn max8998_update_reg(i2c: *mut i2c_client, reg: u8, val: u8, mask: u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
