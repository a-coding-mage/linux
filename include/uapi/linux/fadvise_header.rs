/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const POSIX_FADV_NORMAL: i32 = 0; // No further special treatment.
pub const POSIX_FADV_RANDOM: i32 = 1; // Expect random page references.
pub const POSIX_FADV_SEQUENTIAL: i32 = 2; // Expect sequential page references.
pub const POSIX_FADV_WILLNEED: i32 = 3; // Will need these pages.

/*
 * The advise values for POSIX_FADV_DONTNEED and POSIX_ADV_NOREUSE
 * for s390-64 differ from the values for the rest of the world.
 */
#[cfg(target_arch = "s390x")]
pub const POSIX_FADV_DONTNEED: i32 = 6; // Don't need these pages.
#[cfg(target_arch = "s390x")]
pub const POSIX_FADV_NOREUSE: i32 = 7; // Data will be accessed once.

#[cfg(not(target_arch = "s390x"))]
pub const POSIX_FADV_DONTNEED: i32 = 4; // Don't need these pages.
#[cfg(not(target_arch = "s390x"))]
pub const POSIX_FADV_NOREUSE: i32 = 5; // Data will be accessed once.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
