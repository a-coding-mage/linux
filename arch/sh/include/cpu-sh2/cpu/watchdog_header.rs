/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh2/watchdog.h
 *
 * Copyright (C) 2002, 2003 Paul Mundt
 */

/*
 * More SH-2 brilliance .. its not good enough that we can't read
 * and write the same sizes to WTCNT, now we have to read and write
 * with different sizes at different addresses for WTCNT _and_ RSTCSR.
 *
 * At least on the bright side no one has managed to screw over WTCSR
 * in this fashion .. yet.
 */
/* Register definitions */
pub const WTCNT: usize = 0xffff_fe80;
pub const WTCSR: usize = 0xffff_fe80;
pub const RSTCSR: usize = 0xffff_fe82;

pub const WTCNT_R: usize = WTCNT + 1;
pub const RSTCSR_R: usize = RSTCSR + 1;

/* Bit definitions */
pub const WTCSR_IOVF: u8 = 0x80;
pub const WTCSR_WT: u8 = 0x40;
pub const WTCSR_TME: u8 = 0x20;
pub const WTCSR_RSTS: u8 = 0x00;

pub const RSTCSR_RSTS: u8 = 0x20;

unsafe extern "C" {
    fn __raw_readb(addr: usize) -> u8;
    fn __raw_writeb(value: u16, addr: usize);
}

/**
 * 	sh_wdt_read_rstcsr - Read from Reset Control/Status Register
 *
 *	Reads back the RSTCSR value.
 */
#[inline]
pub unsafe fn sh_wdt_read_rstcsr() -> u8 {
    /*
     * Same read/write brain-damage as for WTCNT here..
     */
    unsafe { __raw_readb(RSTCSR_R) }
}

/**
 * 	sh_wdt_write_csr - Write to Reset Control/Status Register
 *
 * 	@val: Value to write
 *
 * 	Writes the given value @val to the lower byte of the control/status
 * 	register. The upper byte is set manually on each write.
 */
#[inline]
pub unsafe fn sh_wdt_write_rstcsr(val: u8) {
    /*
     * Note: Due to the brain-damaged nature of this register,
     * we can't presently touch the WOVF bit, since the upper byte
     * has to be swapped for this. So just leave it alone..
     */
    unsafe { __raw_writeb((WTCNT_HIGH << 8) | (val as u16), RSTCSR) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
