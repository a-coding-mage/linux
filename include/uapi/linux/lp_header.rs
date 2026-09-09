/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * usr/include/linux/lp.h c.1991-1992 James Wiegand
 * many modifications copyright (C) 1992 Michael K. Johnson
 * Interrupt support added 1993 Nigel Gamble
 * Removed 8255 status defines from inside __KERNEL__ Marcelo Tosatti
 */

/* Per POSIX guidelines, this module reserves the LP and lp prefixes. */
pub const LP_EXIST: u32 = 0x0001;
pub const LP_SELEC: u32 = 0x0002;
pub const LP_BUSY: u32 = 0x0004;
pub const LP_BUSY_BIT_POS: u32 = 2;
pub const LP_OFFL: u32 = 0x0008;
pub const LP_NOPA: u32 = 0x0010;
pub const LP_ERR: u32 = 0x0020;
pub const LP_ABORT: u32 = 0x0040;
pub const LP_CAREFUL: u32 = 0x0080; /* obsoleted -arca */
pub const LP_ABORTOPEN: u32 = 0x0100;

pub const LP_TRUST_IRQ_: u32 = 0x0200; /* obsolete */
pub const LP_NO_REVERSE: u32 = 0x0400; /* No reverse mode available. */
pub const LP_DATA_AVAIL: u32 = 0x0800; /* Data is available. */

/* Bit definitions for the 8255 status port, base + 1. */
pub const LP_PBUSY: u32 = 0x80; /* inverted input, active high */
pub const LP_PACK: u32 = 0x40; /* unchanged input, active low */
pub const LP_POUTPA: u32 = 0x20; /* unchanged input, active high */
pub const LP_PSELECD: u32 = 0x10; /* unchanged input, active high */
pub const LP_PERRORP: u32 = 0x08; /* unchanged input, active low */

pub const LP_INIT_CHAR: u32 = 1000;
pub const LP_INIT_WAIT: u32 = 1;
pub const LP_INIT_TIME: u32 = 2;

/* IOCTL numbers */
pub const LPCHAR: u32 = 0x0601;
pub const LPTIME: u32 = 0x0602;
pub const LPABORT: u32 = 0x0604;
pub const LPSETIRQ: u32 = 0x0605;
pub const LPGETIRQ: u32 = 0x0606;
pub const LPWAIT: u32 = 0x0608;
pub const LPCAREFUL: u32 = 0x0609;
pub const LPABORTOPEN: u32 = 0x060a;
pub const LPGETSTATUS: u32 = 0x060b;
pub const LPRESET: u32 = 0x060c;

#[cfg(feature = "LP_STATS")]
pub const LPGETSTATS: u32 = 0x060d;

pub const LPGETFLAGS: u32 = 0x060e;
pub const LPSETTIMEOUT_OLD: u32 = 0x060f;

/* _IOW(0x6, 0xf, __s64[2]); set parport timeout. */
pub const LPSETTIMEOUT_NEW: u32 = crate::_IOW!(0x6, 0xf, [i64; 2]);

#[cfg(target_pointer_width = "64")]
pub const LPSETTIMEOUT: u32 = LPSETTIMEOUT_OLD;

/* On non-64-bit targets, preserve the original time_t/kernel-long conditional. */
#[cfg(not(target_pointer_width = "64"))]
pub const LPSETTIMEOUT: u32 = LPSETTIMEOUT_OLD;

/* HZ is supplied by the surrounding kernel bindings. */
pub const LP_TIMEOUT_INTERRUPT: u32 = 60 * HZ;
pub const LP_TIMEOUT_POLLED: u32 = 10 * HZ;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
