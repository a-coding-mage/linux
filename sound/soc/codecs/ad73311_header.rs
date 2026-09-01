// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * File:         sound/soc/codec/ad73311.h
 * Based on:
 * Author:       Cliff Cai <cliff.cai@analog.com>
 *
 * Created:      Thur Sep 25, 2008
 * Description:  definitions for AD73311 registers
 *
 * Modified:
 *               Copyright 2006 Analog Devices Inc.
 *
 * Bugs:         Enter bugs at http://blackfin.uclinux.org/
 */

pub const AD_CONTROL: u32 = 0x8000;
pub const AD_DATA: u32 = 0x0000;
pub const AD_READ: u32 = 0x4000;
pub const AD_WRITE: u32 = 0x0000;

/* Control register A */
pub const CTRL_REG_A: u32 = 0 << 8;

pub const REGA_MODE_PRO: u32 = 0x00;
pub const REGA_MODE_DATA: u32 = 0x01;
pub const REGA_MODE_MIXED: u32 = 0x03;
pub const REGA_DLB: u32 = 0x04;
pub const REGA_SLB: u32 = 0x08;
pub const fn REGA_DEVC(x: u32) -> u32 {
    (x & 0x7) << 4
}
pub const REGA_RESET: u32 = 0x80;

/* Control register B */
pub const CTRL_REG_B: u32 = 1 << 8;

pub const fn REGB_DIRATE(x: u32) -> u32 {
    x & 0x3
}
pub const fn REGB_SCDIV(x: u32) -> u32 {
    (x & 0x3) << 2
}
pub const fn REGB_MCDIV(x: u32) -> u32 {
    (x & 0x7) << 4
}
pub const REGB_CEE: u32 = 1 << 7;

/* Control register C */
pub const CTRL_REG_C: u32 = 2 << 8;

pub const REGC_PUDEV: u32 = 1 << 0;
pub const REGC_PUADC: u32 = 1 << 3;
pub const REGC_PUDAC: u32 = 1 << 4;
pub const REGC_PUREF: u32 = 1 << 5;
pub const REGC_REFUSE: u32 = 1 << 6;

/* Control register D */
pub const CTRL_REG_D: u32 = 3 << 8;

pub const fn REGD_IGS(x: u32) -> u32 {
    x & 0x7
}
pub const REGD_RMOD: u32 = 1 << 3;
pub const fn REGD_OGS(x: u32) -> u32 {
    (x & 0x7) << 4
}
pub const REGD_MUTE: u32 = 1 << 7;

/* Control register E */
pub const CTRL_REG_E: u32 = 4 << 8;

pub const fn REGE_DA(x: u32) -> u32 {
    x & 0x1f
}
pub const REGE_IBYP: u32 = 1 << 5;

/* Control register F */
pub const CTRL_REG_F: u32 = 5 << 8;

pub const REGF_SEEN: u32 = 1 << 5;
pub const REGF_INV: u32 = 1 << 6;
pub const REGF_ALB: u32 = 1 << 7;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
