/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Socket-level I/O control calls.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995 by Ralf Baechle
 */

// Dependency supplied by the corresponding ioctl header:
// use crate::asm::ioctl::{_IOR, _IOW};

/* Socket-level I/O control calls. */
pub const FIOGETOWN: u32 = _IOR(b'f' as u32, 123, core::mem::size_of::<i32>());
pub const FIOSETOWN: u32 = _IOW(b'f' as u32, 124, core::mem::size_of::<i32>());

pub const SIOCATMARK: u32 = _IOR(b's' as u32, 7, core::mem::size_of::<i32>());
pub const SIOCSPGRP: u32 = _IOW(b's' as u32, 8, core::mem::size_of::<pid_t>());
pub const SIOCGPGRP: u32 = _IOR(b's' as u32, 9, core::mem::size_of::<pid_t>());

pub const SIOCGSTAMP_OLD: u32 = 0x8906; // Get stamp (timeval)
pub const SIOCGSTAMPNS_OLD: u32 = 0x8907; // Get stamp (timespec)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
