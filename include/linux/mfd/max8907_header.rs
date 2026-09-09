/* SPDX-License-Identifier: GPL-2.0-only */
/* Functions to access MAX8907 power management chip. */

// Dependencies supplied by the surrounding kernel translation.

pub const MAX8907_GEN_I2C_ADDR: u32 = 0x78 >> 1;
pub const MAX8907_ADC_I2C_ADDR: u32 = 0x8e >> 1;
pub const MAX8907_RTC_I2C_ADDR: u32 = 0xd0 >> 1;

/* MAX8907 register map */
pub const MAX8907_REG_SYSENSEL: u32 = 0x00;
pub const MAX8907_REG_ON_OFF_IRQ1: u32 = 0x01;
pub const MAX8907_REG_ON_OFF_IRQ1_MASK: u32 = 0x02;
pub const MAX8907_REG_ON_OFF_STAT: u32 = 0x03;
pub const MAX8907_REG_SDCTL1: u32 = 0x04;
pub const MAX8907_REG_SDSEQCNT1: u32 = 0x05;
pub const MAX8907_REG_SDV1: u32 = 0x06;
pub const MAX8907_REG_SDCTL2: u32 = 0x07;
pub const MAX8907_REG_SDSEQCNT2: u32 = 0x08;
pub const MAX8907_REG_SDV2: u32 = 0x09;
pub const MAX8907_REG_SDCTL3: u32 = 0x0A;
pub const MAX8907_REG_SDSEQCNT3: u32 = 0x0B;
pub const MAX8907_REG_SDV3: u32 = 0x0C;
pub const MAX8907_REG_ON_OFF_IRQ2: u32 = 0x0D;
pub const MAX8907_REG_ON_OFF_IRQ2_MASK: u32 = 0x0E;
pub const MAX8907_REG_RESET_CNFG: u32 = 0x0F;

/* Register groups use the same three-register layout. */
pub const MAX8907_REG_LDOCTL16: u32 = 0x10;
pub const MAX8907_REG_LDOSEQCNT16: u32 = 0x11;
pub const MAX8907_REG_LDO16VOUT: u32 = 0x12;
pub const MAX8907_REG_SDBYSEQCNT: u32 = 0x13;
pub const MAX8907_REG_LDOCTL17: u32 = 0x14;
pub const MAX8907_REG_LDOSEQCNT17: u32 = 0x15;
pub const MAX8907_REG_LDO17VOUT: u32 = 0x16;
pub const MAX8907_REG_LDOCTL1: u32 = 0x18;
pub const MAX8907_REG_LDOSEQCNT1: u32 = 0x19;
pub const MAX8907_REG_LDO1VOUT: u32 = 0x1A;
pub const MAX8907_REG_LDOCTL2: u32 = 0x1C;
pub const MAX8907_REG_LDOSEQCNT2: u32 = 0x1D;
pub const MAX8907_REG_LDO2VOUT: u32 = 0x1E;
pub const MAX8907_REG_LDOCTL3: u32 = 0x20;
pub const MAX8907_REG_LDOSEQCNT3: u32 = 0x21;
pub const MAX8907_REG_LDO3VOUT: u32 = 0x22;
pub const MAX8907_REG_LDOCTL4: u32 = 0x24;
pub const MAX8907_REG_LDOSEQCNT4: u32 = 0x25;
pub const MAX8907_REG_LDO4VOUT: u32 = 0x26;
pub const MAX8907_REG_LDOCTL5: u32 = 0x28;
pub const MAX8907_REG_LDOSEQCNT5: u32 = 0x29;
pub const MAX8907_REG_LDO5VOUT: u32 = 0x2A;
pub const MAX8907_REG_LDOCTL6: u32 = 0x2C;
pub const MAX8907_REG_LDOSEQCNT6: u32 = 0x2D;
pub const MAX8907_REG_LDO6VOUT: u32 = 0x2E;
pub const MAX8907_REG_LDOCTL7: u32 = 0x30;
pub const MAX8907_REG_LDOSEQCNT7: u32 = 0x31;
pub const MAX8907_REG_LDO7VOUT: u32 = 0x32;
pub const MAX8907_REG_LDOCTL8: u32 = 0x34;
pub const MAX8907_REG_LDOSEQCNT8: u32 = 0x35;
pub const MAX8907_REG_LDO8VOUT: u32 = 0x36;
pub const MAX8907_REG_LDOCTL9: u32 = 0x38;
pub const MAX8907_REG_LDOSEQCNT9: u32 = 0x39;
pub const MAX8907_REG_LDO9VOUT: u32 = 0x3A;
pub const MAX8907_REG_LDOCTL10: u32 = 0x3C;
pub const MAX8907_REG_LDOSEQCNT10: u32 = 0x3D;
pub const MAX8907_REG_LDO10VOUT: u32 = 0x3E;
pub const MAX8907_REG_LDOCTL11: u32 = 0x40;
pub const MAX8907_REG_LDOSEQCNT11: u32 = 0x41;
pub const MAX8907_REG_LDO11VOUT: u32 = 0x42;
pub const MAX8907_REG_LDOCTL12: u32 = 0x44;
pub const MAX8907_REG_LDOSEQCNT12: u32 = 0x45;
pub const MAX8907_REG_LDO12VOUT: u32 = 0x46;
pub const MAX8907_REG_LDOCTL13: u32 = 0x48;
pub const MAX8907_REG_LDOSEQCNT13: u32 = 0x49;
pub const MAX8907_REG_LDO13VOUT: u32 = 0x4A;
pub const MAX8907_REG_LDOCTL14: u32 = 0x4C;
pub const MAX8907_REG_LDOSEQCNT14: u32 = 0x4D;
pub const MAX8907_REG_LDO14VOUT: u32 = 0x4E;
pub const MAX8907_REG_LDOCTL15: u32 = 0x50;
pub const MAX8907_REG_LDOSEQCNT15: u32 = 0x51;
pub const MAX8907_REG_LDO15VOUT: u32 = 0x52;
pub const MAX8907_REG_OUT5VEN: u32 = 0x54;
pub const MAX8907_REG_OUT5VSEQ: u32 = 0x55;
pub const MAX8907_REG_OUT33VEN: u32 = 0x58;
pub const MAX8907_REG_OUT33VSEQ: u32 = 0x59;
pub const MAX8907_REG_LDOCTL19: u32 = 0x5C;
pub const MAX8907_REG_LDOSEQCNT19: u32 = 0x5D;
pub const MAX8907_REG_LDO19VOUT: u32 = 0x5E;
pub const MAX8907_REG_LBCNFG: u32 = 0x60;
pub const MAX8907_REG_SEQ1CNFG: u32 = 0x64;
pub const MAX8907_REG_SEQ2CNFG: u32 = 0x65;
pub const MAX8907_REG_SEQ3CNFG: u32 = 0x66;
pub const MAX8907_REG_SEQ4CNFG: u32 = 0x67;
pub const MAX8907_REG_SEQ5CNFG: u32 = 0x68;
pub const MAX8907_REG_SEQ6CNFG: u32 = 0x69;
pub const MAX8907_REG_SEQ7CNFG: u32 = 0x6A;
pub const MAX8907_REG_LDOCTL18: u32 = 0x72;
pub const MAX8907_REG_LDOSEQCNT18: u32 = 0x73;
pub const MAX8907_REG_LDO18VOUT: u32 = 0x74;
pub const MAX8907_REG_BBAT_CNFG: u32 = 0x78;
pub const MAX8907_REG_CHG_CNTL1: u32 = 0x7C;
pub const MAX8907_REG_CHG_CNTL2: u32 = 0x7D;
pub const MAX8907_REG_CHG_IRQ1: u32 = 0x7E;
pub const MAX8907_REG_CHG_IRQ2: u32 = 0x7F;
pub const MAX8907_REG_CHG_IRQ1_MASK: u32 = 0x80;
pub const MAX8907_REG_CHG_IRQ2_MASK: u32 = 0x81;
pub const MAX8907_REG_CHG_STAT: u32 = 0x82;
pub const MAX8907_REG_WLED_MODE_CNTL: u32 = 0x84;
pub const MAX8907_REG_ILED_CNTL: u32 = 0x84;
pub const MAX8907_REG_II1RR: u32 = 0x8E;
pub const MAX8907_REG_II2RR: u32 = 0x8F;
pub const MAX8907_REG_LDOCTL20: u32 = 0x9C;
pub const MAX8907_REG_LDOSEQCNT20: u32 = 0x9D;
pub const MAX8907_REG_LDO20VOUT: u32 = 0x9E;

/* RTC register map */
pub const MAX8907_REG_RTC_SEC: u32 = 0x00;
pub const MAX8907_REG_RTC_MIN: u32 = 0x01;
pub const MAX8907_REG_RTC_HOURS: u32 = 0x02;
pub const MAX8907_REG_RTC_WEEKDAY: u32 = 0x03;
pub const MAX8907_REG_RTC_DATE: u32 = 0x04;
pub const MAX8907_REG_RTC_MONTH: u32 = 0x05;
pub const MAX8907_REG_RTC_YEAR1: u32 = 0x06;
pub const MAX8907_REG_RTC_YEAR2: u32 = 0x07;
pub const MAX8907_REG_ALARM0_SEC: u32 = 0x08;
pub const MAX8907_REG_ALARM0_MIN: u32 = 0x09;
pub const MAX8907_REG_ALARM0_HOURS: u32 = 0x0A;
pub const MAX8907_REG_ALARM0_WEEKDAY: u32 = 0x0B;
pub const MAX8907_REG_ALARM0_DATE: u32 = 0x0C;
pub const MAX8907_REG_ALARM0_MONTH: u32 = 0x0D;
pub const MAX8907_REG_ALARM0_YEAR1: u32 = 0x0E;
pub const MAX8907_REG_ALARM0_YEAR2: u32 = 0x0F;
pub const MAX8907_REG_ALARM1_SEC: u32 = 0x10;
pub const MAX8907_REG_ALARM1_MIN: u32 = 0x11;
pub const MAX8907_REG_ALARM1_HOURS: u32 = 0x12;
pub const MAX8907_REG_ALARM1_WEEKDAY: u32 = 0x13;
pub const MAX8907_REG_ALARM1_DATE: u32 = 0x14;
pub const MAX8907_REG_ALARM1_MONTH: u32 = 0x15;
pub const MAX8907_REG_ALARM1_YEAR1: u32 = 0x16;
pub const MAX8907_REG_ALARM1_YEAR2: u32 = 0x17;
pub const MAX8907_REG_ALARM0_CNTL: u32 = 0x18;
pub const MAX8907_REG_ALARM1_CNTL: u32 = 0x19;
pub const MAX8907_REG_RTC_STATUS: u32 = 0x1A;
pub const MAX8907_REG_RTC_CNTL: u32 = 0x1B;
pub const MAX8907_REG_RTC_IRQ: u32 = 0x1C;
pub const MAX8907_REG_RTC_IRQ_MASK: u32 = 0x1D;
pub const MAX8907_REG_MPL_CNTL: u32 = 0x1E;

/* ADC and Touch Screen Controller register map */
pub const MAX8907_CTL: u32 = 0;
pub const MAX8907_SEQCNT: u32 = 1;
pub const MAX8907_VOUT: u32 = 2;

/* mask bit fields */
pub const MAX8907_MASK_LDO_SEQ: u32 = 0x1C;
pub const MAX8907_MASK_LDO_EN: u32 = 0x01;
pub const MAX8907_MASK_VBBATTCV: u32 = 0x03;
pub const MAX8907_MASK_OUT5V_VINEN: u32 = 0x10;
pub const MAX8907_MASK_OUT5V_ENSRC: u32 = 0x0E;
pub const MAX8907_MASK_OUT5V_EN: u32 = 0x01;
pub const MAX8907_MASK_POWER_OFF: u32 = 0x40;

/* Regulator IDs */
pub const MAX8907_MBATT: u32 = 0;
pub const MAX8907_SD1: u32 = 1;
pub const MAX8907_SD2: u32 = 2;
pub const MAX8907_SD3: u32 = 3;
pub const MAX8907_LDO1: u32 = 4;
pub const MAX8907_LDO2: u32 = 5;
pub const MAX8907_LDO3: u32 = 6;
pub const MAX8907_LDO4: u32 = 7;
pub const MAX8907_LDO5: u32 = 8;
pub const MAX8907_LDO6: u32 = 9;
pub const MAX8907_LDO7: u32 = 10;
pub const MAX8907_LDO8: u32 = 11;
pub const MAX8907_LDO9: u32 = 12;
pub const MAX8907_LDO10: u32 = 13;
pub const MAX8907_LDO11: u32 = 14;
pub const MAX8907_LDO12: u32 = 15;
pub const MAX8907_LDO13: u32 = 16;
pub const MAX8907_LDO14: u32 = 17;
pub const MAX8907_LDO15: u32 = 18;
pub const MAX8907_LDO16: u32 = 19;
pub const MAX8907_LDO17: u32 = 20;
pub const MAX8907_LDO18: u32 = 21;
pub const MAX8907_LDO19: u32 = 22;
pub const MAX8907_LDO20: u32 = 23;
pub const MAX8907_OUT5V: u32 = 24;
pub const MAX8907_OUT33V: u32 = 25;
pub const MAX8907_BBAT: u32 = 26;
pub const MAX8907_SDBY: u32 = 27;
pub const MAX8907_VRTC: u32 = 28;
pub const MAX8907_NUM_REGULATORS: u32 = MAX8907_VRTC + 1;

/* IRQ definitions */
pub const MAX8907_IRQ_VCHG_DC_OVP: u32 = 0;
pub const MAX8907_IRQ_VCHG_DC_F: u32 = 1;
pub const MAX8907_IRQ_VCHG_DC_R: u32 = 2;
pub const MAX8907_IRQ_VCHG_THM_OK_R: u32 = 3;
pub const MAX8907_IRQ_VCHG_THM_OK_F: u32 = 4;
pub const MAX8907_IRQ_VCHG_MBATTLOW_F: u32 = 5;
pub const MAX8907_IRQ_VCHG_MBATTLOW_R: u32 = 6;
pub const MAX8907_IRQ_VCHG_RST: u32 = 7;
pub const MAX8907_IRQ_VCHG_DONE: u32 = 8;
pub const MAX8907_IRQ_VCHG_TOPOFF: u32 = 9;
pub const MAX8907_IRQ_VCHG_TMR_FAULT: u32 = 10;
pub const MAX8907_IRQ_GPM_RSTIN: u32 = 0;
pub const MAX8907_IRQ_GPM_MPL: u32 = 1;
pub const MAX8907_IRQ_GPM_SW_3SEC: u32 = 2;
pub const MAX8907_IRQ_GPM_EXTON_F: u32 = 3;
pub const MAX8907_IRQ_GPM_EXTON_R: u32 = 4;
pub const MAX8907_IRQ_GPM_SW_1SEC: u32 = 5;
pub const MAX8907_IRQ_GPM_SW_F: u32 = 6;
pub const MAX8907_IRQ_GPM_SW_R: u32 = 7;
pub const MAX8907_IRQ_GPM_SYSCKEN_F: u32 = 8;
pub const MAX8907_IRQ_GPM_SYSCKEN_R: u32 = 9;
pub const MAX8907_IRQ_RTC_ALARM1: u32 = 0;
pub const MAX8907_IRQ_RTC_ALARM0: u32 = 1;

#[repr(C)]
pub struct max8907_platform_data {
    pub init_data: [*mut regulator_init_data; MAX8907_NUM_REGULATORS as usize],
    pub pm_off: bool,
}

pub enum regmap_irq_chips_data {}

#[repr(C)]
pub struct max8907 {
    pub dev: *mut device,
    pub irq_lock: mutex,
    pub i2c_gen: *mut i2c_client,
    pub i2c_rtc: *mut i2c_client,
    pub regmap_gen: *mut regmap,
    pub regmap_rtc: *mut regmap,
    pub irqc_chg: *mut regmap_irq_chip_data,
    pub irqc_on_off: *mut regmap_irq_chip_data,
    pub irqc_rtc: *mut regmap_irq_chip_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
