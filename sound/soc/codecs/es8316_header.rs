/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright Everest Semiconductor Co.,Ltd
 *
 * Author: David Yang <yangxiaohua@everest-semi.com>
 */

/*
 * ES8316 register space
 */

/* Reset Control */
pub const ES8316_RESET: u32 = 0x00;

/* Clock Management */
pub const ES8316_CLKMGR_CLKSW: u32 = 0x01;
pub const ES8316_CLKMGR_CLKSEL: u32 = 0x02;
pub const ES8316_CLKMGR_ADCOSR: u32 = 0x03;
pub const ES8316_CLKMGR_ADCDIV1: u32 = 0x04;
pub const ES8316_CLKMGR_ADCDIV2: u32 = 0x05;
pub const ES8316_CLKMGR_DACDIV1: u32 = 0x06;
pub const ES8316_CLKMGR_DACDIV2: u32 = 0x07;
pub const ES8316_CLKMGR_CPDIV: u32 = 0x08;

/* Serial Data Port Control */
pub const ES8316_SERDATA1: u32 = 0x09;
pub const ES8316_SERDATA_ADC: u32 = 0x0a;
pub const ES8316_SERDATA_DAC: u32 = 0x0b;

/* System Control */
pub const ES8316_SYS_VMIDSEL: u32 = 0x0c;
pub const ES8316_SYS_PDN: u32 = 0x0d;
pub const ES8316_SYS_LP1: u32 = 0x0e;
pub const ES8316_SYS_LP2: u32 = 0x0f;
pub const ES8316_SYS_VMIDLOW: u32 = 0x10;
pub const ES8316_SYS_VSEL: u32 = 0x11;
pub const ES8316_SYS_REF: u32 = 0x12;

/* Headphone Mixer */
pub const ES8316_HPMIX_SEL: u32 = 0x13;
pub const ES8316_HPMIX_SWITCH: u32 = 0x14;
pub const ES8316_HPMIX_PDN: u32 = 0x15;
pub const ES8316_HPMIX_VOL: u32 = 0x16;

/* Charge Pump Headphone driver */
pub const ES8316_CPHP_OUTEN: u32 = 0x17;
pub const ES8316_CPHP_ICAL_VOL: u32 = 0x18;
pub const ES8316_CPHP_PDN1: u32 = 0x19;
pub const ES8316_CPHP_PDN2: u32 = 0x1a;
pub const ES8316_CPHP_LDOCTL: u32 = 0x1b;

/* Calibration */
pub const ES8316_CAL_TYPE: u32 = 0x1c;
pub const ES8316_CAL_SET: u32 = 0x1d;
pub const ES8316_CAL_HPLIV: u32 = 0x1e;
pub const ES8316_CAL_HPRIV: u32 = 0x1f;
pub const ES8316_CAL_HPLMV: u32 = 0x20;
pub const ES8316_CAL_HPRMV: u32 = 0x21;

/* ADC Control */
pub const ES8316_ADC_PDN_LINSEL: u32 = 0x22;
pub const ES8316_ADC_PGAGAIN: u32 = 0x23;
pub const ES8316_ADC_D2SEPGA: u32 = 0x24;
pub const ES8316_ADC_DMIC: u32 = 0x25;
pub const ES8316_ADC_MUTE: u32 = 0x26;
pub const ES8316_ADC_VOLUME: u32 = 0x27;
pub const ES8316_ADC_ALC1: u32 = 0x29;
pub const ES8316_ADC_ALC2: u32 = 0x2a;
pub const ES8316_ADC_ALC3: u32 = 0x2b;
pub const ES8316_ADC_ALC4: u32 = 0x2c;
pub const ES8316_ADC_ALC5: u32 = 0x2d;
pub const ES8316_ADC_ALC_NG: u32 = 0x2e;

/* DAC Control */
pub const ES8316_DAC_PDN: u32 = 0x2f;
pub const ES8316_DAC_SET1: u32 = 0x30;
pub const ES8316_DAC_SET2: u32 = 0x31;
pub const ES8316_DAC_SET3: u32 = 0x32;
pub const ES8316_DAC_VOLL: u32 = 0x33;
pub const ES8316_DAC_VOLR: u32 = 0x34;

/* GPIO */
pub const ES8316_GPIO_SEL: u32 = 0x4d;
pub const ES8316_GPIO_DEBOUNCE: u32 = 0x4e;
pub const ES8316_GPIO_FLAG: u32 = 0x4f;

/* Test mode */
pub const ES8316_TESTMODE: u32 = 0x50;
pub const ES8316_TEST1: u32 = 0x51;
pub const ES8316_TEST2: u32 = 0x52;
pub const ES8316_TEST3: u32 = 0x53;

/*
 * Field definitions
 */

/* ES8316_RESET */
pub const ES8316_RESET_CSM_ON: u32 = 0x80;

/* ES8316_CLKMGR_CLKSW */
pub const ES8316_CLKMGR_CLKSW_MCLK_ON: u32 = 0x40;
pub const ES8316_CLKMGR_CLKSW_BCLK_ON: u32 = 0x20;

/* ES8316_SERDATA1 */
pub const ES8316_SERDATA1_MASTER: u32 = 0x80;
pub const ES8316_SERDATA1_BCLK_INV: u32 = 0x20;

/* ES8316_SERDATA_ADC and _DAC */
pub const ES8316_SERDATA2_FMT_MASK: u32 = 0x3;
pub const ES8316_SERDATA2_FMT_I2S: u32 = 0x00;
pub const ES8316_SERDATA2_FMT_LEFTJ: u32 = 0x01;
pub const ES8316_SERDATA2_FMT_RIGHTJ: u32 = 0x02;
pub const ES8316_SERDATA2_FMT_PCM: u32 = 0x03;
pub const ES8316_SERDATA2_ADCLRP: u32 = 0x20;
pub const ES8316_SERDATA2_LEN_MASK: u32 = 0x1c;
pub const ES8316_SERDATA2_LEN_24: u32 = 0x00;
pub const ES8316_SERDATA2_LEN_20: u32 = 0x04;
pub const ES8316_SERDATA2_LEN_18: u32 = 0x08;
pub const ES8316_SERDATA2_LEN_16: u32 = 0x0c;
pub const ES8316_SERDATA2_LEN_32: u32 = 0x10;

/* ES8316_GPIO_DEBOUNCE	*/
pub const ES8316_GPIO_ENABLE_INTERRUPT: u32 = 0x02;

/* ES8316_GPIO_FLAG */
pub const ES8316_GPIO_FLAG_GM_NOT_SHORTED: u32 = 0x02;
pub const ES8316_GPIO_FLAG_HP_NOT_INSERTED: u32 = 0x04;

/* ES8316_CLKMGR_CLKSW */
pub const ES8316_CLKMGR_CLKSW_MCLK_DIV: u32 = 0x80;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
