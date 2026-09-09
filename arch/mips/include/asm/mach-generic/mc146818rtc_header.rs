/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998, 2001, 03 by Ralf Baechle
 *
 * RTC routines for PC style attached Dallas chip.
 */

// Dependency supplied by asm/io.h.
unsafe extern "C" {
    fn outb_p(value: u8, port: u16);
    fn inb_p(port: u16) -> u8;
}

#[inline]
pub const fn rtc_port(x: u16) -> u16 {
    0x70u16.wrapping_add(x)
}

pub const RTC_IRQ: u32 = 8;

#[inline]
pub unsafe fn CMOS_READ(addr: u64) -> u8 {
    unsafe {
        outb_p(addr as u8, rtc_port(0));
        inb_p(rtc_port(1))
    }
}

#[inline]
pub unsafe fn CMOS_WRITE(data: u8, addr: u64) {
    unsafe {
        outb_p(addr as u8, rtc_port(0));
        outb_p(data, rtc_port(1));
    }
}

pub const RTC_ALWAYS_BCD: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
