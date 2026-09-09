/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  CLPS711X system register bits definitions
 *
 *  Copyright (C) 2013 Alexander Shiyan <shc_work@mail.ru>
 */

pub const SYSCON_OFFSET: u32 = 0x00;
pub const SYSFLG_OFFSET: u32 = 0x40;

#[inline]
pub const fn SYSCON1_KBDSCAN(x: u32) -> u32 { x & 15 }
pub const SYSCON1_KBDSCAN_MASK: u32 = 15;
pub const SYSCON1_TC1M: u32 = 1 << 4;
pub const SYSCON1_TC1S: u32 = 1 << 5;
pub const SYSCON1_TC2M: u32 = 1 << 6;
pub const SYSCON1_TC2S: u32 = 1 << 7;
pub const SYSCON1_BZTOG: u32 = 1 << 9;
pub const SYSCON1_BZMOD: u32 = 1 << 10;
pub const SYSCON1_DBGEN: u32 = 1 << 11;
pub const SYSCON1_LCDEN: u32 = 1 << 12;
pub const SYSCON1_CDENTX: u32 = 1 << 13;
pub const SYSCON1_CDENRX: u32 = 1 << 14;
pub const SYSCON1_SIREN: u32 = 1 << 15;
#[inline]
pub const fn SYSCON1_ADCKSEL(x: u32) -> u32 { (x & 3) << 16 }
pub const SYSCON1_ADCKSEL_MASK: u32 = 3 << 16;
pub const SYSCON1_EXCKEN: u32 = 1 << 18;
pub const SYSCON1_WAKEDIS: u32 = 1 << 19;
pub const SYSCON1_IRTXM: u32 = 1 << 20;

pub const SYSCON2_SERSEL: u32 = 1 << 0;
pub const SYSCON2_KBD6: u32 = 1 << 1;
pub const SYSCON2_DRAMZ: u32 = 1 << 2;
pub const SYSCON2_KBWEN: u32 = 1 << 3;
pub const SYSCON2_SS2TXEN: u32 = 1 << 4;
pub const SYSCON2_PCCARD1: u32 = 1 << 5;
pub const SYSCON2_PCCARD2: u32 = 1 << 6;
pub const SYSCON2_SS2RXEN: u32 = 1 << 7;
pub const SYSCON2_SS2MAEN: u32 = 1 << 9;
pub const SYSCON2_OSTB: u32 = 1 << 12;
pub const SYSCON2_CLKENSL: u32 = 1 << 13;
pub const SYSCON2_BUZFREQ: u32 = 1 << 14;

pub const SYSCON3_ADCCON: u32 = 1 << 0;
pub const SYSCON3_CLKCTL0: u32 = 1 << 1;
pub const SYSCON3_CLKCTL1: u32 = 1 << 2;
pub const SYSCON3_DAISEL: u32 = 1 << 3;
pub const SYSCON3_ADCCKNSEN: u32 = 1 << 4;
#[inline]
pub const fn SYSCON3_VERSN(x: u32) -> u32 { (x >> 5) & 7 }
pub const SYSCON3_VERSN_MASK: u32 = 7 << 5;
pub const SYSCON3_FASTWAKE: u32 = 1 << 8;
pub const SYSCON3_DAIEN: u32 = 1 << 9;
pub const SYSCON3_128FS: u32 = SYSCON3_DAIEN;
pub const SYSCON3_ENPD67: u32 = 1 << 10;

pub const SYSCON_UARTEN: u32 = 1 << 8;

pub const SYSFLG1_MCDR: u32 = 1 << 0;
pub const SYSFLG1_DCDET: u32 = 1 << 1;
pub const SYSFLG1_WUDR: u32 = 1 << 2;
pub const SYSFLG1_WUON: u32 = 1 << 3;
pub const SYSFLG1_CTS: u32 = 1 << 8;
pub const SYSFLG1_DSR: u32 = 1 << 9;
pub const SYSFLG1_DCD: u32 = 1 << 10;
pub const SYSFLG1_NBFLG: u32 = 1 << 12;
pub const SYSFLG1_RSTFLG: u32 = 1 << 13;
pub const SYSFLG1_PFFLG: u32 = 1 << 14;
pub const SYSFLG1_CLDFLG: u32 = 1 << 15;
pub const SYSFLG1_CRXFE: u32 = 1 << 24;
pub const SYSFLG1_CTXFF: u32 = 1 << 25;
pub const SYSFLG1_SSIBUSY: u32 = 1 << 26;
pub const SYSFLG1_ID: u32 = 1 << 29;
#[inline]
pub const fn SYSFLG1_VERID(x: u32) -> u32 { (x >> 30) & 3 }
pub const SYSFLG1_VERID_MASK: u32 = 3 << 30;

pub const SYSFLG2_SSRXOF: u32 = 1 << 0;
pub const SYSFLG2_RESVAL: u32 = 1 << 1;
pub const SYSFLG2_RESFRM: u32 = 1 << 2;
pub const SYSFLG2_SS2RXFE: u32 = 1 << 3;
pub const SYSFLG2_SS2TXFF: u32 = 1 << 4;
pub const SYSFLG2_SS2TXUF: u32 = 1 << 5;
pub const SYSFLG2_CKMODE: u32 = 1 << 6;

pub const SYSFLG_UBUSY: u32 = 1 << 11;
pub const SYSFLG_URXFE: u32 = 1 << 22;
pub const SYSFLG_UTXFF: u32 = 1 << 23;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
