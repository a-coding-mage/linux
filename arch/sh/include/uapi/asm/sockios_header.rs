/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: `linux/time_types.h` supplies `__kernel_old_timeval` and
// `__kernel_old_timespec`.

/* Socket-level I/O control calls. */
pub const FIOGETOWN: u32 = _IOR!('f', 123, i32);
pub const FIOSETOWN: u32 = _IOW!('f', 124, i32);

pub const SIOCATMARK: u32 = _IOR!('s', 7, i32);
pub const SIOCSPGRP: u32 = _IOW!('s', 8, pid_t);
pub const SIOCGPGRP: u32 = _IOR!('s', 9, pid_t);

pub const SIOCGSTAMP_OLD: u32 =
    _IOR!('s', 100, __kernel_old_timeval); /* Get stamp (timeval) */
pub const SIOCGSTAMPNS_OLD: u32 =
    _IOR!('s', 101, __kernel_old_timespec); /* Get stamp (timespec) */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
