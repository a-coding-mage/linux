/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *	Just a place holder.
 */

/*
 * C header guard: _UAPI_SPARC_SETUP_H
 */

/* Equivalent to: defined(__sparc__) && defined(__arch64__) */
#[cfg(target_arch = "sparc64")]
pub const COMMAND_LINE_SIZE: usize = 2048;

#[cfg(not(target_arch = "sparc64"))]
pub const COMMAND_LINE_SIZE: usize = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
