/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Socket-level I/O control calls.
 *
 * The _IOR and _IOW macros are supplied by the ioctl definitions used by
 * this header; pid_t is supplied by the surrounding ABI bindings.
 */

pub const FIOGETOWN: u32 = _IOR!(b'f', 123, i32);
pub const FIOSETOWN: u32 = _IOW!(b'f', 124, i32);

pub const SIOCATMARK: u32 = _IOR!(b's', 7, i32);
pub const SIOCSPGRP: u32 = _IOW!(b's', 8, pid_t);
pub const SIOCGPGRP: u32 = _IOR!(b's', 9, pid_t);

pub const SIOCGSTAMP_OLD: u32 = 0x8906; // Get stamp (timeval)
pub const SIOCGSTAMPNS_OLD: u32 = 0x8907; // Get stamp (timespec)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
