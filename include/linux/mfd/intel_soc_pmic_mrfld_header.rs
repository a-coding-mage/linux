/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header file for Intel Merrifield Basin Cove PMIC
 *
 * Copyright (C) 2019 Intel Corporation. All rights reserved.
 */

// Translated from intel_soc_pmic_mrfld.h. The original dependency on
// <linux/bits.h> is represented by the local BIT and GENMASK equivalents.

pub const BCOVE_ID: u32 = 0x00;

pub const BCOVE_ID_MINREV0: u32 = 0x0000_0007;
pub const BCOVE_ID_MAJREV0: u32 = 0x0000_0038;
pub const BCOVE_ID_VENDID0: u32 = 0x0000_00C0;

#[inline]
pub const fn BCOVE_MINOR(x: u32) -> u32 {
    (x & BCOVE_ID_MINREV0) >> 0
}

#[inline]
pub const fn BCOVE_MAJOR(x: u32) -> u32 {
    (x & BCOVE_ID_MAJREV0) >> 3
}

#[inline]
pub const fn BCOVE_VENDOR(x: u32) -> u32 {
    (x & BCOVE_ID_VENDID0) >> 6
}

pub const BCOVE_IRQLVL1: u32 = 0x01;

pub const BCOVE_PBIRQ: u32 = 0x02;
pub const BCOVE_TMUIRQ: u32 = 0x03;
pub const BCOVE_THRMIRQ: u32 = 0x04;
pub const BCOVE_BCUIRQ: u32 = 0x05;
pub const BCOVE_ADCIRQ: u32 = 0x06;
pub const BCOVE_CHGRIRQ0: u32 = 0x07;
pub const BCOVE_CHGRIRQ1: u32 = 0x08;
pub const BCOVE_GPIOIRQ: u32 = 0x09;
pub const BCOVE_CRITIRQ: u32 = 0x0B;

pub const BCOVE_MIRQLVL1: u32 = 0x0C;

pub const BCOVE_MPBIRQ: u32 = 0x0D;
pub const BCOVE_MTMUIRQ: u32 = 0x0E;
pub const BCOVE_MTHRMIRQ: u32 = 0x0F;
pub const BCOVE_MBCUIRQ: u32 = 0x10;
pub const BCOVE_MADCIRQ: u32 = 0x11;
pub const BCOVE_MCHGRIRQ0: u32 = 0x12;
pub const BCOVE_MCHGRIRQ1: u32 = 0x13;
pub const BCOVE_MGPIOIRQ: u32 = 0x14;
pub const BCOVE_MCRITIRQ: u32 = 0x16;

pub const BCOVE_SCHGRIRQ0: u32 = 0x4E;
pub const BCOVE_SCHGRIRQ1: u32 = 0x4F;

/* Level 1 IRQs */
pub const BCOVE_LVL1_PWRBTN: u32 = 1 << 0; /* power button */
pub const BCOVE_LVL1_TMU: u32 = 1 << 1; /* time management unit */
pub const BCOVE_LVL1_THRM: u32 = 1 << 2; /* thermal */
pub const BCOVE_LVL1_BCU: u32 = 1 << 3; /* burst control unit */
pub const BCOVE_LVL1_ADC: u32 = 1 << 4; /* ADC */
pub const BCOVE_LVL1_CHGR: u32 = 1 << 5; /* charger */
pub const BCOVE_LVL1_GPIO: u32 = 1 << 6; /* GPIO */
pub const BCOVE_LVL1_CRIT: u32 = 1 << 7; /* critical event */

/* Level 2 IRQs: power button */
pub const BCOVE_PBIRQ_PBTN: u32 = 1 << 0;
pub const BCOVE_PBIRQ_UBTN: u32 = 1 << 1;

/* Level 2 IRQs: ADC */
pub const BCOVE_ADCIRQ_BATTEMP: u32 = 1 << 2;
pub const BCOVE_ADCIRQ_SYSTEMP: u32 = 1 << 3;
pub const BCOVE_ADCIRQ_BATTID: u32 = 1 << 4;
pub const BCOVE_ADCIRQ_VIBATT: u32 = 1 << 5;
pub const BCOVE_ADCIRQ_CCTICK: u32 = 1 << 7;

/* Level 2 IRQs: charger */
pub const BCOVE_CHGRIRQ_BAT0ALRT: u32 = 1 << 4;
pub const BCOVE_CHGRIRQ_BAT1ALRT: u32 = 1 << 5;
pub const BCOVE_CHGRIRQ_BATCRIT: u32 = 1 << 6;

pub const BCOVE_CHGRIRQ_VBUSDET: u32 = 1 << 0;
pub const BCOVE_CHGRIRQ_DCDET: u32 = 1 << 1;
pub const BCOVE_CHGRIRQ_BATTDET: u32 = 1 << 2;
pub const BCOVE_CHGRIRQ_USBIDDET: u32 = 1 << 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
