/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 1999,2000 MIPS Technologies, Inc.  All rights reserved.
 * Copyright (C) 2003 by Ralf Baechle
 *
 * RTC routines for Malta style attached PIIX4 device, which contains a
 * Motorola MC146818A-compatible Real Time Clock.
 */

// Dependencies supplied by the surrounding platform headers:
// asm/io.h, asm/mips-boards/generic.h, asm/mips-boards/malta.h

#[inline]
pub const fn RTC_PORT(x: usize) -> usize {
    0x70usize.wrapping_add(x)
}

pub const RTC_IRQ: usize = 8;

#[inline]
pub unsafe fn CMOS_READ(addr: usize) -> u8 {
    unsafe {
        outb(addr as u8, MALTA_RTC_ADR_REG);
        inb(MALTA_RTC_DAT_REG)
    }
}

#[inline]
pub unsafe fn CMOS_WRITE(data: u8, addr: usize) {
    unsafe {
        outb(addr as u8, MALTA_RTC_ADR_REG);
        outb(data, MALTA_RTC_DAT_REG);
    }
}

pub const RTC_ALWAYS_BCD: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
