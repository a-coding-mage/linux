/* SPDX-License-Identifier: GPL-2.0+
 *
 * include/asm-sh/watchdog.h
 *
 * Copyright (C) 2002, 2003 Paul Mundt
 * Copyright (C) 2009 Siemens AG
 * Copyright (C) 2009 Valentin Sitdikov
 */

// Dependencies supplied by the surrounding kernel translation.

pub const WTCNT_HIGH: u32 = 0x5a;
pub const WTCSR_HIGH: u32 = 0xa5;

pub const WTCSR_CKS2: u32 = 0x04;
pub const WTCSR_CKS1: u32 = 0x02;
pub const WTCSR_CKS0: u32 = 0x01;

// See cpu-sh2/watchdog.h for explanation of this stupidity..
// WTCNT_R defaults to WTCNT when not supplied by the CPU-specific header.
// WTCSR_R defaults to WTCSR when not supplied by the CPU-specific header.

/*
 * CKS0-2 supports a number of clock division ratios. At the time the watchdog
 * is enabled, it defaults to a 41 usec overflow period .. we overload this to
 * something a little more reasonable, and really can't deal with anything
 * lower than WTCSR_CKS_1024, else we drop back into the usec range.
 *
 * Clock Division Ratio         Overflow Period
 * --------------------------------------------
 *     1/32 (initial value)       41 usecs
 *     1/64                       82 usecs
 *     1/128                     164 usecs
 *     1/256                     328 usecs
 *     1/512                     656 usecs
 *     1/1024                   1.31 msecs
 *     1/2048                   2.62 msecs
 *     1/4096                   5.25 msecs
 */
pub const WTCSR_CKS_32: u32 = 0x00;
pub const WTCSR_CKS_64: u32 = 0x01;
pub const WTCSR_CKS_128: u32 = 0x02;
pub const WTCSR_CKS_256: u32 = 0x03;
pub const WTCSR_CKS_512: u32 = 0x04;
pub const WTCSR_CKS_1024: u32 = 0x05;
pub const WTCSR_CKS_2048: u32 = 0x06;
pub const WTCSR_CKS_4096: u32 = 0x07;

#[cfg(any(CONFIG_CPU_SUBTYPE_SH7785, CONFIG_CPU_SUBTYPE_SH7780))]
/// sh_wdt_read_cnt - Read from Counter
/// Reads back the WTCNT value.
#[inline]
pub unsafe fn sh_wdt_read_cnt() -> u32 {
    __raw_readl(WTCNT_R)
}

#[cfg(any(CONFIG_CPU_SUBTYPE_SH7785, CONFIG_CPU_SUBTYPE_SH7780))]
/// sh_wdt_write_cnt - Write to Counter
/// @val: Value to write
///
/// Writes the given value @val to the lower byte of the timer counter.
/// The upper byte is set manually on each write.
#[inline]
pub unsafe fn sh_wdt_write_cnt(val: u32) {
    __raw_writel((WTCNT_HIGH << 24) | val as u32, WTCNT);
}

#[cfg(any(CONFIG_CPU_SUBTYPE_SH7785, CONFIG_CPU_SUBTYPE_SH7780))]
/// sh_wdt_write_bst - Write to Counter
/// @val: Value to write
///
/// Writes the given value @val to the lower byte of the timer counter.
/// The upper byte is set manually on each write.
#[inline]
pub unsafe fn sh_wdt_write_bst(val: u32) {
    __raw_writel((WTBST_HIGH << 24) | val as u32, WTBST);
}

#[cfg(any(CONFIG_CPU_SUBTYPE_SH7785, CONFIG_CPU_SUBTYPE_SH7780))]
/// sh_wdt_read_csr - Read from Control/Status Register
/// Reads back the WTCSR value.
#[inline]
pub unsafe fn sh_wdt_read_csr() -> u32 {
    __raw_readl(WTCSR_R)
}

#[cfg(any(CONFIG_CPU_SUBTYPE_SH7785, CONFIG_CPU_SUBTYPE_SH7780))]
/// sh_wdt_write_csr - Write to Control/Status Register
/// @val: Value to write
///
/// Writes the given value @val to the lower byte of the control/status
/// register. The upper byte is set manually on each write.
#[inline]
pub unsafe fn sh_wdt_write_csr(val: u32) {
    __raw_writel((WTCSR_HIGH << 24) | val as u32, WTCSR);
}

#[cfg(not(any(CONFIG_CPU_SUBTYPE_SH7785, CONFIG_CPU_SUBTYPE_SH7780)))]
/// sh_wdt_read_cnt - Read from Counter
/// Reads back the WTCNT value.
#[inline]
pub unsafe fn sh_wdt_read_cnt() -> u8 {
    __raw_readb(WTCNT_R)
}

#[cfg(not(any(CONFIG_CPU_SUBTYPE_SH7785, CONFIG_CPU_SUBTYPE_SH7780)))]
/// sh_wdt_write_cnt - Write to Counter
/// @val: Value to write
///
/// Writes the given value @val to the lower byte of the timer counter.
/// The upper byte is set manually on each write.
#[inline]
pub unsafe fn sh_wdt_write_cnt(val: u8) {
    __raw_writew((WTCNT_HIGH << 8) | val as u16 as u32, WTCNT);
}

#[cfg(not(any(CONFIG_CPU_SUBTYPE_SH7785, CONFIG_CPU_SUBTYPE_SH7780)))]
/// sh_wdt_read_csr - Read from Control/Status Register
/// Reads back the WTCSR value.
#[inline]
pub unsafe fn sh_wdt_read_csr() -> u8 {
    __raw_readb(WTCSR_R)
}

#[cfg(not(any(CONFIG_CPU_SUBTYPE_SH7785, CONFIG_CPU_SUBTYPE_SH7780)))]
/// sh_wdt_write_csr - Write to Control/Status Register
/// @val: Value to write
///
/// Writes the given value @val to the lower byte of the control/status
/// register. The upper byte is set manually on each write.
#[inline]
pub unsafe fn sh_wdt_write_csr(val: u8) {
    __raw_writew((WTCSR_HIGH << 8) | val as u16 as u32, WTCSR);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
