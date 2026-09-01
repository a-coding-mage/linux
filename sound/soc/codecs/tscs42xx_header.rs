// SPDX-License-Identifier: GPL-2.0
// tscs42xx.h -- TSCS42xx ALSA SoC Audio driver
// Copyright 2017 Tempo Semiconductor, Inc.
// Author: Steven Eckhoff <steven.eckhoff.opensource@gmail.com>

pub const TSCS42XX_PLL_SRC_XTAL: u32 = 0;
pub const TSCS42XX_PLL_SRC_MCLK1: u32 = 1;
pub const TSCS42XX_PLL_SRC_MCLK2: u32 = 2;
pub const TSCS42XX_PLL_SRC_CNT: u32 = 3;

pub const fn RM(m: u32, b: u32) -> u32 {
    m << b
}

pub const fn RV(v: u32, b: u32) -> u32 {
    v << b
}




pub const R_HPVOLL: u32 = 0x0;
pub const R_HPVOLR: u32 = 0x1;
pub const R_SPKVOLL: u32 = 0x2;
pub const R_SPKVOLR: u32 = 0x3;
pub const R_DACVOLL: u32 = 0x4;
pub const R_DACVOLR: u32 = 0x5;
pub const R_ADCVOLL: u32 = 0x6;
pub const R_ADCVOLR: u32 = 0x7;
pub const R_INVOLL: u32 = 0x8;
pub const R_INVOLR: u32 = 0x9;
pub const R_INMODE: u32 = 0x0B;
pub const R_INSELL: u32 = 0x0C;
pub const R_INSELR: u32 = 0x0D;
pub const R_AIC1: u32 = 0x13;
pub const R_AIC2: u32 = 0x14;
pub const R_CNVRTR0: u32 = 0x16;
pub const R_ADCSR: u32 = 0x17;
pub const R_CNVRTR1: u32 = 0x18;
pub const R_DACSR: u32 = 0x19;
pub const R_PWRM1: u32 = 0x1A;
pub const R_PWRM2: u32 = 0x1B;
pub const R_CTL: u32 = 0x1C;
pub const R_CONFIG0: u32 = 0x1F;
pub const R_CONFIG1: u32 = 0x20;
pub const R_DMICCTL: u32 = 0x24;
pub const R_CLECTL: u32 = 0x25;
pub const R_MUGAIN: u32 = 0x26;
pub const R_COMPTH: u32 = 0x27;
pub const R_CMPRAT: u32 = 0x28;
pub const R_CATKTCL: u32 = 0x29;
pub const R_CATKTCH: u32 = 0x2A;
pub const R_CRELTCL: u32 = 0x2B;
pub const R_CRELTCH: u32 = 0x2C;
pub const R_LIMTH: u32 = 0x2D;
pub const R_LIMTGT: u32 = 0x2E;
pub const R_LATKTCL: u32 = 0x2F;
pub const R_LATKTCH: u32 = 0x30;
pub const R_LRELTCL: u32 = 0x31;
pub const R_LRELTCH: u32 = 0x32;
pub const R_EXPTH: u32 = 0x33;
pub const R_EXPRAT: u32 = 0x34;
pub const R_XATKTCL: u32 = 0x35;
pub const R_XATKTCH: u32 = 0x36;
pub const R_XRELTCL: u32 = 0x37;
pub const R_XRELTCH: u32 = 0x38;
pub const R_FXCTL: u32 = 0x39;
pub const R_DACCRWRL: u32 = 0x3A;
pub const R_DACCRWRM: u32 = 0x3B;
pub const R_DACCRWRH: u32 = 0x3C;
pub const R_DACCRRDL: u32 = 0x3D;
pub const R_DACCRRDM: u32 = 0x3E;
pub const R_DACCRRDH: u32 = 0x3F;
pub const R_DACCRADDR: u32 = 0x40;
pub const R_DCOFSEL: u32 = 0x41;
pub const R_PLLCTL9: u32 = 0x4E;
pub const R_PLLCTLA: u32 = 0x4F;
pub const R_PLLCTLB: u32 = 0x50;
pub const R_PLLCTLC: u32 = 0x51;
pub const R_PLLCTLD: u32 = 0x52;
pub const R_PLLCTLE: u32 = 0x53;
pub const R_PLLCTLF: u32 = 0x54;
pub const R_PLLCTL10: u32 = 0x55;
pub const R_PLLCTL11: u32 = 0x56;
pub const R_PLLCTL12: u32 = 0x57;
pub const R_PLLCTL1B: u32 = 0x60;
pub const R_PLLCTL1C: u32 = 0x61;
pub const R_TIMEBASE: u32 = 0x77;
pub const R_DEVIDL: u32 = 0x7D;
pub const R_DEVIDH: u32 = 0x7E;
pub const R_RESET: u32 = 0x80;
pub const R_DACCRSTAT: u32 = 0x8A;
pub const R_PLLCTL0: u32 = 0x8E;
pub const R_PLLREFSEL: u32 = 0x8F;
pub const R_DACMBCEN: u32 = 0xC7;
pub const R_DACMBCCTL: u32 = 0xC8;
pub const R_DACMBCMUG1: u32 = 0xC9;
pub const R_DACMBCTHR1: u32 = 0xCA;
pub const R_DACMBCRAT1: u32 = 0xCB;
pub const R_DACMBCATK1L: u32 = 0xCC;
pub const R_DACMBCATK1H: u32 = 0xCD;
pub const R_DACMBCREL1L: u32 = 0xCE;
pub const R_DACMBCREL1H: u32 = 0xCF;
pub const R_DACMBCMUG2: u32 = 0xD0;
pub const R_DACMBCTHR2: u32 = 0xD1;
pub const R_DACMBCRAT2: u32 = 0xD2;
pub const R_DACMBCATK2L: u32 = 0xD3;
pub const R_DACMBCATK2H: u32 = 0xD4;
pub const R_DACMBCREL2L: u32 = 0xD5;
pub const R_DACMBCREL2H: u32 = 0xD6;
pub const R_DACMBCMUG3: u32 = 0xD7;
pub const R_DACMBCTHR3: u32 = 0xD8;
pub const R_DACMBCRAT3: u32 = 0xD9;
pub const R_DACMBCATK3L: u32 = 0xDA;
pub const R_DACMBCATK3H: u32 = 0xDB;
pub const R_DACMBCREL3L: u32 = 0xDC;
pub const R_DACMBCREL3H: u32 = 0xDD;

/* Helpers */

 *      R_HPVOLL (0x0)      *
 ****************************/

/* Field Offsets */
pub const FB_HPVOLL: u32 = 0;

/* Field Masks */
pub const FM_HPVOLL: u32 = 0x7F;

/* Field Values */
pub const FV_HPVOLL_P6DB: u32 = 0x7F;
pub const FV_HPVOLL_N88PT5DB: u32 = 0x1;
pub const FV_HPVOLL_MUTE: u32 = 0x0;

/* Register Masks */
pub const RM_HPVOLL: u32 = RM(FM_HPVOLL, FB_HPVOLL);

/* Register Values */
pub const RV_HPVOLL_P6DB: u32 = RV(FV_HPVOLL_P6DB, FB_HPVOLL);
pub const RV_HPVOLL_N88PT5DB: u32 = RV(FV_HPVOLL_N88PT5DB, FB_HPVOLL);
pub const RV_HPVOLL_MUTE: u32 = RV(FV_HPVOLL_MUTE, FB_HPVOLL);

 *      R_HPVOLR (0x1)      *
 ****************************/

/* Field Offsets */
pub const FB_HPVOLR: u32 = 0;

/* Field Masks */
pub const FM_HPVOLR: u32 = 0x7F;

/* Field Values */
pub const FV_HPVOLR_P6DB: u32 = 0x7F;
pub const FV_HPVOLR_N88PT5DB: u32 = 0x1;
pub const FV_HPVOLR_MUTE: u32 = 0x0;

/* Register Masks */
pub const RM_HPVOLR: u32 = RM(FM_HPVOLR, FB_HPVOLR);

/* Register Values */
pub const RV_HPVOLR_P6DB: u32 = RV(FV_HPVOLR_P6DB, FB_HPVOLR);
pub const RV_HPVOLR_N88PT5DB: u32 = RV(FV_HPVOLR_N88PT5DB, FB_HPVOLR);
pub const RV_HPVOLR_MUTE: u32 = RV(FV_HPVOLR_MUTE, FB_HPVOLR);

 *      R_SPKVOLL (0x2)      *
 *****************************/

/* Field Offsets */
pub const FB_SPKVOLL: u32 = 0;

/* Field Masks */
pub const FM_SPKVOLL: u32 = 0x7F;

/* Field Values */
pub const FV_SPKVOLL_P12DB: u32 = 0x7F;
pub const FV_SPKVOLL_N77PT25DB: u32 = 0x8;
pub const FV_SPKVOLL_MUTE: u32 = 0x0;

/* Register Masks */
pub const RM_SPKVOLL: u32 = RM(FM_SPKVOLL, FB_SPKVOLL);

/* Register Values */
pub const RV_SPKVOLL_P12DB: u32 = RV(FV_SPKVOLL_P12DB, FB_SPKVOLL);
pub const RV_SPKVOLL_N77PT25DB: u32 = RV(FV_SPKVOLL_N77PT25DB, FB_SPKVOLL);

pub const RV_SPKVOLL_MUTE: u32 = RV(FV_SPKVOLL_MUTE, FB_SPKVOLL);

 *      R_SPKVOLR (0x3)      *
 *****************************/

/* Field Offsets */
pub const FB_SPKVOLR: u32 = 0;

/* Field Masks */
pub const FM_SPKVOLR: u32 = 0x7F;

/* Field Values */
pub const FV_SPKVOLR_P12DB: u32 = 0x7F;
pub const FV_SPKVOLR_N77PT25DB: u32 = 0x8;
pub const FV_SPKVOLR_MUTE: u32 = 0x0;

/* Register Masks */
pub const RM_SPKVOLR: u32 = RM(FM_SPKVOLR, FB_SPKVOLR);

/* Register Values */
pub const RV_SPKVOLR_P12DB: u32 = RV(FV_SPKVOLR_P12DB, FB_SPKVOLR);
pub const RV_SPKVOLR_N77PT25DB: u32 = RV(FV_SPKVOLR_N77PT25DB, FB_SPKVOLR);

pub const RV_SPKVOLR_MUTE: u32 = RV(FV_SPKVOLR_MUTE, FB_SPKVOLR);

 *      R_DACVOLL (0x4)      *
 *****************************/

/* Field Offsets */
pub const FB_DACVOLL: u32 = 0;

/* Field Masks */
pub const FM_DACVOLL: u32 = 0xFF;

/* Field Values */
pub const FV_DACVOLL_0DB: u32 = 0xFF;
pub const FV_DACVOLL_N95PT625DB: u32 = 0x1;
pub const FV_DACVOLL_MUTE: u32 = 0x0;

/* Register Masks */
pub const RM_DACVOLL: u32 = RM(FM_DACVOLL, FB_DACVOLL);

/* Register Values */
pub const RV_DACVOLL_0DB: u32 = RV(FV_DACVOLL_0DB, FB_DACVOLL);
pub const RV_DACVOLL_N95PT625DB: u32 = RV(FV_DACVOLL_N95PT625DB, FB_DACVOLL);

pub const RV_DACVOLL_MUTE: u32 = RV(FV_DACVOLL_MUTE, FB_DACVOLL);

 *      R_DACVOLR (0x5)      *
 *****************************/

/* Field Offsets */
pub const FB_DACVOLR: u32 = 0;

/* Field Masks */
pub const FM_DACVOLR: u32 = 0xFF;

/* Field Values */
pub const FV_DACVOLR_0DB: u32 = 0xFF;
pub const FV_DACVOLR_N95PT625DB: u32 = 0x1;
pub const FV_DACVOLR_MUTE: u32 = 0x0;

/* Register Masks */
pub const RM_DACVOLR: u32 = RM(FM_DACVOLR, FB_DACVOLR);

/* Register Values */
pub const RV_DACVOLR_0DB: u32 = RV(FV_DACVOLR_0DB, FB_DACVOLR);
pub const RV_DACVOLR_N95PT625DB: u32 = RV(FV_DACVOLR_N95PT625DB, FB_DACVOLR);

pub const RV_DACVOLR_MUTE: u32 = RV(FV_DACVOLR_MUTE, FB_DACVOLR);

 *      R_ADCVOLL (0x6)      *
 *****************************/

/* Field Offsets */
pub const FB_ADCVOLL: u32 = 0;

/* Field Masks */
pub const FM_ADCVOLL: u32 = 0xFF;

/* Field Values */
pub const FV_ADCVOLL_P24DB: u32 = 0xFF;
pub const FV_ADCVOLL_N71PT25DB: u32 = 0x1;
pub const FV_ADCVOLL_MUTE: u32 = 0x0;

/* Register Masks */
pub const RM_ADCVOLL: u32 = RM(FM_ADCVOLL, FB_ADCVOLL);

/* Register Values */
pub const RV_ADCVOLL_P24DB: u32 = RV(FV_ADCVOLL_P24DB, FB_ADCVOLL);
pub const RV_ADCVOLL_N71PT25DB: u32 = RV(FV_ADCVOLL_N71PT25DB, FB_ADCVOLL);

pub const RV_ADCVOLL_MUTE: u32 = RV(FV_ADCVOLL_MUTE, FB_ADCVOLL);

 *      R_ADCVOLR (0x7)      *
 *****************************/

/* Field Offsets */
pub const FB_ADCVOLR: u32 = 0;

/* Field Masks */
pub const FM_ADCVOLR: u32 = 0xFF;

/* Field Values */
pub const FV_ADCVOLR_P24DB: u32 = 0xFF;
pub const FV_ADCVOLR_N71PT25DB: u32 = 0x1;
pub const FV_ADCVOLR_MUTE: u32 = 0x0;

/* Register Masks */
pub const RM_ADCVOLR: u32 = RM(FM_ADCVOLR, FB_ADCVOLR);

/* Register Values */
pub const RV_ADCVOLR_P24DB: u32 = RV(FV_ADCVOLR_P24DB, FB_ADCVOLR);
pub const RV_ADCVOLR_N71PT25DB: u32 = RV(FV_ADCVOLR_N71PT25DB, FB_ADCVOLR);

pub const RV_ADCVOLR_MUTE: u32 = RV(FV_ADCVOLR_MUTE, FB_ADCVOLR);

 *      R_INVOLL (0x8)      *
 ****************************/

/* Field Offsets */
pub const FB_INVOLL_INMUTEL: u32 = 7;
pub const FB_INVOLL_IZCL: u32 = 6;
pub const FB_INVOLL: u32 = 0;

/* Field Masks */
pub const FM_INVOLL_INMUTEL: u32 = 0x1;
pub const FM_INVOLL_IZCL: u32 = 0x1;
pub const FM_INVOLL: u32 = 0x3F;

/* Field Values */
pub const FV_INVOLL_INMUTEL_ENABLE: u32 = 0x1;
pub const FV_INVOLL_INMUTEL_DISABLE: u32 = 0x0;
pub const FV_INVOLL_IZCL_ENABLE: u32 = 0x1;
pub const FV_INVOLL_IZCL_DISABLE: u32 = 0x0;
pub const FV_INVOLL_P30DB: u32 = 0x3F;
pub const FV_INVOLL_N17PT25DB: u32 = 0x0;

/* Register Masks */
pub const RM_INVOLL_INMUTEL: u32 = RM(FM_INVOLL_INMUTEL, FB_INVOLL_INMUTEL);

pub const RM_INVOLL_IZCL: u32 = RM(FM_INVOLL_IZCL, FB_INVOLL_IZCL);
pub const RM_INVOLL: u32 = RM(FM_INVOLL, FB_INVOLL);

/* Register Values */
pub const RV_INVOLL_INMUTEL_ENABLE: u32 = RV(FV_INVOLL_INMUTEL_ENABLE, FB_INVOLL_INMUTEL);

pub const RV_INVOLL_INMUTEL_DISABLE: u32 = RV(FV_INVOLL_INMUTEL_DISABLE, FB_INVOLL_INMUTEL);

pub const RV_INVOLL_IZCL_ENABLE: u32 = RV(FV_INVOLL_IZCL_ENABLE, FB_INVOLL_IZCL);

pub const RV_INVOLL_IZCL_DISABLE: u32 = RV(FV_INVOLL_IZCL_DISABLE, FB_INVOLL_IZCL);

pub const RV_INVOLL_P30DB: u32 = RV(FV_INVOLL_P30DB, FB_INVOLL);
pub const RV_INVOLL_N17PT25DB: u32 = RV(FV_INVOLL_N17PT25DB, FB_INVOLL);

 *      R_INVOLR (0x9)      *
 ****************************/

/* Field Offsets */
pub const FB_INVOLR_INMUTER: u32 = 7;
pub const FB_INVOLR_IZCR: u32 = 6;
pub const FB_INVOLR: u32 = 0;

/* Field Masks */
pub const FM_INVOLR_INMUTER: u32 = 0x1;
pub const FM_INVOLR_IZCR: u32 = 0x1;
pub const FM_INVOLR: u32 = 0x3F;

/* Field Values */
pub const FV_INVOLR_INMUTER_ENABLE: u32 = 0x1;
pub const FV_INVOLR_INMUTER_DISABLE: u32 = 0x0;
pub const FV_INVOLR_IZCR_ENABLE: u32 = 0x1;
pub const FV_INVOLR_IZCR_DISABLE: u32 = 0x0;
pub const FV_INVOLR_P30DB: u32 = 0x3F;
pub const FV_INVOLR_N17PT25DB: u32 = 0x0;

/* Register Masks */
pub const RM_INVOLR_INMUTER: u32 = RM(FM_INVOLR_INMUTER, FB_INVOLR_INMUTER);

pub const RM_INVOLR_IZCR: u32 = RM(FM_INVOLR_IZCR, FB_INVOLR_IZCR);
pub const RM_INVOLR: u32 = RM(FM_INVOLR, FB_INVOLR);

/* Register Values */
pub const RV_INVOLR_INMUTER_ENABLE: u32 = RV(FV_INVOLR_INMUTER_ENABLE, FB_INVOLR_INMUTER);

pub const RV_INVOLR_INMUTER_DISABLE: u32 = RV(FV_INVOLR_INMUTER_DISABLE, FB_INVOLR_INMUTER);

pub const RV_INVOLR_IZCR_ENABLE: u32 = RV(FV_INVOLR_IZCR_ENABLE, FB_INVOLR_IZCR);

pub const RV_INVOLR_IZCR_DISABLE: u32 = RV(FV_INVOLR_IZCR_DISABLE, FB_INVOLR_IZCR);

pub const RV_INVOLR_P30DB: u32 = RV(FV_INVOLR_P30DB, FB_INVOLR);
pub const RV_INVOLR_N17PT25DB: u32 = RV(FV_INVOLR_N17PT25DB, FB_INVOLR);

 *      R_INMODE (0x0B)      *
 *****************************/

/* Field Offsets */
pub const FB_INMODE_DS: u32 = 0;

/* Field Masks */
pub const FM_INMODE_DS: u32 = 0x1;

/* Field Values */
pub const FV_INMODE_DS_LRIN1: u32 = 0x0;
pub const FV_INMODE_DS_LRIN2: u32 = 0x1;

/* Register Masks */
pub const RM_INMODE_DS: u32 = RM(FM_INMODE_DS, FB_INMODE_DS);

/* Register Values */
pub const RV_INMODE_DS_LRIN1: u32 = RV(FV_INMODE_DS_LRIN1, FB_INMODE_DS);

pub const RV_INMODE_DS_LRIN2: u32 = RV(FV_INMODE_DS_LRIN2, FB_INMODE_DS);


 *      R_INSELL (0x0C)      *
 *****************************/

/* Field Offsets */
pub const FB_INSELL: u32 = 6;
pub const FB_INSELL_MICBSTL: u32 = 4;

/* Field Masks */
pub const FM_INSELL: u32 = 0x3;
pub const FM_INSELL_MICBSTL: u32 = 0x3;

/* Field Values */
pub const FV_INSELL_IN1: u32 = 0x0;
pub const FV_INSELL_IN2: u32 = 0x1;
pub const FV_INSELL_IN3: u32 = 0x2;
pub const FV_INSELL_D2S: u32 = 0x3;
pub const FV_INSELL_MICBSTL_OFF: u32 = 0x0;
pub const FV_INSELL_MICBSTL_10DB: u32 = 0x1;
pub const FV_INSELL_MICBSTL_20DB: u32 = 0x2;
pub const FV_INSELL_MICBSTL_30DB: u32 = 0x3;

/* Register Masks */
pub const RM_INSELL: u32 = RM(FM_INSELL, FB_INSELL);
pub const RM_INSELL_MICBSTL: u32 = RM(FM_INSELL_MICBSTL, FB_INSELL_MICBSTL);


/* Register Values */
pub const RV_INSELL_IN1: u32 = RV(FV_INSELL_IN1, FB_INSELL);
pub const RV_INSELL_IN2: u32 = RV(FV_INSELL_IN2, FB_INSELL);
pub const RV_INSELL_IN3: u32 = RV(FV_INSELL_IN3, FB_INSELL);
pub const RV_INSELL_D2S: u32 = RV(FV_INSELL_D2S, FB_INSELL);
pub const RV_INSELL_MICBSTL_OFF: u32 = RV(FV_INSELL_MICBSTL_OFF, FB_INSELL_MICBSTL);

pub const RV_INSELL_MICBSTL_10DB: u32 = RV(FV_INSELL_MICBSTL_10DB, FB_INSELL_MICBSTL);

pub const RV_INSELL_MICBSTL_20DB: u32 = RV(FV_INSELL_MICBSTL_20DB, FB_INSELL_MICBSTL);

pub const RV_INSELL_MICBSTL_30DB: u32 = RV(FV_INSELL_MICBSTL_30DB, FB_INSELL_MICBSTL);


 *      R_INSELR (0x0D)      *
 *****************************/

/* Field Offsets */
pub const FB_INSELR: u32 = 6;
pub const FB_INSELR_MICBSTR: u32 = 4;

/* Field Masks */
pub const FM_INSELR: u32 = 0x3;
pub const FM_INSELR_MICBSTR: u32 = 0x3;

/* Field Values */
pub const FV_INSELR_IN1: u32 = 0x0;
pub const FV_INSELR_IN2: u32 = 0x1;
pub const FV_INSELR_IN3: u32 = 0x2;
pub const FV_INSELR_D2S: u32 = 0x3;
pub const FV_INSELR_MICBSTR_OFF: u32 = 0x0;
pub const FV_INSELR_MICBSTR_10DB: u32 = 0x1;
pub const FV_INSELR_MICBSTR_20DB: u32 = 0x2;
pub const FV_INSELR_MICBSTR_30DB: u32 = 0x3;

/* Register Masks */
pub const RM_INSELR: u32 = RM(FM_INSELR, FB_INSELR);
pub const RM_INSELR_MICBSTR: u32 = RM(FM_INSELR_MICBSTR, FB_INSELR_MICBSTR);


/* Register Values */
pub const RV_INSELR_IN1: u32 = RV(FV_INSELR_IN1, FB_INSELR);
pub const RV_INSELR_IN2: u32 = RV(FV_INSELR_IN2, FB_INSELR);
pub const RV_INSELR_IN3: u32 = RV(FV_INSELR_IN3, FB_INSELR);
pub const RV_INSELR_D2S: u32 = RV(FV_INSELR_D2S, FB_INSELR);
pub const RV_INSELR_MICBSTR_OFF: u32 = RV(FV_INSELR_MICBSTR_OFF, FB_INSELR_MICBSTR);

pub const RV_INSELR_MICBSTR_10DB: u32 = RV(FV_INSELR_MICBSTR_10DB, FB_INSELR_MICBSTR);

pub const RV_INSELR_MICBSTR_20DB: u32 = RV(FV_INSELR_MICBSTR_20DB, FB_INSELR_MICBSTR);

pub const RV_INSELR_MICBSTR_30DB: u32 = RV(FV_INSELR_MICBSTR_30DB, FB_INSELR_MICBSTR);


 *      R_AIC1 (0x13)      *
 ***************************/

/* Field Offsets */
pub const FB_AIC1_BCLKINV: u32 = 6;
pub const FB_AIC1_MS: u32 = 5;
pub const FB_AIC1_LRP: u32 = 4;
pub const FB_AIC1_WL: u32 = 2;
pub const FB_AIC1_FORMAT: u32 = 0;

/* Field Masks */
pub const FM_AIC1_BCLKINV: u32 = 0x1;
pub const FM_AIC1_MS: u32 = 0x1;
pub const FM_AIC1_LRP: u32 = 0x1;
pub const FM_AIC1_WL: u32 = 0x3;
pub const FM_AIC1_FORMAT: u32 = 0x3;

/* Field Values */
pub const FV_AIC1_BCLKINV_ENABLE: u32 = 0x1;
pub const FV_AIC1_BCLKINV_DISABLE: u32 = 0x0;
pub const FV_AIC1_MS_MASTER: u32 = 0x1;
pub const FV_AIC1_MS_SLAVE: u32 = 0x0;
pub const FV_AIC1_LRP_INVERT: u32 = 0x1;
pub const FV_AIC1_LRP_NORMAL: u32 = 0x0;
pub const FV_AIC1_WL_16: u32 = 0x0;
pub const FV_AIC1_WL_20: u32 = 0x1;
pub const FV_AIC1_WL_24: u32 = 0x2;
pub const FV_AIC1_WL_32: u32 = 0x3;
pub const FV_AIC1_FORMAT_RIGHT: u32 = 0x0;
pub const FV_AIC1_FORMAT_LEFT: u32 = 0x1;
pub const FV_AIC1_FORMAT_I2S: u32 = 0x2;

/* Register Masks */
pub const RM_AIC1_BCLKINV: u32 = RM(FM_AIC1_BCLKINV, FB_AIC1_BCLKINV);

pub const RM_AIC1_MS: u32 = RM(FM_AIC1_MS, FB_AIC1_MS);
pub const RM_AIC1_LRP: u32 = RM(FM_AIC1_LRP, FB_AIC1_LRP);
pub const RM_AIC1_WL: u32 = RM(FM_AIC1_WL, FB_AIC1_WL);
pub const RM_AIC1_FORMAT: u32 = RM(FM_AIC1_FORMAT, FB_AIC1_FORMAT);

/* Register Values */
pub const RV_AIC1_BCLKINV_ENABLE: u32 = RV(FV_AIC1_BCLKINV_ENABLE, FB_AIC1_BCLKINV);

pub const RV_AIC1_BCLKINV_DISABLE: u32 = RV(FV_AIC1_BCLKINV_DISABLE, FB_AIC1_BCLKINV);

pub const RV_AIC1_MS_MASTER: u32 = RV(FV_AIC1_MS_MASTER, FB_AIC1_MS);
pub const RV_AIC1_MS_SLAVE: u32 = RV(FV_AIC1_MS_SLAVE, FB_AIC1_MS);
pub const RV_AIC1_LRP_INVERT: u32 = RV(FV_AIC1_LRP_INVERT, FB_AIC1_LRP);

pub const RV_AIC1_LRP_NORMAL: u32 = RV(FV_AIC1_LRP_NORMAL, FB_AIC1_LRP);

pub const RV_AIC1_WL_16: u32 = RV(FV_AIC1_WL_16, FB_AIC1_WL);
pub const RV_AIC1_WL_20: u32 = RV(FV_AIC1_WL_20, FB_AIC1_WL);
pub const RV_AIC1_WL_24: u32 = RV(FV_AIC1_WL_24, FB_AIC1_WL);
pub const RV_AIC1_WL_32: u32 = RV(FV_AIC1_WL_32, FB_AIC1_WL);
pub const RV_AIC1_FORMAT_RIGHT: u32 = RV(FV_AIC1_FORMAT_RIGHT, FB_AIC1_FORMAT);

pub const RV_AIC1_FORMAT_LEFT: u32 = RV(FV_AIC1_FORMAT_LEFT, FB_AIC1_FORMAT);

pub const RV_AIC1_FORMAT_I2S: u32 = RV(FV_AIC1_FORMAT_I2S, FB_AIC1_FORMAT);


 *      R_AIC2 (0x14)      *
 ***************************/

/* Field Offsets */
pub const FB_AIC2_DACDSEL: u32 = 6;
pub const FB_AIC2_ADCDSEL: u32 = 4;
pub const FB_AIC2_TRI: u32 = 3;
pub const FB_AIC2_BLRCM: u32 = 0;

/* Field Masks */
pub const FM_AIC2_DACDSEL: u32 = 0x3;
pub const FM_AIC2_ADCDSEL: u32 = 0x3;
pub const FM_AIC2_TRI: u32 = 0x1;
pub const FM_AIC2_BLRCM: u32 = 0x7;

/* Field Values */
pub const FV_AIC2_BLRCM_DAC_BCLK_LRCLK_SHARED: u32 = 0x3;

/* Register Masks */
pub const RM_AIC2_DACDSEL: u32 = RM(FM_AIC2_DACDSEL, FB_AIC2_DACDSEL);

pub const RM_AIC2_ADCDSEL: u32 = RM(FM_AIC2_ADCDSEL, FB_AIC2_ADCDSEL);

pub const RM_AIC2_TRI: u32 = RM(FM_AIC2_TRI, FB_AIC2_TRI);
pub const RM_AIC2_BLRCM: u32 = RM(FM_AIC2_BLRCM, FB_AIC2_BLRCM);

/* Register Values */
pub const RV_AIC2_BLRCM_DAC_BCLK_LRCLK_SHARED: u32 = RV(FV_AIC2_BLRCM_DAC_BCLK_LRCLK_SHARED, FB_AIC2_BLRCM);


 *      R_CNVRTR0 (0x16)      *
 ******************************/

/* Field Offsets */
pub const FB_CNVRTR0_ADCPOLR: u32 = 7;
pub const FB_CNVRTR0_ADCPOLL: u32 = 6;
pub const FB_CNVRTR0_AMONOMIX: u32 = 4;
pub const FB_CNVRTR0_ADCMU: u32 = 3;
pub const FB_CNVRTR0_HPOR: u32 = 2;
pub const FB_CNVRTR0_ADCHPDR: u32 = 1;
pub const FB_CNVRTR0_ADCHPDL: u32 = 0;

/* Field Masks */
pub const FM_CNVRTR0_ADCPOLR: u32 = 0x1;
pub const FM_CNVRTR0_ADCPOLL: u32 = 0x1;
pub const FM_CNVRTR0_AMONOMIX: u32 = 0x3;
pub const FM_CNVRTR0_ADCMU: u32 = 0x1;
pub const FM_CNVRTR0_HPOR: u32 = 0x1;
pub const FM_CNVRTR0_ADCHPDR: u32 = 0x1;
pub const FM_CNVRTR0_ADCHPDL: u32 = 0x1;

/* Field Values */
pub const FV_CNVRTR0_ADCPOLR_INVERT: u32 = 0x1;
pub const FV_CNVRTR0_ADCPOLR_NORMAL: u32 = 0x0;
pub const FV_CNVRTR0_ADCPOLL_INVERT: u32 = 0x1;
pub const FV_CNVRTR0_ADCPOLL_NORMAL: u32 = 0x0;
pub const FV_CNVRTR0_ADCMU_ENABLE: u32 = 0x1;
pub const FV_CNVRTR0_ADCMU_DISABLE: u32 = 0x0;
pub const FV_CNVRTR0_ADCHPDR_ENABLE: u32 = 0x1;
pub const FV_CNVRTR0_ADCHPDR_DISABLE: u32 = 0x0;
pub const FV_CNVRTR0_ADCHPDL_ENABLE: u32 = 0x1;
pub const FV_CNVRTR0_ADCHPDL_DISABLE: u32 = 0x0;

/* Register Masks */
pub const RM_CNVRTR0_ADCPOLR: u32 = RM(FM_CNVRTR0_ADCPOLR, FB_CNVRTR0_ADCPOLR);

pub const RM_CNVRTR0_ADCPOLL: u32 = RM(FM_CNVRTR0_ADCPOLL, FB_CNVRTR0_ADCPOLL);

pub const RM_CNVRTR0_AMONOMIX: u32 = RM(FM_CNVRTR0_AMONOMIX, FB_CNVRTR0_AMONOMIX);

pub const RM_CNVRTR0_ADCMU: u32 = RM(FM_CNVRTR0_ADCMU, FB_CNVRTR0_ADCMU);

pub const RM_CNVRTR0_HPOR: u32 = RM(FM_CNVRTR0_HPOR, FB_CNVRTR0_HPOR);

pub const RM_CNVRTR0_ADCHPDR: u32 = RM(FM_CNVRTR0_ADCHPDR, FB_CNVRTR0_ADCHPDR);

pub const RM_CNVRTR0_ADCHPDL: u32 = RM(FM_CNVRTR0_ADCHPDL, FB_CNVRTR0_ADCHPDL);


/* Register Values */
pub const RV_CNVRTR0_ADCPOLR_INVERT: u32 = RV(FV_CNVRTR0_ADCPOLR_INVERT, FB_CNVRTR0_ADCPOLR);

pub const RV_CNVRTR0_ADCPOLR_NORMAL: u32 = RV(FV_CNVRTR0_ADCPOLR_NORMAL, FB_CNVRTR0_ADCPOLR);

pub const RV_CNVRTR0_ADCPOLL_INVERT: u32 = RV(FV_CNVRTR0_ADCPOLL_INVERT, FB_CNVRTR0_ADCPOLL);

pub const RV_CNVRTR0_ADCPOLL_NORMAL: u32 = RV(FV_CNVRTR0_ADCPOLL_NORMAL, FB_CNVRTR0_ADCPOLL);

pub const RV_CNVRTR0_ADCMU_ENABLE: u32 = RV(FV_CNVRTR0_ADCMU_ENABLE, FB_CNVRTR0_ADCMU);

pub const RV_CNVRTR0_ADCMU_DISABLE: u32 = RV(FV_CNVRTR0_ADCMU_DISABLE, FB_CNVRTR0_ADCMU);

pub const RV_CNVRTR0_ADCHPDR_ENABLE: u32 = RV(FV_CNVRTR0_ADCHPDR_ENABLE, FB_CNVRTR0_ADCHPDR);

pub const RV_CNVRTR0_ADCHPDR_DISABLE: u32 = RV(FV_CNVRTR0_ADCHPDR_DISABLE, FB_CNVRTR0_ADCHPDR);

pub const RV_CNVRTR0_ADCHPDL_ENABLE: u32 = RV(FV_CNVRTR0_ADCHPDL_ENABLE, FB_CNVRTR0_ADCHPDL);

pub const RV_CNVRTR0_ADCHPDL_DISABLE: u32 = RV(FV_CNVRTR0_ADCHPDL_DISABLE, FB_CNVRTR0_ADCHPDL);


 *      R_ADCSR (0x17)      *
 ****************************/

/* Field Offsets */
pub const FB_ADCSR_ABCM: u32 = 6;
pub const FB_ADCSR_ABR: u32 = 3;
pub const FB_ADCSR_ABM: u32 = 0;

/* Field Masks */
pub const FM_ADCSR_ABCM: u32 = 0x3;
pub const FM_ADCSR_ABR: u32 = 0x3;
pub const FM_ADCSR_ABM: u32 = 0x7;

/* Field Values */
pub const FV_ADCSR_ABCM_AUTO: u32 = 0x0;
pub const FV_ADCSR_ABCM_32: u32 = 0x1;
pub const FV_ADCSR_ABCM_40: u32 = 0x2;
pub const FV_ADCSR_ABCM_64: u32 = 0x3;
pub const FV_ADCSR_ABR_32: u32 = 0x0;
pub const FV_ADCSR_ABR_44_1: u32 = 0x1;
pub const FV_ADCSR_ABR_48: u32 = 0x2;
pub const FV_ADCSR_ABM_PT25: u32 = 0x0;
pub const FV_ADCSR_ABM_PT5: u32 = 0x1;
pub const FV_ADCSR_ABM_1: u32 = 0x2;
pub const FV_ADCSR_ABM_2: u32 = 0x3;

/* Register Masks */
pub const RM_ADCSR_ABCM: u32 = RM(FM_ADCSR_ABCM, FB_ADCSR_ABCM);
pub const RM_ADCSR_ABR: u32 = RM(FM_ADCSR_ABR, FB_ADCSR_ABR);
pub const RM_ADCSR_ABM: u32 = RM(FM_ADCSR_ABM, FB_ADCSR_ABM);

/* Register Values */
pub const RV_ADCSR_ABCM_AUTO: u32 = RV(FV_ADCSR_ABCM_AUTO, FB_ADCSR_ABCM);

pub const RV_ADCSR_ABCM_32: u32 = RV(FV_ADCSR_ABCM_32, FB_ADCSR_ABCM);

pub const RV_ADCSR_ABCM_40: u32 = RV(FV_ADCSR_ABCM_40, FB_ADCSR_ABCM);

pub const RV_ADCSR_ABCM_64: u32 = RV(FV_ADCSR_ABCM_64, FB_ADCSR_ABCM);

// TODO: The C macro `RV_ADCSR_ABR_32` references undefined token `FV_ADCSR_ABR_` in this header: `RV(FV_ADCSR_ABR_32, FB_ADCSR_ABR)`.
// TODO: The C macro `RV_ADCSR_ABR_44_1` references undefined token `FV_ADCSR_ABR_` in this header: `RV(FV_ADCSR_ABR_44_1, FB_ADCSR_ABR)`.

// TODO: The C macro `RV_ADCSR_ABR_48` references undefined token `FV_ADCSR_ABR_` in this header: `RV(FV_ADCSR_ABR_48, FB_ADCSR_ABR)`.
// TODO: The C macro `RV_ADCSR_ABR_` references undefined token `FV_ADCSR_ABR_` in this header: `RV(FV_ADCSR_ABR_, FB_ADCSR_ABR)`.
pub const RV_ADCSR_ABM_PT25: u32 = RV(FV_ADCSR_ABM_PT25, FB_ADCSR_ABM);

pub const RV_ADCSR_ABM_PT5: u32 = RV(FV_ADCSR_ABM_PT5, FB_ADCSR_ABM);
pub const RV_ADCSR_ABM_1: u32 = RV(FV_ADCSR_ABM_1, FB_ADCSR_ABM);
pub const RV_ADCSR_ABM_2: u32 = RV(FV_ADCSR_ABM_2, FB_ADCSR_ABM);

 *      R_CNVRTR1 (0x18)      *
 ******************************/

/* Field Offsets */
pub const FB_CNVRTR1_DACPOLR: u32 = 7;
pub const FB_CNVRTR1_DACPOLL: u32 = 6;
pub const FB_CNVRTR1_DMONOMIX: u32 = 4;
pub const FB_CNVRTR1_DACMU: u32 = 3;
pub const FB_CNVRTR1_DEEMPH: u32 = 2;
pub const FB_CNVRTR1_DACDITH: u32 = 0;

/* Field Masks */
pub const FM_CNVRTR1_DACPOLR: u32 = 0x1;
pub const FM_CNVRTR1_DACPOLL: u32 = 0x1;
pub const FM_CNVRTR1_DMONOMIX: u32 = 0x3;
pub const FM_CNVRTR1_DACMU: u32 = 0x1;
pub const FM_CNVRTR1_DEEMPH: u32 = 0x1;
pub const FM_CNVRTR1_DACDITH: u32 = 0x3;

/* Field Values */
pub const FV_CNVRTR1_DACPOLR_INVERT: u32 = 0x1;
pub const FV_CNVRTR1_DACPOLR_NORMAL: u32 = 0x0;
pub const FV_CNVRTR1_DACPOLL_INVERT: u32 = 0x1;
pub const FV_CNVRTR1_DACPOLL_NORMAL: u32 = 0x0;
pub const FV_CNVRTR1_DMONOMIX_ENABLE: u32 = 0x1;
pub const FV_CNVRTR1_DMONOMIX_DISABLE: u32 = 0x0;
pub const FV_CNVRTR1_DACMU_ENABLE: u32 = 0x1;
pub const FV_CNVRTR1_DACMU_DISABLE: u32 = 0x0;

/* Register Masks */
pub const RM_CNVRTR1_DACPOLR: u32 = RM(FM_CNVRTR1_DACPOLR, FB_CNVRTR1_DACPOLR);

pub const RM_CNVRTR1_DACPOLL: u32 = RM(FM_CNVRTR1_DACPOLL, FB_CNVRTR1_DACPOLL);

pub const RM_CNVRTR1_DMONOMIX: u32 = RM(FM_CNVRTR1_DMONOMIX, FB_CNVRTR1_DMONOMIX);

pub const RM_CNVRTR1_DACMU: u32 = RM(FM_CNVRTR1_DACMU, FB_CNVRTR1_DACMU);

pub const RM_CNVRTR1_DEEMPH: u32 = RM(FM_CNVRTR1_DEEMPH, FB_CNVRTR1_DEEMPH);

pub const RM_CNVRTR1_DACDITH: u32 = RM(FM_CNVRTR1_DACDITH, FB_CNVRTR1_DACDITH);


/* Register Values */
pub const RV_CNVRTR1_DACPOLR_INVERT: u32 = RV(FV_CNVRTR1_DACPOLR_INVERT, FB_CNVRTR1_DACPOLR);

pub const RV_CNVRTR1_DACPOLR_NORMAL: u32 = RV(FV_CNVRTR1_DACPOLR_NORMAL, FB_CNVRTR1_DACPOLR);

pub const RV_CNVRTR1_DACPOLL_INVERT: u32 = RV(FV_CNVRTR1_DACPOLL_INVERT, FB_CNVRTR1_DACPOLL);

pub const RV_CNVRTR1_DACPOLL_NORMAL: u32 = RV(FV_CNVRTR1_DACPOLL_NORMAL, FB_CNVRTR1_DACPOLL);

pub const RV_CNVRTR1_DMONOMIX_ENABLE: u32 = RV(FV_CNVRTR1_DMONOMIX_ENABLE, FB_CNVRTR1_DMONOMIX);

pub const RV_CNVRTR1_DMONOMIX_DISABLE: u32 = RV(FV_CNVRTR1_DMONOMIX_DISABLE, FB_CNVRTR1_DMONOMIX);

pub const RV_CNVRTR1_DACMU_ENABLE: u32 = RV(FV_CNVRTR1_DACMU_ENABLE, FB_CNVRTR1_DACMU);

pub const RV_CNVRTR1_DACMU_DISABLE: u32 = RV(FV_CNVRTR1_DACMU_DISABLE, FB_CNVRTR1_DACMU);


 *      R_DACSR (0x19)      *
 ****************************/

/* Field Offsets */
pub const FB_DACSR_DBCM: u32 = 6;
pub const FB_DACSR_DBR: u32 = 3;
pub const FB_DACSR_DBM: u32 = 0;

/* Field Masks */
pub const FM_DACSR_DBCM: u32 = 0x3;
pub const FM_DACSR_DBR: u32 = 0x3;
pub const FM_DACSR_DBM: u32 = 0x7;

/* Field Values */
pub const FV_DACSR_DBCM_AUTO: u32 = 0x0;
pub const FV_DACSR_DBCM_32: u32 = 0x1;
pub const FV_DACSR_DBCM_40: u32 = 0x2;
pub const FV_DACSR_DBCM_64: u32 = 0x3;
pub const FV_DACSR_DBR_32: u32 = 0x0;
pub const FV_DACSR_DBR_44_1: u32 = 0x1;
pub const FV_DACSR_DBR_48: u32 = 0x2;
pub const FV_DACSR_DBM_PT25: u32 = 0x0;
pub const FV_DACSR_DBM_PT5: u32 = 0x1;
pub const FV_DACSR_DBM_1: u32 = 0x2;
pub const FV_DACSR_DBM_2: u32 = 0x3;

/* Register Masks */
pub const RM_DACSR_DBCM: u32 = RM(FM_DACSR_DBCM, FB_DACSR_DBCM);
pub const RM_DACSR_DBR: u32 = RM(FM_DACSR_DBR, FB_DACSR_DBR);
pub const RM_DACSR_DBM: u32 = RM(FM_DACSR_DBM, FB_DACSR_DBM);

/* Register Values */
pub const RV_DACSR_DBCM_AUTO: u32 = RV(FV_DACSR_DBCM_AUTO, FB_DACSR_DBCM);

pub const RV_DACSR_DBCM_32: u32 = RV(FV_DACSR_DBCM_32, FB_DACSR_DBCM);

pub const RV_DACSR_DBCM_40: u32 = RV(FV_DACSR_DBCM_40, FB_DACSR_DBCM);

pub const RV_DACSR_DBCM_64: u32 = RV(FV_DACSR_DBCM_64, FB_DACSR_DBCM);

pub const RV_DACSR_DBR_32: u32 = RV(FV_DACSR_DBR_32, FB_DACSR_DBR);
pub const RV_DACSR_DBR_44_1: u32 = RV(FV_DACSR_DBR_44_1, FB_DACSR_DBR);

pub const RV_DACSR_DBR_48: u32 = RV(FV_DACSR_DBR_48, FB_DACSR_DBR);
pub const RV_DACSR_DBM_PT25: u32 = RV(FV_DACSR_DBM_PT25, FB_DACSR_DBM);

pub const RV_DACSR_DBM_PT5: u32 = RV(FV_DACSR_DBM_PT5, FB_DACSR_DBM);
pub const RV_DACSR_DBM_1: u32 = RV(FV_DACSR_DBM_1, FB_DACSR_DBM);
pub const RV_DACSR_DBM_2: u32 = RV(FV_DACSR_DBM_2, FB_DACSR_DBM);

 *      R_PWRM1 (0x1A)      *
 ****************************/

/* Field Offsets */
pub const FB_PWRM1_BSTL: u32 = 7;
pub const FB_PWRM1_BSTR: u32 = 6;
pub const FB_PWRM1_PGAL: u32 = 5;
pub const FB_PWRM1_PGAR: u32 = 4;
pub const FB_PWRM1_ADCL: u32 = 3;
pub const FB_PWRM1_ADCR: u32 = 2;
pub const FB_PWRM1_MICB: u32 = 1;
pub const FB_PWRM1_DIGENB: u32 = 0;

/* Field Masks */
pub const FM_PWRM1_BSTL: u32 = 0x1;
pub const FM_PWRM1_BSTR: u32 = 0x1;
pub const FM_PWRM1_PGAL: u32 = 0x1;
pub const FM_PWRM1_PGAR: u32 = 0x1;
pub const FM_PWRM1_ADCL: u32 = 0x1;
pub const FM_PWRM1_ADCR: u32 = 0x1;
pub const FM_PWRM1_MICB: u32 = 0x1;
pub const FM_PWRM1_DIGENB: u32 = 0x1;

/* Field Values */
pub const FV_PWRM1_BSTL_ENABLE: u32 = 0x1;
pub const FV_PWRM1_BSTL_DISABLE: u32 = 0x0;
pub const FV_PWRM1_BSTR_ENABLE: u32 = 0x1;
pub const FV_PWRM1_BSTR_DISABLE: u32 = 0x0;
pub const FV_PWRM1_PGAL_ENABLE: u32 = 0x1;
pub const FV_PWRM1_PGAL_DISABLE: u32 = 0x0;
pub const FV_PWRM1_PGAR_ENABLE: u32 = 0x1;
pub const FV_PWRM1_PGAR_DISABLE: u32 = 0x0;
pub const FV_PWRM1_ADCL_ENABLE: u32 = 0x1;
pub const FV_PWRM1_ADCL_DISABLE: u32 = 0x0;
pub const FV_PWRM1_ADCR_ENABLE: u32 = 0x1;
pub const FV_PWRM1_ADCR_DISABLE: u32 = 0x0;
pub const FV_PWRM1_MICB_ENABLE: u32 = 0x1;
pub const FV_PWRM1_MICB_DISABLE: u32 = 0x0;
pub const FV_PWRM1_DIGENB_DISABLE: u32 = 0x1;
pub const FV_PWRM1_DIGENB_ENABLE: u32 = 0x0;

/* Register Masks */
pub const RM_PWRM1_BSTL: u32 = RM(FM_PWRM1_BSTL, FB_PWRM1_BSTL);
pub const RM_PWRM1_BSTR: u32 = RM(FM_PWRM1_BSTR, FB_PWRM1_BSTR);
pub const RM_PWRM1_PGAL: u32 = RM(FM_PWRM1_PGAL, FB_PWRM1_PGAL);
pub const RM_PWRM1_PGAR: u32 = RM(FM_PWRM1_PGAR, FB_PWRM1_PGAR);
pub const RM_PWRM1_ADCL: u32 = RM(FM_PWRM1_ADCL, FB_PWRM1_ADCL);
pub const RM_PWRM1_ADCR: u32 = RM(FM_PWRM1_ADCR, FB_PWRM1_ADCR);
pub const RM_PWRM1_MICB: u32 = RM(FM_PWRM1_MICB, FB_PWRM1_MICB);
pub const RM_PWRM1_DIGENB: u32 = RM(FM_PWRM1_DIGENB, FB_PWRM1_DIGENB);


/* Register Values */
pub const RV_PWRM1_BSTL_ENABLE: u32 = RV(FV_PWRM1_BSTL_ENABLE, FB_PWRM1_BSTL);

pub const RV_PWRM1_BSTL_DISABLE: u32 = RV(FV_PWRM1_BSTL_DISABLE, FB_PWRM1_BSTL);

pub const RV_PWRM1_BSTR_ENABLE: u32 = RV(FV_PWRM1_BSTR_ENABLE, FB_PWRM1_BSTR);

pub const RV_PWRM1_BSTR_DISABLE: u32 = RV(FV_PWRM1_BSTR_DISABLE, FB_PWRM1_BSTR);

pub const RV_PWRM1_PGAL_ENABLE: u32 = RV(FV_PWRM1_PGAL_ENABLE, FB_PWRM1_PGAL);

pub const RV_PWRM1_PGAL_DISABLE: u32 = RV(FV_PWRM1_PGAL_DISABLE, FB_PWRM1_PGAL);

pub const RV_PWRM1_PGAR_ENABLE: u32 = RV(FV_PWRM1_PGAR_ENABLE, FB_PWRM1_PGAR);

pub const RV_PWRM1_PGAR_DISABLE: u32 = RV(FV_PWRM1_PGAR_DISABLE, FB_PWRM1_PGAR);

pub const RV_PWRM1_ADCL_ENABLE: u32 = RV(FV_PWRM1_ADCL_ENABLE, FB_PWRM1_ADCL);

pub const RV_PWRM1_ADCL_DISABLE: u32 = RV(FV_PWRM1_ADCL_DISABLE, FB_PWRM1_ADCL);

pub const RV_PWRM1_ADCR_ENABLE: u32 = RV(FV_PWRM1_ADCR_ENABLE, FB_PWRM1_ADCR);

pub const RV_PWRM1_ADCR_DISABLE: u32 = RV(FV_PWRM1_ADCR_DISABLE, FB_PWRM1_ADCR);

pub const RV_PWRM1_MICB_ENABLE: u32 = RV(FV_PWRM1_MICB_ENABLE, FB_PWRM1_MICB);

pub const RV_PWRM1_MICB_DISABLE: u32 = RV(FV_PWRM1_MICB_DISABLE, FB_PWRM1_MICB);

pub const RV_PWRM1_DIGENB_DISABLE: u32 = RV(FV_PWRM1_DIGENB_DISABLE, FB_PWRM1_DIGENB);

pub const RV_PWRM1_DIGENB_ENABLE: u32 = RV(FV_PWRM1_DIGENB_ENABLE, FB_PWRM1_DIGENB);


 *      R_PWRM2 (0x1B)      *
 ****************************/

/* Field Offsets */
pub const FB_PWRM2_D2S: u32 = 7;
pub const FB_PWRM2_HPL: u32 = 6;
pub const FB_PWRM2_HPR: u32 = 5;
pub const FB_PWRM2_SPKL: u32 = 4;
pub const FB_PWRM2_SPKR: u32 = 3;
pub const FB_PWRM2_INSELL: u32 = 2;
pub const FB_PWRM2_INSELR: u32 = 1;
pub const FB_PWRM2_VREF: u32 = 0;

/* Field Masks */
pub const FM_PWRM2_D2S: u32 = 0x1;
pub const FM_PWRM2_HPL: u32 = 0x1;
pub const FM_PWRM2_HPR: u32 = 0x1;
pub const FM_PWRM2_SPKL: u32 = 0x1;
pub const FM_PWRM2_SPKR: u32 = 0x1;
pub const FM_PWRM2_INSELL: u32 = 0x1;
pub const FM_PWRM2_INSELR: u32 = 0x1;
pub const FM_PWRM2_VREF: u32 = 0x1;

/* Field Values */
pub const FV_PWRM2_D2S_ENABLE: u32 = 0x1;
pub const FV_PWRM2_D2S_DISABLE: u32 = 0x0;
pub const FV_PWRM2_HPL_ENABLE: u32 = 0x1;
pub const FV_PWRM2_HPL_DISABLE: u32 = 0x0;
pub const FV_PWRM2_HPR_ENABLE: u32 = 0x1;
pub const FV_PWRM2_HPR_DISABLE: u32 = 0x0;
pub const FV_PWRM2_SPKL_ENABLE: u32 = 0x1;
pub const FV_PWRM2_SPKL_DISABLE: u32 = 0x0;
pub const FV_PWRM2_SPKR_ENABLE: u32 = 0x1;
pub const FV_PWRM2_SPKR_DISABLE: u32 = 0x0;
pub const FV_PWRM2_INSELL_ENABLE: u32 = 0x1;
pub const FV_PWRM2_INSELL_DISABLE: u32 = 0x0;
pub const FV_PWRM2_INSELR_ENABLE: u32 = 0x1;
pub const FV_PWRM2_INSELR_DISABLE: u32 = 0x0;
pub const FV_PWRM2_VREF_ENABLE: u32 = 0x1;
pub const FV_PWRM2_VREF_DISABLE: u32 = 0x0;

/* Register Masks */
pub const RM_PWRM2_D2S: u32 = RM(FM_PWRM2_D2S, FB_PWRM2_D2S);
pub const RM_PWRM2_HPL: u32 = RM(FM_PWRM2_HPL, FB_PWRM2_HPL);
pub const RM_PWRM2_HPR: u32 = RM(FM_PWRM2_HPR, FB_PWRM2_HPR);
pub const RM_PWRM2_SPKL: u32 = RM(FM_PWRM2_SPKL, FB_PWRM2_SPKL);
pub const RM_PWRM2_SPKR: u32 = RM(FM_PWRM2_SPKR, FB_PWRM2_SPKR);
pub const RM_PWRM2_INSELL: u32 = RM(FM_PWRM2_INSELL, FB_PWRM2_INSELL);

pub const RM_PWRM2_INSELR: u32 = RM(FM_PWRM2_INSELR, FB_PWRM2_INSELR);

pub const RM_PWRM2_VREF: u32 = RM(FM_PWRM2_VREF, FB_PWRM2_VREF);

/* Register Values */
pub const RV_PWRM2_D2S_ENABLE: u32 = RV(FV_PWRM2_D2S_ENABLE, FB_PWRM2_D2S);

pub const RV_PWRM2_D2S_DISABLE: u32 = RV(FV_PWRM2_D2S_DISABLE, FB_PWRM2_D2S);

pub const RV_PWRM2_HPL_ENABLE: u32 = RV(FV_PWRM2_HPL_ENABLE, FB_PWRM2_HPL);

pub const RV_PWRM2_HPL_DISABLE: u32 = RV(FV_PWRM2_HPL_DISABLE, FB_PWRM2_HPL);

pub const RV_PWRM2_HPR_ENABLE: u32 = RV(FV_PWRM2_HPR_ENABLE, FB_PWRM2_HPR);

pub const RV_PWRM2_HPR_DISABLE: u32 = RV(FV_PWRM2_HPR_DISABLE, FB_PWRM2_HPR);

pub const RV_PWRM2_SPKL_ENABLE: u32 = RV(FV_PWRM2_SPKL_ENABLE, FB_PWRM2_SPKL);

pub const RV_PWRM2_SPKL_DISABLE: u32 = RV(FV_PWRM2_SPKL_DISABLE, FB_PWRM2_SPKL);

pub const RV_PWRM2_SPKR_ENABLE: u32 = RV(FV_PWRM2_SPKR_ENABLE, FB_PWRM2_SPKR);

pub const RV_PWRM2_SPKR_DISABLE: u32 = RV(FV_PWRM2_SPKR_DISABLE, FB_PWRM2_SPKR);

pub const RV_PWRM2_INSELL_ENABLE: u32 = RV(FV_PWRM2_INSELL_ENABLE, FB_PWRM2_INSELL);

pub const RV_PWRM2_INSELL_DISABLE: u32 = RV(FV_PWRM2_INSELL_DISABLE, FB_PWRM2_INSELL);

pub const RV_PWRM2_INSELR_ENABLE: u32 = RV(FV_PWRM2_INSELR_ENABLE, FB_PWRM2_INSELR);

pub const RV_PWRM2_INSELR_DISABLE: u32 = RV(FV_PWRM2_INSELR_DISABLE, FB_PWRM2_INSELR);

pub const RV_PWRM2_VREF_ENABLE: u32 = RV(FV_PWRM2_VREF_ENABLE, FB_PWRM2_VREF);

pub const RV_PWRM2_VREF_DISABLE: u32 = RV(FV_PWRM2_VREF_DISABLE, FB_PWRM2_VREF);

 *      R_CTL (0x1C)          *
 ******************************/

/* Fiel Offsets */
pub const FB_CTL_HPSWEN: u32 = 7;
pub const FB_CTL_HPSWPOL: u32 = 6;

 *      R_CONFIG0 (0x1F)      *
 ******************************/

/* Field Offsets */
pub const FB_CONFIG0_ASDM: u32 = 6;
pub const FB_CONFIG0_DSDM: u32 = 4;
pub const FB_CONFIG0_DC_BYPASS: u32 = 1;
pub const FB_CONFIG0_SD_FORCE_ON: u32 = 0;

/* Field Masks */
pub const FM_CONFIG0_ASDM: u32 = 0x3;
pub const FM_CONFIG0_DSDM: u32 = 0x3;
pub const FM_CONFIG0_DC_BYPASS: u32 = 0x1;
pub const FM_CONFIG0_SD_FORCE_ON: u32 = 0x1;

/* Field Values */
pub const FV_CONFIG0_ASDM_HALF: u32 = 0x1;
pub const FV_CONFIG0_ASDM_FULL: u32 = 0x2;
pub const FV_CONFIG0_ASDM_AUTO: u32 = 0x3;
pub const FV_CONFIG0_DSDM_HALF: u32 = 0x1;
pub const FV_CONFIG0_DSDM_FULL: u32 = 0x2;
pub const FV_CONFIG0_DSDM_AUTO: u32 = 0x3;
pub const FV_CONFIG0_DC_BYPASS_ENABLE: u32 = 0x1;
pub const FV_CONFIG0_DC_BYPASS_DISABLE: u32 = 0x0;
pub const FV_CONFIG0_SD_FORCE_ON_ENABLE: u32 = 0x1;
pub const FV_CONFIG0_SD_FORCE_ON_DISABLE: u32 = 0x0;

/* Register Masks */
pub const RM_CONFIG0_ASDM: u32 = RM(FM_CONFIG0_ASDM, FB_CONFIG0_ASDM);

pub const RM_CONFIG0_DSDM: u32 = RM(FM_CONFIG0_DSDM, FB_CONFIG0_DSDM);

pub const RM_CONFIG0_DC_BYPASS: u32 = RM(FM_CONFIG0_DC_BYPASS, FB_CONFIG0_DC_BYPASS);

pub const RM_CONFIG0_SD_FORCE_ON: u32 = RM(FM_CONFIG0_SD_FORCE_ON, FB_CONFIG0_SD_FORCE_ON);


/* Register Values */
pub const RV_CONFIG0_ASDM_HALF: u32 = RV(FV_CONFIG0_ASDM_HALF, FB_CONFIG0_ASDM);

pub const RV_CONFIG0_ASDM_FULL: u32 = RV(FV_CONFIG0_ASDM_FULL, FB_CONFIG0_ASDM);

pub const RV_CONFIG0_ASDM_AUTO: u32 = RV(FV_CONFIG0_ASDM_AUTO, FB_CONFIG0_ASDM);

pub const RV_CONFIG0_DSDM_HALF: u32 = RV(FV_CONFIG0_DSDM_HALF, FB_CONFIG0_DSDM);

pub const RV_CONFIG0_DSDM_FULL: u32 = RV(FV_CONFIG0_DSDM_FULL, FB_CONFIG0_DSDM);

pub const RV_CONFIG0_DSDM_AUTO: u32 = RV(FV_CONFIG0_DSDM_AUTO, FB_CONFIG0_DSDM);

pub const RV_CONFIG0_DC_BYPASS_ENABLE: u32 = RV(FV_CONFIG0_DC_BYPASS_ENABLE, FB_CONFIG0_DC_BYPASS);

pub const RV_CONFIG0_DC_BYPASS_DISABLE: u32 = RV(FV_CONFIG0_DC_BYPASS_DISABLE, FB_CONFIG0_DC_BYPASS);

pub const RV_CONFIG0_SD_FORCE_ON_ENABLE: u32 = RV(FV_CONFIG0_SD_FORCE_ON_ENABLE, FB_CONFIG0_SD_FORCE_ON);

pub const RV_CONFIG0_SD_FORCE_ON_DISABLE: u32 = RV(FV_CONFIG0_SD_FORCE_ON_DISABLE, FB_CONFIG0_SD_FORCE_ON);


 *      R_CONFIG1 (0x20)      *
 ******************************/

/* Field Offsets */
pub const FB_CONFIG1_EQ2_EN: u32 = 7;
pub const FB_CONFIG1_EQ2_BE: u32 = 4;
pub const FB_CONFIG1_EQ1_EN: u32 = 3;
pub const FB_CONFIG1_EQ1_BE: u32 = 0;

/* Field Masks */
pub const FM_CONFIG1_EQ2_EN: u32 = 0x1;
pub const FM_CONFIG1_EQ2_BE: u32 = 0x7;
pub const FM_CONFIG1_EQ1_EN: u32 = 0x1;
pub const FM_CONFIG1_EQ1_BE: u32 = 0x7;

/* Field Values */
pub const FV_CONFIG1_EQ2_EN_ENABLE: u32 = 0x1;
pub const FV_CONFIG1_EQ2_EN_DISABLE: u32 = 0x0;
pub const FV_CONFIG1_EQ2_BE_PRE: u32 = 0x0;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ_0: u32 = 0x1;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ0_1: u32 = 0x2;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ0_2: u32 = 0x3;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ0_3: u32 = 0x4;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ0_4: u32 = 0x5;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ0_5: u32 = 0x6;
pub const FV_CONFIG1_EQ1_EN_ENABLE: u32 = 0x1;
pub const FV_CONFIG1_EQ1_EN_DISABLE: u32 = 0x0;
pub const FV_CONFIG1_EQ1_BE_PRE: u32 = 0x0;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ_0: u32 = 0x1;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ0_1: u32 = 0x2;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ0_2: u32 = 0x3;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ0_3: u32 = 0x4;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ0_4: u32 = 0x5;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ0_5: u32 = 0x6;

/* Register Masks */
pub const RM_CONFIG1_EQ2_EN: u32 = RM(FM_CONFIG1_EQ2_EN, FB_CONFIG1_EQ2_EN);

pub const RM_CONFIG1_EQ2_BE: u32 = RM(FM_CONFIG1_EQ2_BE, FB_CONFIG1_EQ2_BE);

pub const RM_CONFIG1_EQ1_EN: u32 = RM(FM_CONFIG1_EQ1_EN, FB_CONFIG1_EQ1_EN);

pub const RM_CONFIG1_EQ1_BE: u32 = RM(FM_CONFIG1_EQ1_BE, FB_CONFIG1_EQ1_BE);


/* Register Values */
pub const RV_CONFIG1_EQ2_EN_ENABLE: u32 = RV(FV_CONFIG1_EQ2_EN_ENABLE, FB_CONFIG1_EQ2_EN);

pub const RV_CONFIG1_EQ2_EN_DISABLE: u32 = RV(FV_CONFIG1_EQ2_EN_DISABLE, FB_CONFIG1_EQ2_EN);

pub const RV_CONFIG1_EQ2_BE_PRE: u32 = RV(FV_CONFIG1_EQ2_BE_PRE, FB_CONFIG1_EQ2_BE);

pub const RV_CONFIG1_EQ2_BE_PRE_EQ_0: u32 = RV(FV_CONFIG1_EQ2_BE_PRE_EQ_0, FB_CONFIG1_EQ2_BE);

pub const RV_CONFIG1_EQ2_BE_PRE_EQ0_1: u32 = RV(FV_CONFIG1_EQ2_BE_PRE_EQ0_1, FB_CONFIG1_EQ2_BE);

pub const RV_CONFIG1_EQ2_BE_PRE_EQ0_2: u32 = RV(FV_CONFIG1_EQ2_BE_PRE_EQ0_2, FB_CONFIG1_EQ2_BE);

pub const RV_CONFIG1_EQ2_BE_PRE_EQ0_3: u32 = RV(FV_CONFIG1_EQ2_BE_PRE_EQ0_3, FB_CONFIG1_EQ2_BE);

pub const RV_CONFIG1_EQ2_BE_PRE_EQ0_4: u32 = RV(FV_CONFIG1_EQ2_BE_PRE_EQ0_4, FB_CONFIG1_EQ2_BE);

pub const RV_CONFIG1_EQ2_BE_PRE_EQ0_5: u32 = RV(FV_CONFIG1_EQ2_BE_PRE_EQ0_5, FB_CONFIG1_EQ2_BE);

pub const RV_CONFIG1_EQ1_EN_ENABLE: u32 = RV(FV_CONFIG1_EQ1_EN_ENABLE, FB_CONFIG1_EQ1_EN);

pub const RV_CONFIG1_EQ1_EN_DISABLE: u32 = RV(FV_CONFIG1_EQ1_EN_DISABLE, FB_CONFIG1_EQ1_EN);

pub const RV_CONFIG1_EQ1_BE_PRE: u32 = RV(FV_CONFIG1_EQ1_BE_PRE, FB_CONFIG1_EQ1_BE);

pub const RV_CONFIG1_EQ1_BE_PRE_EQ_0: u32 = RV(FV_CONFIG1_EQ1_BE_PRE_EQ_0, FB_CONFIG1_EQ1_BE);

pub const RV_CONFIG1_EQ1_BE_PRE_EQ0_1: u32 = RV(FV_CONFIG1_EQ1_BE_PRE_EQ0_1, FB_CONFIG1_EQ1_BE);

pub const RV_CONFIG1_EQ1_BE_PRE_EQ0_2: u32 = RV(FV_CONFIG1_EQ1_BE_PRE_EQ0_2, FB_CONFIG1_EQ1_BE);

pub const RV_CONFIG1_EQ1_BE_PRE_EQ0_3: u32 = RV(FV_CONFIG1_EQ1_BE_PRE_EQ0_3, FB_CONFIG1_EQ1_BE);

pub const RV_CONFIG1_EQ1_BE_PRE_EQ0_4: u32 = RV(FV_CONFIG1_EQ1_BE_PRE_EQ0_4, FB_CONFIG1_EQ1_BE);

pub const RV_CONFIG1_EQ1_BE_PRE_EQ0_5: u32 = RV(FV_CONFIG1_EQ1_BE_PRE_EQ0_5, FB_CONFIG1_EQ1_BE);


 *      R_DMICCTL (0x24)      *
 ******************************/

/* Field Offsets */
pub const FB_DMICCTL_DMICEN: u32 = 7;
pub const FB_DMICCTL_DMONO: u32 = 4;
pub const FB_DMICCTL_DMPHADJ: u32 = 2;
pub const FB_DMICCTL_DMRATE: u32 = 0;

/* Field Masks */
pub const FM_DMICCTL_DMICEN: u32 = 0x1;
pub const FM_DMICCTL_DMONO: u32 = 0x1;
pub const FM_DMICCTL_DMPHADJ: u32 = 0x3;
pub const FM_DMICCTL_DMRATE: u32 = 0x3;

/* Field Values */
pub const FV_DMICCTL_DMICEN_ENABLE: u32 = 0x1;
pub const FV_DMICCTL_DMICEN_DISABLE: u32 = 0x0;
pub const FV_DMICCTL_DMONO_STEREO: u32 = 0x0;
pub const FV_DMICCTL_DMONO_MONO: u32 = 0x1;

/* Register Masks */
pub const RM_DMICCTL_DMICEN: u32 = RM(FM_DMICCTL_DMICEN, FB_DMICCTL_DMICEN);

pub const RM_DMICCTL_DMONO: u32 = RM(FM_DMICCTL_DMONO, FB_DMICCTL_DMONO);

pub const RM_DMICCTL_DMPHADJ: u32 = RM(FM_DMICCTL_DMPHADJ, FB_DMICCTL_DMPHADJ);

pub const RM_DMICCTL_DMRATE: u32 = RM(FM_DMICCTL_DMRATE, FB_DMICCTL_DMRATE);


/* Register Values */
pub const RV_DMICCTL_DMICEN_ENABLE: u32 = RV(FV_DMICCTL_DMICEN_ENABLE, FB_DMICCTL_DMICEN);

pub const RV_DMICCTL_DMICEN_DISABLE: u32 = RV(FV_DMICCTL_DMICEN_DISABLE, FB_DMICCTL_DMICEN);

pub const RV_DMICCTL_DMONO_STEREO: u32 = RV(FV_DMICCTL_DMONO_STEREO, FB_DMICCTL_DMONO);

pub const RV_DMICCTL_DMONO_MONO: u32 = RV(FV_DMICCTL_DMONO_MONO, FB_DMICCTL_DMONO);


 *      R_CLECTL (0x25)      *
 *****************************/

/* Field Offsets */
pub const FB_CLECTL_LVL_MODE: u32 = 4;
pub const FB_CLECTL_WINDOWSEL: u32 = 3;
pub const FB_CLECTL_EXP_EN: u32 = 2;
pub const FB_CLECTL_LIMIT_EN: u32 = 1;
pub const FB_CLECTL_COMP_EN: u32 = 0;

/* Field Masks */
pub const FM_CLECTL_LVL_MODE: u32 = 0x1;
pub const FM_CLECTL_WINDOWSEL: u32 = 0x1;
pub const FM_CLECTL_EXP_EN: u32 = 0x1;
pub const FM_CLECTL_LIMIT_EN: u32 = 0x1;
pub const FM_CLECTL_COMP_EN: u32 = 0x1;

/* Field Values */
pub const FV_CLECTL_LVL_MODE_AVG: u32 = 0x0;
pub const FV_CLECTL_LVL_MODE_PEAK: u32 = 0x1;
pub const FV_CLECTL_WINDOWSEL_512: u32 = 0x0;
pub const FV_CLECTL_WINDOWSEL_64: u32 = 0x1;
pub const FV_CLECTL_EXP_EN_ENABLE: u32 = 0x1;
pub const FV_CLECTL_EXP_EN_DISABLE: u32 = 0x0;
pub const FV_CLECTL_LIMIT_EN_ENABLE: u32 = 0x1;
pub const FV_CLECTL_LIMIT_EN_DISABLE: u32 = 0x0;
pub const FV_CLECTL_COMP_EN_ENABLE: u32 = 0x1;
pub const FV_CLECTL_COMP_EN_DISABLE: u32 = 0x0;

/* Register Masks */
pub const RM_CLECTL_LVL_MODE: u32 = RM(FM_CLECTL_LVL_MODE, FB_CLECTL_LVL_MODE);

pub const RM_CLECTL_WINDOWSEL: u32 = RM(FM_CLECTL_WINDOWSEL, FB_CLECTL_WINDOWSEL);

pub const RM_CLECTL_EXP_EN: u32 = RM(FM_CLECTL_EXP_EN, FB_CLECTL_EXP_EN);

pub const RM_CLECTL_LIMIT_EN: u32 = RM(FM_CLECTL_LIMIT_EN, FB_CLECTL_LIMIT_EN);

pub const RM_CLECTL_COMP_EN: u32 = RM(FM_CLECTL_COMP_EN, FB_CLECTL_COMP_EN);


/* Register Values */
pub const RV_CLECTL_LVL_MODE_AVG: u32 = RV(FV_CLECTL_LVL_MODE_AVG, FB_CLECTL_LVL_MODE);

pub const RV_CLECTL_LVL_MODE_PEAK: u32 = RV(FV_CLECTL_LVL_MODE_PEAK, FB_CLECTL_LVL_MODE);

pub const RV_CLECTL_WINDOWSEL_512: u32 = RV(FV_CLECTL_WINDOWSEL_512, FB_CLECTL_WINDOWSEL);

pub const RV_CLECTL_WINDOWSEL_64: u32 = RV(FV_CLECTL_WINDOWSEL_64, FB_CLECTL_WINDOWSEL);

pub const RV_CLECTL_EXP_EN_ENABLE: u32 = RV(FV_CLECTL_EXP_EN_ENABLE, FB_CLECTL_EXP_EN);

pub const RV_CLECTL_EXP_EN_DISABLE: u32 = RV(FV_CLECTL_EXP_EN_DISABLE, FB_CLECTL_EXP_EN);

pub const RV_CLECTL_LIMIT_EN_ENABLE: u32 = RV(FV_CLECTL_LIMIT_EN_ENABLE, FB_CLECTL_LIMIT_EN);

pub const RV_CLECTL_LIMIT_EN_DISABLE: u32 = RV(FV_CLECTL_LIMIT_EN_DISABLE, FB_CLECTL_LIMIT_EN);

pub const RV_CLECTL_COMP_EN_ENABLE: u32 = RV(FV_CLECTL_COMP_EN_ENABLE, FB_CLECTL_COMP_EN);

pub const RV_CLECTL_COMP_EN_DISABLE: u32 = RV(FV_CLECTL_COMP_EN_DISABLE, FB_CLECTL_COMP_EN);


 *      R_MUGAIN (0x26)      *
 *****************************/

/* Field Offsets */
pub const FB_MUGAIN_CLEMUG: u32 = 0;

/* Field Masks */
pub const FM_MUGAIN_CLEMUG: u32 = 0x1F;

/* Field Values */
pub const FV_MUGAIN_CLEMUG_46PT5DB: u32 = 0x1F;
pub const FV_MUGAIN_CLEMUG_0DB: u32 = 0x0;

/* Register Masks */
pub const RM_MUGAIN_CLEMUG: u32 = RM(FM_MUGAIN_CLEMUG, FB_MUGAIN_CLEMUG);


/* Register Values */
pub const RV_MUGAIN_CLEMUG_46PT5DB: u32 = RV(FV_MUGAIN_CLEMUG_46PT5DB, FB_MUGAIN_CLEMUG);

pub const RV_MUGAIN_CLEMUG_0DB: u32 = RV(FV_MUGAIN_CLEMUG_0DB, FB_MUGAIN_CLEMUG);


 *      R_COMPTH (0x27)      *
 *****************************/

/* Field Offsets */
pub const FB_COMPTH: u32 = 0;

/* Field Masks */
pub const FM_COMPTH: u32 = 0xFF;

/* Field Values */
pub const FV_COMPTH_0DB: u32 = 0xFF;
pub const FV_COMPTH_N95PT625DB: u32 = 0x0;

/* Register Masks */
pub const RM_COMPTH: u32 = RM(FM_COMPTH, FB_COMPTH);

/* Register Values */
pub const RV_COMPTH_0DB: u32 = RV(FV_COMPTH_0DB, FB_COMPTH);
pub const RV_COMPTH_N95PT625DB: u32 = RV(FV_COMPTH_N95PT625DB, FB_COMPTH);


 *      R_CMPRAT (0x28)      *
 *****************************/

/* Field Offsets */
pub const FB_CMPRAT: u32 = 0;

/* Field Masks */
pub const FM_CMPRAT: u32 = 0x1F;

/* Register Masks */
pub const RM_CMPRAT: u32 = RM(FM_CMPRAT, FB_CMPRAT);

 *      R_CATKTCL (0x29)      *
 ******************************/

/* Field Offsets */
pub const FB_CATKTCL: u32 = 0;

/* Field Masks */
pub const FM_CATKTCL: u32 = 0xFF;

/* Register Masks */
pub const RM_CATKTCL: u32 = RM(FM_CATKTCL, FB_CATKTCL);

 *      R_CATKTCH (0x2A)      *
 ******************************/

/* Field Offsets */
pub const FB_CATKTCH: u32 = 0;

/* Field Masks */
pub const FM_CATKTCH: u32 = 0xFF;

/* Register Masks */
pub const RM_CATKTCH: u32 = RM(FM_CATKTCH, FB_CATKTCH);

 *      R_CRELTCL (0x2B)      *
 ******************************/

/* Field Offsets */
pub const FB_CRELTCL: u32 = 0;

/* Field Masks */
pub const FM_CRELTCL: u32 = 0xFF;

/* Register Masks */
pub const RM_CRELTCL: u32 = RM(FM_CRELTCL, FB_CRELTCL);

 *      R_CRELTCH (0x2C)      *
 ******************************/

/* Field Offsets */
pub const FB_CRELTCH: u32 = 0;

/* Field Masks */
pub const FM_CRELTCH: u32 = 0xFF;

/* Register Masks */
pub const RM_CRELTCH: u32 = RM(FM_CRELTCH, FB_CRELTCH);

 *      R_LIMTH (0x2D)      *
 ****************************/

/* Field Offsets */
pub const FB_LIMTH: u32 = 0;

/* Field Masks */
pub const FM_LIMTH: u32 = 0xFF;

/* Field Values */
pub const FV_LIMTH_0DB: u32 = 0xFF;
pub const FV_LIMTH_N95PT625DB: u32 = 0x0;

/* Register Masks */
pub const RM_LIMTH: u32 = RM(FM_LIMTH, FB_LIMTH);

/* Register Values */
pub const RV_LIMTH_0DB: u32 = RV(FV_LIMTH_0DB, FB_LIMTH);
pub const RV_LIMTH_N95PT625DB: u32 = RV(FV_LIMTH_N95PT625DB, FB_LIMTH);

 *      R_LIMTGT (0x2E)      *
 *****************************/

/* Field Offsets */
pub const FB_LIMTGT: u32 = 0;

/* Field Masks */
pub const FM_LIMTGT: u32 = 0xFF;

/* Field Values */
pub const FV_LIMTGT_0DB: u32 = 0xFF;
pub const FV_LIMTGT_N95PT625DB: u32 = 0x0;

/* Register Masks */
pub const RM_LIMTGT: u32 = RM(FM_LIMTGT, FB_LIMTGT);

/* Register Values */
pub const RV_LIMTGT_0DB: u32 = RV(FV_LIMTGT_0DB, FB_LIMTGT);
pub const RV_LIMTGT_N95PT625DB: u32 = RV(FV_LIMTGT_N95PT625DB, FB_LIMTGT);


 *      R_LATKTCL (0x2F)      *
 ******************************/

/* Field Offsets */
pub const FB_LATKTCL: u32 = 0;

/* Field Masks */
pub const FM_LATKTCL: u32 = 0xFF;

/* Register Masks */
pub const RM_LATKTCL: u32 = RM(FM_LATKTCL, FB_LATKTCL);

 *      R_LATKTCH (0x30)      *
 ******************************/

/* Field Offsets */
pub const FB_LATKTCH: u32 = 0;

/* Field Masks */
pub const FM_LATKTCH: u32 = 0xFF;

/* Register Masks */
pub const RM_LATKTCH: u32 = RM(FM_LATKTCH, FB_LATKTCH);

 *      R_LRELTCL (0x31)      *
 ******************************/

/* Field Offsets */
pub const FB_LRELTCL: u32 = 0;

/* Field Masks */
pub const FM_LRELTCL: u32 = 0xFF;

/* Register Masks */
pub const RM_LRELTCL: u32 = RM(FM_LRELTCL, FB_LRELTCL);

 *      R_LRELTCH (0x32)      *
 ******************************/

/* Field Offsets */
pub const FB_LRELTCH: u32 = 0;

/* Field Masks */
pub const FM_LRELTCH: u32 = 0xFF;

/* Register Masks */
pub const RM_LRELTCH: u32 = RM(FM_LRELTCH, FB_LRELTCH);

 *      R_EXPTH (0x33)      *
 ****************************/

/* Field Offsets */
pub const FB_EXPTH: u32 = 0;

/* Field Masks */
pub const FM_EXPTH: u32 = 0xFF;

/* Field Values */
pub const FV_EXPTH_0DB: u32 = 0xFF;
pub const FV_EXPTH_N95PT625DB: u32 = 0x0;

/* Register Masks */
pub const RM_EXPTH: u32 = RM(FM_EXPTH, FB_EXPTH);

/* Register Values */
pub const RV_EXPTH_0DB: u32 = RV(FV_EXPTH_0DB, FB_EXPTH);
pub const RV_EXPTH_N95PT625DB: u32 = RV(FV_EXPTH_N95PT625DB, FB_EXPTH);

 *      R_EXPRAT (0x34)      *
 *****************************/

/* Field Offsets */
pub const FB_EXPRAT: u32 = 0;

/* Field Masks */
pub const FM_EXPRAT: u32 = 0x7;

/* Register Masks */
pub const RM_EXPRAT: u32 = RM(FM_EXPRAT, FB_EXPRAT);

 *      R_XATKTCL (0x35)      *
 ******************************/

/* Field Offsets */
pub const FB_XATKTCL: u32 = 0;

/* Field Masks */
pub const FM_XATKTCL: u32 = 0xFF;

/* Register Masks */
pub const RM_XATKTCL: u32 = RM(FM_XATKTCL, FB_XATKTCL);

 *      R_XATKTCH (0x36)      *
 ******************************/

/* Field Offsets */
pub const FB_XATKTCH: u32 = 0;

/* Field Masks */
pub const FM_XATKTCH: u32 = 0xFF;

/* Register Masks */
pub const RM_XATKTCH: u32 = RM(FM_XATKTCH, FB_XATKTCH);

 *      R_XRELTCL (0x37)      *
 ******************************/

/* Field Offsets */
pub const FB_XRELTCL: u32 = 0;

/* Field Masks */
pub const FM_XRELTCL: u32 = 0xFF;

/* Register Masks */
pub const RM_XRELTCL: u32 = RM(FM_XRELTCL, FB_XRELTCL);

 *      R_XRELTCH (0x38)      *
 ******************************/

/* Field Offsets */
pub const FB_XRELTCH: u32 = 0;

/* Field Masks */
pub const FM_XRELTCH: u32 = 0xFF;

/* Register Masks */
pub const RM_XRELTCH: u32 = RM(FM_XRELTCH, FB_XRELTCH);

 *      R_FXCTL (0x39)      *
 ****************************/

/* Field Offsets */
pub const FB_FXCTL_3DEN: u32 = 4;
pub const FB_FXCTL_TEEN: u32 = 3;
pub const FB_FXCTL_TNLFBYPASS: u32 = 2;
pub const FB_FXCTL_BEEN: u32 = 1;
pub const FB_FXCTL_BNLFBYPASS: u32 = 0;

/* Field Masks */
pub const FM_FXCTL_3DEN: u32 = 0x1;
pub const FM_FXCTL_TEEN: u32 = 0x1;
pub const FM_FXCTL_TNLFBYPASS: u32 = 0x1;
pub const FM_FXCTL_BEEN: u32 = 0x1;
pub const FM_FXCTL_BNLFBYPASS: u32 = 0x1;

/* Field Values */
pub const FV_FXCTL_3DEN_ENABLE: u32 = 0x1;
pub const FV_FXCTL_3DEN_DISABLE: u32 = 0x0;
pub const FV_FXCTL_TEEN_ENABLE: u32 = 0x1;
pub const FV_FXCTL_TEEN_DISABLE: u32 = 0x0;
pub const FV_FXCTL_TNLFBYPASS_ENABLE: u32 = 0x1;
pub const FV_FXCTL_TNLFBYPASS_DISABLE: u32 = 0x0;
pub const FV_FXCTL_BEEN_ENABLE: u32 = 0x1;
pub const FV_FXCTL_BEEN_DISABLE: u32 = 0x0;
pub const FV_FXCTL_BNLFBYPASS_ENABLE: u32 = 0x1;
pub const FV_FXCTL_BNLFBYPASS_DISABLE: u32 = 0x0;

/* Register Masks */
pub const RM_FXCTL_3DEN: u32 = RM(FM_FXCTL_3DEN, FB_FXCTL_3DEN);
pub const RM_FXCTL_TEEN: u32 = RM(FM_FXCTL_TEEN, FB_FXCTL_TEEN);
pub const RM_FXCTL_TNLFBYPASS: u32 = RM(FM_FXCTL_TNLFBYPASS, FB_FXCTL_TNLFBYPASS);

pub const RM_FXCTL_BEEN: u32 = RM(FM_FXCTL_BEEN, FB_FXCTL_BEEN);
pub const RM_FXCTL_BNLFBYPASS: u32 = RM(FM_FXCTL_BNLFBYPASS, FB_FXCTL_BNLFBYPASS);


/* Register Values */
pub const RV_FXCTL_3DEN_ENABLE: u32 = RV(FV_FXCTL_3DEN_ENABLE, FB_FXCTL_3DEN);

pub const RV_FXCTL_3DEN_DISABLE: u32 = RV(FV_FXCTL_3DEN_DISABLE, FB_FXCTL_3DEN);

pub const RV_FXCTL_TEEN_ENABLE: u32 = RV(FV_FXCTL_TEEN_ENABLE, FB_FXCTL_TEEN);

pub const RV_FXCTL_TEEN_DISABLE: u32 = RV(FV_FXCTL_TEEN_DISABLE, FB_FXCTL_TEEN);

pub const RV_FXCTL_TNLFBYPASS_ENABLE: u32 = RV(FV_FXCTL_TNLFBYPASS_ENABLE, FB_FXCTL_TNLFBYPASS);

pub const RV_FXCTL_TNLFBYPASS_DISABLE: u32 = RV(FV_FXCTL_TNLFBYPASS_DISABLE, FB_FXCTL_TNLFBYPASS);

pub const RV_FXCTL_BEEN_ENABLE: u32 = RV(FV_FXCTL_BEEN_ENABLE, FB_FXCTL_BEEN);

pub const RV_FXCTL_BEEN_DISABLE: u32 = RV(FV_FXCTL_BEEN_DISABLE, FB_FXCTL_BEEN);

pub const RV_FXCTL_BNLFBYPASS_ENABLE: u32 = RV(FV_FXCTL_BNLFBYPASS_ENABLE, FB_FXCTL_BNLFBYPASS);

pub const RV_FXCTL_BNLFBYPASS_DISABLE: u32 = RV(FV_FXCTL_BNLFBYPASS_DISABLE, FB_FXCTL_BNLFBYPASS);


 *      R_DACCRWRL (0x3A)      *
 *******************************/

/* Field Offsets */
pub const FB_DACCRWRL_DACCRWDL: u32 = 0;

/* Field Masks */
pub const FM_DACCRWRL_DACCRWDL: u32 = 0xFF;

/* Register Masks */
pub const RM_DACCRWRL_DACCRWDL: u32 = RM(FM_DACCRWRL_DACCRWDL, FB_DACCRWRL_DACCRWDL);


 *      R_DACCRWRM (0x3B)      *
 *******************************/

/* Field Offsets */
pub const FB_DACCRWRM_DACCRWDM: u32 = 0;

/* Field Masks */
pub const FM_DACCRWRM_DACCRWDM: u32 = 0xFF;

/* Register Masks */
pub const RM_DACCRWRM_DACCRWDM: u32 = RM(FM_DACCRWRM_DACCRWDM, FB_DACCRWRM_DACCRWDM);


 *      R_DACCRWRH (0x3C)      *
 *******************************/

/* Field Offsets */
pub const FB_DACCRWRH_DACCRWDH: u32 = 0;

/* Field Masks */
pub const FM_DACCRWRH_DACCRWDH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACCRWRH_DACCRWDH: u32 = RM(FM_DACCRWRH_DACCRWDH, FB_DACCRWRH_DACCRWDH);


 *      R_DACCRRDL (0x3D)      *
 *******************************/

/* Field Offsets */
pub const FB_DACCRRDL: u32 = 0;

/* Field Masks */
pub const FM_DACCRRDL: u32 = 0xFF;

/* Register Masks */
pub const RM_DACCRRDL: u32 = RM(FM_DACCRRDL, FB_DACCRRDL);

 *      R_DACCRRDM (0x3E)      *
 *******************************/

/* Field Offsets */
pub const FB_DACCRRDM: u32 = 0;

/* Field Masks */
pub const FM_DACCRRDM: u32 = 0xFF;

/* Register Masks */
pub const RM_DACCRRDM: u32 = RM(FM_DACCRRDM, FB_DACCRRDM);

 *      R_DACCRRDH (0x3F)      *
 *******************************/

/* Field Offsets */
pub const FB_DACCRRDH: u32 = 0;

/* Field Masks */
pub const FM_DACCRRDH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACCRRDH: u32 = RM(FM_DACCRRDH, FB_DACCRRDH);

 *      R_DACCRADDR (0x40)      *
 ********************************/

/* Field Offsets */
pub const FB_DACCRADDR_DACCRADD: u32 = 0;

/* Field Masks */
pub const FM_DACCRADDR_DACCRADD: u32 = 0xFF;

/* Register Masks */
pub const RM_DACCRADDR_DACCRADD: u32 = RM(FM_DACCRADDR_DACCRADD, FB_DACCRADDR_DACCRADD);


 *      R_DCOFSEL (0x41)      *
 ******************************/

/* Field Offsets */
pub const FB_DCOFSEL_DC_COEF_SEL: u32 = 0;

/* Field Masks */
pub const FM_DCOFSEL_DC_COEF_SEL: u32 = 0x7;

/* Field Values */
pub const FV_DCOFSEL_DC_COEF_SEL_2_N8: u32 = 0x0;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N9: u32 = 0x1;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N10: u32 = 0x2;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N11: u32 = 0x3;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N12: u32 = 0x4;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N13: u32 = 0x5;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N14: u32 = 0x6;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N15: u32 = 0x7;

/* Register Masks */
pub const RM_DCOFSEL_DC_COEF_SEL: u32 = RM(FM_DCOFSEL_DC_COEF_SEL, FB_DCOFSEL_DC_COEF_SEL);


/* Register Values */
pub const RV_DCOFSEL_DC_COEF_SEL_2_N8: u32 = RV(FV_DCOFSEL_DC_COEF_SEL_2_N8, FB_DCOFSEL_DC_COEF_SEL);

pub const RV_DCOFSEL_DC_COEF_SEL_2_N9: u32 = RV(FV_DCOFSEL_DC_COEF_SEL_2_N9, FB_DCOFSEL_DC_COEF_SEL);

pub const RV_DCOFSEL_DC_COEF_SEL_2_N10: u32 = RV(FV_DCOFSEL_DC_COEF_SEL_2_N10, FB_DCOFSEL_DC_COEF_SEL);

pub const RV_DCOFSEL_DC_COEF_SEL_2_N11: u32 = RV(FV_DCOFSEL_DC_COEF_SEL_2_N11, FB_DCOFSEL_DC_COEF_SEL);

pub const RV_DCOFSEL_DC_COEF_SEL_2_N12: u32 = RV(FV_DCOFSEL_DC_COEF_SEL_2_N12, FB_DCOFSEL_DC_COEF_SEL);

pub const RV_DCOFSEL_DC_COEF_SEL_2_N13: u32 = RV(FV_DCOFSEL_DC_COEF_SEL_2_N13, FB_DCOFSEL_DC_COEF_SEL);

pub const RV_DCOFSEL_DC_COEF_SEL_2_N14: u32 = RV(FV_DCOFSEL_DC_COEF_SEL_2_N14, FB_DCOFSEL_DC_COEF_SEL);

pub const RV_DCOFSEL_DC_COEF_SEL_2_N15: u32 = RV(FV_DCOFSEL_DC_COEF_SEL_2_N15, FB_DCOFSEL_DC_COEF_SEL);


 *      R_PLLCTL9 (0x4E)      *
 ******************************/

/* Field Offsets */
pub const FB_PLLCTL9_REFDIV_PLL1: u32 = 0;

/* Field Masks */
pub const FM_PLLCTL9_REFDIV_PLL1: u32 = 0xFF;

/* Register Masks */
pub const RM_PLLCTL9_REFDIV_PLL1: u32 = RM(FM_PLLCTL9_REFDIV_PLL1, FB_PLLCTL9_REFDIV_PLL1);


 *      R_PLLCTLA (0x4F)      *
 ******************************/

/* Field Offsets */
pub const FB_PLLCTLA_OUTDIV_PLL1: u32 = 0;

/* Field Masks */
pub const FM_PLLCTLA_OUTDIV_PLL1: u32 = 0xFF;

/* Register Masks */
pub const RM_PLLCTLA_OUTDIV_PLL1: u32 = RM(FM_PLLCTLA_OUTDIV_PLL1, FB_PLLCTLA_OUTDIV_PLL1);


 *      R_PLLCTLB (0x50)      *
 ******************************/

/* Field Offsets */
pub const FB_PLLCTLB_FBDIV_PLL1L: u32 = 0;

/* Field Masks */
pub const FM_PLLCTLB_FBDIV_PLL1L: u32 = 0xFF;

/* Register Masks */
pub const RM_PLLCTLB_FBDIV_PLL1L: u32 = RM(FM_PLLCTLB_FBDIV_PLL1L, FB_PLLCTLB_FBDIV_PLL1L);


 *      R_PLLCTLC (0x51)      *
 ******************************/

/* Field Offsets */
pub const FB_PLLCTLC_FBDIV_PLL1H: u32 = 0;

/* Field Masks */
pub const FM_PLLCTLC_FBDIV_PLL1H: u32 = 0x7;

/* Register Masks */
pub const RM_PLLCTLC_FBDIV_PLL1H: u32 = RM(FM_PLLCTLC_FBDIV_PLL1H, FB_PLLCTLC_FBDIV_PLL1H);


 *      R_PLLCTLD (0x52)      *
 ******************************/

/* Field Offsets */
pub const FB_PLLCTLD_RZ_PLL1: u32 = 3;
pub const FB_PLLCTLD_CP_PLL1: u32 = 0;

/* Field Masks */
pub const FM_PLLCTLD_RZ_PLL1: u32 = 0x7;
pub const FM_PLLCTLD_CP_PLL1: u32 = 0x7;

/* Register Masks */
pub const RM_PLLCTLD_RZ_PLL1: u32 = RM(FM_PLLCTLD_RZ_PLL1, FB_PLLCTLD_RZ_PLL1);

pub const RM_PLLCTLD_CP_PLL1: u32 = RM(FM_PLLCTLD_CP_PLL1, FB_PLLCTLD_CP_PLL1);


 *      R_PLLCTLE (0x53)      *
 ******************************/

/* Field Offsets */
pub const FB_PLLCTLE_REFDIV_PLL2: u32 = 0;

/* Field Masks */
pub const FM_PLLCTLE_REFDIV_PLL2: u32 = 0xFF;

/* Register Masks */
pub const RM_PLLCTLE_REFDIV_PLL2: u32 = RM(FM_PLLCTLE_REFDIV_PLL2, FB_PLLCTLE_REFDIV_PLL2);


 *      R_PLLCTLF (0x54)      *
 ******************************/

/* Field Offsets */
pub const FB_PLLCTLF_OUTDIV_PLL2: u32 = 0;

/* Field Masks */
pub const FM_PLLCTLF_OUTDIV_PLL2: u32 = 0xFF;

/* Register Masks */
pub const RM_PLLCTLF_OUTDIV_PLL2: u32 = RM(FM_PLLCTLF_OUTDIV_PLL2, FB_PLLCTLF_OUTDIV_PLL2);


 *      R_PLLCTL10 (0x55)      *
 *******************************/

/* Field Offsets */
pub const FB_PLLCTL10_FBDIV_PLL2L: u32 = 0;

/* Field Masks */
pub const FM_PLLCTL10_FBDIV_PLL2L: u32 = 0xFF;

/* Register Masks */
pub const RM_PLLCTL10_FBDIV_PLL2L: u32 = RM(FM_PLLCTL10_FBDIV_PLL2L, FB_PLLCTL10_FBDIV_PLL2L);


 *      R_PLLCTL11 (0x56)      *
 *******************************/

/* Field Offsets */
pub const FB_PLLCTL11_FBDIV_PLL2H: u32 = 0;

/* Field Masks */
pub const FM_PLLCTL11_FBDIV_PLL2H: u32 = 0x7;

/* Register Masks */
pub const RM_PLLCTL11_FBDIV_PLL2H: u32 = RM(FM_PLLCTL11_FBDIV_PLL2H, FB_PLLCTL11_FBDIV_PLL2H);


 *      R_PLLCTL12 (0x57)      *
 *******************************/

/* Field Offsets */
pub const FB_PLLCTL12_RZ_PLL2: u32 = 3;
pub const FB_PLLCTL12_CP_PLL2: u32 = 0;

/* Field Masks */
pub const FM_PLLCTL12_RZ_PLL2: u32 = 0x7;
pub const FM_PLLCTL12_CP_PLL2: u32 = 0x7;

/* Register Masks */
pub const RM_PLLCTL12_RZ_PLL2: u32 = RM(FM_PLLCTL12_RZ_PLL2, FB_PLLCTL12_RZ_PLL2);

pub const RM_PLLCTL12_CP_PLL2: u32 = RM(FM_PLLCTL12_CP_PLL2, FB_PLLCTL12_CP_PLL2);


 *      R_PLLCTL1B (0x60)      *
 *******************************/

/* Field Offsets */
pub const FB_PLLCTL1B_VCOI_PLL2: u32 = 4;
pub const FB_PLLCTL1B_VCOI_PLL1: u32 = 2;

/* Field Masks */
pub const FM_PLLCTL1B_VCOI_PLL2: u32 = 0x3;
pub const FM_PLLCTL1B_VCOI_PLL1: u32 = 0x3;

/* Register Masks */
pub const RM_PLLCTL1B_VCOI_PLL2: u32 = RM(FM_PLLCTL1B_VCOI_PLL2, FB_PLLCTL1B_VCOI_PLL2);

pub const RM_PLLCTL1B_VCOI_PLL1: u32 = RM(FM_PLLCTL1B_VCOI_PLL1, FB_PLLCTL1B_VCOI_PLL1);


 *      R_PLLCTL1C (0x61)      *
 *******************************/

/* Field Offsets */
pub const FB_PLLCTL1C_PDB_PLL2: u32 = 2;
pub const FB_PLLCTL1C_PDB_PLL1: u32 = 1;

/* Field Masks */
pub const FM_PLLCTL1C_PDB_PLL2: u32 = 0x1;
pub const FM_PLLCTL1C_PDB_PLL1: u32 = 0x1;

/* Field Values */
pub const FV_PLLCTL1C_PDB_PLL2_ENABLE: u32 = 0x1;
pub const FV_PLLCTL1C_PDB_PLL2_DISABLE: u32 = 0x0;
pub const FV_PLLCTL1C_PDB_PLL1_ENABLE: u32 = 0x1;
pub const FV_PLLCTL1C_PDB_PLL1_DISABLE: u32 = 0x0;

/* Register Masks */
pub const RM_PLLCTL1C_PDB_PLL2: u32 = RM(FM_PLLCTL1C_PDB_PLL2, FB_PLLCTL1C_PDB_PLL2);

pub const RM_PLLCTL1C_PDB_PLL1: u32 = RM(FM_PLLCTL1C_PDB_PLL1, FB_PLLCTL1C_PDB_PLL1);


/* Register Values */
pub const RV_PLLCTL1C_PDB_PLL2_ENABLE: u32 = RV(FV_PLLCTL1C_PDB_PLL2_ENABLE, FB_PLLCTL1C_PDB_PLL2);

pub const RV_PLLCTL1C_PDB_PLL2_DISABLE: u32 = RV(FV_PLLCTL1C_PDB_PLL2_DISABLE, FB_PLLCTL1C_PDB_PLL2);

pub const RV_PLLCTL1C_PDB_PLL1_ENABLE: u32 = RV(FV_PLLCTL1C_PDB_PLL1_ENABLE, FB_PLLCTL1C_PDB_PLL1);

pub const RV_PLLCTL1C_PDB_PLL1_DISABLE: u32 = RV(FV_PLLCTL1C_PDB_PLL1_DISABLE, FB_PLLCTL1C_PDB_PLL1);


 *      R_TIMEBASE (0x77)      *
 *******************************/

/* Field Offsets */
pub const FB_TIMEBASE_DIVIDER: u32 = 0;

/* Field Masks */
pub const FM_TIMEBASE_DIVIDER: u32 = 0xFF;

/* Register Masks */
pub const RM_TIMEBASE_DIVIDER: u32 = RM(FM_TIMEBASE_DIVIDER, FB_TIMEBASE_DIVIDER);


 *      R_DEVIDL (0x7D)      *
 *****************************/

/* Field Offsets */
pub const FB_DEVIDL_DIDL: u32 = 0;

/* Field Masks */
pub const FM_DEVIDL_DIDL: u32 = 0xFF;

/* Register Masks */
pub const RM_DEVIDL_DIDL: u32 = RM(FM_DEVIDL_DIDL, FB_DEVIDL_DIDL);

 *      R_DEVIDH (0x7E)      *
 *****************************/

/* Field Offsets */
pub const FB_DEVIDH_DIDH: u32 = 0;

/* Field Masks */
pub const FM_DEVIDH_DIDH: u32 = 0xFF;

/* Register Masks */
pub const RM_DEVIDH_DIDH: u32 = RM(FM_DEVIDH_DIDH, FB_DEVIDH_DIDH);

 *      R_RESET (0x80)      *
 ****************************/

/* Field Offsets */
pub const FB_RESET: u32 = 0;

/* Field Masks */
pub const FM_RESET: u32 = 0xFF;

/* Field Values */
pub const FV_RESET_ENABLE: u32 = 0x85;

/* Register Masks */
pub const RM_RESET: u32 = RM(FM_RESET, FB_RESET);

/* Register Values */
pub const RV_RESET_ENABLE: u32 = RV(FV_RESET_ENABLE, FB_RESET);

 *      R_DACCRSTAT (0x8A)      *
 ********************************/

/* Field Offsets */
pub const FB_DACCRSTAT_DACCR_BUSY: u32 = 7;

/* Field Masks */
pub const FM_DACCRSTAT_DACCR_BUSY: u32 = 0x1;

/* Register Masks */
pub const RM_DACCRSTAT_DACCR_BUSY: u32 = RM(FM_DACCRSTAT_DACCR_BUSY, FB_DACCRSTAT_DACCR_BUSY);


 *      R_PLLCTL0 (0x8E)      *
 ******************************/

/* Field Offsets */
pub const FB_PLLCTL0_PLL2_LOCK: u32 = 1;
pub const FB_PLLCTL0_PLL1_LOCK: u32 = 0;

/* Field Masks */
pub const FM_PLLCTL0_PLL2_LOCK: u32 = 0x1;
pub const FM_PLLCTL0_PLL1_LOCK: u32 = 0x1;

/* Register Masks */
pub const RM_PLLCTL0_PLL2_LOCK: u32 = RM(FM_PLLCTL0_PLL2_LOCK, FB_PLLCTL0_PLL2_LOCK);

pub const RM_PLLCTL0_PLL1_LOCK: u32 = RM(FM_PLLCTL0_PLL1_LOCK, FB_PLLCTL0_PLL1_LOCK);


 *      R_PLLREFSEL (0x8F)      *
 ********************************/

/* Field Offsets */
pub const FB_PLLREFSEL_PLL2_REF_SEL: u32 = 4;
pub const FB_PLLREFSEL_PLL1_REF_SEL: u32 = 0;

/* Field Masks */
pub const FM_PLLREFSEL_PLL2_REF_SEL: u32 = 0x7;
pub const FM_PLLREFSEL_PLL1_REF_SEL: u32 = 0x7;

/* Field Values */
pub const FV_PLLREFSEL_PLL2_REF_SEL_XTAL_MCLK1: u32 = 0x0;
pub const FV_PLLREFSEL_PLL2_REF_SEL_MCLK2: u32 = 0x1;
pub const FV_PLLREFSEL_PLL1_REF_SEL_XTAL_MCLK1: u32 = 0x0;
pub const FV_PLLREFSEL_PLL1_REF_SEL_MCLK2: u32 = 0x1;

/* Register Masks */
pub const RM_PLLREFSEL_PLL2_REF_SEL: u32 = RM(FM_PLLREFSEL_PLL2_REF_SEL, FB_PLLREFSEL_PLL2_REF_SEL);

pub const RM_PLLREFSEL_PLL1_REF_SEL: u32 = RM(FM_PLLREFSEL_PLL1_REF_SEL, FB_PLLREFSEL_PLL1_REF_SEL);


/* Register Values */
pub const RV_PLLREFSEL_PLL2_REF_SEL_XTAL_MCLK1: u32 = RV(FV_PLLREFSEL_PLL2_REF_SEL_XTAL_MCLK1, FB_PLLREFSEL_PLL2_REF_SEL);

pub const RV_PLLREFSEL_PLL2_REF_SEL_MCLK2: u32 = RV(FV_PLLREFSEL_PLL2_REF_SEL_MCLK2, FB_PLLREFSEL_PLL2_REF_SEL);

pub const RV_PLLREFSEL_PLL1_REF_SEL_XTAL_MCLK1: u32 = RV(FV_PLLREFSEL_PLL1_REF_SEL_XTAL_MCLK1, FB_PLLREFSEL_PLL1_REF_SEL);

pub const RV_PLLREFSEL_PLL1_REF_SEL_MCLK2: u32 = RV(FV_PLLREFSEL_PLL1_REF_SEL_MCLK2, FB_PLLREFSEL_PLL1_REF_SEL);


 *      R_DACMBCEN (0xC7)      *
 *******************************/

/* Field Offsets */
pub const FB_DACMBCEN_MBCEN3: u32 = 2;
pub const FB_DACMBCEN_MBCEN2: u32 = 1;
pub const FB_DACMBCEN_MBCEN1: u32 = 0;

/* Field Masks */
pub const FM_DACMBCEN_MBCEN3: u32 = 0x1;
pub const FM_DACMBCEN_MBCEN2: u32 = 0x1;
pub const FM_DACMBCEN_MBCEN1: u32 = 0x1;

/* Register Masks */
pub const RM_DACMBCEN_MBCEN3: u32 = RM(FM_DACMBCEN_MBCEN3, FB_DACMBCEN_MBCEN3);

pub const RM_DACMBCEN_MBCEN2: u32 = RM(FM_DACMBCEN_MBCEN2, FB_DACMBCEN_MBCEN2);

pub const RM_DACMBCEN_MBCEN1: u32 = RM(FM_DACMBCEN_MBCEN1, FB_DACMBCEN_MBCEN1);


 *      R_DACMBCCTL (0xC8)      *
 ********************************/

/* Field Offsets */
pub const FB_DACMBCCTL_LVLMODE3: u32 = 5;
pub const FB_DACMBCCTL_WINSEL3: u32 = 4;
pub const FB_DACMBCCTL_LVLMODE2: u32 = 3;
pub const FB_DACMBCCTL_WINSEL2: u32 = 2;
pub const FB_DACMBCCTL_LVLMODE1: u32 = 1;
pub const FB_DACMBCCTL_WINSEL1: u32 = 0;

/* Field Masks */
pub const FM_DACMBCCTL_LVLMODE3: u32 = 0x1;
pub const FM_DACMBCCTL_WINSEL3: u32 = 0x1;
pub const FM_DACMBCCTL_LVLMODE2: u32 = 0x1;
pub const FM_DACMBCCTL_WINSEL2: u32 = 0x1;
pub const FM_DACMBCCTL_LVLMODE1: u32 = 0x1;
pub const FM_DACMBCCTL_WINSEL1: u32 = 0x1;

/* Register Masks */
pub const RM_DACMBCCTL_LVLMODE3: u32 = RM(FM_DACMBCCTL_LVLMODE3, FB_DACMBCCTL_LVLMODE3);

pub const RM_DACMBCCTL_WINSEL3: u32 = RM(FM_DACMBCCTL_WINSEL3, FB_DACMBCCTL_WINSEL3);

pub const RM_DACMBCCTL_LVLMODE2: u32 = RM(FM_DACMBCCTL_LVLMODE2, FB_DACMBCCTL_LVLMODE2);

pub const RM_DACMBCCTL_WINSEL2: u32 = RM(FM_DACMBCCTL_WINSEL2, FB_DACMBCCTL_WINSEL2);

pub const RM_DACMBCCTL_LVLMODE1: u32 = RM(FM_DACMBCCTL_LVLMODE1, FB_DACMBCCTL_LVLMODE1);

pub const RM_DACMBCCTL_WINSEL1: u32 = RM(FM_DACMBCCTL_WINSEL1, FB_DACMBCCTL_WINSEL1);


 *      R_DACMBCMUG1 (0xC9)      *
 *********************************/

/* Field Offsets */
pub const FB_DACMBCMUG1_PHASE: u32 = 5;
pub const FB_DACMBCMUG1_MUGAIN: u32 = 0;

/* Field Masks */
pub const FM_DACMBCMUG1_PHASE: u32 = 0x1;
pub const FM_DACMBCMUG1_MUGAIN: u32 = 0x1F;

/* Register Masks */
pub const RM_DACMBCMUG1_PHASE: u32 = RM(FM_DACMBCMUG1_PHASE, FB_DACMBCMUG1_PHASE);

pub const RM_DACMBCMUG1_MUGAIN: u32 = RM(FM_DACMBCMUG1_MUGAIN, FB_DACMBCMUG1_MUGAIN);


 *      R_DACMBCTHR1 (0xCA)      *
 *********************************/

/* Field Offsets */
pub const FB_DACMBCTHR1_THRESH: u32 = 0;

/* Field Masks */
pub const FM_DACMBCTHR1_THRESH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCTHR1_THRESH: u32 = RM(FM_DACMBCTHR1_THRESH, FB_DACMBCTHR1_THRESH);


 *      R_DACMBCRAT1 (0xCB)      *
 *********************************/

/* Field Offsets */
pub const FB_DACMBCRAT1_RATIO: u32 = 0;

/* Field Masks */
pub const FM_DACMBCRAT1_RATIO: u32 = 0x1F;

/* Register Masks */
pub const RM_DACMBCRAT1_RATIO: u32 = RM(FM_DACMBCRAT1_RATIO, FB_DACMBCRAT1_RATIO);


 *      R_DACMBCATK1L (0xCC)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCATK1L_TCATKL: u32 = 0;

/* Field Masks */
pub const FM_DACMBCATK1L_TCATKL: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCATK1L_TCATKL: u32 = RM(FM_DACMBCATK1L_TCATKL, FB_DACMBCATK1L_TCATKL);


 *      R_DACMBCATK1H (0xCD)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCATK1H_TCATKH: u32 = 0;

/* Field Masks */
pub const FM_DACMBCATK1H_TCATKH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCATK1H_TCATKH: u32 = RM(FM_DACMBCATK1H_TCATKH, FB_DACMBCATK1H_TCATKH);


 *      R_DACMBCREL1L (0xCE)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCREL1L_TCRELL: u32 = 0;

/* Field Masks */
pub const FM_DACMBCREL1L_TCRELL: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCREL1L_TCRELL: u32 = RM(FM_DACMBCREL1L_TCRELL, FB_DACMBCREL1L_TCRELL);


 *      R_DACMBCREL1H (0xCF)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCREL1H_TCRELH: u32 = 0;

/* Field Masks */
pub const FM_DACMBCREL1H_TCRELH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCREL1H_TCRELH: u32 = RM(FM_DACMBCREL1H_TCRELH, FB_DACMBCREL1H_TCRELH);


 *      R_DACMBCMUG2 (0xD0)      *
 *********************************/

/* Field Offsets */
pub const FB_DACMBCMUG2_PHASE: u32 = 5;
pub const FB_DACMBCMUG2_MUGAIN: u32 = 0;

/* Field Masks */
pub const FM_DACMBCMUG2_PHASE: u32 = 0x1;
pub const FM_DACMBCMUG2_MUGAIN: u32 = 0x1F;

/* Register Masks */
pub const RM_DACMBCMUG2_PHASE: u32 = RM(FM_DACMBCMUG2_PHASE, FB_DACMBCMUG2_PHASE);

pub const RM_DACMBCMUG2_MUGAIN: u32 = RM(FM_DACMBCMUG2_MUGAIN, FB_DACMBCMUG2_MUGAIN);


 *      R_DACMBCTHR2 (0xD1)      *
 *********************************/

/* Field Offsets */
pub const FB_DACMBCTHR2_THRESH: u32 = 0;

/* Field Masks */
pub const FM_DACMBCTHR2_THRESH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCTHR2_THRESH: u32 = RM(FM_DACMBCTHR2_THRESH, FB_DACMBCTHR2_THRESH);


 *      R_DACMBCRAT2 (0xD2)      *
 *********************************/

/* Field Offsets */
pub const FB_DACMBCRAT2_RATIO: u32 = 0;

/* Field Masks */
pub const FM_DACMBCRAT2_RATIO: u32 = 0x1F;

/* Register Masks */
pub const RM_DACMBCRAT2_RATIO: u32 = RM(FM_DACMBCRAT2_RATIO, FB_DACMBCRAT2_RATIO);


 *      R_DACMBCATK2L (0xD3)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCATK2L_TCATKL: u32 = 0;

/* Field Masks */
pub const FM_DACMBCATK2L_TCATKL: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCATK2L_TCATKL: u32 = RM(FM_DACMBCATK2L_TCATKL, FB_DACMBCATK2L_TCATKL);


 *      R_DACMBCATK2H (0xD4)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCATK2H_TCATKH: u32 = 0;

/* Field Masks */
pub const FM_DACMBCATK2H_TCATKH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCATK2H_TCATKH: u32 = RM(FM_DACMBCATK2H_TCATKH, FB_DACMBCATK2H_TCATKH);


 *      R_DACMBCREL2L (0xD5)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCREL2L_TCRELL: u32 = 0;

/* Field Masks */
pub const FM_DACMBCREL2L_TCRELL: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCREL2L_TCRELL: u32 = RM(FM_DACMBCREL2L_TCRELL, FB_DACMBCREL2L_TCRELL);


 *      R_DACMBCREL2H (0xD6)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCREL2H_TCRELH: u32 = 0;

/* Field Masks */
pub const FM_DACMBCREL2H_TCRELH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCREL2H_TCRELH: u32 = RM(FM_DACMBCREL2H_TCRELH, FB_DACMBCREL2H_TCRELH);


 *      R_DACMBCMUG3 (0xD7)      *
 *********************************/

/* Field Offsets */
pub const FB_DACMBCMUG3_PHASE: u32 = 5;
pub const FB_DACMBCMUG3_MUGAIN: u32 = 0;

/* Field Masks */
pub const FM_DACMBCMUG3_PHASE: u32 = 0x1;
pub const FM_DACMBCMUG3_MUGAIN: u32 = 0x1F;

/* Register Masks */
pub const RM_DACMBCMUG3_PHASE: u32 = RM(FM_DACMBCMUG3_PHASE, FB_DACMBCMUG3_PHASE);

pub const RM_DACMBCMUG3_MUGAIN: u32 = RM(FM_DACMBCMUG3_MUGAIN, FB_DACMBCMUG3_MUGAIN);


 *      R_DACMBCTHR3 (0xD8)      *
 *********************************/

/* Field Offsets */
pub const FB_DACMBCTHR3_THRESH: u32 = 0;

/* Field Masks */
pub const FM_DACMBCTHR3_THRESH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCTHR3_THRESH: u32 = RM(FM_DACMBCTHR3_THRESH, FB_DACMBCTHR3_THRESH);


 *      R_DACMBCRAT3 (0xD9)      *
 *********************************/

/* Field Offsets */
pub const FB_DACMBCRAT3_RATIO: u32 = 0;

/* Field Masks */
pub const FM_DACMBCRAT3_RATIO: u32 = 0x1F;

/* Register Masks */
pub const RM_DACMBCRAT3_RATIO: u32 = RM(FM_DACMBCRAT3_RATIO, FB_DACMBCRAT3_RATIO);


 *      R_DACMBCATK3L (0xDA)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCATK3L_TCATKL: u32 = 0;

/* Field Masks */
pub const FM_DACMBCATK3L_TCATKL: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCATK3L_TCATKL: u32 = RM(FM_DACMBCATK3L_TCATKL, FB_DACMBCATK3L_TCATKL);


 *      R_DACMBCATK3H (0xDB)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCATK3H_TCATKH: u32 = 0;

/* Field Masks */
pub const FM_DACMBCATK3H_TCATKH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCATK3H_TCATKH: u32 = RM(FM_DACMBCATK3H_TCATKH, FB_DACMBCATK3H_TCATKH);


 *      R_DACMBCREL3L (0xDC)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCREL3L_TCRELL: u32 = 0;

/* Field Masks */
pub const FM_DACMBCREL3L_TCRELL: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCREL3L_TCRELL: u32 = RM(FM_DACMBCREL3L_TCRELL, FB_DACMBCREL3L_TCRELL);


 *      R_DACMBCREL3H (0xDD)      *
 **********************************/

/* Field Offsets */
pub const FB_DACMBCREL3H_TCRELH: u32 = 0;

/* Field Masks */
pub const FM_DACMBCREL3H_TCRELH: u32 = 0xFF;

/* Register Masks */
pub const RM_DACMBCREL3H_TCRELH: u32 = RM(FM_DACMBCREL3H_TCRELH, FB_DACMBCREL3H_TCRELH);



// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
