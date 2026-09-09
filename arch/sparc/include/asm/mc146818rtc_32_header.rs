/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Machine dependent access functions for RTC registers.
 */

// Dependency supplied by asm/io.h in the original source.
unsafe extern "C" {
    pub fn outb_p(value: u8, port: u16);
    pub fn inb_p(port: u16) -> u8;
}

// RTC_PORT may be supplied by the build configuration in place of this definition.
#[inline]
pub const fn RTC_PORT(x: u16) -> u16 {
    0x70u16.wrapping_add(x)
}

// RTC operates in binary mode.
pub const RTC_ALWAYS_BCD: i32 = 1;

/*
 * The yet supported machines all access the RTC index register via
 * an ISA port access but the way to access the date register differs ...
 */
#[inline]
pub unsafe fn CMOS_READ(addr: u8) -> u8 {
    unsafe {
        outb_p(addr, RTC_PORT(0));
        inb_p(RTC_PORT(1))
    }
}

#[inline]
pub unsafe fn CMOS_WRITE(val: u8, addr: u8) {
    unsafe {
        outb_p(addr, RTC_PORT(0));
        outb_p(val, RTC_PORT(1));
    }
}

pub const RTC_IRQ: i32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
