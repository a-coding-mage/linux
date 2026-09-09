/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com/
 *	Andrew F. Davis <afd@ti.com>
 *
 * Based on the TPS65912 driver
 */

// Translated from the C header. Types supplied by the Linux device/regmap
// headers are intentionally left as external dependencies.

/* List of registers for TPS65086 */
pub const TPS65086_DEVICEID1: u32 = 0x00;
pub const TPS65086_DEVICEID2: u32 = 0x01;
pub const TPS65086_IRQ: u32 = 0x02;
pub const TPS65086_IRQ_MASK: u32 = 0x03;
pub const TPS65086_PMICSTAT: u32 = 0x04;
pub const TPS65086_SHUTDNSRC: u32 = 0x05;
pub const TPS65086_BUCK1CTRL: u32 = 0x20;
pub const TPS65086_BUCK2CTRL: u32 = 0x21;
pub const TPS65086_BUCK3DECAY: u32 = 0x22;
pub const TPS65086_BUCK3VID: u32 = 0x23;
pub const TPS65086_BUCK3SLPCTRL: u32 = 0x24;
pub const TPS65086_BUCK4CTRL: u32 = 0x25;
pub const TPS65086_BUCK5CTRL: u32 = 0x26;
pub const TPS65086_BUCK6CTRL: u32 = 0x27;
pub const TPS65086_LDOA2CTRL: u32 = 0x28;
pub const TPS65086_LDOA3CTRL: u32 = 0x29;
pub const TPS65086_DISCHCTRL1: u32 = 0x40;
pub const TPS65086_DISCHCTRL2: u32 = 0x41;
pub const TPS65086_DISCHCTRL3: u32 = 0x42;
pub const TPS65086_PG_DELAY1: u32 = 0x43;
pub const TPS65086_FORCESHUTDN: u32 = 0x91;
pub const TPS65086_BUCK1SLPCTRL: u32 = 0x92;
pub const TPS65086_BUCK2SLPCTRL: u32 = 0x93;
pub const TPS65086_BUCK4VID: u32 = 0x94;
pub const TPS65086_BUCK4SLPVID: u32 = 0x95;
pub const TPS65086_BUCK5VID: u32 = 0x96;
pub const TPS65086_BUCK5SLPVID: u32 = 0x97;
pub const TPS65086_BUCK6VID: u32 = 0x98;
pub const TPS65086_BUCK6SLPVID: u32 = 0x99;
pub const TPS65086_LDOA2VID: u32 = 0x9A;
pub const TPS65086_LDOA3VID: u32 = 0x9B;
pub const TPS65086_BUCK123CTRL: u32 = 0x9C;
pub const TPS65086_PG_DELAY2: u32 = 0x9D;
pub const TPS65086_PIN_EN_MASK1: u32 = 0x9E;
pub const TPS65086_PIN_EN_MASK2: u32 = 0x9F;
pub const TPS65086_SWVTT_EN: u32 = 0x9F;
pub const TPS65086_PIN_EN_OVR1: u32 = 0xA0;
pub const TPS65086_PIN_EN_OVR2: u32 = 0xA1;
pub const TPS65086_GPOCTRL: u32 = 0xA1;
pub const TPS65086_PWR_FAULT_MASK1: u32 = 0xA2;
pub const TPS65086_PWR_FAULT_MASK2: u32 = 0xA3;
pub const TPS65086_GPO1PG_CTRL1: u32 = 0xA4;
pub const TPS65086_GPO1PG_CTRL2: u32 = 0xA5;
pub const TPS65086_GPO4PG_CTRL1: u32 = 0xA6;
pub const TPS65086_GPO4PG_CTRL2: u32 = 0xA7;
pub const TPS65086_GPO2PG_CTRL1: u32 = 0xA8;
pub const TPS65086_GPO2PG_CTRL2: u32 = 0xA9;
pub const TPS65086_GPO3PG_CTRL1: u32 = 0xAA;
pub const TPS65086_GPO3PG_CTRL2: u32 = 0xAB;
pub const TPS65086_LDOA1CTRL: u32 = 0xAE;
pub const TPS65086_PG_STATUS1: u32 = 0xB0;
pub const TPS65086_PG_STATUS2: u32 = 0xB1;
pub const TPS65086_PWR_FAULT_STATUS1: u32 = 0xB2;
pub const TPS65086_PWR_FAULT_STATUS2: u32 = 0xB3;
pub const TPS65086_TEMPCRIT: u32 = 0xB4;
pub const TPS65086_TEMPHOT: u32 = 0xB5;
pub const TPS65086_OC_STATUS: u32 = 0xB6;

/* IRQ Register field definitions */
pub const TPS65086_IRQ_DIETEMP_MASK: u32 = 1 << 0;
pub const TPS65086_IRQ_SHUTDN_MASK: u32 = 1 << 3;
pub const TPS65086_IRQ_FAULT_MASK: u32 = 1 << 7;

/* DEVICEID1 Register field definitions */
pub const TPS6508640_ID: u32 = 0x00;
pub const TPS65086401_ID: u32 = 0x01;
pub const TPS6508641_ID: u32 = 0x10;
pub const TPS65086470_ID: u32 = 0x70;

/* DEVICEID2 Register field definitions */
pub const TPS65086_DEVICEID2_PART_MASK: u32 = 0x0F;
pub const TPS65086_DEVICEID2_OTP_MASK: u32 = 0x30;
pub const TPS65086_DEVICEID2_REV_MASK: u32 = 0xC0;

/* VID Masks */
pub const BUCK_VID_MASK: u32 = 0xFE;
pub const VDOA1_VID_MASK: u32 = 0x1E;
pub const VDOA23_VID_MASK: u32 = 0x0F;

/* Define the TPS65086 IRQ numbers */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tps65086_irqs {
    TPS65086_IRQ_DIETEMP = 0,
    TPS65086_IRQ_SHUTDN = 1,
    TPS65086_IRQ_FAULT = 2,
}

pub struct tps65086_regulator_config;

/**
 * struct tps65086 - state holder for the tps65086 driver
 *
 * Device data may be used to access the TPS65086 chip
 */
#[repr(C)]
pub struct tps65086 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub chip_id: u32,
    pub reg_config: *const tps65086_regulator_config,

    /* IRQ Data */
    pub irq: i32,
    pub irq_data: *mut regmap_irq_chip_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
