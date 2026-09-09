/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Machine dependent access functions for RTC registers.
 */

/* The following declarations are active when CONFIG_ATARI is enabled. */
#[cfg(feature = "CONFIG_ATARI")]
/* RTC in Atari machines */
/* Dependency corresponding to <asm/atarihw.h>. */

#[cfg(feature = "CONFIG_ATARI")]
extern "C" {
    pub static TT_RTC_BAS: usize;
    pub fn atari_outb_p(value: u8, port: usize);
    pub fn atari_inb_p(port: usize) -> u8;
}

#[cfg(feature = "CONFIG_ATARI")]
#[inline]
pub unsafe fn ATARI_RTC_PORT(x: usize) -> usize {
    TT_RTC_BAS + 2 * x
}

#[cfg(feature = "CONFIG_ATARI")]
pub const RTC_ALWAYS_BCD: i32 = 0;

#[cfg(feature = "CONFIG_ATARI")]
#[inline]
pub unsafe fn CMOS_READ(addr: u8) -> u8 {
    atari_outb_p(addr, ATARI_RTC_PORT(0));
    atari_inb_p(ATARI_RTC_PORT(1))
}

#[cfg(feature = "CONFIG_ATARI")]
#[inline]
pub unsafe fn CMOS_WRITE(val: u8, addr: u8) {
    atari_outb_p(addr, ATARI_RTC_PORT(0));
    atari_outb_p(val, ATARI_RTC_PORT(1));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
