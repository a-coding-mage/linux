/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Machine dependent access functions for RTC registers.
 *
 * The declarations below are applicable when building the kernel
 * (__KERNEL__). The original C header included <asm/io.h>; the I/O symbols
 * are intentionally left as external dependencies here.
 */

#[cfg(kernel)]
extern "C" {
    pub fn outb_p(value: u8, port: u16);
    pub fn inb_p(port: u16) -> u8;
}

#[cfg(kernel)]
#[inline]
pub const fn RTC_PORT(x: u16) -> u16 {
    0x70u16.wrapping_add(x)
}

#[cfg(kernel)]
pub const RTC_ALWAYS_BCD: i32 = 1; /* RTC operates in binary mode */

#[cfg(kernel)]
#[inline]
pub unsafe fn CMOS_READ(addr: u16) -> u8 {
    outb_p(addr as u8, RTC_PORT(0));
    inb_p(RTC_PORT(1))
}

#[cfg(kernel)]
#[inline]
pub unsafe fn CMOS_WRITE(val: u8, addr: u16) {
    outb_p(addr as u8, RTC_PORT(0));
    outb_p(val, RTC_PORT(1));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
