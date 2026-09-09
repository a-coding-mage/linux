/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998, 2001, 03 by Ralf Baechle
 * Copyright (C) 2007 Thomas Bogendoerfer
 *
 * RTC routines for Jazz style attached Dallas chip.
 */

// Dependency intent from <linux/delay.h>, <asm/io.h>, and <asm/jazz.h>.

pub const RTC_IRQ: i32 = 8;

#[inline]
pub const fn rtc_port(x: usize) -> usize {
    0x70usize.wrapping_add(x)
}

#[inline]
pub unsafe fn CMOS_READ(addr: usize) -> u8 {
    outb_p(addr as u8, rtc_port(0) as u16);
    core::ptr::read_volatile(JAZZ_RTC_BASE as *const u8)
}

#[inline]
pub unsafe fn CMOS_WRITE(data: u8, addr: usize) {
    outb_p(addr as u8, rtc_port(0) as u16);
    core::ptr::write_volatile(JAZZ_RTC_BASE as *mut u8, data);
}

pub const RTC_ALWAYS_BCD: i32 = 0;

// Supplied by the architecture I/O dependency.
unsafe extern "C" {
    fn outb_p(value: u8, port: u16);
}

// Supplied by the architecture Jazz dependency.
unsafe extern "C" {
    static JAZZ_RTC_BASE: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
