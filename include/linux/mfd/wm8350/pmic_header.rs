/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * pmic.h  --  Power Management Driver for Wolfson WM8350 PMIC
 *
 * Copyright 2007 Wolfson Microelectronics PLC
 */

/*
 * Register values.
 */

pub const WM8350_CURRENT_SINK_DRIVER_A: i32 = 0xAC;
pub const WM8350_CSA_FLASH_CONTROL: i32 = 0xAD;
pub const WM8350_CURRENT_SINK_DRIVER_B: i32 = 0xAE;
pub const WM8350_CSB_FLASH_CONTROL: i32 = 0xAF;
pub const WM8350_DCDC_LDO_REQUESTED: i32 = 0xB0;
pub const WM8350_DCDC_ACTIVE_OPTIONS: i32 = 0xB1;
pub const WM8350_DCDC_SLEEP_OPTIONS: i32 = 0xB2;
pub const WM8350_POWER_CHECK_COMPARATOR: i32 = 0xB3;
pub const WM8350_DCDC1_CONTROL: i32 = 0xB4;
pub const WM8350_DCDC1_TIMEOUTS: i32 = 0xB5;
pub const WM8350_DCDC1_LOW_POWER: i32 = 0xB6;
pub const WM8350_DCDC2_CONTROL: i32 = 0xB7;
pub const WM8350_DCDC2_TIMEOUTS: i32 = 0xB8;
pub const WM8350_DCDC3_CONTROL: i32 = 0xBA;
pub const WM8350_DCDC3_TIMEOUTS: i32 = 0xBB;
pub const WM8350_DCDC3_LOW_POWER: i32 = 0xBC;
pub const WM8350_DCDC4_CONTROL: i32 = 0xBD;
pub const WM8350_DCDC4_TIMEOUTS: i32 = 0xBE;
pub const WM8350_DCDC4_LOW_POWER: i32 = 0xBF;
pub const WM8350_DCDC5_CONTROL: i32 = 0xC0;
pub const WM8350_DCDC5_TIMEOUTS: i32 = 0xC1;
pub const WM8350_DCDC6_CONTROL: i32 = 0xC3;
pub const WM8350_DCDC6_TIMEOUTS: i32 = 0xC4;
pub const WM8350_DCDC6_LOW_POWER: i32 = 0xC5;
pub const WM8350_LIMIT_SWITCH_CONTROL: i32 = 0xC7;
pub const WM8350_LDO1_CONTROL: i32 = 0xC8;
pub const WM8350_LDO1_TIMEOUTS: i32 = 0xC9;
pub const WM8350_LDO1_LOW_POWER: i32 = 0xCA;
pub const WM8350_LDO2_CONTROL: i32 = 0xCB;
pub const WM8350_LDO2_TIMEOUTS: i32 = 0xCC;
pub const WM8350_LDO2_LOW_POWER: i32 = 0xCD;
pub const WM8350_LDO3_CONTROL: i32 = 0xCE;
pub const WM8350_LDO3_TIMEOUTS: i32 = 0xCF;
pub const WM8350_LDO3_LOW_POWER: i32 = 0xD0;
pub const WM8350_LDO4_CONTROL: i32 = 0xD1;
pub const WM8350_LDO4_TIMEOUTS: i32 = 0xD2;
pub const WM8350_LDO4_LOW_POWER: i32 = 0xD3;
pub const WM8350_VCC_FAULT_MASKS: i32 = 0xD7;
pub const WM8350_MAIN_BANDGAP_CONTROL: i32 = 0xD8;
pub const WM8350_OSC_CONTROL: i32 = 0xD9;
pub const WM8350_RTC_TICK_CONTROL: i32 = 0xDA;
pub const WM8350_SECURITY: i32 = 0xDB;
pub const WM8350_RAM_BIST_1: i32 = 0xDC;
pub const WM8350_DCDC_LDO_STATUS: i32 = 0xE1;
pub const WM8350_GPIO_PIN_STATUS: i32 = 0xE6;

pub const WM8350_DCDC1_FORCE_PWM: i32 = 0xF8;
pub const WM8350_DCDC3_FORCE_PWM: i32 = 0xFA;
pub const WM8350_DCDC4_FORCE_PWM: i32 = 0xFB;
pub const WM8350_DCDC6_FORCE_PWM: i32 = 0xFD;

/*
 * R172 (0xAC) - Current Sink Driver A
 */
pub const WM8350_CS1_HIB_MODE: i32 = 0x1000;
pub const WM8350_CS1_HIB_MODE_MASK: i32 = 0x1000;
pub const WM8350_CS1_HIB_MODE_SHIFT: i32 = 12;
pub const WM8350_CS1_ISEL_MASK: i32 = 0x003F;
pub const WM8350_CS1_ISEL_SHIFT: i32 = 0;

/* Bit values for R172 (0xAC) */
pub const WM8350_CS1_HIB_MODE_DISABLE: i32 = 0;
pub const WM8350_CS1_HIB_MODE_LEAVE: i32 = 1;

pub const WM8350_CS1_ISEL_220M: i32 = 0x3F;

/*
 * R173 (0xAD) - CSA Flash control
 */
pub const WM8350_CS1_FLASH_MODE: i32 = 0x8000;
pub const WM8350_CS1_TRIGSRC: i32 = 0x4000;
pub const WM8350_CS1_DRIVE: i32 = 0x2000;
pub const WM8350_CS1_FLASH_DUR_MASK: i32 = 0x0300;
pub const WM8350_CS1_OFF_RAMP_MASK: i32 = 0x0030;
pub const WM8350_CS1_ON_RAMP_MASK: i32 = 0x0003;

/*
 * R174 (0xAE) - Current Sink Driver B
 */
pub const WM8350_CS2_HIB_MODE: i32 = 0x1000;
pub const WM8350_CS2_ISEL_MASK: i32 = 0x003F;

/*
 * R175 (0xAF) - CSB Flash control
 */
pub const WM8350_CS2_FLASH_MODE: i32 = 0x8000;
pub const WM8350_CS2_TRIGSRC: i32 = 0x4000;
pub const WM8350_CS2_DRIVE: i32 = 0x2000;
pub const WM8350_CS2_FLASH_DUR_MASK: i32 = 0x0300;
pub const WM8350_CS2_OFF_RAMP_MASK: i32 = 0x0030;
pub const WM8350_CS2_ON_RAMP_MASK: i32 = 0x0003;

/*
 * R176 (0xB0) - DCDC/LDO requested
 */
pub const WM8350_LS_ENA: i32 = 0x8000;
pub const WM8350_LDO4_ENA: i32 = 0x0800;
pub const WM8350_LDO3_ENA: i32 = 0x0400;
pub const WM8350_LDO2_ENA: i32 = 0x0200;
pub const WM8350_LDO1_ENA: i32 = 0x0100;
pub const WM8350_DC6_ENA: i32 = 0x0020;
pub const WM8350_DC5_ENA: i32 = 0x0010;
pub const WM8350_DC4_ENA: i32 = 0x0008;
pub const WM8350_DC3_ENA: i32 = 0x0004;
pub const WM8350_DC2_ENA: i32 = 0x0002;
pub const WM8350_DC1_ENA: i32 = 0x0001;

/*
 * R177 (0xB1) - DCDC Active options
 */
pub const WM8350_PUTO_MASK: i32 = 0x3000;
pub const WM8350_PWRUP_DELAY_MASK: i32 = 0x0300;
pub const WM8350_DC6_ACTIVE: i32 = 0x0020;
pub const WM8350_DC4_ACTIVE: i32 = 0x0008;
pub const WM8350_DC3_ACTIVE: i32 = 0x0004;
pub const WM8350_DC1_ACTIVE: i32 = 0x0001;

/*
 * R178 (0xB2) - DCDC Sleep options
 */
pub const WM8350_DC6_SLEEP: i32 = 0x0020;
pub const WM8350_DC4_SLEEP: i32 = 0x0008;
pub const WM8350_DC3_SLEEP: i32 = 0x0004;
pub const WM8350_DC1_SLEEP: i32 = 0x0001;

/*
 * R179 (0xB3) - Power-check comparator
 */
pub const WM8350_PCCMP_ERRACT: i32 = 0x4000;
pub const WM8350_PCCMP_RAIL: i32 = 0x0100;
pub const WM8350_PCCMP_OFF_THR_MASK: i32 = 0x0070;
pub const WM8350_PCCMP_ON_THR_MASK: i32 = 0x0007;

/*
 * R180 (0xB4) - DCDC1 Control
 */
pub const WM8350_DC1_OPFLT: i32 = 0x0400;
pub const WM8350_DC1_VSEL_MASK: i32 = 0x007F;
pub const WM8350_DC1_VSEL_SHIFT: i32 = 0;

/*
 * R181 (0xB5) - DCDC1 Timeouts
 */
pub const WM8350_DC1_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_DC1_ERRACT_SHIFT: i32 = 14;
pub const WM8350_DC1_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_DC1_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_DC1_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_DC1_UVTO_MASK: i32 = 0x0030;
pub const WM8350_DC1_SDSLOT_SHIFT: i32 = 6;

/* Bit values for R181 (0xB5) */
pub const WM8350_DC1_ERRACT_NONE: i32 = 0;
pub const WM8350_DC1_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_DC1_ERRACT_SHUTDOWN_SYS: i32 = 2;

/*
 * R182 (0xB6) - DCDC1 Low Power
 */
pub const WM8350_DC1_HIB_MODE_MASK: i32 = 0x7000;
pub const WM8350_DC1_HIB_TRIG_MASK: i32 = 0x0300;
pub const WM8350_DC1_VIMG_MASK: i32 = 0x007F;

/*
 * R183 (0xB7) - DCDC2 Control
 */
pub const WM8350_DC2_MODE: i32 = 0x4000;
pub const WM8350_DC2_MODE_MASK: i32 = 0x4000;
pub const WM8350_DC2_MODE_SHIFT: i32 = 14;
pub const WM8350_DC2_HIB_MODE: i32 = 0x1000;
pub const WM8350_DC2_HIB_MODE_MASK: i32 = 0x1000;
pub const WM8350_DC2_HIB_MODE_SHIFT: i32 = 12;
pub const WM8350_DC2_HIB_TRIG_MASK: i32 = 0x0300;
pub const WM8350_DC2_HIB_TRIG_SHIFT: i32 = 8;
pub const WM8350_DC2_ILIM: i32 = 0x0040;
pub const WM8350_DC2_ILIM_MASK: i32 = 0x0040;
pub const WM8350_DC2_ILIM_SHIFT: i32 = 6;
pub const WM8350_DC2_RMP_MASK: i32 = 0x0018;
pub const WM8350_DC2_RMP_SHIFT: i32 = 3;
pub const WM8350_DC2_FBSRC_MASK: i32 = 0x0003;
pub const WM8350_DC2_FBSRC_SHIFT: i32 = 0;

/* Bit values for R183 (0xB7) */
pub const WM8350_DC2_MODE_BOOST: i32 = 0;
pub const WM8350_DC2_MODE_SWITCH: i32 = 1;

pub const WM8350_DC2_HIB_MODE_ACTIVE: i32 = 1;
pub const WM8350_DC2_HIB_MODE_DISABLE: i32 = 0;

pub const WM8350_DC2_HIB_TRIG_NONE: i32 = 0;
pub const WM8350_DC2_HIB_TRIG_LPWR1: i32 = 1;
pub const WM8350_DC2_HIB_TRIG_LPWR2: i32 = 2;
pub const WM8350_DC2_HIB_TRIG_LPWR3: i32 = 3;

pub const WM8350_DC2_ILIM_HIGH: i32 = 0;
pub const WM8350_DC2_ILIM_LOW: i32 = 1;

pub const WM8350_DC2_RMP_30V: i32 = 0;
pub const WM8350_DC2_RMP_20V: i32 = 1;
pub const WM8350_DC2_RMP_10V: i32 = 2;
pub const WM8350_DC2_RMP_5V: i32 = 3;

pub const WM8350_DC2_FBSRC_FB2: i32 = 0;
pub const WM8350_DC2_FBSRC_ISINKA: i32 = 1;
pub const WM8350_DC2_FBSRC_ISINKB: i32 = 2;
pub const WM8350_DC2_FBSRC_USB: i32 = 3;

/*
 * R184 (0xB8) - DCDC2 Timeouts
 */
pub const WM8350_DC2_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_DC2_ERRACT_SHIFT: i32 = 14;
pub const WM8350_DC2_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_DC2_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_DC2_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_DC2_UVTO_MASK: i32 = 0x0030;

/* Bit values for R184 (0xB8) */
pub const WM8350_DC2_ERRACT_NONE: i32 = 0;
pub const WM8350_DC2_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_DC2_ERRACT_SHUTDOWN_SYS: i32 = 2;

/*
 * R186 (0xBA) - DCDC3 Control
 */
pub const WM8350_DC3_OPFLT: i32 = 0x0400;
pub const WM8350_DC3_VSEL_MASK: i32 = 0x007F;
pub const WM8350_DC3_VSEL_SHIFT: i32 = 0;

/*
 * R187 (0xBB) - DCDC3 Timeouts
 */
pub const WM8350_DC3_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_DC3_ERRACT_SHIFT: i32 = 14;
pub const WM8350_DC3_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_DC3_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_DC3_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_DC3_UVTO_MASK: i32 = 0x0030;
pub const WM8350_DC3_SDSLOT_SHIFT: i32 = 6;

/* Bit values for R187 (0xBB) */
pub const WM8350_DC3_ERRACT_NONE: i32 = 0;
pub const WM8350_DC3_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_DC3_ERRACT_SHUTDOWN_SYS: i32 = 2;
/*
 * R188 (0xBC) - DCDC3 Low Power
 */
pub const WM8350_DC3_HIB_MODE_MASK: i32 = 0x7000;
pub const WM8350_DC3_HIB_TRIG_MASK: i32 = 0x0300;
pub const WM8350_DC3_VIMG_MASK: i32 = 0x007F;

/*
 * R189 (0xBD) - DCDC4 Control
 */
pub const WM8350_DC4_OPFLT: i32 = 0x0400;
pub const WM8350_DC4_VSEL_MASK: i32 = 0x007F;
pub const WM8350_DC4_VSEL_SHIFT: i32 = 0;

/*
 * R190 (0xBE) - DCDC4 Timeouts
 */
pub const WM8350_DC4_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_DC4_ERRACT_SHIFT: i32 = 14;
pub const WM8350_DC4_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_DC4_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_DC4_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_DC4_UVTO_MASK: i32 = 0x0030;
pub const WM8350_DC4_SDSLOT_SHIFT: i32 = 6;

/* Bit values for R190 (0xBE) */
pub const WM8350_DC4_ERRACT_NONE: i32 = 0;
pub const WM8350_DC4_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_DC4_ERRACT_SHUTDOWN_SYS: i32 = 2;

/*
 * R191 (0xBF) - DCDC4 Low Power
 */
pub const WM8350_DC4_HIB_MODE_MASK: i32 = 0x7000;
pub const WM8350_DC4_HIB_TRIG_MASK: i32 = 0x0300;
pub const WM8350_DC4_VIMG_MASK: i32 = 0x007F;

/*
 * R192 (0xC0) - DCDC5 Control
 */
pub const WM8350_DC5_MODE: i32 = 0x4000;
pub const WM8350_DC5_MODE_MASK: i32 = 0x4000;
pub const WM8350_DC5_MODE_SHIFT: i32 = 14;
pub const WM8350_DC5_HIB_MODE: i32 = 0x1000;
pub const WM8350_DC5_HIB_MODE_MASK: i32 = 0x1000;
pub const WM8350_DC5_HIB_MODE_SHIFT: i32 = 12;
pub const WM8350_DC5_HIB_TRIG_MASK: i32 = 0x0300;
pub const WM8350_DC5_HIB_TRIG_SHIFT: i32 = 8;
pub const WM8350_DC5_ILIM: i32 = 0x0040;
pub const WM8350_DC5_ILIM_MASK: i32 = 0x0040;
pub const WM8350_DC5_ILIM_SHIFT: i32 = 6;
pub const WM8350_DC5_RMP_MASK: i32 = 0x0018;
pub const WM8350_DC5_RMP_SHIFT: i32 = 3;
pub const WM8350_DC5_FBSRC_MASK: i32 = 0x0003;
pub const WM8350_DC5_FBSRC_SHIFT: i32 = 0;

/* Bit values for R192 (0xC0) */
pub const WM8350_DC5_MODE_BOOST: i32 = 0;
pub const WM8350_DC5_MODE_SWITCH: i32 = 1;

pub const WM8350_DC5_HIB_MODE_ACTIVE: i32 = 1;
pub const WM8350_DC5_HIB_MODE_DISABLE: i32 = 0;

pub const WM8350_DC5_HIB_TRIG_NONE: i32 = 0;
pub const WM8350_DC5_HIB_TRIG_LPWR1: i32 = 1;
pub const WM8350_DC5_HIB_TRIG_LPWR2: i32 = 2;
pub const WM8350_DC5_HIB_TRIG_LPWR3: i32 = 3;

pub const WM8350_DC5_ILIM_HIGH: i32 = 0;
pub const WM8350_DC5_ILIM_LOW: i32 = 1;

pub const WM8350_DC5_RMP_30V: i32 = 0;
pub const WM8350_DC5_RMP_20V: i32 = 1;
pub const WM8350_DC5_RMP_10V: i32 = 2;
pub const WM8350_DC5_RMP_5V: i32 = 3;

pub const WM8350_DC5_FBSRC_FB2: i32 = 0;
pub const WM8350_DC5_FBSRC_ISINKA: i32 = 1;
pub const WM8350_DC5_FBSRC_ISINKB: i32 = 2;
pub const WM8350_DC5_FBSRC_USB: i32 = 3;

/*
 * R193 (0xC1) - DCDC5 Timeouts
 */
pub const WM8350_DC5_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_DC5_ERRACT_SHIFT: i32 = 14;
pub const WM8350_DC5_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_DC5_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_DC5_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_DC5_UVTO_MASK: i32 = 0x0030;
pub const WM8350_DC5_SDSLOT_SHIFT: i32 = 6;

/* Bit values for R193 (0xC1) */
pub const WM8350_DC5_ERRACT_NONE: i32 = 0;
pub const WM8350_DC5_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_DC5_ERRACT_SHUTDOWN_SYS: i32 = 2;

/*
 * R195 (0xC3) - DCDC6 Control
 */
pub const WM8350_DC6_OPFLT: i32 = 0x0400;
pub const WM8350_DC6_VSEL_MASK: i32 = 0x007F;
pub const WM8350_DC6_VSEL_SHIFT: i32 = 0;

/*
 * R196 (0xC4) - DCDC6 Timeouts
 */
pub const WM8350_DC6_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_DC6_ERRACT_SHIFT: i32 = 14;
pub const WM8350_DC6_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_DC6_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_DC6_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_DC6_UVTO_MASK: i32 = 0x0030;
pub const WM8350_DC6_SDSLOT_SHIFT: i32 = 6;

/* Bit values for R196 (0xC4) */
pub const WM8350_DC6_ERRACT_NONE: i32 = 0;
pub const WM8350_DC6_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_DC6_ERRACT_SHUTDOWN_SYS: i32 = 2;

/*
 * R197 (0xC5) - DCDC6 Low Power
 */
pub const WM8350_DC6_HIB_MODE_MASK: i32 = 0x7000;
pub const WM8350_DC6_HIB_TRIG_MASK: i32 = 0x0300;
pub const WM8350_DC6_VIMG_MASK: i32 = 0x007F;

/*
 * R199 (0xC7) - Limit Switch Control
 */
pub const WM8350_LS_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_LS_ERRACT_SHIFT: i32 = 14;
pub const WM8350_LS_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_LS_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_LS_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_LS_SDSLOT_SHIFT: i32 = 6;
pub const WM8350_LS_HIB_MODE: i32 = 0x0010;
pub const WM8350_LS_HIB_MODE_MASK: i32 = 0x0010;
pub const WM8350_LS_HIB_MODE_SHIFT: i32 = 4;
pub const WM8350_LS_HIB_PROT: i32 = 0x0002;
pub const WM8350_LS_HIB_PROT_MASK: i32 = 0x0002;
pub const WM8350_LS_HIB_PROT_SHIFT: i32 = 1;
pub const WM8350_LS_PROT: i32 = 0x0001;
pub const WM8350_LS_PROT_MASK: i32 = 0x0001;
pub const WM8350_LS_PROT_SHIFT: i32 = 0;

/* Bit values for R199 (0xC7) */
pub const WM8350_LS_ERRACT_NONE: i32 = 0;
pub const WM8350_LS_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_LS_ERRACT_SHUTDOWN_SYS: i32 = 2;

/*
 * R200 (0xC8) - LDO1 Control
 */
pub const WM8350_LDO1_SWI: i32 = 0x4000;
pub const WM8350_LDO1_OPFLT: i32 = 0x0400;
pub const WM8350_LDO1_VSEL_MASK: i32 = 0x001F;
pub const WM8350_LDO1_VSEL_SHIFT: i32 = 0;

/*
 * R201 (0xC9) - LDO1 Timeouts
 */
pub const WM8350_LDO1_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_LDO1_ERRACT_SHIFT: i32 = 14;
pub const WM8350_LDO1_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_LDO1_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_LDO1_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_LDO1_UVTO_MASK: i32 = 0x0030;
pub const WM8350_LDO1_SDSLOT_SHIFT: i32 = 6;

/* Bit values for R201 (0xC9) */
pub const WM8350_LDO1_ERRACT_NONE: i32 = 0;
pub const WM8350_LDO1_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_LDO1_ERRACT_SHUTDOWN_SYS: i32 = 2;

/*
 * R202 (0xCA) - LDO1 Low Power
 */
pub const WM8350_LDO1_HIB_MODE_MASK: i32 = 0x3000;
pub const WM8350_LDO1_HIB_TRIG_MASK: i32 = 0x0300;
pub const WM8350_LDO1_VIMG_MASK: i32 = 0x001F;
pub const WM8350_LDO1_HIB_MODE_DIS: i32 = (0x1 << 12);


/*
 * R203 (0xCB) - LDO2 Control
 */
pub const WM8350_LDO2_SWI: i32 = 0x4000;
pub const WM8350_LDO2_OPFLT: i32 = 0x0400;
pub const WM8350_LDO2_VSEL_MASK: i32 = 0x001F;
pub const WM8350_LDO2_VSEL_SHIFT: i32 = 0;

/*
 * R204 (0xCC) - LDO2 Timeouts
 */
pub const WM8350_LDO2_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_LDO2_ERRACT_SHIFT: i32 = 14;
pub const WM8350_LDO2_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_LDO2_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_LDO2_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_LDO2_SDSLOT_SHIFT: i32 = 6;

/* Bit values for R204 (0xCC) */
pub const WM8350_LDO2_ERRACT_NONE: i32 = 0;
pub const WM8350_LDO2_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_LDO2_ERRACT_SHUTDOWN_SYS: i32 = 2;

/*
 * R205 (0xCD) - LDO2 Low Power
 */
pub const WM8350_LDO2_HIB_MODE_MASK: i32 = 0x3000;
pub const WM8350_LDO2_HIB_TRIG_MASK: i32 = 0x0300;
pub const WM8350_LDO2_VIMG_MASK: i32 = 0x001F;

/*
 * R206 (0xCE) - LDO3 Control
 */
pub const WM8350_LDO3_SWI: i32 = 0x4000;
pub const WM8350_LDO3_OPFLT: i32 = 0x0400;
pub const WM8350_LDO3_VSEL_MASK: i32 = 0x001F;
pub const WM8350_LDO3_VSEL_SHIFT: i32 = 0;

/*
 * R207 (0xCF) - LDO3 Timeouts
 */
pub const WM8350_LDO3_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_LDO3_ERRACT_SHIFT: i32 = 14;
pub const WM8350_LDO3_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_LDO3_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_LDO3_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_LDO3_UVTO_MASK: i32 = 0x0030;
pub const WM8350_LDO3_SDSLOT_SHIFT: i32 = 6;

/* Bit values for R207 (0xCF) */
pub const WM8350_LDO3_ERRACT_NONE: i32 = 0;
pub const WM8350_LDO3_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_LDO3_ERRACT_SHUTDOWN_SYS: i32 = 2;

/*
 * R208 (0xD0) - LDO3 Low Power
 */
pub const WM8350_LDO3_HIB_MODE_MASK: i32 = 0x3000;
pub const WM8350_LDO3_HIB_TRIG_MASK: i32 = 0x0300;
pub const WM8350_LDO3_VIMG_MASK: i32 = 0x001F;

/*
 * R209 (0xD1) - LDO4 Control
 */
pub const WM8350_LDO4_SWI: i32 = 0x4000;
pub const WM8350_LDO4_OPFLT: i32 = 0x0400;
pub const WM8350_LDO4_VSEL_MASK: i32 = 0x001F;
pub const WM8350_LDO4_VSEL_SHIFT: i32 = 0;

/*
 * R210 (0xD2) - LDO4 Timeouts
 */
pub const WM8350_LDO4_ERRACT_MASK: i32 = 0xC000;
pub const WM8350_LDO4_ERRACT_SHIFT: i32 = 14;
pub const WM8350_LDO4_ENSLOT_MASK: i32 = 0x3C00;
pub const WM8350_LDO4_ENSLOT_SHIFT: i32 = 10;
pub const WM8350_LDO4_SDSLOT_MASK: i32 = 0x03C0;
pub const WM8350_LDO4_UVTO_MASK: i32 = 0x0030;
pub const WM8350_LDO4_SDSLOT_SHIFT: i32 = 6;

/* Bit values for R210 (0xD2) */
pub const WM8350_LDO4_ERRACT_NONE: i32 = 0;
pub const WM8350_LDO4_ERRACT_SHUTDOWN_CONV: i32 = 1;
pub const WM8350_LDO4_ERRACT_SHUTDOWN_SYS: i32 = 2;

/*
 * R211 (0xD3) - LDO4 Low Power
 */
pub const WM8350_LDO4_HIB_MODE_MASK: i32 = 0x3000;
pub const WM8350_LDO4_HIB_TRIG_MASK: i32 = 0x0300;
pub const WM8350_LDO4_VIMG_MASK: i32 = 0x001F;

/*
 * R215 (0xD7) - VCC_FAULT Masks
 */
pub const WM8350_LS_FAULT: i32 = 0x8000;
pub const WM8350_LDO4_FAULT: i32 = 0x0800;
pub const WM8350_LDO3_FAULT: i32 = 0x0400;
pub const WM8350_LDO2_FAULT: i32 = 0x0200;
pub const WM8350_LDO1_FAULT: i32 = 0x0100;
pub const WM8350_DC6_FAULT: i32 = 0x0020;
pub const WM8350_DC5_FAULT: i32 = 0x0010;
pub const WM8350_DC4_FAULT: i32 = 0x0008;
pub const WM8350_DC3_FAULT: i32 = 0x0004;
pub const WM8350_DC2_FAULT: i32 = 0x0002;
pub const WM8350_DC1_FAULT: i32 = 0x0001;

/*
 * R216 (0xD8) - Main Bandgap Control
 */
pub const WM8350_MBG_LOAD_FUSES: i32 = 0x8000;
pub const WM8350_MBG_FUSE_WPREP: i32 = 0x4000;
pub const WM8350_MBG_FUSE_WRITE: i32 = 0x2000;
pub const WM8350_MBG_FUSE_TRIM_MASK: i32 = 0x1F00;
pub const WM8350_MBG_TRIM_SRC: i32 = 0x0020;
pub const WM8350_MBG_USER_TRIM_MASK: i32 = 0x001F;

/*
 * R217 (0xD9) - OSC Control
 */
pub const WM8350_OSC_LOAD_FUSES: i32 = 0x8000;
pub const WM8350_OSC_FUSE_WPREP: i32 = 0x4000;
pub const WM8350_OSC_FUSE_WRITE: i32 = 0x2000;
pub const WM8350_OSC_FUSE_TRIM_MASK: i32 = 0x0F00;
pub const WM8350_OSC_TRIM_SRC: i32 = 0x0020;
pub const WM8350_OSC_USER_TRIM_MASK: i32 = 0x000F;

/*
 * R248 (0xF8) - DCDC1 Force PWM
 */
pub const WM8350_DCDC1_FORCE_PWM_ENA: i32 = 0x0010;

/*
 * R250 (0xFA) - DCDC3 Force PWM
 */
pub const WM8350_DCDC3_FORCE_PWM_ENA: i32 = 0x0010;

/*
 * R251 (0xFB) - DCDC4 Force PWM
 */
pub const WM8350_DCDC4_FORCE_PWM_ENA: i32 = 0x0010;

/*
 * R253 (0xFD) - DCDC1 Force PWM
 */
pub const WM8350_DCDC6_FORCE_PWM_ENA: i32 = 0x0010;

/*
 * DCDC's
 */
pub const WM8350_DCDC_1: i32 = 0;
pub const WM8350_DCDC_2: i32 = 1;
pub const WM8350_DCDC_3: i32 = 2;
pub const WM8350_DCDC_4: i32 = 3;
pub const WM8350_DCDC_5: i32 = 4;
pub const WM8350_DCDC_6: i32 = 5;

/* DCDC modes */
pub const WM8350_DCDC_ACTIVE_STANDBY: i32 = 0;
pub const WM8350_DCDC_ACTIVE_PULSE: i32 = 1;
pub const WM8350_DCDC_SLEEP_NORMAL: i32 = 0;
pub const WM8350_DCDC_SLEEP_LOW: i32 = 1;

/* DCDC Low power (Hibernate) mode */
pub const WM8350_DCDC_HIB_MODE_CUR: i32 = (0 << 12);
pub const WM8350_DCDC_HIB_MODE_IMAGE: i32 = (1 << 12);
pub const WM8350_DCDC_HIB_MODE_STANDBY: i32 = (2 << 12);
pub const WM8350_DCDC_HIB_MODE_LDO: i32 = (4 << 12);
pub const WM8350_DCDC_HIB_MODE_LDO_IM: i32 = (5 << 12);
pub const WM8350_DCDC_HIB_MODE_DIS: i32 = (7 << 12);
pub const WM8350_DCDC_HIB_MODE_MASK: i32 = (7 << 12);

/* DCDC Low Power (Hibernate) signal */
pub const WM8350_DCDC_HIB_SIG_REG: i32 = (0 << 8);
pub const WM8350_DCDC_HIB_SIG_LPWR1: i32 = (1 << 8);
pub const WM8350_DCDC_HIB_SIG_LPWR2: i32 = (2 << 8);
pub const WM8350_DCDC_HIB_SIG_LPWR3: i32 = (3 << 8);

/* LDO Low power (Hibernate) mode */
pub const WM8350_LDO_HIB_MODE_IMAGE: i32 = (0 << 0);
pub const WM8350_LDO_HIB_MODE_DIS: i32 = (1 << 0);

/* LDO Low Power (Hibernate) signal */
pub const WM8350_LDO_HIB_SIG_REG: i32 = (0 << 8);
pub const WM8350_LDO_HIB_SIG_LPWR1: i32 = (1 << 8);
pub const WM8350_LDO_HIB_SIG_LPWR2: i32 = (2 << 8);
pub const WM8350_LDO_HIB_SIG_LPWR3: i32 = (3 << 8);

/*
 * LDOs
 */
pub const WM8350_LDO_1: i32 = 6;
pub const WM8350_LDO_2: i32 = 7;
pub const WM8350_LDO_3: i32 = 8;
pub const WM8350_LDO_4: i32 = 9;

/*
 * ISINKs
 */
pub const WM8350_ISINK_A: i32 = 10;
pub const WM8350_ISINK_B: i32 = 11;

pub const WM8350_ISINK_MODE_BOOST: i32 = 0;
pub const WM8350_ISINK_MODE_SWITCH: i32 = 1;
pub const WM8350_ISINK_ILIM_NORMAL: i32 = 0;
pub const WM8350_ISINK_ILIM_LOW: i32 = 1;

pub const WM8350_ISINK_FLASH_DISABLE: i32 = 0;
pub const WM8350_ISINK_FLASH_ENABLE: i32 = 1;
pub const WM8350_ISINK_FLASH_TRIG_BIT: i32 = 0;
pub const WM8350_ISINK_FLASH_TRIG_GPIO: i32 = 1;
pub const WM8350_ISINK_FLASH_MODE_EN: i32 = (1 << 13);
pub const WM8350_ISINK_FLASH_MODE_DIS: i32 = (0 << 13);
pub const WM8350_ISINK_FLASH_DUR_32MS: i32 = (0 << 8);
pub const WM8350_ISINK_FLASH_DUR_64MS: i32 = (1 << 8);
pub const WM8350_ISINK_FLASH_DUR_96MS: i32 = (2 << 8);
pub const WM8350_ISINK_FLASH_DUR_1024MS: i32 = (3 << 8);
pub const WM8350_ISINK_FLASH_ON_INSTANT: i32 = (0 << 0);
pub const WM8350_ISINK_FLASH_ON_0_25S: i32 = (1 << 0);
pub const WM8350_ISINK_FLASH_ON_0_50S: i32 = (2 << 0);
pub const WM8350_ISINK_FLASH_ON_1_00S: i32 = (3 << 0);
pub const WM8350_ISINK_FLASH_ON_1_95S: i32 = (1 << 0);
pub const WM8350_ISINK_FLASH_ON_3_91S: i32 = (2 << 0);
pub const WM8350_ISINK_FLASH_ON_7_80S: i32 = (3 << 0);
pub const WM8350_ISINK_FLASH_OFF_INSTANT: i32 = (0 << 4);
pub const WM8350_ISINK_FLASH_OFF_0_25S: i32 = (1 << 4);
pub const WM8350_ISINK_FLASH_OFF_0_50S: i32 = (2 << 4);
pub const WM8350_ISINK_FLASH_OFF_1_00S: i32 = (3 << 4);
pub const WM8350_ISINK_FLASH_OFF_1_95S: i32 = (1 << 4);
pub const WM8350_ISINK_FLASH_OFF_3_91S: i32 = (2 << 4);
pub const WM8350_ISINK_FLASH_OFF_7_80S: i32 = (3 << 4);

/*
 * Regulator Interrupts.
 */
pub const WM8350_IRQ_CS1: i32 = 13;
pub const WM8350_IRQ_CS2: i32 = 14;
pub const WM8350_IRQ_UV_LDO4: i32 = 25;
pub const WM8350_IRQ_UV_LDO3: i32 = 26;
pub const WM8350_IRQ_UV_LDO2: i32 = 27;
pub const WM8350_IRQ_UV_LDO1: i32 = 28;
pub const WM8350_IRQ_UV_DC6: i32 = 29;
pub const WM8350_IRQ_UV_DC5: i32 = 30;
pub const WM8350_IRQ_UV_DC4: i32 = 31;
pub const WM8350_IRQ_UV_DC3: i32 = 32;
pub const WM8350_IRQ_UV_DC2: i32 = 33;
pub const WM8350_IRQ_UV_DC1: i32 = 34;
pub const WM8350_IRQ_OC_LS: i32 = 35;

pub const NUM_WM8350_REGULATORS: i32 = 12;


#[repr(C)] pub struct wm8350 { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct regulator_init_data { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct led_classdev { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct regulator_consumer_supply { _private: [u8; 0] }
pub type led_brightness = i32;


/*
 * WM8350 LED platform data
 */






int wm8350_register_regulator(struct wm8350 *wm8350, int reg,
			      struct regulator_init_data *initdata);
int wm8350_register_led(struct wm8350 *wm8350, int lednum, int dcdc, int isink,
			struct wm8350_led_platform_data *pdata);

/*
 * Additional DCDC control not supported via regulator API
 */
int wm8350_dcdc_set_slot(struct wm8350 *wm8350, int dcdc, u16 start,
			 u16 stop, u16 fault);
int wm8350_dcdc25_set_mode(struct wm8350 *wm8350, int dcdc, u16 mode,
			   u16 ilim, u16 ramp, u16 feedback);

/*
 * Additional LDO control not supported via regulator API
 */
int wm8350_ldo_set_slot(struct wm8350 *wm8350, int ldo, u16 start, u16 stop);

/*
 * Additional ISINK control not supported via regulator API
 */
int wm8350_isink_set_flash(struct wm8350 *wm8350, int isink, u16 mode,
			   u16 trigger, u16 duration, u16 on_ramp,
			   u16 off_ramp, u16 drive);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
