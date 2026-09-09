/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 2004  Andriy Skulysh
 */

/* Translated from the SH3 ADC header. */

pub const ADDRAH: u32 = 0xa4000080;
pub const ADDRAL: u32 = 0xa4000082;
pub const ADDRBH: u32 = 0xa4000084;
pub const ADDRBL: u32 = 0xa4000086;
pub const ADDRCH: u32 = 0xa4000088;
pub const ADDRCL: u32 = 0xa400008a;
pub const ADDRDH: u32 = 0xa400008c;
pub const ADDRDL: u32 = 0xa400008e;
pub const ADCSR: u32 = 0xa4000090;

pub const ADCSR_ADF: u32 = 0x80;
pub const ADCSR_ADIE: u32 = 0x40;
pub const ADCSR_ADST: u32 = 0x20;
pub const ADCSR_MULTI: u32 = 0x10;
pub const ADCSR_CKS: u32 = 0x08;
pub const ADCSR_CH_MASK: u32 = 0x07;

pub const ADCR: u32 = 0xa4000092;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
