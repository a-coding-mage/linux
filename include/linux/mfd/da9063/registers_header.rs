/* SPDX-License-Identifier: GPL-2.0+ */
/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Registers definition for DA9063 modules
 *
 * Copyright 2012 Dialog Semiconductor Ltd.
 *
 * Author: Michal Hajduk, Dialog Semiconductor
 * Author: Krystian Garbaciak, Dialog Semiconductor
 */


pub const DA9063_I2C_PAGE_SEL_SHIFT: u32 = 1;
pub const DA9063_EVENT_REG_NUM: u32 = 4;

/* Page selection I2C or SPI always in the begining of any page. */
/* Page 0 : I2C access 0x000 - 0x0FF	SPI access 0x000 - 0x07F */
/* Page 1 :				SPI access 0x080 - 0x0FF */
/* Page 2 : I2C access 0x100 - 0x1FF	SPI access 0x100 - 0x17F */
/* Page 3 :				SPI access 0x180 - 0x1FF */
pub const DA9063_REG_PAGE_CON: u32 = 0x00;

/* System Control and Event Registers */
pub const DA9063_REG_STATUS_A: u32 = 0x01;
pub const DA9063_REG_STATUS_B: u32 = 0x02;
pub const DA9063_REG_STATUS_C: u32 = 0x03;
pub const DA9063_REG_STATUS_D: u32 = 0x04;
pub const DA9063_REG_FAULT_LOG: u32 = 0x05;
pub const DA9063_REG_EVENT_A: u32 = 0x06;
pub const DA9063_REG_EVENT_B: u32 = 0x07;
pub const DA9063_REG_EVENT_C: u32 = 0x08;
pub const DA9063_REG_EVENT_D: u32 = 0x09;
pub const DA9063_REG_IRQ_MASK_A: u32 = 0x0A;
pub const DA9063_REG_IRQ_MASK_B: u32 = 0x0B;
pub const DA9063_REG_IRQ_MASK_C: u32 = 0x0C;
pub const DA9063_REG_IRQ_MASK_D: u32 = 0x0D;
pub const DA9063_REG_CONTROL_A: u32 = 0x0E;
pub const DA9063_REG_CONTROL_B: u32 = 0x0F;
pub const DA9063_REG_CONTROL_C: u32 = 0x10;
pub const DA9063_REG_CONTROL_D: u32 = 0x11;
pub const DA9063_REG_CONTROL_E: u32 = 0x12;
pub const DA9063_REG_CONTROL_F: u32 = 0x13;
pub const DA9063_REG_PD_DIS: u32 = 0x14;

/* GPIO Control Registers */
pub const DA9063_REG_GPIO_0_1: u32 = 0x15;
pub const DA9063_REG_GPIO_2_3: u32 = 0x16;
pub const DA9063_REG_GPIO_4_5: u32 = 0x17;
pub const DA9063_REG_GPIO_6_7: u32 = 0x18;
pub const DA9063_REG_GPIO_8_9: u32 = 0x19;
pub const DA9063_REG_GPIO_10_11: u32 = 0x1A;
pub const DA9063_REG_GPIO_12_13: u32 = 0x1B;
pub const DA9063_REG_GPIO_14_15: u32 = 0x1C;
pub const DA9063_REG_GPIO_MODE0_7: u32 = 0x1D;
pub const DA9063_REG_GPIO_MODE8_15: u32 = 0x1E;
pub const DA9063_REG_SWITCH_CONT: u32 = 0x1F;

/* Regulator Control Registers */
pub const DA9063_REG_BCORE2_CONT: u32 = 0x20;
pub const DA9063_REG_BCORE1_CONT: u32 = 0x21;
pub const DA9063_REG_BPRO_CONT: u32 = 0x22;
pub const DA9063_REG_BMEM_CONT: u32 = 0x23;
pub const DA9063_REG_BIO_CONT: u32 = 0x24;
pub const DA9063_REG_BPERI_CONT: u32 = 0x25;
pub const DA9063_REG_LDO1_CONT: u32 = 0x26;
pub const DA9063_REG_LDO2_CONT: u32 = 0x27;
pub const DA9063_REG_LDO3_CONT: u32 = 0x28;
pub const DA9063_REG_LDO4_CONT: u32 = 0x29;
pub const DA9063_REG_LDO5_CONT: u32 = 0x2A;
pub const DA9063_REG_LDO6_CONT: u32 = 0x2B;
pub const DA9063_REG_LDO7_CONT: u32 = 0x2C;
pub const DA9063_REG_LDO8_CONT: u32 = 0x2D;
pub const DA9063_REG_LDO9_CONT: u32 = 0x2E;
pub const DA9063_REG_LDO10_CONT: u32 = 0x2F;
pub const DA9063_REG_LDO11_CONT: u32 = 0x30;
pub const DA9063_REG_SUPPLIES: u32 = 0x31;
pub const DA9063_REG_DVC_1: u32 = 0x32;
pub const DA9063_REG_DVC_2: u32 = 0x33;

/* GP-ADC Control Registers */
pub const DA9063_REG_ADC_MAN: u32 = 0x34;
pub const DA9063_REG_ADC_CONT: u32 = 0x35;
pub const DA9063_REG_VSYS_MON: u32 = 0x36;
pub const DA9063_REG_ADC_RES_L: u32 = 0x37;
pub const DA9063_REG_ADC_RES_H: u32 = 0x38;
pub const DA9063_REG_VSYS_RES: u32 = 0x39;
pub const DA9063_REG_ADCIN1_RES: u32 = 0x3A;
pub const DA9063_REG_ADCIN2_RES: u32 = 0x3B;
pub const DA9063_REG_ADCIN3_RES: u32 = 0x3C;
pub const DA9063_REG_MON_A8_RES: u32 = 0x3D;
pub const DA9063_REG_MON_A9_RES: u32 = 0x3E;
pub const DA9063_REG_MON_A10_RES: u32 = 0x3F;

/* RTC Calendar and Alarm Registers */
pub const DA9063_REG_COUNT_S: u32 = 0x40;
pub const DA9063_REG_COUNT_MI: u32 = 0x41;
pub const DA9063_REG_COUNT_H: u32 = 0x42;
pub const DA9063_REG_COUNT_D: u32 = 0x43;
pub const DA9063_REG_COUNT_MO: u32 = 0x44;
pub const DA9063_REG_COUNT_Y: u32 = 0x45;

pub const DA9063_AD_REG_ALARM_MI: u32 = 0x46;
pub const DA9063_AD_REG_ALARM_H: u32 = 0x47;
pub const DA9063_AD_REG_ALARM_D: u32 = 0x48;
pub const DA9063_AD_REG_ALARM_MO: u32 = 0x49;
pub const DA9063_AD_REG_ALARM_Y: u32 = 0x4A;
pub const DA9063_AD_REG_SECOND_A: u32 = 0x4B;
pub const DA9063_AD_REG_SECOND_B: u32 = 0x4C;
pub const DA9063_AD_REG_SECOND_C: u32 = 0x4D;
pub const DA9063_AD_REG_SECOND_D: u32 = 0x4E;

pub const DA9063_BB_REG_ALARM_S: u32 = 0x46;
pub const DA9063_BB_REG_ALARM_MI: u32 = 0x47;
pub const DA9063_BB_REG_ALARM_H: u32 = 0x48;
pub const DA9063_BB_REG_ALARM_D: u32 = 0x49;
pub const DA9063_BB_REG_ALARM_MO: u32 = 0x4A;
pub const DA9063_BB_REG_ALARM_Y: u32 = 0x4B;
pub const DA9063_BB_REG_SECOND_A: u32 = 0x4C;
pub const DA9063_BB_REG_SECOND_B: u32 = 0x4D;
pub const DA9063_BB_REG_SECOND_C: u32 = 0x4E;
pub const DA9063_BB_REG_SECOND_D: u32 = 0x4F;

/* Sequencer Control Registers */
pub const DA9063_REG_SEQ: u32 = 0x81;
pub const DA9063_REG_SEQ_TIMER: u32 = 0x82;
pub const DA9063_REG_ID_2_1: u32 = 0x83;
pub const DA9063_REG_ID_4_3: u32 = 0x84;
pub const DA9063_REG_ID_6_5: u32 = 0x85;
pub const DA9063_REG_ID_8_7: u32 = 0x86;
pub const DA9063_REG_ID_10_9: u32 = 0x87;
pub const DA9063_REG_ID_12_11: u32 = 0x88;
pub const DA9063_REG_ID_14_13: u32 = 0x89;
pub const DA9063_REG_ID_16_15: u32 = 0x8A;
pub const DA9063_REG_ID_18_17: u32 = 0x8B;
pub const DA9063_REG_ID_20_19: u32 = 0x8C;
pub const DA9063_REG_ID_22_21: u32 = 0x8D;
pub const DA9063_REG_ID_24_23: u32 = 0x8E;
pub const DA9063_REG_ID_26_25: u32 = 0x8F;
pub const DA9063_REG_ID_28_27: u32 = 0x90;
pub const DA9063_REG_ID_30_29: u32 = 0x91;
pub const DA9063_REG_ID_32_31: u32 = 0x92;
pub const DA9063_REG_SEQ_A: u32 = 0x95;
pub const DA9063_REG_SEQ_B: u32 = 0x96;
pub const DA9063_REG_WAIT: u32 = 0x97;
pub const DA9063_REG_EN_32K: u32 = 0x98;
pub const DA9063_REG_RESET: u32 = 0x99;

/* Regulator Setting Registers */
pub const DA9063_REG_BUCK_ILIM_A: u32 = 0x9A;
pub const DA9063_REG_BUCK_ILIM_B: u32 = 0x9B;
pub const DA9063_REG_BUCK_ILIM_C: u32 = 0x9C;
pub const DA9063_REG_BCORE2_CFG: u32 = 0x9D;
pub const DA9063_REG_BCORE1_CFG: u32 = 0x9E;
pub const DA9063_REG_BPRO_CFG: u32 = 0x9F;
pub const DA9063_REG_BIO_CFG: u32 = 0xA0;
pub const DA9063_REG_BMEM_CFG: u32 = 0xA1;
pub const DA9063_REG_BPERI_CFG: u32 = 0xA2;
pub const DA9063_REG_VBCORE2_A: u32 = 0xA3;
pub const DA9063_REG_VBCORE1_A: u32 = 0xA4;
pub const DA9063_REG_VBPRO_A: u32 = 0xA5;
pub const DA9063_REG_VBMEM_A: u32 = 0xA6;
pub const DA9063_REG_VBIO_A: u32 = 0xA7;
pub const DA9063_REG_VBPERI_A: u32 = 0xA8;
pub const DA9063_REG_VLDO1_A: u32 = 0xA9;
pub const DA9063_REG_VLDO2_A: u32 = 0xAA;
pub const DA9063_REG_VLDO3_A: u32 = 0xAB;
pub const DA9063_REG_VLDO4_A: u32 = 0xAC;
pub const DA9063_REG_VLDO5_A: u32 = 0xAD;
pub const DA9063_REG_VLDO6_A: u32 = 0xAE;
pub const DA9063_REG_VLDO7_A: u32 = 0xAF;
pub const DA9063_REG_VLDO8_A: u32 = 0xB0;
pub const DA9063_REG_VLDO9_A: u32 = 0xB1;
pub const DA9063_REG_VLDO10_A: u32 = 0xB2;
pub const DA9063_REG_VLDO11_A: u32 = 0xB3;
pub const DA9063_REG_VBCORE2_B: u32 = 0xB4;
pub const DA9063_REG_VBCORE1_B: u32 = 0xB5;
pub const DA9063_REG_VBPRO_B: u32 = 0xB6;
pub const DA9063_REG_VBMEM_B: u32 = 0xB7;
pub const DA9063_REG_VBIO_B: u32 = 0xB8;
pub const DA9063_REG_VBPERI_B: u32 = 0xB9;
pub const DA9063_REG_VLDO1_B: u32 = 0xBA;
pub const DA9063_REG_VLDO2_B: u32 = 0xBB;
pub const DA9063_REG_VLDO3_B: u32 = 0xBC;
pub const DA9063_REG_VLDO4_B: u32 = 0xBD;
pub const DA9063_REG_VLDO5_B: u32 = 0xBE;
pub const DA9063_REG_VLDO6_B: u32 = 0xBF;
pub const DA9063_REG_VLDO7_B: u32 = 0xC0;
pub const DA9063_REG_VLDO8_B: u32 = 0xC1;
pub const DA9063_REG_VLDO9_B: u32 = 0xC2;
pub const DA9063_REG_VLDO10_B: u32 = 0xC3;
pub const DA9063_REG_VLDO11_B: u32 = 0xC4;

/* Backup Battery Charger Control Register */
pub const DA9063_REG_BBAT_CONT: u32 = 0xC5;

/* GPIO PWM (LED) */
pub const DA9063_REG_GPO11_LED: u32 = 0xC6;
pub const DA9063_REG_GPO14_LED: u32 = 0xC7;
pub const DA9063_REG_GPO15_LED: u32 = 0xC8;

/* GP-ADC Threshold Registers */
pub const DA9063_REG_ADC_CFG: u32 = 0xC9;
pub const DA9063_REG_AUTO1_HIGH: u32 = 0xCA;
pub const DA9063_REG_AUTO1_LOW: u32 = 0xCB;
pub const DA9063_REG_AUTO2_HIGH: u32 = 0xCC;
pub const DA9063_REG_AUTO2_LOW: u32 = 0xCD;
pub const DA9063_REG_AUTO3_HIGH: u32 = 0xCE;
pub const DA9063_REG_AUTO3_LOW: u32 = 0xCF;

/* DA9063 Configuration registers */
/* OTP */
pub const DA9063_REG_OTP_CONT: u32 = 0x101;
pub const DA9063_REG_OTP_ADDR: u32 = 0x102;
pub const DA9063_REG_OTP_DATA: u32 = 0x103;

/* Customer Trim and Configuration */
pub const DA9063_REG_T_OFFSET: u32 = 0x104;
pub const DA9063_REG_INTERFACE: u32 = 0x105;
pub const DA9063_REG_CONFIG_A: u32 = 0x106;
pub const DA9063_REG_CONFIG_B: u32 = 0x107;
pub const DA9063_REG_CONFIG_C: u32 = 0x108;
pub const DA9063_REG_CONFIG_D: u32 = 0x109;
pub const DA9063_REG_CONFIG_E: u32 = 0x10A;
pub const DA9063_REG_CONFIG_F: u32 = 0x10B;
pub const DA9063_REG_CONFIG_G: u32 = 0x10C;
pub const DA9063_REG_CONFIG_H: u32 = 0x10D;
pub const DA9063_REG_CONFIG_I: u32 = 0x10E;
pub const DA9063_REG_CONFIG_J: u32 = 0x10F;
pub const DA9063_REG_CONFIG_K: u32 = 0x110;
pub const DA9063_REG_CONFIG_L: u32 = 0x111;

pub const DA9063_AD_REG_MON_REG_1: u32 = 0x112;
pub const DA9063_AD_REG_MON_REG_2: u32 = 0x113;
pub const DA9063_AD_REG_MON_REG_3: u32 = 0x114;
pub const DA9063_AD_REG_MON_REG_4: u32 = 0x115;
pub const DA9063_AD_REG_MON_REG_5: u32 = 0x116;
pub const DA9063_AD_REG_MON_REG_6: u32 = 0x117;
pub const DA9063_AD_REG_TRIM_CLDR: u32 = 0x118;

pub const DA9063_AD_REG_GP_ID_0: u32 = 0x119;
pub const DA9063_AD_REG_GP_ID_1: u32 = 0x11A;
pub const DA9063_AD_REG_GP_ID_2: u32 = 0x11B;
pub const DA9063_AD_REG_GP_ID_3: u32 = 0x11C;
pub const DA9063_AD_REG_GP_ID_4: u32 = 0x11D;
pub const DA9063_AD_REG_GP_ID_5: u32 = 0x11E;
pub const DA9063_AD_REG_GP_ID_6: u32 = 0x11F;
pub const DA9063_AD_REG_GP_ID_7: u32 = 0x120;
pub const DA9063_AD_REG_GP_ID_8: u32 = 0x121;
pub const DA9063_AD_REG_GP_ID_9: u32 = 0x122;
pub const DA9063_AD_REG_GP_ID_10: u32 = 0x123;
pub const DA9063_AD_REG_GP_ID_11: u32 = 0x124;
pub const DA9063_AD_REG_GP_ID_12: u32 = 0x125;
pub const DA9063_AD_REG_GP_ID_13: u32 = 0x126;
pub const DA9063_AD_REG_GP_ID_14: u32 = 0x127;
pub const DA9063_AD_REG_GP_ID_15: u32 = 0x128;
pub const DA9063_AD_REG_GP_ID_16: u32 = 0x129;
pub const DA9063_AD_REG_GP_ID_17: u32 = 0x12A;
pub const DA9063_AD_REG_GP_ID_18: u32 = 0x12B;
pub const DA9063_AD_REG_GP_ID_19: u32 = 0x12C;

pub const DA9063_BB_REG_CONFIG_M: u32 = 0x112;
pub const DA9063_BB_REG_CONFIG_N: u32 = 0x113;

pub const DA9063_BB_REG_MON_REG_1: u32 = 0x114;
pub const DA9063_BB_REG_MON_REG_2: u32 = 0x115;
pub const DA9063_BB_REG_MON_REG_3: u32 = 0x116;
pub const DA9063_BB_REG_MON_REG_4: u32 = 0x117;
pub const DA9063_BB_REG_MON_REG_5: u32 = 0x11E;
pub const DA9063_BB_REG_MON_REG_6: u32 = 0x11F;
pub const DA9063_BB_REG_TRIM_CLDR: u32 = 0x120;
/* General Purpose Registers */
pub const DA9063_BB_REG_GP_ID_0: u32 = 0x121;
pub const DA9063_BB_REG_GP_ID_1: u32 = 0x122;
pub const DA9063_BB_REG_GP_ID_2: u32 = 0x123;
pub const DA9063_BB_REG_GP_ID_3: u32 = 0x124;
pub const DA9063_BB_REG_GP_ID_4: u32 = 0x125;
pub const DA9063_BB_REG_GP_ID_5: u32 = 0x126;
pub const DA9063_BB_REG_GP_ID_6: u32 = 0x127;
pub const DA9063_BB_REG_GP_ID_7: u32 = 0x128;
pub const DA9063_BB_REG_GP_ID_8: u32 = 0x129;
pub const DA9063_BB_REG_GP_ID_9: u32 = 0x12A;
pub const DA9063_BB_REG_GP_ID_10: u32 = 0x12B;
pub const DA9063_BB_REG_GP_ID_11: u32 = 0x12C;
pub const DA9063_BB_REG_GP_ID_12: u32 = 0x12D;
pub const DA9063_BB_REG_GP_ID_13: u32 = 0x12E;
pub const DA9063_BB_REG_GP_ID_14: u32 = 0x12F;
pub const DA9063_BB_REG_GP_ID_15: u32 = 0x130;
pub const DA9063_BB_REG_GP_ID_16: u32 = 0x131;
pub const DA9063_BB_REG_GP_ID_17: u32 = 0x132;
pub const DA9063_BB_REG_GP_ID_18: u32 = 0x133;
pub const DA9063_BB_REG_GP_ID_19: u32 = 0x134;

/* Chip ID and variant */
pub const DA9063_REG_DEVICE_ID: u32 = 0x181;
pub const DA9063_REG_VARIANT_ID: u32 = 0x182;
pub const DA9063_REG_CUSTOMER_ID: u32 = 0x183;
pub const DA9063_REG_CONFIG_ID: u32 = 0x184;

/*
 * PMIC registers bits
 */
/* DA9063_REG_PAGE_CON (addr=0x00) */
pub const DA9063_PEG_PAGE_SHIFT: u32 = 0;
pub const DA9063_REG_PAGE_MASK: u32 = 0x07;
pub const DA9063_REG_PAGE0: u32 = 0x00;
pub const DA9063_REG_PAGE2: u32 = 0x02;
pub const DA9063_PAGE_WRITE_MODE: u32 = 0x00;
pub const DA9063_REPEAT_WRITE_MODE: u32 = 0x40;
pub const DA9063_PAGE_REVERT: u32 = 0x80;

/* DA9063_REG_STATUS_A (addr=0x01) */
pub const DA9063_NONKEY: u32 = 0x01;
pub const DA9063_WAKE: u32 = 0x02;
pub const DA9063_DVC_BUSY: u32 = 0x04;
pub const DA9063_COMP_1V2: u32 = 0x08;

/* DA9063_REG_STATUS_B (addr=0x02) */
pub const DA9063_GPI0: u32 = 0x01;
pub const DA9063_GPI1: u32 = 0x02;
pub const DA9063_GPI2: u32 = 0x04;
pub const DA9063_GPI3: u32 = 0x08;
pub const DA9063_GPI4: u32 = 0x10;
pub const DA9063_GPI5: u32 = 0x20;
pub const DA9063_GPI6: u32 = 0x40;
pub const DA9063_GPI7: u32 = 0x80;

/* DA9063_REG_STATUS_C (addr=0x03) */
pub const DA9063_GPI8: u32 = 0x01;
pub const DA9063_GPI9: u32 = 0x02;
pub const DA9063_GPI10: u32 = 0x04;
pub const DA9063_GPI11: u32 = 0x08;
pub const DA9063_GPI12: u32 = 0x10;
pub const DA9063_GPI13: u32 = 0x20;
pub const DA9063_GPI14: u32 = 0x40;
pub const DA9063_GPI15: u32 = 0x80;

/* DA9063_REG_STATUS_D (addr=0x04) */
pub const DA9063_LDO3_LIM: u32 = 0x08;
pub const DA9063_LDO4_LIM: u32 = 0x10;
pub const DA9063_LDO7_LIM: u32 = 0x20;
pub const DA9063_LDO8_LIM: u32 = 0x40;
pub const DA9063_LDO11_LIM: u32 = 0x80;

/* DA9063_REG_FAULT_LOG (addr=0x05) */
pub const DA9063_TWD_ERROR: u32 = 0x01;
pub const DA9063_POR: u32 = 0x02;
pub const DA9063_VDD_FAULT: u32 = 0x04;
pub const DA9063_VDD_START: u32 = 0x08;
pub const DA9063_TEMP_CRIT: u32 = 0x10;
pub const DA9063_KEY_RESET: u32 = 0x20;
pub const DA9063_NSHUTDOWN: u32 = 0x40;
pub const DA9063_WAIT_SHUT: u32 = 0x80;

/* DA9063_REG_EVENT_A (addr=0x06) */
pub const DA9063_E_NONKEY: u32 = 0x01;
pub const DA9063_E_ALARM: u32 = 0x02;
pub const DA9063_E_TICK: u32 = 0x04;
pub const DA9063_E_ADC_RDY: u32 = 0x08;
pub const DA9063_E_SEQ_RDY: u32 = 0x10;
pub const DA9063_EVENTS_B: u32 = 0x20;
pub const DA9063_EVENTS_C: u32 = 0x40;
pub const DA9063_EVENTS_D: u32 = 0x80;

/* DA9063_REG_EVENT_B (addr=0x07) */
pub const DA9063_E_WAKE: u32 = 0x01;
pub const DA9063_E_TEMP: u32 = 0x02;
pub const DA9063_E_COMP_1V2: u32 = 0x04;
pub const DA9063_E_LDO_LIM: u32 = 0x08;
pub const DA9063_E_REG_UVOV: u32 = 0x10;
pub const DA9063_E_DVC_RDY: u32 = 0x20;
pub const DA9063_E_VDD_MON: u32 = 0x40;
pub const DA9063_E_VDD_WARN: u32 = 0x80;

/* DA9063_REG_EVENT_C (addr=0x08) */
pub const DA9063_E_GPI0: u32 = 0x01;
pub const DA9063_E_GPI1: u32 = 0x02;
pub const DA9063_E_GPI2: u32 = 0x04;
pub const DA9063_E_GPI3: u32 = 0x08;
pub const DA9063_E_GPI4: u32 = 0x10;
pub const DA9063_E_GPI5: u32 = 0x20;
pub const DA9063_E_GPI6: u32 = 0x40;
pub const DA9063_E_GPI7: u32 = 0x80;

/* DA9063_REG_EVENT_D (addr=0x09) */
pub const DA9063_E_GPI8: u32 = 0x01;
pub const DA9063_E_GPI9: u32 = 0x02;
pub const DA9063_E_GPI10: u32 = 0x04;
pub const DA9063_E_GPI11: u32 = 0x08;
pub const DA9063_E_GPI12: u32 = 0x10;
pub const DA9063_E_GPI13: u32 = 0x20;
pub const DA9063_E_GPI14: u32 = 0x40;
pub const DA9063_E_GPI15: u32 = 0x80;

/* DA9063_REG_IRQ_MASK_A (addr=0x0A) */
pub const DA9063_M_ONKEY: u32 = 0x01;
pub const DA9063_M_ALARM: u32 = 0x02;
pub const DA9063_M_TICK: u32 = 0x04;
pub const DA9063_M_ADC_RDY: u32 = 0x08;
pub const DA9063_M_SEQ_RDY: u32 = 0x10;

/* DA9063_REG_IRQ_MASK_B (addr=0x0B) */
pub const DA9063_M_WAKE: u32 = 0x01;
pub const DA9063_M_TEMP: u32 = 0x02;
pub const DA9063_M_COMP_1V2: u32 = 0x04;
pub const DA9063_M_LDO_LIM: u32 = 0x08;
pub const DA9063_M_UVOV: u32 = 0x10;
pub const DA9063_M_DVC_RDY: u32 = 0x20;
pub const DA9063_M_VDD_MON: u32 = 0x40;
pub const DA9063_M_VDD_WARN: u32 = 0x80;

/* DA9063_REG_IRQ_MASK_C (addr=0x0C) */
pub const DA9063_M_GPI0: u32 = 0x01;
pub const DA9063_M_GPI1: u32 = 0x02;
pub const DA9063_M_GPI2: u32 = 0x04;
pub const DA9063_M_GPI3: u32 = 0x08;
pub const DA9063_M_GPI4: u32 = 0x10;
pub const DA9063_M_GPI5: u32 = 0x20;
pub const DA9063_M_GPI6: u32 = 0x40;
pub const DA9063_M_GPI7: u32 = 0x80;

/* DA9063_REG_IRQ_MASK_D (addr=0x0D) */
pub const DA9063_M_GPI8: u32 = 0x01;
pub const DA9063_M_GPI9: u32 = 0x02;
pub const DA9063_M_GPI10: u32 = 0x04;
pub const DA9063_M_GPI11: u32 = 0x08;
pub const DA9063_M_GPI12: u32 = 0x10;
pub const DA9063_M_GPI13: u32 = 0x20;
pub const DA9063_M_GPI14: u32 = 0x40;
pub const DA9063_M_GPI15: u32 = 0x80;

/* DA9063_REG_CONTROL_A (addr=0x0E) */
pub const DA9063_SYSTEM_EN: u32 = 0x01;
pub const DA9063_POWER_EN: u32 = 0x02;
pub const DA9063_POWER1_EN: u32 = 0x04;
pub const DA9063_STANDBY: u32 = 0x08;
pub const DA9063_M_SYSTEM_EN: u32 = 0x10;
pub const DA9063_M_POWER_EN: u32 = 0x20;
pub const DA9063_M_POWER1_EN: u32 = 0x40;
pub const DA9063_CP_EN: u32 = 0x80;

/* DA9063_REG_CONTROL_B (addr=0x0F) */
pub const DA9063_CHG_SEL: u32 = 0x01;
pub const DA9063_WATCHDOG_PD: u32 = 0x02;
pub const DA9063_BB_RESET_BLINKING: u32 = 0x04;
pub const DA9063_NRES_MODE: u32 = 0x08;
pub const DA9063_NONKEY_LOCK: u32 = 0x10;
pub const DA9063_BB_BUCK_SLOWSTART: u32 = 0x80;

/* DA9063_REG_CONTROL_C (addr=0x10) */
pub const DA9063_DEBOUNCING_MASK: u32 = 0x07;
pub const DA9063_DEBOUNCING_OFF: u32 = 0x0;
pub const DA9063_DEBOUNCING_0MS1: u32 = 0x1;
pub const DA9063_DEBOUNCING_1MS: u32 = 0x2;
pub const DA9063_DEBOUNCING_10MS24: u32 = 0x3;
pub const DA9063_DEBOUNCING_51MS2: u32 = 0x4;
pub const DA9063_DEBOUNCING_256MS: u32 = 0x5;
pub const DA9063_DEBOUNCING_512MS: u32 = 0x6;
pub const DA9063_DEBOUNCING_1024MS: u32 = 0x7;

pub const DA9063_AUTO_BOOT: u32 = 0x08;
pub const DA9063_OTPREAD_EN: u32 = 0x10;
pub const DA9063_SLEW_RATE_MASK: u32 = 0x60;
pub const DA9063_SLEW_RATE_4US: u32 = 0x00;
pub const DA9063_SLEW_RATE_3US: u32 = 0x20;
pub const DA9063_SLEW_RATE_1US: u32 = 0x40;
pub const DA9063_SLEW_RATE_0US5: u32 = 0x60;
pub const DA9063_DEF_SUPPLY: u32 = 0x80;

/* DA9063_REG_CONTROL_D (addr=0x11) */
pub const DA9063_TWDSCALE_MASK: u32 = 0x07;
pub const DA9063_BLINK_FRQ_MASK: u32 = 0x38;
pub const DA9063_BLINK_FRQ_OFF: u32 = 0x00;
pub const DA9063_BLINK_FRQ_1S0: u32 = 0x08;
pub const DA9063_BLINK_FRQ_2S0: u32 = 0x10;
pub const DA9063_BLINK_FRQ_4S0: u32 = 0x18;
pub const DA9063_BLINK_FRQ_0S18: u32 = 0x20;
pub const DA9063_BLINK_FRQ_2S0_VDD: u32 = 0x28;
pub const DA9063_BLINK_FRQ_4S0_VDD: u32 = 0x30;
pub const DA9063_BLINK_FRQ_0S18_VDD: u32 = 0x38;

pub const DA9063_BLINK_DUR_MASK: u32 = 0xC0;
pub const DA9063_BLINK_DUR_10MS: u32 = 0x00;
pub const DA9063_BLINK_DUR_20MS: u32 = 0x40;
pub const DA9063_BLINK_DUR_40MS: u32 = 0x80;
pub const DA9063_BLINK_DUR_20MSDBL: u32 = 0xC0;

/* DA9063_REG_CONTROL_E (addr=0x12) */
pub const DA9063_RTC_MODE_PD: u32 = 0x01;
pub const DA9063_RTC_MODE_SD: u32 = 0x02;
pub const DA9063_RTC_EN: u32 = 0x04;
pub const DA9063_ECO_MODE: u32 = 0x08;
pub const DA9063_PM_FB1_PIN: u32 = 0x10;
pub const DA9063_PM_FB2_PIN: u32 = 0x20;
pub const DA9063_PM_FB3_PIN: u32 = 0x40;
pub const DA9063_V_LOCK: u32 = 0x80;

/* DA9063_REG_CONTROL_F (addr=0x13) */
pub const DA9063_WATCHDOG: u32 = 0x01;
pub const DA9063_SHUTDOWN: u32 = 0x02;
pub const DA9063_WAKE_UP: u32 = 0x04;

/* DA9063_REG_PD_DIS (addr=0x14) */
pub const DA9063_GPI_DIS: u32 = 0x01;
pub const DA9063_GPADC_PAUSE: u32 = 0x02;
pub const DA9063_PMIF_DIS: u32 = 0x04;
pub const DA9063_HS2WIRE_DIS: u32 = 0x08;
pub const DA9063_BB_CLDR_PAUSE: u32 = 0x10;
pub const DA9063_BBAT_DIS: u32 = 0x20;
pub const DA9063_OUT_32K_PAUSE: u32 = 0x40;
pub const DA9063_PMCONT_DIS: u32 = 0x80;

/* DA9063_REG_GPIO_0_1 (addr=0x15) */
pub const DA9063_GPIO0_PIN_MASK: u32 = 0x03;
pub const DA9063_GPIO0_PIN_ADCIN1: u32 = 0x00;
pub const DA9063_GPIO0_PIN_GPI: u32 = 0x01;
pub const DA9063_GPIO0_PIN_GPO_OD: u32 = 0x02;
pub const DA9063_GPIO0_PIN_GPO: u32 = 0x03;
pub const DA9063_GPIO0_TYPE: u32 = 0x04;
pub const DA9063_GPIO0_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO0_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO0_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO0_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO0_NO_WAKEUP: u32 = 0x08;
pub const DA9063_GPIO1_PIN_MASK: u32 = 0x30;
pub const DA9063_GPIO1_PIN_ADCIN2_COMP: u32 = 0x00;
pub const DA9063_GPIO1_PIN_GPI: u32 = 0x10;
pub const DA9063_GPIO1_PIN_GPO_OD: u32 = 0x20;
pub const DA9063_GPIO1_PIN_GPO: u32 = 0x30;
pub const DA9063_GPIO1_TYPE: u32 = 0x40;
pub const DA9063_GPIO1_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO1_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO1_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO1_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO1_NO_WAKEUP: u32 = 0x80;

/* DA9063_REG_GPIO_2_3 (addr=0x16) */
pub const DA9063_GPIO2_PIN_MASK: u32 = 0x03;
pub const DA9063_GPIO2_PIN_ADCIN3: u32 = 0x00;
pub const DA9063_GPIO2_PIN_GPI: u32 = 0x01;
pub const DA9063_GPIO2_PIN_GPO_PSS: u32 = 0x02;
pub const DA9063_GPIO2_PIN_GPO: u32 = 0x03;
pub const DA9063_GPIO2_TYPE: u32 = 0x04;
pub const DA9063_GPIO2_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO2_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO2_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO2_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO2_NO_WAKEUP: u32 = 0x08;
pub const DA9063_GPIO3_PIN_MASK: u32 = 0x30;
pub const DA9063_GPIO3_PIN_CORE_SW_G: u32 = 0x00;
pub const DA9063_GPIO3_PIN_GPI: u32 = 0x10;
pub const DA9063_GPIO3_PIN_GPO_OD: u32 = 0x20;
pub const DA9063_GPIO3_PIN_GPO: u32 = 0x30;
pub const DA9063_GPIO3_TYPE: u32 = 0x40;
pub const DA9063_GPIO3_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO3_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO3_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO3_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO3_NO_WAKEUP: u32 = 0x80;

/* DA9063_REG_GPIO_4_5 (addr=0x17) */
pub const DA9063_GPIO4_PIN_MASK: u32 = 0x03;
pub const DA9063_GPIO4_PIN_CORE_SW_S: u32 = 0x00;
pub const DA9063_GPIO4_PIN_GPI: u32 = 0x01;
pub const DA9063_GPIO4_PIN_GPO_OD: u32 = 0x02;
pub const DA9063_GPIO4_PIN_GPO: u32 = 0x03;
pub const DA9063_GPIO4_TYPE: u32 = 0x04;
pub const DA9063_GPIO4_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO4_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO4_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO4_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO4_NO_WAKEUP: u32 = 0x08;
pub const DA9063_GPIO5_PIN_MASK: u32 = 0x30;
pub const DA9063_GPIO5_PIN_PERI_SW_G: u32 = 0x00;
pub const DA9063_GPIO5_PIN_GPI: u32 = 0x10;
pub const DA9063_GPIO5_PIN_GPO_OD: u32 = 0x20;
pub const DA9063_GPIO5_PIN_GPO: u32 = 0x30;
pub const DA9063_GPIO5_TYPE: u32 = 0x40;
pub const DA9063_GPIO5_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO5_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO5_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO5_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO5_NO_WAKEUP: u32 = 0x80;

/* DA9063_REG_GPIO_6_7 (addr=0x18) */
pub const DA9063_GPIO6_PIN_MASK: u32 = 0x03;
pub const DA9063_GPIO6_PIN_PERI_SW_S: u32 = 0x00;
pub const DA9063_GPIO6_PIN_GPI: u32 = 0x01;
pub const DA9063_GPIO6_PIN_GPO_OD: u32 = 0x02;
pub const DA9063_GPIO6_PIN_GPO: u32 = 0x03;
pub const DA9063_GPIO6_TYPE: u32 = 0x04;
pub const DA9063_GPIO6_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO6_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO6_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO6_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO6_NO_WAKEUP: u32 = 0x08;
pub const DA9063_GPIO7_PIN_MASK: u32 = 0x30;
pub const DA9063_GPIO7_PIN_GPI: u32 = 0x10;
pub const DA9063_GPIO7_PIN_GPO_PSS: u32 = 0x20;
pub const DA9063_GPIO7_PIN_GPO: u32 = 0x30;
pub const DA9063_GPIO7_TYPE: u32 = 0x40;
pub const DA9063_GPIO7_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO7_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO7_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO7_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO7_NO_WAKEUP: u32 = 0x80;

/* DA9063_REG_GPIO_8_9 (addr=0x19) */
pub const DA9063_GPIO8_PIN_MASK: u32 = 0x03;
pub const DA9063_GPIO8_PIN_GPI_SYS_EN: u32 = 0x00;
pub const DA9063_GPIO8_PIN_GPI: u32 = 0x01;
pub const DA9063_GPIO8_PIN_GPO_PSS: u32 = 0x02;
pub const DA9063_GPIO8_PIN_GPO: u32 = 0x03;
pub const DA9063_GPIO8_TYPE: u32 = 0x04;
pub const DA9063_GPIO8_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO8_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO8_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO8_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO8_NO_WAKEUP: u32 = 0x08;
pub const DA9063_GPIO9_PIN_MASK: u32 = 0x30;
pub const DA9063_GPIO9_PIN_GPI_PWR_EN: u32 = 0x00;
pub const DA9063_GPIO9_PIN_GPI: u32 = 0x10;
pub const DA9063_GPIO9_PIN_GPO_PSS: u32 = 0x20;
pub const DA9063_GPIO9_PIN_GPO: u32 = 0x30;
pub const DA9063_GPIO9_TYPE: u32 = 0x40;
pub const DA9063_GPIO9_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO9_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO9_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO9_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO9_NO_WAKEUP: u32 = 0x80;

/* DA9063_REG_GPIO_10_11 (addr=0x1A) */
pub const DA9063_GPIO10_PIN_MASK: u32 = 0x03;
pub const DA9063_GPIO10_PIN_GPI_PWR1_EN: u32 = 0x00;
pub const DA9063_GPIO10_PIN_GPI: u32 = 0x01;
pub const DA9063_GPIO10_PIN_GPO_OD: u32 = 0x02;
pub const DA9063_GPIO10_PIN_GPO: u32 = 0x03;
pub const DA9063_GPIO10_TYPE: u32 = 0x04;
pub const DA9063_GPIO10_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO10_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO10_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO10_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO10_NO_WAKEUP: u32 = 0x08;
pub const DA9063_GPIO11_PIN_MASK: u32 = 0x30;
pub const DA9063_GPIO11_PIN_GPO_OD: u32 = 0x00;
pub const DA9063_GPIO11_PIN_GPI: u32 = 0x10;
pub const DA9063_GPIO11_PIN_GPO_PSS: u32 = 0x20;
pub const DA9063_GPIO11_PIN_GPO: u32 = 0x30;
pub const DA9063_GPIO11_TYPE: u32 = 0x40;
pub const DA9063_GPIO11_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO11_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO11_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO11_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO11_NO_WAKEUP: u32 = 0x80;

/* DA9063_REG_GPIO_12_13 (addr=0x1B) */
pub const DA9063_GPIO12_PIN_MASK: u32 = 0x03;
pub const DA9063_GPIO12_PIN_NVDDFLT_OUT: u32 = 0x00;
pub const DA9063_GPIO12_PIN_GPI: u32 = 0x01;
pub const DA9063_GPIO12_PIN_VSYSMON_OUT: u32 = 0x02;
pub const DA9063_GPIO12_PIN_GPO: u32 = 0x03;
pub const DA9063_GPIO12_TYPE: u32 = 0x04;
pub const DA9063_GPIO12_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO12_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO12_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO12_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO12_NO_WAKEUP: u32 = 0x08;
pub const DA9063_GPIO13_PIN_MASK: u32 = 0x30;
pub const DA9063_GPIO13_PIN_GPFB1_OUT: u32 = 0x00;
pub const DA9063_GPIO13_PIN_GPI: u32 = 0x10;
pub const DA9063_GPIO13_PIN_GPFB1_OUTOD: u32 = 0x20;
pub const DA9063_GPIO13_PIN_GPO: u32 = 0x30;
pub const DA9063_GPIO13_TYPE: u32 = 0x40;
pub const DA9063_GPIO13_TYPE_GPFB1_OUT: u32 = 0x00;
pub const DA9063_GPIO13_TYPE_GPI: u32 = 0x00;
pub const DA9063_GPIO13_TYPE_GPFB1_OUTOD: u32 = 0x04;
pub const DA9063_GPIO13_TYPE_GPO: u32 = 0x04;
pub const DA9063_GPIO13_NO_WAKEUP: u32 = 0x80;

/* DA9063_REG_GPIO_14_15 (addr=0x1C) */
pub const DA9063_GPIO14_PIN_MASK: u32 = 0x03;
pub const DA9063_GPIO14_PIN_GPO_OD: u32 = 0x00;
pub const DA9063_GPIO14_PIN_GPI: u32 = 0x01;
pub const DA9063_GPIO14_PIN_HS2DATA: u32 = 0x02;
pub const DA9063_GPIO14_PIN_GPO: u32 = 0x03;
pub const DA9063_GPIO14_TYPE: u32 = 0x04;
pub const DA9063_GPIO14_TYPE_GPI_ACT_LOW: u32 = 0x00;
pub const DA9063_GPIO14_TYPE_GPO_VDD_IO1: u32 = 0x00;
pub const DA9063_GPIO14_TYPE_GPI_ACT_HIGH: u32 = 0x04;
pub const DA9063_GPIO14_TYPE_GPO_VDD_IO2: u32 = 0x04;
pub const DA9063_GPIO14_NO_WAKEUP: u32 = 0x08;
pub const DA9063_GPIO15_PIN_MASK: u32 = 0x30;
pub const DA9063_GPIO15_PIN_GPO_OD: u32 = 0x00;
pub const DA9063_GPIO15_PIN_GPI: u32 = 0x10;
pub const DA9063_GPIO15_PIN_GPO: u32 = 0x30;
pub const DA9063_GPIO15_TYPE: u32 = 0x40;
pub const DA9063_GPIO15_TYPE_GPFB1_OUT: u32 = 0x00;
pub const DA9063_GPIO15_TYPE_GPI: u32 = 0x00;
pub const DA9063_GPIO15_TYPE_GPFB1_OUTOD: u32 = 0x04;
pub const DA9063_GPIO15_TYPE_GPO: u32 = 0x04;
pub const DA9063_GPIO15_NO_WAKEUP: u32 = 0x80;

/* DA9063_REG_GPIO_MODE0_7 (addr=0x1D) */
pub const DA9063_GPIO0_MODE: u32 = 0x01;
pub const DA9063_GPIO1_MODE: u32 = 0x02;
pub const DA9063_GPIO2_MODE: u32 = 0x04;
pub const DA9063_GPIO3_MODE: u32 = 0x08;
pub const DA9063_GPIO4_MODE: u32 = 0x10;
pub const DA9063_GPIO5_MODE: u32 = 0x20;
pub const DA9063_GPIO6_MODE: u32 = 0x40;
pub const DA9063_GPIO7_MODE: u32 = 0x80;

/* DA9063_REG_GPIO_MODE8_15 (addr=0x1E) */
pub const DA9063_GPIO8_MODE: u32 = 0x01;
pub const DA9063_GPIO9_MODE: u32 = 0x02;
pub const DA9063_GPIO10_MODE: u32 = 0x04;
pub const DA9063_GPIO11_MODE: u32 = 0x08;
pub const DA9063_GPIO11_MODE_LED_ACT_HIGH: u32 = 0x00;
pub const DA9063_GPIO11_MODE_LED_ACT_LOW: u32 = 0x08;
pub const DA9063_GPIO12_MODE: u32 = 0x10;
pub const DA9063_GPIO13_MODE: u32 = 0x20;
pub const DA9063_GPIO14_MODE: u32 = 0x40;
pub const DA9063_GPIO14_MODE_LED_ACT_HIGH: u32 = 0x00;
pub const DA9063_GPIO14_MODE_LED_ACT_LOW: u32 = 0x40;
pub const DA9063_GPIO15_MODE: u32 = 0x80;
pub const DA9063_GPIO15_MODE_LED_ACT_HIGH: u32 = 0x00;
pub const DA9063_GPIO15_MODE_LED_ACT_LOW: u32 = 0x80;

/* DA9063_REG_SWITCH_CONT (addr=0x1F) */
pub const DA9063_CORE_SW_GPI_MASK: u32 = 0x03;
pub const DA9063_CORE_SW_GPI_OFF: u32 = 0x00;
pub const DA9063_CORE_SW_GPI_GPIO1: u32 = 0x01;
pub const DA9063_CORE_SW_GPI_GPIO2: u32 = 0x02;
pub const DA9063_CORE_SW_GPI_GPIO13: u32 = 0x03;
pub const DA9063_PERI_SW_GPI_MASK: u32 = 0x0C;
pub const DA9063_PERI_SW_GPI_OFF: u32 = 0x00;
pub const DA9063_PERI_SW_GPI_GPIO1: u32 = 0x04;
pub const DA9063_PERI_SW_GPI_GPIO2: u32 = 0x08;
pub const DA9063_PERI_SW_GPI_GPIO13: u32 = 0x0C;
pub const DA9063_SWITCH_SR_MASK: u32 = 0x30;
pub const DA9063_SWITCH_SR_1MV: u32 = 0x00;
pub const DA9063_SWITCH_SR_5MV: u32 = 0x10;
pub const DA9063_SWITCH_SR_10MV: u32 = 0x20;
pub const DA9063_SWITCH_SR_50MV: u32 = 0x30;
pub const DA9063_CORE_SW_INTERNAL: u32 = 0x40;
pub const DA9063_CP_EN_MODE: u32 = 0x80;

/* DA9063_REGL_Bxxxx_CONT common bits (addr=0x20-0x25) */
pub const DA9063_BUCK_EN: u32 = 0x01;
pub const DA9063_BUCK_GPI_MASK: u32 = 0x06;
pub const DA9063_BUCK_GPI_OFF: u32 = 0x00;
pub const DA9063_BUCK_GPI_GPIO1: u32 = 0x02;
pub const DA9063_BUCK_GPI_GPIO2: u32 = 0x04;
pub const DA9063_BUCK_GPI_GPIO13: u32 = 0x06;
pub const DA9063_BUCK_CONF: u32 = 0x08;
pub const DA9063_VBUCK_GPI_MASK: u32 = 0x60;
pub const DA9063_VBUCK_GPI_OFF: u32 = 0x00;
pub const DA9063_VBUCK_GPI_GPIO1: u32 = 0x20;
pub const DA9063_VBUCK_GPI_GPIO2: u32 = 0x40;
pub const DA9063_VBUCK_GPI_GPIO13: u32 = 0x60;

/* DA9063_REG_BCORE1_CONT specific bits (addr=0x21) */
pub const DA9063_CORE_SW_EN: u32 = 0x10;
pub const DA9063_CORE_SW_CONF: u32 = 0x80;

/* DA9063_REG_BPERI_CONT specific bits (addr=0x25) */
pub const DA9063_PERI_SW_EN: u32 = 0x10;
pub const DA9063_PERI_SW_CONF: u32 = 0x80;

/* DA9063_REG_LDOx_CONT common bits (addr=0x26-0x30) */
pub const DA9063_LDO_EN: u32 = 0x01;
pub const DA9063_LDO_GPI_MASK: u32 = 0x06;
pub const DA9063_LDO_GPI_OFF: u32 = 0x00;
pub const DA9063_LDO_GPI_GPIO1: u32 = 0x02;
pub const DA9063_LDO_GPI_GPIO2: u32 = 0x04;
pub const DA9063_LDO_GPI_GPIO13: u32 = 0x06;
pub const DA9063_LDO_PD_DIS: u32 = 0x08;
pub const DA9063_VLDO_GPI_MASK: u32 = 0x60;
pub const DA9063_VLDO_GPI_OFF: u32 = 0x00;
pub const DA9063_VLDO_GPI_GPIO1: u32 = 0x20;
pub const DA9063_VLDO_GPI_GPIO2: u32 = 0x40;
pub const DA9063_VLDO_GPI_GPIO13: u32 = 0x60;
pub const DA9063_LDO_CONF: u32 = 0x80;

/* DA9063_REG_LDO5_CONT specific bits (addr=0x2A) */
pub const DA9063_VLDO5_SEL: u32 = 0x10;

/* DA9063_REG_LDO6_CONT specific bits (addr=0x2B) */
pub const DA9063_VLDO6_SEL: u32 = 0x10;

/* DA9063_REG_LDO7_CONT specific bits (addr=0x2C) */
pub const DA9063_VLDO7_SEL: u32 = 0x10;

/* DA9063_REG_LDO8_CONT specific bits (addr=0x2D) */
pub const DA9063_VLDO8_SEL: u32 = 0x10;

/* DA9063_REG_LDO9_CONT specific bits (addr=0x2E) */
pub const DA9063_VLDO9_SEL: u32 = 0x10;

/* DA9063_REG_LDO10_CONT specific bits (addr=0x2F) */
pub const DA9063_VLDO10_SEL: u32 = 0x10;

/* DA9063_REG_LDO11_CONT specific bits (addr=0x30) */
pub const DA9063_VLDO11_SEL: u32 = 0x10;

/* DA9063_REG_VIB (addr=0x31) */
pub const DA9063_VIB_SET_MASK: u32 = 0x3F;
pub const DA9063_VIB_SET_OFF: u32 = 0;
pub const DA9063_VIB_SET_MAX: u32 = 0x3F;

/* DA9063_REG_DVC_1 (addr=0x32) */
pub const DA9063_VBCORE1_SEL: u32 = 0x01;
pub const DA9063_VBCORE2_SEL: u32 = 0x02;
pub const DA9063_VBPRO_SEL: u32 = 0x04;
pub const DA9063_VBMEM_SEL: u32 = 0x08;
pub const DA9063_VBPERI_SEL: u32 = 0x10;
pub const DA9063_VLDO1_SEL: u32 = 0x20;
pub const DA9063_VLDO2_SEL: u32 = 0x40;
pub const DA9063_VLDO3_SEL: u32 = 0x80;

/* DA9063_REG_DVC_2 (addr=0x33) */
pub const DA9063_VBIO_SEL: u32 = 0x01;
pub const DA9063_VLDO4_SEL: u32 = 0x80;

/* DA9063_REG_ADC_MAN (addr=0x34) */
pub const DA9063_ADC_MUX_MASK: u32 = 0x0F;
pub const DA9063_ADC_MUX_VSYS: u32 = 0x00;
pub const DA9063_ADC_MUX_ADCIN1: u32 = 0x01;
pub const DA9063_ADC_MUX_ADCIN2: u32 = 0x02;
pub const DA9063_ADC_MUX_ADCIN3: u32 = 0x03;
pub const DA9063_ADC_MUX_T_SENSE: u32 = 0x04;
pub const DA9063_ADC_MUX_VBBAT: u32 = 0x05;
pub const DA9063_ADC_MUX_LDO_G1: u32 = 0x08;
pub const DA9063_ADC_MUX_LDO_G2: u32 = 0x09;
pub const DA9063_ADC_MUX_LDO_G3: u32 = 0x0A;
pub const DA9063_ADC_MAN: u32 = 0x10;
pub const DA9063_ADC_MODE: u32 = 0x20;

/* DA9063_REG_ADC_CONT (addr=0x35) */
pub const DA9063_ADC_AUTO_VSYS_EN: u32 = 0x01;
pub const DA9063_ADC_AUTO_AD1_EN: u32 = 0x02;
pub const DA9063_ADC_AUTO_AD2_EN: u32 = 0x04;
pub const DA9063_ADC_AUTO_AD3_EN: u32 = 0x08;
pub const DA9063_ADC_AD1_ISRC_EN: u32 = 0x10;
pub const DA9063_ADC_AD2_ISRC_EN: u32 = 0x20;
pub const DA9063_ADC_AD3_ISRC_EN: u32 = 0x40;
pub const DA9063_COMP1V2_EN: u32 = 0x80;

/* DA9063_REG_VSYS_MON (addr=0x36) */
pub const DA9063_VSYS_VAL_MASK: u32 = 0xFF;
pub const DA9063_VSYS_VAL_BASE: u32 = 0x00;

/* DA9063_REG_ADC_RES_L (addr=0x37) */
pub const DA9063_ADC_RES_L_BITS: u32 = 2;
pub const DA9063_ADC_RES_L_MASK: u32 = 0xC0;

/* DA9063_REG_ADC_RES_H (addr=0x38) */
pub const DA9063_ADC_RES_M_BITS: u32 = 8;
pub const DA9063_ADC_RES_M_MASK: u32 = 0xFF;

/* DA9063_REG_(xxx_RES/ADC_RES_H) (addr=0x39-0x3F) */
pub const DA9063_ADC_VAL_MASK: u32 = 0xFF;

/* DA9063_REG_COUNT_S (addr=0x40) */
pub const DA9063_RTC_READ: u32 = 0x80;
pub const DA9063_COUNT_SEC_MASK: u32 = 0x3F;

/* DA9063_REG_COUNT_MI (addr=0x41) */
pub const DA9063_COUNT_MIN_MASK: u32 = 0x3F;

/* DA9063_REG_COUNT_H (addr=0x42) */
pub const DA9063_COUNT_HOUR_MASK: u32 = 0x1F;

/* DA9063_REG_COUNT_D (addr=0x43) */
pub const DA9063_COUNT_DAY_MASK: u32 = 0x1F;

/* DA9063_REG_COUNT_MO (addr=0x44) */
pub const DA9063_COUNT_MONTH_MASK: u32 = 0x0F;

/* DA9063_REG_COUNT_Y (addr=0x45) */
pub const DA9063_COUNT_YEAR_MASK: u32 = 0x3F;
pub const DA9063_MONITOR: u32 = 0x40;

/* DA9063_REG_ALARM_S (addr=0x46) */
pub const DA9063_BB_ALARM_S_MASK: u32 = 0x3F;
pub const DA9063_ALARM_STATUS_ALARM: u32 = 0x80;
pub const DA9063_ALARM_STATUS_TICK: u32 = 0x40;
/* DA9063_REG_ALARM_MI (addr=0x47) */
pub const DA9063_ALARM_MIN_MASK: u32 = 0x3F;

/* DA9063_REG_ALARM_H (addr=0x48) */
pub const DA9063_ALARM_HOUR_MASK: u32 = 0x1F;

/* DA9063_REG_ALARM_D (addr=0x49) */
pub const DA9063_ALARM_DAY_MASK: u32 = 0x1F;

/* DA9063_REG_ALARM_MO (addr=0x4A) */
pub const DA9063_TICK_WAKE: u32 = 0x20;
pub const DA9063_TICK_TYPE: u32 = 0x10;
pub const DA9063_TICK_TYPE_SEC: u32 = 0x00;
pub const DA9063_TICK_TYPE_MIN: u32 = 0x10;
pub const DA9063_ALARM_MONTH_MASK: u32 = 0x0F;

/* DA9063_REG_ALARM_Y (addr=0x4B) */
pub const DA9063_TICK_ON: u32 = 0x80;
pub const DA9063_ALARM_ON: u32 = 0x40;
pub const DA9063_ALARM_YEAR_MASK: u32 = 0x3F;

/* DA9063_REG_WAIT (addr=0x97)*/
pub const DA9063_REG_WAIT_TIME_MASK: u32 = 0xF;
pub const DA9063_WAIT_TIME_0_US: u32 = 0x0;
pub const DA9063_WAIT_TIME_512_US: u32 = 0x1;
pub const DA9063_WAIT_TIME_1_MS: u32 = 0x2;
pub const DA9063_WAIT_TIME_2_MS: u32 = 0x3;
pub const DA9063_WAIT_TIME_4_1_MS: u32 = 0x4;
pub const DA9063_WAIT_TIME_8_2_MS: u32 = 0x5;
pub const DA9063_WAIT_TIME_16_4_MS: u32 = 0x6;
pub const DA9063_WAIT_TIME_32_8_MS: u32 = 0x7;
pub const DA9063_WAIT_TIME_65_5_MS: u32 = 0x8;
pub const DA9063_WAIT_TIME_128_MS: u32 = 0x9;
pub const DA9063_WAIT_TIME_256_MS: u32 = 0xA;
pub const DA9063_WAIT_TIME_512_MS: u32 = 0xB;
pub const DA9063_WAIT_TIME_1_S: u32 = 0xC;
pub const DA9063_WAIT_TIME_2_1_S: u32 = 0xD;

/* DA9063_REG_EN_32K  (addr=0x98)*/
pub const DA9063_STABILIZ_TIME_MASK: u32 = 0x7;
pub const DA9063_CRYSTAL: u32 = 0x08;
pub const DA9063_DELAY_MODE: u32 = 0x10;
pub const DA9063_OUT_CLOCK: u32 = 0x20;
pub const DA9063_RTC_CLOCK: u32 = 0x40;
pub const DA9063_OUT_32K_EN: u32 = 0x80;

/* DA9063_REG_BUCK_ILIM_A (addr=0x9A) */
pub const DA9063_BIO_ILIM_MASK: u32 = 0x0F;
pub const DA9063_BMEM_ILIM_MASK: u32 = 0xF0;

/* DA9063_REG_BUCK_ILIM_B (addr=0x9B) */
pub const DA9063_BPRO_ILIM_MASK: u32 = 0x0F;
pub const DA9063_BPERI_ILIM_MASK: u32 = 0xF0;

/* DA9063_REG_BUCK_ILIM_C (addr=0x9C) */
pub const DA9063_BCORE1_ILIM_MASK: u32 = 0x0F;
pub const DA9063_BCORE2_ILIM_MASK: u32 = 0xF0;

/* DA9063_REG_Bxxxx_CFG common bits (addr=0x9D-0xA2) */
pub const DA9063_BUCK_FB_MASK: u32 = 0x07;
pub const DA9063_BUCK_PD_DIS_MASK: u32 = 0x20;
pub const DA9063_BUCK_MODE_MASK: u32 = 0xC0;
pub const DA9063_BUCK_MODE_MANUAL: u32 = 0x00;
pub const DA9063_BUCK_MODE_SLEEP: u32 = 0x40;
pub const DA9063_BUCK_MODE_SYNC: u32 = 0x80;
pub const DA9063_BUCK_MODE_AUTO: u32 = 0xC0;

/* DA9063_REG_BPRO_CFG (addr=0x9F) */
pub const DA9063_BPRO_VTTR_EN: u32 = 0x08;
pub const DA9063_BPRO_VTT_EN: u32 = 0x10;

/* DA9063_REG_VBxxxx_A/B (addr=0xA3-0xA8, 0xB4-0xB9) */
pub const DA9063_VBUCK_MASK: u32 = 0x7F;
pub const DA9063_VBUCK_BIAS: u32 = 0;
pub const DA9063_BUCK_SL: u32 = 0x80;

/* DA9063_REG_VLDOx_A/B (addr=0xA9-0x3, 0xBA-0xC4) */
pub const DA9063_LDO_SL: u32 = 0x80;

/* DA9063_REG_VLDO1_A/B (addr=0xA9, 0xBA) */
pub const DA9063_VLDO1_MASK: u32 = 0x3F;
pub const DA9063_VLDO1_BIAS: u32 = 0;

/* DA9063_REG_VLDO2_A/B (addr=0xAA, 0xBB) */
pub const DA9063_VLDO2_MASK: u32 = 0x3F;
pub const DA9063_VLDO2_BIAS: u32 = 0;

/* DA9063_REG_VLDO3_A/B (addr=0xAB, 0xBC) */
pub const DA9063_VLDO3_MASK: u32 = 0x7F;
pub const DA9063_VLDO3_BIAS: u32 = 0;

/* DA9063_REG_VLDO4_A/B (addr=0xAC, 0xBD) */
pub const DA9063_VLDO4_MASK: u32 = 0x7F;
pub const DA9063_VLDO4_BIAS: u32 = 0;

/* DA9063_REG_VLDO5_A/B (addr=0xAD, 0xBE) */
pub const DA9063_VLDO5_MASK: u32 = 0x3F;
pub const DA9063_VLDO5_BIAS: u32 = 2;

/* DA9063_REG_VLDO6_A/B (addr=0xAE, 0xBF) */
pub const DA9063_VLDO6_MASK: u32 = 0x3F;
pub const DA9063_VLDO6_BIAS: u32 = 2;

/* DA9063_REG_VLDO7_A/B (addr=0xAF, 0xC0) */
pub const DA9063_VLDO7_MASK: u32 = 0x3F;
pub const DA9063_VLDO7_BIAS: u32 = 2;

/* DA9063_REG_VLDO8_A/B (addr=0xB0, 0xC1) */
pub const DA9063_VLDO8_MASK: u32 = 0x3F;
pub const DA9063_VLDO8_BIAS: u32 = 2;

/* DA9063_REG_VLDO9_A/B (addr=0xB1, 0xC2) */
pub const DA9063_VLDO9_MASK: u32 = 0x3F;
pub const DA9063_VLDO9_BIAS: u32 = 3;

/* DA9063_REG_VLDO10_A/B (addr=0xB2, 0xC3) */
pub const DA9063_VLDO10_MASK: u32 = 0x3F;
pub const DA9063_VLDO10_BIAS: u32 = 2;

/* DA9063_REG_VLDO11_A/B (addr=0xB3, 0xC4) */
pub const DA9063_VLDO11_MASK: u32 = 0x3F;
pub const DA9063_VLDO11_BIAS: u32 = 2;

/* DA9063_REG_GPO11_LED (addr=0xC6) */
/* DA9063_REG_GPO14_LED (addr=0xC7) */
/* DA9063_REG_GPO15_LED (addr=0xC8) */
pub const DA9063_GPIO_DIM: u32 = 0x80;
pub const DA9063_GPIO_PWM_MASK: u32 = 0x7F;

/* DA9063_REG_CONFIG_H (addr=0x10D) */
pub const DA9063_PWM_CLK_MASK: u32 = 0x01;
pub const DA9063_PWM_CLK_PWM2MHZ: u32 = 0x00;
pub const DA9063_PWM_CLK_PWM1MHZ: u32 = 0x01;
pub const DA9063_LDO8_MODE_MASK: u32 = 0x02;
pub const DA9063_LDO8_MODE_LDO: u32 = 0;
pub const DA9063_LDO8_MODE_VIBR: u32 = 0x02;
pub const DA9063_MERGE_SENSE_MASK: u32 = 0x04;
pub const DA9063_MERGE_SENSE_GP_FB2: u32 = 0x00;
pub const DA9063_MERGE_SENSE_GPIO4: u32 = 0x04;
pub const DA9063_BCORE_MERGE: u32 = 0x08;
pub const DA9063_BPRO_OD: u32 = 0x10;
pub const DA9063_BCORE2_OD: u32 = 0x20;
pub const DA9063_BCORE1_OD: u32 = 0x40;
pub const DA9063_BUCK_MERGE: u32 = 0x80;

/* DA9063_REG_CONFIG_I (addr=0x10E) */
pub const DA9063_NONKEY_PIN_MASK: u32 = 0x03;
pub const DA9063_NONKEY_PIN_PORT: u32 = 0x00;
pub const DA9063_NONKEY_PIN_SWDOWN: u32 = 0x01;
pub const DA9063_NONKEY_PIN_AUTODOWN: u32 = 0x02;
pub const DA9063_NONKEY_PIN_AUTOFLPRT: u32 = 0x03;

/* DA9063_REG_CONFIG_J (addr=0x10F) */
pub const DA9063_TWOWIRE_TO: u32 = 0x40;

/* DA9063_REG_MON_REG_2 (addr=0x115) */
pub const DA9063_LDO1_MON_EN: u32 = 0x01;
pub const DA9063_LDO2_MON_EN: u32 = 0x02;
pub const DA9063_LDO3_MON_EN: u32 = 0x04;
pub const DA9063_LDO4_MON_EN: u32 = 0x08;
pub const DA9063_LDO5_MON_EN: u32 = 0x10;
pub const DA9063_LDO6_MON_EN: u32 = 0x20;
pub const DA9063_LDO7_MON_EN: u32 = 0x40;
pub const DA9063_LDO8_MON_EN: u32 = 0x80;

/* DA9063_REG_MON_REG_3 (addr=0x116) */
pub const DA9063_LDO9_MON_EN: u32 = 0x01;
pub const DA9063_LDO10_MON_EN: u32 = 0x02;
pub const DA9063_LDO11_MON_EN: u32 = 0x04;

/* DA9063_REG_MON_REG_4 (addr=0x117) */
pub const DA9063_BCORE1_MON_EN: u32 = 0x04;
pub const DA9063_BCORE2_MON_EN: u32 = 0x08;
pub const DA9063_BPRO_MON_EN: u32 = 0x10;
pub const DA9063_BIO_MON_EN: u32 = 0x20;
pub const DA9063_BMEM_MON_EN: u32 = 0x40;
pub const DA9063_BPERI_MON_EN: u32 = 0x80;

/* DA9063_REG_MON_REG_5 (addr=0x116) */
pub const DA9063_MON_A8_IDX_MASK: u32 = 0x07;
pub const DA9063_MON_A8_IDX_NONE: u32 = 0x00;
pub const DA9063_MON_A8_IDX_BCORE1: u32 = 0x01;
pub const DA9063_MON_A8_IDX_BCORE2: u32 = 0x02;
pub const DA9063_MON_A8_IDX_BPRO: u32 = 0x03;
pub const DA9063_MON_A8_IDX_LDO3: u32 = 0x04;
pub const DA9063_MON_A8_IDX_LDO4: u32 = 0x05;
pub const DA9063_MON_A8_IDX_LDO11: u32 = 0x06;
pub const DA9063_MON_A9_IDX_MASK: u32 = 0x70;
pub const DA9063_MON_A9_IDX_NONE: u32 = 0x00;
pub const DA9063_MON_A9_IDX_BIO: u32 = 0x01;
pub const DA9063_MON_A9_IDX_BMEM: u32 = 0x02;
pub const DA9063_MON_A9_IDX_BPERI: u32 = 0x03;
pub const DA9063_MON_A9_IDX_LDO1: u32 = 0x04;
pub const DA9063_MON_A9_IDX_LDO2: u32 = 0x05;
pub const DA9063_MON_A9_IDX_LDO5: u32 = 0x06;

/* DA9063_REG_MON_REG_6 (addr=0x117) */
pub const DA9063_MON_A10_IDX_MASK: u32 = 0x07;
pub const DA9063_MON_A10_IDX_NONE: u32 = 0x00;
pub const DA9063_MON_A10_IDX_LDO6: u32 = 0x01;
pub const DA9063_MON_A10_IDX_LDO7: u32 = 0x02;
pub const DA9063_MON_A10_IDX_LDO8: u32 = 0x03;
pub const DA9063_MON_A10_IDX_LDO9: u32 = 0x04;
pub const DA9063_MON_A10_IDX_LDO10: u32 = 0x05;

/* DA9063_REG_VARIANT_ID (addr=0x182) */
pub const DA9063_VARIANT_ID_VRC_SHIFT: u32 = 0;
pub const DA9063_VARIANT_ID_VRC_MASK: u32 = 0x0F;
pub const DA9063_VARIANT_ID_MRC_SHIFT: u32 = 4;
pub const DA9063_VARIANT_ID_MRC_MASK: u32 = 0xF0;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
