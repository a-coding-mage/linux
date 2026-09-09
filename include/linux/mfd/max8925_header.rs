/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Maxim8925 Interface
 *
 * Copyright (C) 2009 Marvell International Ltd.
 *	Haojian Zhuang <haojian.zhuang@marvell.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_char;

pub enum device {}
pub enum i2c_client {}
pub enum mutex {}
pub enum regulator_init_data {}

/* Unified sub device IDs for MAX8925 */
pub const MAX8925_ID_SD1: i32 = 0;
pub const MAX8925_ID_SD2: i32 = 1;
pub const MAX8925_ID_SD3: i32 = 2;
pub const MAX8925_ID_LDO1: i32 = 3;
pub const MAX8925_ID_LDO2: i32 = 4;
pub const MAX8925_ID_LDO3: i32 = 5;
pub const MAX8925_ID_LDO4: i32 = 6;
pub const MAX8925_ID_LDO5: i32 = 7;
pub const MAX8925_ID_LDO6: i32 = 8;
pub const MAX8925_ID_LDO7: i32 = 9;
pub const MAX8925_ID_LDO8: i32 = 10;
pub const MAX8925_ID_LDO9: i32 = 11;
pub const MAX8925_ID_LDO10: i32 = 12;
pub const MAX8925_ID_LDO11: i32 = 13;
pub const MAX8925_ID_LDO12: i32 = 14;
pub const MAX8925_ID_LDO13: i32 = 15;
pub const MAX8925_ID_LDO14: i32 = 16;
pub const MAX8925_ID_LDO15: i32 = 17;
pub const MAX8925_ID_LDO16: i32 = 18;
pub const MAX8925_ID_LDO17: i32 = 19;
pub const MAX8925_ID_LDO18: i32 = 20;
pub const MAX8925_ID_LDO19: i32 = 21;
pub const MAX8925_ID_LDO20: i32 = 22;
pub const MAX8925_ID_MAX: i32 = 23;

/* Charging current threshold trigger going from fast charge to TOPOFF charge. */
pub const MAX8925_TOPOFF_THR_5PER: i32 = 0;
pub const MAX8925_TOPOFF_THR_10PER: i32 = 1;
pub const MAX8925_TOPOFF_THR_15PER: i32 = 2;
pub const MAX8925_TOPOFF_THR_20PER: i32 = 3;

/* Fast charging current */
pub const MAX8925_FCHG_85MA: i32 = 0;
pub const MAX8925_FCHG_300MA: i32 = 1;
pub const MAX8925_FCHG_460MA: i32 = 2;
pub const MAX8925_FCHG_600MA: i32 = 3;
pub const MAX8925_FCHG_700MA: i32 = 4;
pub const MAX8925_FCHG_800MA: i32 = 5;
pub const MAX8925_FCHG_900MA: i32 = 6;
pub const MAX8925_FCHG_1000MA: i32 = 7;

/* Charger registers */
pub const MAX8925_CHG_IRQ1: u32 = 0x7e;
pub const MAX8925_CHG_IRQ2: u32 = 0x7f;
pub const MAX8925_CHG_IRQ1_MASK: u32 = 0x80;
pub const MAX8925_CHG_IRQ2_MASK: u32 = 0x81;
pub const MAX8925_CHG_STATUS: u32 = 0x82;
/* GPM registers */
pub const MAX8925_SYSENSEL: u32 = 0x00;
pub const MAX8925_ON_OFF_IRQ1: u32 = 0x01;
pub const MAX8925_ON_OFF_IRQ1_MASK: u32 = 0x02;
pub const MAX8925_ON_OFF_STATUS: u32 = 0x03;
pub const MAX8925_ON_OFF_IRQ2: u32 = 0x0d;
pub const MAX8925_ON_OFF_IRQ2_MASK: u32 = 0x0e;
pub const MAX8925_RESET_CNFG: u32 = 0x0f;
/* Touch registers */
pub const MAX8925_TSC_IRQ: u32 = 0x00;
pub const MAX8925_TSC_IRQ_MASK: u32 = 0x01;
pub const MAX8925_TSC_CNFG1: u32 = 0x02;
pub const MAX8925_ADC_SCHED: u32 = 0x10;
pub const MAX8925_ADC_RES_END: u32 = 0x6f;
pub const MAX8925_NREF_OK: u32 = 1 << 4;
/* RTC registers */
pub const MAX8925_ALARM0_CNTL: u32 = 0x18;
pub const MAX8925_ALARM1_CNTL: u32 = 0x19;
pub const MAX8925_RTC_IRQ: u32 = 0x1c;
pub const MAX8925_RTC_IRQ_MASK: u32 = 0x1d;
pub const MAX8925_MPL_CNTL: u32 = 0x1e;
/* WLED registers */
pub const MAX8925_WLED_MODE_CNTL: u32 = 0x84;
pub const MAX8925_WLED_CNTL: u32 = 0x85;

/* MAX8925 Registers */
pub const MAX8925_SDCTL1: u32 = 0x04;
pub const MAX8925_SDCTL2: u32 = 0x0A - 3;
pub const MAX8925_SDCTL3: u32 = 0x0A;
pub const MAX8925_SDV1: u32 = 0x06;
pub const MAX8925_SDV2: u32 = 0x09;
pub const MAX8925_SDV3: u32 = 0x0C;
pub const MAX8925_LDOCTL1: u32 = 0x18;
pub const MAX8925_LDOCTL2: u32 = 0x1C;
pub const MAX8925_LDOCTL3: u32 = 0x20;
pub const MAX8925_LDOCTL4: u32 = 0x24;
pub const MAX8925_LDOCTL5: u32 = 0x28;
pub const MAX8925_LDOCTL6: u32 = 0x2C;
pub const MAX8925_LDOCTL7: u32 = 0x30;
pub const MAX8925_LDOCTL8: u32 = 0x34;
pub const MAX8925_LDOCTL9: u32 = 0x38;
pub const MAX8925_LDOCTL10: u32 = 0x3C;
pub const MAX8925_LDOCTL11: u32 = 0x40;
pub const MAX8925_LDOCTL12: u32 = 0x44;
pub const MAX8925_LDOCTL13: u32 = 0x48;
pub const MAX8925_LDOCTL14: u32 = 0x4C;
pub const MAX8925_LDOCTL15: u32 = 0x50;
pub const MAX8925_LDOCTL16: u32 = 0x10;
pub const MAX8925_LDOCTL17: u32 = 0x14;
pub const MAX8925_LDOCTL18: u32 = 0x72;
pub const MAX8925_LDOCTL19: u32 = 0x5C;
pub const MAX8925_LDOCTL20: u32 = 0x9C;
pub const MAX8925_LDOVOUT1: u32 = 0x1A;
pub const MAX8925_LDOVOUT2: u32 = 0x1E;
pub const MAX8925_LDOVOUT3: u32 = 0x22;
pub const MAX8925_LDOVOUT4: u32 = 0x26;
pub const MAX8925_LDOVOUT5: u32 = 0x2A;
pub const MAX8925_LDOVOUT6: u32 = 0x2E;
pub const MAX8925_LDOVOUT7: u32 = 0x32;
pub const MAX8925_LDOVOUT8: u32 = 0x36;
pub const MAX8925_LDOVOUT9: u32 = 0x3A;
pub const MAX8925_LDOVOUT10: u32 = 0x3E;
pub const MAX8925_LDOVOUT11: u32 = 0x42;
pub const MAX8925_LDOVOUT12: u32 = 0x46;
pub const MAX8925_LDOVOUT13: u32 = 0x4A;
pub const MAX8925_LDOVOUT14: u32 = 0x4E;
pub const MAX8925_LDOVOUT15: u32 = 0x52;
pub const MAX8925_LDOVOUT16: u32 = 0x12;
pub const MAX8925_LDOVOUT17: u32 = 0x16;
pub const MAX8925_LDOVOUT18: u32 = 0x74;
pub const MAX8925_LDOVOUT19: u32 = 0x5E;
pub const MAX8925_LDOVOUT20: u32 = 0x9E;

/* bit definitions */
pub const CHG_IRQ1_MASK: u32 = 0x07;
pub const CHG_IRQ2_MASK: u32 = 0xff;
pub const ON_OFF_IRQ1_MASK: u32 = 0xff;
pub const ON_OFF_IRQ2_MASK: u32 = 0x03;
pub const TSC_IRQ_MASK: u32 = 0x03;
pub const RTC_IRQ_MASK: u32 = 0x0c;
pub const MAX8925_NAME_SIZE: usize = 32;

/* IRQ definitions */
pub const MAX8925_IRQ_VCHG_DC_OVP: i32 = 0;
pub const MAX8925_IRQ_VCHG_DC_F: i32 = 1;
pub const MAX8925_IRQ_VCHG_DC_R: i32 = 2;
pub const MAX8925_IRQ_VCHG_THM_OK_R: i32 = 3;
pub const MAX8925_IRQ_VCHG_THM_OK_F: i32 = 4;
pub const MAX8925_IRQ_VCHG_SYSLOW_F: i32 = 5;
pub const MAX8925_IRQ_VCHG_SYSLOW_R: i32 = 6;
pub const MAX8925_IRQ_VCHG_RST: i32 = 7;
pub const MAX8925_IRQ_VCHG_DONE: i32 = 8;
pub const MAX8925_IRQ_VCHG_TOPOFF: i32 = 9;
pub const MAX8925_IRQ_VCHG_TMR_FAULT: i32 = 10;
pub const MAX8925_IRQ_GPM_RSTIN: i32 = 11;
pub const MAX8925_IRQ_GPM_MPL: i32 = 12;
pub const MAX8925_IRQ_GPM_SW_3SEC: i32 = 13;
pub const MAX8925_IRQ_GPM_EXTON_F: i32 = 14;
pub const MAX8925_IRQ_GPM_EXTON_R: i32 = 15;
pub const MAX8925_IRQ_GPM_SW_1SEC: i32 = 16;
pub const MAX8925_IRQ_GPM_SW_F: i32 = 17;
pub const MAX8925_IRQ_GPM_SW_R: i32 = 18;
pub const MAX8925_IRQ_GPM_SYSCKEN_F: i32 = 19;
pub const MAX8925_IRQ_GPM_SYSCKEN_R: i32 = 20;
pub const MAX8925_IRQ_RTC_ALARM1: i32 = 21;
pub const MAX8925_IRQ_RTC_ALARM0: i32 = 22;
pub const MAX8925_IRQ_TSC_STICK: i32 = 23;
pub const MAX8925_IRQ_TSC_NSTICK: i32 = 24;
pub const MAX8925_NR_IRQS: i32 = 25;

#[repr(C)]
pub struct max8925_chip {
    pub dev: *mut device,
    pub i2c: *mut i2c_client,
    pub adc: *mut i2c_client,
    pub rtc: *mut i2c_client,
    pub io_lock: mutex,
    pub irq_lock: mutex,
    pub irq_base: i32,
    pub core_irq: i32,
    pub tsc_irq: i32,
    pub wakeup_flag: u32,
}

#[repr(C)]
pub struct max8925_backlight_pdata {
    pub lxw_scl: i32,
    pub lxw_freq: i32,
    pub dual_string: i32,
}

#[repr(C)]
pub struct max8925_touch_pdata {
    pub flags: u32,
}

#[repr(C)]
pub struct max8925_power_pdata {
    pub set_charger: Option<unsafe extern "C" fn(i32) -> i32>,
    pub batt_detect: u32,
    pub topoff_threshold: u32,
    pub fast_charge: u32,
    pub no_temp_support: u32,
    pub no_insert_detect: u32,
    pub supplied_to: *mut *mut c_char,
    pub num_supplicants: i32,
}

#[repr(C)]
pub struct max8925_platform_data {
    pub backlight: *mut max8925_backlight_pdata,
    pub touch: *mut max8925_touch_pdata,
    pub power: *mut max8925_power_pdata,
    pub sd1: *mut regulator_init_data,
    pub sd2: *mut regulator_init_data,
    pub sd3: *mut regulator_init_data,
    pub ldo1: *mut regulator_init_data,
    pub ldo2: *mut regulator_init_data,
    pub ldo3: *mut regulator_init_data,
    pub ldo4: *mut regulator_init_data,
    pub ldo5: *mut regulator_init_data,
    pub ldo6: *mut regulator_init_data,
    pub ldo7: *mut regulator_init_data,
    pub ldo8: *mut regulator_init_data,
    pub ldo9: *mut regulator_init_data,
    pub ldo10: *mut regulator_init_data,
    pub ldo11: *mut regulator_init_data,
    pub ldo12: *mut regulator_init_data,
    pub ldo13: *mut regulator_init_data,
    pub ldo14: *mut regulator_init_data,
    pub ldo15: *mut regulator_init_data,
    pub ldo16: *mut regulator_init_data,
    pub ldo17: *mut regulator_init_data,
    pub ldo18: *mut regulator_init_data,
    pub ldo19: *mut regulator_init_data,
    pub ldo20: *mut regulator_init_data,
    pub irq_base: i32,
    pub tsc_irq: i32,
}

unsafe extern "C" {
    pub fn max8925_reg_read(client: *mut i2c_client, reg: i32) -> i32;
    pub fn max8925_reg_write(client: *mut i2c_client, reg: i32, value: u8) -> i32;
    pub fn max8925_bulk_read(client: *mut i2c_client, reg: i32, count: i32, value: *mut u8) -> i32;
    pub fn max8925_bulk_write(client: *mut i2c_client, reg: i32, count: i32, value: *mut u8) -> i32;
    pub fn max8925_set_bits(client: *mut i2c_client, reg: i32, mask: u8, value: u8) -> i32;
    pub fn max8925_device_init(chip: *mut max8925_chip, pdata: *mut max8925_platform_data) -> i32;
    pub fn max8925_device_exit(chip: *mut max8925_chip);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
