/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MFD core driver for Ricoh RN5T618 PMIC
 *
 * Copyright (C) 2014 Beniamino Galvani <b.galvani@gmail.com>
 */

/* Dependency supplied by the surrounding kernel translation. */
use crate::regmap::{Regmap, RegmapIrqChipData};
use crate::device::Device;

pub const RN5T618_LSIVER: u8 = 0x00;
pub const RN5T618_OTPVER: u8 = 0x01;
pub const RN5T618_IODAC: u8 = 0x02;
pub const RN5T618_VINDAC: u8 = 0x03;
pub const RN5T618_OUT32KEN: u8 = 0x05;
pub const RN5T618_CPUCNT: u8 = 0x06;
pub const RN5T618_PSWR: u8 = 0x07;
pub const RN5T618_PONHIS: u8 = 0x09;
pub const RN5T618_POFFHIS: u8 = 0x0a;
pub const RN5T618_WATCHDOG: u8 = 0x0b;
pub const RN5T618_WATCHDOGCNT: u8 = 0x0c;
pub const RN5T618_PWRFUNC: u8 = 0x0d;
pub const RN5T618_SLPCNT: u8 = 0x0e;
pub const RN5T618_REPCNT: u8 = 0x0f;
pub const RN5T618_PWRONTIMSET: u8 = 0x10;
pub const RN5T618_NOETIMSETCNT: u8 = 0x11;
pub const RN5T618_PWRIREN: u8 = 0x12;
pub const RN5T618_PWRIRQ: u8 = 0x13;
pub const RN5T618_PWRMON: u8 = 0x14;
pub const RN5T618_PWRIRSEL: u8 = 0x15;
pub const RN5T618_DC1_SLOT: u8 = 0x16;
pub const RN5T618_DC2_SLOT: u8 = 0x17;
pub const RN5T618_DC3_SLOT: u8 = 0x18;
pub const RN5T618_DC4_SLOT: u8 = 0x19;
pub const RN5T618_LDO1_SLOT: u8 = 0x1b;
pub const RN5T618_LDO2_SLOT: u8 = 0x1c;
pub const RN5T618_LDO3_SLOT: u8 = 0x1d;
pub const RN5T618_LDO4_SLOT: u8 = 0x1e;
pub const RN5T618_LDO5_SLOT: u8 = 0x1f;
pub const RN5T618_PSO0_SLOT: u8 = 0x25;
pub const RN5T618_PSO1_SLOT: u8 = 0x26;
pub const RN5T618_PSO2_SLOT: u8 = 0x27;
pub const RN5T618_PSO3_SLOT: u8 = 0x28;
pub const RN5T618_LDORTC1_SLOT: u8 = 0x2a;
pub const RN5T618_DC1CTL: u8 = 0x2c;
pub const RN5T618_DC1CTL2: u8 = 0x2d;
pub const RN5T618_DC2CTL: u8 = 0x2e;
pub const RN5T618_DC2CTL2: u8 = 0x2f;
pub const RN5T618_DC3CTL: u8 = 0x30;
pub const RN5T618_DC3CTL2: u8 = 0x31;
pub const RN5T618_DC4CTL: u8 = 0x32;
pub const RN5T618_DC4CTL2: u8 = 0x33;
pub const RN5T618_DC5CTL: u8 = 0x34;
pub const RN5T618_DC5CTL2: u8 = 0x35;
pub const RN5T618_DC1DAC: u8 = 0x36;
pub const RN5T618_DC2DAC: u8 = 0x37;
pub const RN5T618_DC3DAC: u8 = 0x38;
pub const RN5T618_DC4DAC: u8 = 0x39;
pub const RN5T618_DC5DAC: u8 = 0x3a;
pub const RN5T618_DC1DAC_SLP: u8 = 0x3b;
pub const RN5T618_DC2DAC_SLP: u8 = 0x3c;
pub const RN5T618_DC3DAC_SLP: u8 = 0x3d;
pub const RN5T618_DC4DAC_SLP: u8 = 0x3e;
pub const RN5T618_DCIREN: u8 = 0x40;
pub const RN5T618_DCIRQ: u8 = 0x41;
pub const RN5T618_DCIRMON: u8 = 0x42;
pub const RN5T618_LDOEN1: u8 = 0x44;
pub const RN5T618_LDOEN2: u8 = 0x45;
pub const RN5T618_LDODIS: u8 = 0x46;
pub const RN5T618_LDO1DAC: u8 = 0x4c;
pub const RN5T618_LDO2DAC: u8 = 0x4d;
pub const RN5T618_LDO3DAC: u8 = 0x4e;
pub const RN5T618_LDO4DAC: u8 = 0x4f;
pub const RN5T618_LDO5DAC: u8 = 0x50;
pub const RN5T618_LDO6DAC: u8 = 0x51;
pub const RN5T618_LDO7DAC: u8 = 0x52;
pub const RN5T618_LDO8DAC: u8 = 0x53;
pub const RN5T618_LDO9DAC: u8 = 0x54;
pub const RN5T618_LDO10DAC: u8 = 0x55;
pub const RN5T618_LDORTCDAC: u8 = 0x56;
pub const RN5T618_LDORTC2DAC: u8 = 0x57;
pub const RN5T618_LDO1DAC_SLP: u8 = 0x58;
pub const RN5T618_LDO2DAC_SLP: u8 = 0x59;
pub const RN5T618_LDO3DAC_SLP: u8 = 0x5a;
pub const RN5T618_LDO4DAC_SLP: u8 = 0x5b;
pub const RN5T618_LDO5DAC_SLP: u8 = 0x5c;
pub const RN5T618_ADCCNT1: u8 = 0x64;
pub const RN5T618_ADCCNT2: u8 = 0x65;
pub const RN5T618_ADCCNT3: u8 = 0x66;
pub const RN5T618_ILIMDATAH: u8 = 0x68;
pub const RN5T618_ILIMDATAL: u8 = 0x69;
pub const RN5T618_VBATDATAH: u8 = 0x6a;
pub const RN5T618_VBATDATAL: u8 = 0x6b;
pub const RN5T618_VADPDATAH: u8 = 0x6c;
pub const RN5T618_VADPDATAL: u8 = 0x6d;
pub const RN5T618_VUSBDATAH: u8 = 0x6e;
pub const RN5T618_VUSBDATAL: u8 = 0x6f;
pub const RN5T618_VSYSDATAH: u8 = 0x70;
pub const RN5T618_VSYSDATAL: u8 = 0x71;
pub const RN5T618_VTHMDATAH: u8 = 0x72;
pub const RN5T618_VTHMDATAL: u8 = 0x73;
pub const RN5T618_AIN1DATAH: u8 = 0x74;
pub const RN5T618_AIN1DATAL: u8 = 0x75;
pub const RN5T618_AIN0DATAH: u8 = 0x76;
pub const RN5T618_AIN0DATAL: u8 = 0x77;
pub const RN5T618_ILIMTHL: u8 = 0x78;
pub const RN5T618_ILIMTHH: u8 = 0x79;
pub const RN5T618_VBATTHL: u8 = 0x7a;
pub const RN5T618_VBATTHH: u8 = 0x7b;
pub const RN5T618_VADPTHL: u8 = 0x7c;
pub const RN5T618_VADPTHH: u8 = 0x7d;
pub const RN5T618_VUSBTHL: u8 = 0x7e;
pub const RN5T618_VUSBTHH: u8 = 0x7f;
pub const RN5T618_VSYSTHL: u8 = 0x80;
pub const RN5T618_VSYSTHH: u8 = 0x81;
pub const RN5T618_VTHMTHL: u8 = 0x82;
pub const RN5T618_VTHMTHH: u8 = 0x83;
pub const RN5T618_AIN1THL: u8 = 0x84;
pub const RN5T618_AIN1THH: u8 = 0x85;
pub const RN5T618_AIN0THL: u8 = 0x86;
pub const RN5T618_AIN0THH: u8 = 0x87;
pub const RN5T618_EN_ADCIR1: u8 = 0x88;
pub const RN5T618_EN_ADCIR2: u8 = 0x89;
pub const RN5T618_EN_ADCIR3: u8 = 0x8a;
pub const RN5T618_IR_ADC1: u8 = 0x8c;
pub const RN5T618_IR_ADC2: u8 = 0x8d;
pub const RN5T618_IR_ADC3: u8 = 0x8e;
pub const RN5T618_IOSEL: u8 = 0x90;
pub const RN5T618_IOOUT: u8 = 0x91;
pub const RN5T618_GPEDGE1: u8 = 0x92;
pub const RN5T618_GPEDGE2: u8 = 0x93;
pub const RN5T618_EN_GPIR: u8 = 0x94;
pub const RN5T618_IR_GPR: u8 = 0x95;
pub const RN5T618_IR_GPF: u8 = 0x96;
pub const RN5T618_MON_IOIN: u8 = 0x97;
pub const RN5T618_GPLED_FUNC: u8 = 0x98;
pub const RN5T618_INTPOL: u8 = 0x9c;
pub const RN5T618_INTEN: u8 = 0x9d;
pub const RN5T618_INTMON: u8 = 0x9e;
pub const RN5T618_RTC_SECONDS: u8 = 0xA0;
pub const RN5T618_RTC_MDAY: u8 = 0xA4;
pub const RN5T618_RTC_MONTH: u8 = 0xA5;
pub const RN5T618_RTC_YEAR: u8 = 0xA6;
pub const RN5T618_RTC_ADJUST: u8 = 0xA7;
pub const RN5T618_RTC_ALARM_Y_SEC: u8 = 0xA8;
pub const RN5T618_RTC_DAL_MONTH: u8 = 0xAC;
pub const RN5T618_RTC_CTRL1: u8 = 0xAE;
pub const RN5T618_RTC_CTRL2: u8 = 0xAF;
pub const RN5T618_PREVINDAC: u8 = 0xb0;
pub const RN5T618_BATDAC: u8 = 0xb1;
pub const RN5T618_CHGCTL1: u8 = 0xb3;
pub const RN5T618_CHGCTL2: u8 = 0xb4;
pub const RN5T618_VSYSSET: u8 = 0xb5;
pub const RN5T618_REGISET1: u8 = 0xb6;
pub const RN5T618_REGISET2: u8 = 0xb7;
pub const RN5T618_CHGISET: u8 = 0xb8;
pub const RN5T618_TIMSET: u8 = 0xb9;
pub const RN5T618_BATSET1: u8 = 0xba;
pub const RN5T618_BATSET2: u8 = 0xbb;
pub const RN5T618_DIESET: u8 = 0xbc;
pub const RN5T618_CHGSTATE: u8 = 0xbd;
pub const RN5T618_CHGCTRL_IRFMASK: u8 = 0xbe;
pub const RN5T618_CHGSTAT_IRFMASK1: u8 = 0xbf;
pub const RN5T618_CHGSTAT_IRFMASK2: u8 = 0xc0;
pub const RN5T618_CHGERR_IRFMASK: u8 = 0xc1;
pub const RN5T618_CHGCTRL_IRR: u8 = 0xc2;
pub const RN5T618_CHGSTAT_IRR1: u8 = 0xc3;
pub const RN5T618_CHGSTAT_IRR2: u8 = 0xc4;
pub const RN5T618_CHGERR_IRR: u8 = 0xc5;
pub const RN5T618_CHGCTRL_MONI: u8 = 0xc6;
pub const RN5T618_CHGSTAT_MONI1: u8 = 0xc7;
pub const RN5T618_CHGSTAT_MONI2: u8 = 0xc8;
pub const RN5T618_CHGERR_MONI: u8 = 0xc9;
pub const RN5T618_CHGCTRL_DETMOD1: u8 = 0xca;
pub const RN5T618_CHGCTRL_DETMOD2: u8 = 0xcb;
pub const RN5T618_CHGSTAT_DETMOD1: u8 = 0xcc;
pub const RN5T618_CHGSTAT_DETMOD2: u8 = 0xcd;
pub const RN5T618_CHGSTAT_DETMOD3: u8 = 0xce;
pub const RN5T618_CHGERR_DETMOD1: u8 = 0xcf;
pub const RN5T618_CHGERR_DETMOD2: u8 = 0xd0;
pub const RN5T618_CHGOSCCTL: u8 = 0xd4;
pub const RN5T618_CHGOSCSCORESET1: u8 = 0xd5;
pub const RN5T618_CHGOSCSCORESET2: u8 = 0xd6;
pub const RN5T618_CHGOSCSCORESET3: u8 = 0xd7;
pub const RN5T618_CHGOSCFREQSET1: u8 = 0xd8;
pub const RN5T618_CHGOSCFREQSET2: u8 = 0xd9;
pub const RN5T618_GCHGDET: u8 = 0xda;
pub const RN5T618_CONTROL: u8 = 0xe0;
pub const RN5T618_SOC: u8 = 0xe1;
pub const RN5T618_RE_CAP_H: u8 = 0xe2;
pub const RN5T618_RE_CAP_L: u8 = 0xe3;
pub const RN5T618_FA_CAP_H: u8 = 0xe4;
pub const RN5T618_FA_CAP_L: u8 = 0xe5;
pub const RN5T618_AGE: u8 = 0xe6;
pub const RN5T618_TT_EMPTY_H: u8 = 0xe7;
pub const RN5T618_TT_EMPTY_L: u8 = 0xe8;
pub const RN5T618_TT_FULL_H: u8 = 0xe9;
pub const RN5T618_TT_FULL_L: u8 = 0xea;
pub const RN5T618_VOLTAGE_1: u8 = 0xeb;
pub const RN5T618_VOLTAGE_0: u8 = 0xec;
pub const RN5T618_TEMP_1: u8 = 0xed;
pub const RN5T618_TEMP_0: u8 = 0xee;
pub const RN5T618_CC_CTRL: u8 = 0xef;
pub const RN5T618_CC_COUNT2: u8 = 0xf0;
pub const RN5T618_CC_COUNT1: u8 = 0xf1;
pub const RN5T618_CC_COUNT0: u8 = 0xf2;
pub const RN5T618_CC_SUMREG3: u8 = 0xf3;
pub const RN5T618_CC_SUMREG2: u8 = 0xf4;
pub const RN5T618_CC_SUMREG1: u8 = 0xf5;
pub const RN5T618_CC_SUMREG0: u8 = 0xf6;
pub const RN5T618_CC_OFFREG1: u8 = 0xf7;
pub const RN5T618_CC_OFFREG0: u8 = 0xf8;
pub const RN5T618_CC_GAINREG1: u8 = 0xf9;
pub const RN5T618_CC_GAINREG0: u8 = 0xfa;
pub const RN5T618_CC_AVEREG1: u8 = 0xfb;
pub const RN5T618_CC_AVEREG0: u8 = 0xfc;
pub const RN5T618_MAX_REG: u8 = 0xfc;

pub const RN5T618_REPCNT_REPWRON: u8 = 1 << 0;
pub const RN5T618_SLPCNT_SWPWROFF: u8 = 1 << 0;
pub const RN5T618_WATCHDOG_WDOGEN: u8 = 1 << 2;
pub const RN5T618_WATCHDOG_WDOGTIM_M: u8 = (1 << 0) | (1 << 1);
pub const RN5T618_WATCHDOG_WDOGTIM_S: u8 = 0;
pub const RN5T618_PWRIRQ_IR_WDOG: u8 = 1 << 6;
pub const RN5T618_POFFHIS_PWRON: u8 = 1 << 0;
pub const RN5T618_POFFHIS_TSHUT: u8 = 1 << 1;
pub const RN5T618_POFFHIS_VINDET: u8 = 1 << 2;
pub const RN5T618_POFFHIS_IODET: u8 = 1 << 3;
pub const RN5T618_POFFHIS_CPU: u8 = 1 << 4;
pub const RN5T618_POFFHIS_WDG: u8 = 1 << 5;
pub const RN5T618_POFFHIS_DCLIM: u8 = 1 << 6;
pub const RN5T618_POFFHIS_N_OE: u8 = 1 << 7;

#[repr(usize)]
pub enum Rn5t618Regulator {
    RN5T618_DCDC1,
    RN5T618_DCDC2,
    RN5T618_DCDC3,
    RN5T618_DCDC4,
    RN5T618_DCDC5,
    RN5T618_LDO1,
    RN5T618_LDO2,
    RN5T618_LDO3,
    RN5T618_LDO4,
    RN5T618_LDO5,
    RN5T618_LDO6,
    RN5T618_LDO7,
    RN5T618_LDO8,
    RN5T618_LDO9,
    RN5T618_LDO10,
    RN5T618_LDORTC1,
    RN5T618_LDORTC2,
    RN5T618_REG_NUM,
}

#[repr(usize)]
pub enum Rn5t618Variant {
    RN5T567 = 0,
    RN5T618,
    RC5T619,
}

/* RN5T618 IRQ definitions */
#[repr(usize)]
pub enum Rn5t618Irq {
    RN5T618_IRQ_SYS = 0,
    RN5T618_IRQ_DCDC,
    RN5T618_IRQ_RTC,
    RN5T618_IRQ_ADC,
    RN5T618_IRQ_GPIO,
    RN5T618_IRQ_CHG,
    RN5T618_NR_IRQS,
}

#[repr(C)]
pub struct rn5t618 {
    pub regmap: *mut Regmap,
    pub dev: *mut Device,
    pub variant: core::ffi::c_long,
    pub irq: core::ffi::c_int,
    pub irq_data: *mut RegmapIrqChipData,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
