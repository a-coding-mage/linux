/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2010 Yong Shen <yong.shen@linaro.org>
 * Copyright 2009-2010 Pengutronix
 * Uwe Kleine-Koenig <u.kleine-koenig@pengutronix.de>
 */

// Dependency supplied by linux/mfd/mc13xxx.h in the original header.

pub const MC13783_REG_SW1A: i32 = 0;
pub const MC13783_REG_SW1B: i32 = 1;
pub const MC13783_REG_SW2A: i32 = 2;
pub const MC13783_REG_SW2B: i32 = 3;
pub const MC13783_REG_SW3: i32 = 4;
pub const MC13783_REG_PLL: i32 = 5;
pub const MC13783_REG_VAUDIO: i32 = 6;
pub const MC13783_REG_VIOHI: i32 = 7;
pub const MC13783_REG_VIOLO: i32 = 8;
pub const MC13783_REG_VDIG: i32 = 9;
pub const MC13783_REG_VGEN: i32 = 10;
pub const MC13783_REG_VRFDIG: i32 = 11;
pub const MC13783_REG_VRFREF: i32 = 12;
pub const MC13783_REG_VRFCP: i32 = 13;
pub const MC13783_REG_VSIM: i32 = 14;
pub const MC13783_REG_VESIM: i32 = 15;
pub const MC13783_REG_VCAM: i32 = 16;
pub const MC13783_REG_VRFBG: i32 = 17;
pub const MC13783_REG_VVIB: i32 = 18;
pub const MC13783_REG_VRF1: i32 = 19;
pub const MC13783_REG_VRF2: i32 = 20;
pub const MC13783_REG_VMMC1: i32 = 21;
pub const MC13783_REG_VMMC2: i32 = 22;
pub const MC13783_REG_GPO1: i32 = 23;
pub const MC13783_REG_GPO2: i32 = 24;
pub const MC13783_REG_GPO3: i32 = 25;
pub const MC13783_REG_GPO4: i32 = 26;
pub const MC13783_REG_V1: i32 = 27;
pub const MC13783_REG_V2: i32 = 28;
pub const MC13783_REG_V3: i32 = 29;
pub const MC13783_REG_V4: i32 = 30;
pub const MC13783_REG_PWGT1SPI: i32 = 31;
pub const MC13783_REG_PWGT2SPI: i32 = 32;

pub const MC13783_IRQ_ADCDONE: i32 = MC13XXX_IRQ_ADCDONE;
pub const MC13783_IRQ_ADCBISDONE: i32 = MC13XXX_IRQ_ADCBISDONE;
pub const MC13783_IRQ_TS: i32 = MC13XXX_IRQ_TS;
pub const MC13783_IRQ_WHIGH: i32 = 3;
pub const MC13783_IRQ_WLOW: i32 = 4;
pub const MC13783_IRQ_CHGDET: i32 = MC13XXX_IRQ_CHGDET;
pub const MC13783_IRQ_CHGOV: i32 = 7;
pub const MC13783_IRQ_CHGREV: i32 = MC13XXX_IRQ_CHGREV;
pub const MC13783_IRQ_CHGSHORT: i32 = MC13XXX_IRQ_CHGSHORT;
pub const MC13783_IRQ_CCCV: i32 = MC13XXX_IRQ_CCCV;
pub const MC13783_IRQ_CHGCURR: i32 = MC13XXX_IRQ_CHGCURR;
pub const MC13783_IRQ_BPON: i32 = MC13XXX_IRQ_BPON;
pub const MC13783_IRQ_LOBATL: i32 = MC13XXX_IRQ_LOBATL;
pub const MC13783_IRQ_LOBATH: i32 = MC13XXX_IRQ_LOBATH;
pub const MC13783_IRQ_UDP: i32 = 15;
pub const MC13783_IRQ_USB: i32 = 16;
pub const MC13783_IRQ_ID: i32 = 19;
pub const MC13783_IRQ_SE1: i32 = 21;
pub const MC13783_IRQ_CKDET: i32 = 22;
pub const MC13783_IRQ_UDM: i32 = 23;
pub const MC13783_IRQ_1HZ: i32 = MC13XXX_IRQ_1HZ;
pub const MC13783_IRQ_TODA: i32 = MC13XXX_IRQ_TODA;
pub const MC13783_IRQ_ONOFD1: i32 = 27;
pub const MC13783_IRQ_ONOFD2: i32 = 28;
pub const MC13783_IRQ_ONOFD3: i32 = 29;
pub const MC13783_IRQ_SYSRST: i32 = MC13XXX_IRQ_SYSRST;
pub const MC13783_IRQ_RTCRST: i32 = MC13XXX_IRQ_RTCRST;
pub const MC13783_IRQ_PC: i32 = MC13XXX_IRQ_PC;
pub const MC13783_IRQ_WARM: i32 = MC13XXX_IRQ_WARM;
pub const MC13783_IRQ_MEMHLD: i32 = MC13XXX_IRQ_MEMHLD;
pub const MC13783_IRQ_PWRRDY: i32 = 35;
pub const MC13783_IRQ_THWARNL: i32 = MC13XXX_IRQ_THWARNL;
pub const MC13783_IRQ_THWARNH: i32 = MC13XXX_IRQ_THWARNH;
pub const MC13783_IRQ_CLK: i32 = MC13XXX_IRQ_CLK;
pub const MC13783_IRQ_SEMAF: i32 = 39;
pub const MC13783_IRQ_MC2B: i32 = 41;
pub const MC13783_IRQ_HSDET: i32 = 42;
pub const MC13783_IRQ_HSL: i32 = 43;
pub const MC13783_IRQ_ALSPTH: i32 = 44;
pub const MC13783_IRQ_AHSSHORT: i32 = 45;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
