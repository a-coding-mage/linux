/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Definitions for async_struct (and serial_struct) flags field also
 * shared by the tty_port flags structures.
 *
 * Define ASYNCB_* for convenient use with {test,set,clear}_bit.
 *
 * Bits [0..ASYNCB_LAST_USER] are userspace defined/visible/changeable
 * [x] in the bit comments indicates the flag is defunct and no longer used.
 */
pub const ASYNCB_HUP_NOTIFY: u32 = 0;
pub const ASYNCB_FOURPORT: u32 = 1;
pub const ASYNCB_SAK: u32 = 2;
pub const ASYNCB_SPLIT_TERMIOS: u32 = 3;
pub const ASYNCB_SPD_HI: u32 = 4;
pub const ASYNCB_SPD_VHI: u32 = 5;
pub const ASYNCB_SKIP_TEST: u32 = 6;
pub const ASYNCB_AUTO_IRQ: u32 = 7;
pub const ASYNCB_SESSION_LOCKOUT: u32 = 8;
pub const ASYNCB_PGRP_LOCKOUT: u32 = 9;
pub const ASYNCB_CALLOUT_NOHUP: u32 = 10;
pub const ASYNCB_HARDPPS_CD: u32 = 11;
pub const ASYNCB_SPD_SHI: u32 = 12;
pub const ASYNCB_LOW_LATENCY: u32 = 13;
pub const ASYNCB_BUGGY_UART: u32 = 14;
pub const ASYNCB_AUTOPROBE: u32 = 15;
pub const ASYNCB_MAGIC_MULTIPLIER: u32 = 16;
pub const ASYNCB_LAST_USER: u32 = 16;

/*
 * Internal flags used only by kernel (read-only)
 *
 * WARNING: These flags are no longer used and have been superceded by the
 *           TTY_PORT_ flags in the iflags field (and not userspace-visible)
 *
 * These declarations are present when __KERNEL__ is not defined in C.
 */
pub const ASYNCB_INITIALIZED: u32 = 31;
pub const ASYNCB_SUSPENDED: u32 = 30;
pub const ASYNCB_NORMAL_ACTIVE: u32 = 29;
pub const ASYNCB_BOOT_AUTOCONF: u32 = 28;
pub const ASYNCB_CLOSING: u32 = 27;
pub const ASYNCB_CTS_FLOW: u32 = 26;
pub const ASYNCB_CHECK_CD: u32 = 25;
pub const ASYNCB_SHARE_IRQ: u32 = 24;
pub const ASYNCB_CONS_FLOW: u32 = 23;
pub const ASYNCB_FIRST_KERNEL: u32 = 22;

/* Masks */
pub const ASYNC_HUP_NOTIFY: u32 = 1u32 << ASYNCB_HUP_NOTIFY;
pub const ASYNC_SUSPENDED: u32 = 1u32 << ASYNCB_SUSPENDED;
pub const ASYNC_FOURPORT: u32 = 1u32 << ASYNCB_FOURPORT;
pub const ASYNC_SAK: u32 = 1u32 << ASYNCB_SAK;
pub const ASYNC_SPLIT_TERMIOS: u32 = 1u32 << ASYNCB_SPLIT_TERMIOS;
pub const ASYNC_SPD_HI: u32 = 1u32 << ASYNCB_SPD_HI;
pub const ASYNC_SPD_VHI: u32 = 1u32 << ASYNCB_SPD_VHI;
pub const ASYNC_SKIP_TEST: u32 = 1u32 << ASYNCB_SKIP_TEST;
pub const ASYNC_AUTO_IRQ: u32 = 1u32 << ASYNCB_AUTO_IRQ;
pub const ASYNC_SESSION_LOCKOUT: u32 = 1u32 << ASYNCB_SESSION_LOCKOUT;
pub const ASYNC_PGRP_LOCKOUT: u32 = 1u32 << ASYNCB_PGRP_LOCKOUT;
pub const ASYNC_CALLOUT_NOHUP: u32 = 1u32 << ASYNCB_CALLOUT_NOHUP;
pub const ASYNC_HARDPPS_CD: u32 = 1u32 << ASYNCB_HARDPPS_CD;
pub const ASYNC_SPD_SHI: u32 = 1u32 << ASYNCB_SPD_SHI;
pub const ASYNC_LOW_LATENCY: u32 = 1u32 << ASYNCB_LOW_LATENCY;
pub const ASYNC_BUGGY_UART: u32 = 1u32 << ASYNCB_BUGGY_UART;
pub const ASYNC_AUTOPROBE: u32 = 1u32 << ASYNCB_AUTOPROBE;
pub const ASYNC_MAGIC_MULTIPLIER: u32 = 1u32 << ASYNCB_MAGIC_MULTIPLIER;

pub const ASYNC_FLAGS: u32 = (1u32 << (ASYNCB_LAST_USER + 1)) - 1;
pub const ASYNC_DEPRECATED: u32 = ASYNC_SPLIT_TERMIOS
    | ASYNC_SESSION_LOCKOUT
    | ASYNC_PGRP_LOCKOUT
    | ASYNC_CALLOUT_NOHUP
    | ASYNC_AUTOPROBE;
pub const ASYNC_USR_MASK: u32 = ASYNC_SPD_MASK | ASYNC_CALLOUT_NOHUP | ASYNC_LOW_LATENCY;
pub const ASYNC_SPD_CUST: u32 = ASYNC_SPD_HI | ASYNC_SPD_VHI;
pub const ASYNC_SPD_WARP: u32 = ASYNC_SPD_HI | ASYNC_SPD_SHI;
pub const ASYNC_SPD_MASK: u32 = ASYNC_SPD_HI | ASYNC_SPD_VHI | ASYNC_SPD_SHI;

/* These flags are no longer used (and were always masked from userspace). */
pub const ASYNC_INITIALIZED: u32 = 1u32 << ASYNCB_INITIALIZED;
pub const ASYNC_NORMAL_ACTIVE: u32 = 1u32 << ASYNCB_NORMAL_ACTIVE;
pub const ASYNC_BOOT_AUTOCONF: u32 = 1u32 << ASYNCB_BOOT_AUTOCONF;
pub const ASYNC_CLOSING: u32 = 1u32 << ASYNCB_CLOSING;
pub const ASYNC_CTS_FLOW: u32 = 1u32 << ASYNCB_CTS_FLOW;
pub const ASYNC_CHECK_CD: u32 = 1u32 << ASYNCB_CHECK_CD;
pub const ASYNC_SHARE_IRQ: u32 = 1u32 << ASYNCB_SHARE_IRQ;
pub const ASYNC_CONS_FLOW: u32 = 1u32 << ASYNCB_CONS_FLOW;
pub const ASYNC_INTERNAL_FLAGS: u32 = !((1u32 << ASYNCB_FIRST_KERNEL) - 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
