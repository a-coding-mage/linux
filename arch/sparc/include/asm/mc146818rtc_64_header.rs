/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Machine dependent access functions for RTC registers.
 */

// Dependency supplied by <asm/io.h> in the C source.
unsafe extern "C" {
    pub fn outb_p(value: u8, port: usize);
    pub fn inb_p(port: usize) -> u8;
}

// When RTC_PORT is not supplied by the including translation unit:
unsafe extern "C" {
    pub static mut cmos_regs: usize;
}

#[inline]
pub unsafe fn rtc_port(x: usize) -> usize {
    cmos_regs.wrapping_add(x)
}

pub const RTC_ALWAYS_BCD: i32 = 0;

/*
 * The yet supported machines all access the RTC index register via
 * an ISA port access but the way to access the date register differs ...
 */
#[inline]
pub unsafe fn CMOS_READ(addr: u8) -> u8 {
    outb_p(addr, rtc_port(0));
    inb_p(rtc_port(1))
}

#[inline]
pub unsafe fn CMOS_WRITE(val: u8, addr: u8) {
    outb_p(addr, rtc_port(0));
    outb_p(val, rtc_port(1));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
