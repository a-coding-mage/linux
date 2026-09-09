/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * System calls under the Sparc.
 *
 * Don't be scared by the ugly clobbers, it is the only way I can
 * think of right now to force the arguments into fixed registers
 * before the trap into the system call with gcc 'asm' statements.
 *
 * Copyright (C) 1995, 2007 David S. Miller (davem@davemloft.net)
 *
 * SunOS compatibility based upon preliminary work which is:
 *
 * Copyright (C) 1995 Adrian M. Rodriguez (adrian@remus.rutgers.edu)
 */

/*
 * The C header guard _UAPI_SPARC_UNISTD_H prevents repeated inclusion.
 *
 * The following build-time conditions select the architecture-specific
 * syscall-number definitions.  They are preserved here as dependency intent:
 * __32bit_syscall_numbers__ is defined when __arch64__ is not enabled, and
 * asm/unistd_64.h or asm/unistd_32.h is supplied by the surrounding build.
 */

/* Bitmask values returned from kern_features system call. */
pub const KERN_FEATURE_MIXED_MODE_STACK: u32 = 0x0000_0001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
