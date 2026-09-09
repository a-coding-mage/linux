/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Equivalent of the C preprocessor condition:
 * #if defined(__sparc__) && defined(__arch64__)
 */
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub type __ARCH_SI_BAND_T = i32;

/* Dependency supplied by asm-generic/siginfo.h. */

/// No information in siginfo_t.
pub const SI_NOINFO: i32 = 32767;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
