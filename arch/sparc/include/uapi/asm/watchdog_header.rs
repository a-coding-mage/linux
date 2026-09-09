/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *
 * watchdog - Driver interface for the hardware watchdog timers
 * present on Sun Microsystems boardsets
 *
 * Copyright (c) 2000 Eric Brower <ebrower@usa.net>
 *
 */

// Dependency: the Linux watchdog interface supplies `WATCHDOG_IOCTL_BASE`,
// `_IO`, and `_IOR` with their target-specific ioctl encodings.

/* Solaris compatibility ioctls--
 * Ref. <linux/watchdog.h> for standard linux watchdog ioctls
 */
pub const WIOCSTART: usize = _IO(WATCHDOG_IOCTL_BASE, 10); // Start Timer
pub const WIOCSTOP: usize = _IO(WATCHDOG_IOCTL_BASE, 11); // Stop Timer
pub const WIOCGSTAT: usize = _IOR(WATCHDOG_IOCTL_BASE, 12, core::ffi::c_int); // Get Timer Status

/* Status flags from WIOCGSTAT ioctl
 */
pub const WD_FREERUN: u32 = 0x01; // timer is running, interrupts disabled
pub const WD_EXPIRED: u32 = 0x02; // timer has expired
pub const WD_RUNNING: u32 = 0x04; // timer is running, interrupts enabled
pub const WD_STOPPED: u32 = 0x08; // timer has not been started
pub const WD_SERVICED: u32 = 0x10; // timer interrupt was serviced

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
