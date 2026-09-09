/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Machine dependent access functions for RTC registers.
 */

// The C definition is `BUILD_BUG_ON(1)`, which intentionally fails at build
// time when RTC_IRQ is used.  Preserve that intent as a deliberately failing
// constant expression.
pub const RTC_IRQ: bool = false && panic!("BUILD_BUG_ON(1)");

// Defaulted in the C header when RTC_PORT has not been supplied by the build.
pub const RTC_ALWAYS_BCD: i32 = 1; // RTC operates in binary mode

#[inline(always)]
pub const fn RTC_PORT(x: u8) -> u8 {
    0x70u8.wrapping_add(x)
}

extern "C" {
    pub fn outb_p(value: u8, port: u8);
    pub fn inb_p(port: u8) -> u8;
}

/*
 * The yet supported machines all access the RTC index register via
 * an ISA port access but the way to access the date register differs ...
 */
#[inline(always)]
pub unsafe fn CMOS_READ(addr: u8) -> u8 {
    outb_p(addr, RTC_PORT(0));
    inb_p(RTC_PORT(1))
}

#[inline(always)]
pub unsafe fn CMOS_WRITE(val: u8, addr: u8) {
    outb_p(addr, RTC_PORT(0));
    outb_p(val, RTC_PORT(1));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
