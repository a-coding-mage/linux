/* SPDX-License-Identifier: GPL-2.0 */
/*
 * File:	m54xxgpt.h
 * Purpose:	Register and bit definitions for the MCF54XX
 */

/* General Purpose Timers (GPT) */

/* Register read/write definitions. MCF_MBAR is supplied by another header. */
pub const MCF_GPT_GMS0: usize = MCF_MBAR + 0x000800;
pub const MCF_GPT_GCIR0: usize = MCF_MBAR + 0x000804;
pub const MCF_GPT_GPWM0: usize = MCF_MBAR + 0x000808;
pub const MCF_GPT_GSR0: usize = MCF_MBAR + 0x00080C;
pub const MCF_GPT_GMS1: usize = MCF_MBAR + 0x000810;
pub const MCF_GPT_GCIR1: usize = MCF_MBAR + 0x000814;
pub const MCF_GPT_GPWM1: usize = MCF_MBAR + 0x000818;
pub const MCF_GPT_GSR1: usize = MCF_MBAR + 0x00081C;
pub const MCF_GPT_GMS2: usize = MCF_MBAR + 0x000820;
pub const MCF_GPT_GCIR2: usize = MCF_MBAR + 0x000824;
pub const MCF_GPT_GPWM2: usize = MCF_MBAR + 0x000828;
pub const MCF_GPT_GSR2: usize = MCF_MBAR + 0x00082C;
pub const MCF_GPT_GMS3: usize = MCF_MBAR + 0x000830;
pub const MCF_GPT_GCIR3: usize = MCF_MBAR + 0x000834;
pub const MCF_GPT_GPWM3: usize = MCF_MBAR + 0x000838;
pub const MCF_GPT_GSR3: usize = MCF_MBAR + 0x00083C;

pub const fn MCF_GPT_GMS(x: usize) -> usize { MCF_MBAR + 0x000800 + x * 0x010 }
pub const fn MCF_GPT_GCIR(x: usize) -> usize { MCF_MBAR + 0x000804 + x * 0x010 }
pub const fn MCF_GPT_GPWM(x: usize) -> usize { MCF_MBAR + 0x000808 + x * 0x010 }
pub const fn MCF_GPT_GSR(x: usize) -> usize { MCF_MBAR + 0x00080C + x * 0x010 }

pub const fn MCF_GPT_GMS_TMS(x: u32) -> u32 { (x & 0x00000007) << 0 }
pub const fn MCF_GPT_GMS_GPIO(x: u32) -> u32 { (x & 0x00000003) << 4 }
pub const MCF_GPT_GMS_IEN: u32 = 0x00000100;
pub const MCF_GPT_GMS_OD: u32 = 0x00000200;
pub const MCF_GPT_GMS_SC: u32 = 0x00000400;
pub const MCF_GPT_GMS_CE: u32 = 0x00001000;
pub const MCF_GPT_GMS_WDEN: u32 = 0x00008000;
pub const fn MCF_GPT_GMS_ICT(x: u32) -> u32 { (x & 0x00000003) << 16 }
pub const fn MCF_GPT_GMS_OCT(x: u32) -> u32 { (x & 0x00000003) << 20 }
pub const fn MCF_GPT_GMS_OCPW(x: u32) -> u32 { (x & 0x000000FF) << 24 }
pub const MCF_GPT_GMS_OCT_FRCLOW: u32 = 0x00000000;
pub const MCF_GPT_GMS_OCT_PULSEHI: u32 = 0x00100000;
pub const MCF_GPT_GMS_OCT_PULSELO: u32 = 0x00200000;
pub const MCF_GPT_GMS_OCT_TOGGLE: u32 = 0x00300000;
pub const MCF_GPT_GMS_ICT_ANY: u32 = 0x00000000;
pub const MCF_GPT_GMS_ICT_RISE: u32 = 0x00010000;
pub const MCF_GPT_GMS_ICT_FALL: u32 = 0x00020000;
pub const MCF_GPT_GMS_ICT_PULSE: u32 = 0x00030000;
pub const MCF_GPT_GMS_GPIO_INPUT: u32 = 0x00000000;
pub const MCF_GPT_GMS_GPIO_OUTLO: u32 = 0x00000020;
pub const MCF_GPT_GMS_GPIO_OUTHI: u32 = 0x00000030;
pub const MCF_GPT_GMS_GPIO_MASK: u32 = 0x00000030;
pub const MCF_GPT_GMS_TMS_DISABLE: u32 = 0x00000000;
pub const MCF_GPT_GMS_TMS_INCAPT: u32 = 0x00000001;
pub const MCF_GPT_GMS_TMS_OUTCAPT: u32 = 0x00000002;
pub const MCF_GPT_GMS_TMS_PWM: u32 = 0x00000003;
pub const MCF_GPT_GMS_TMS_GPIO: u32 = 0x00000004;
pub const MCF_GPT_GMS_TMS_MASK: u32 = 0x00000007;

pub const fn MCF_GPT_GCIR_CNT(x: u32) -> u32 { (x & 0x0000FFFF) << 0 }
pub const fn MCF_GPT_GCIR_PRE(x: u32) -> u32 { (x & 0x0000FFFF) << 16 }

pub const MCF_GPT_GPWM_LOAD: u32 = 0x00000001;
pub const MCF_GPT_GPWM_PWMOP: u32 = 0x00000100;
pub const fn MCF_GPT_GPWM_WIDTH(x: u32) -> u32 { (x & 0x0000FFFF) << 16 }

pub const MCF_GPT_GSR_CAPT: u32 = 0x00000001;
pub const MCF_GPT_GSR_COMP: u32 = 0x00000002;
pub const MCF_GPT_GSR_PWMP: u32 = 0x00000004;
pub const MCF_GPT_GSR_TEXP: u32 = 0x00000008;
pub const MCF_GPT_GSR_PIN: u32 = 0x00000100;
pub const fn MCF_GPT_GSR_OVF(x: u32) -> u32 { (x & 0x00000007) << 12 }
pub const fn MCF_GPT_GSR_CAPTURE(x: u32) -> u32 { (x & 0x0000FFFF) << 16 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
