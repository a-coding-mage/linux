/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Intel Multimedia Timer device interface
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 2001-2004 Silicon Graphics, Inc.  All rights reserved.
 *
 * This file should define an interface compatible with the IA-PC Multimedia
 * Timers Draft Specification (rev. 0.97) from Intel.  Note that some
 * hardware may not be able to safely export its registers to userspace,
 * so the ioctl interface should support all necessary functionality.
 *
 * 11/01/01 - jbarnes - initial revision
 * 9/10/04 - Christoph Lameter - remove interrupt support
 * 9/17/04 - jbarnes - remove test program, move some #defines to the driver
 */

/*
 * Breakdown of the ioctl's available.  An 'optional' next to the command
 * indicates that supporting this command is optional, while 'required'
 * commands must be implemented if conformance is desired.
 *
 * MMTIMER_GETOFFSET - optional
 *   Should return the offset (relative to the start of the page where the
 *   registers are mapped) for the counter in question.
 *
 * MMTIMER_GETRES - required
 *   The resolution of the clock in femto (10^-15) seconds
 *
 * MMTIMER_GETFREQ - required
 *   Frequency of the clock in Hz
 *
 * MMTIMER_GETBITS - required
 *   Number of bits in the clock's counter
 *
 * MMTIMER_MMAPAVAIL - required
 *   Returns nonzero if the registers can be mmap'd into userspace, 0 otherwise
 *
 * MMTIMER_GETCOUNTER - required
 *   Gets the current value in the counter
 */
pub const MMTIMER_IOCTL_BASE: u32 = b'm' as u32;

/* `_IO` and `_IOR` are supplied by the target ioctl definitions. */
pub const MMTIMER_GETOFFSET: u32 = _IO!(MMTIMER_IOCTL_BASE, 0);
pub const MMTIMER_GETRES: u32 = _IOR!(MMTIMER_IOCTL_BASE, 1, ::core::ffi::c_ulong);
pub const MMTIMER_GETFREQ: u32 = _IOR!(MMTIMER_IOCTL_BASE, 2, ::core::ffi::c_ulong);
pub const MMTIMER_GETBITS: u32 = _IO!(MMTIMER_IOCTL_BASE, 4);
pub const MMTIMER_MMAPAVAIL: u32 = _IO!(MMTIMER_IOCTL_BASE, 6);
pub const MMTIMER_GETCOUNTER: u32 = _IOR!(MMTIMER_IOCTL_BASE, 9, ::core::ffi::c_ulong);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
