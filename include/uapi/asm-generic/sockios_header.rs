/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Socket-level I/O control calls. */
pub const FIOSETOWN: i32 = 0x8901;
pub const SIOCSPGRP: i32 = 0x8902;
pub const FIOGETOWN: i32 = 0x8903;
pub const SIOCGPGRP: i32 = 0x8904;
pub const SIOCATMARK: i32 = 0x8905;
pub const SIOCGSTAMP_OLD: i32 = 0x8906; // Get stamp (timeval)
pub const SIOCGSTAMPNS_OLD: i32 = 0x8907; // Get stamp (timespec)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
