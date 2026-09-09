/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * This file is never included by application software unless
 * explicitly requested (e.g., via linux/types.h) in which case the
 * application is Linux specific so (user-) name space pollution is
 * not a major issue.  However, for interoperability, libraries still
 * need to be careful to avoid a name clashes.
 */

/*
 * This is here because we used to use l64 for alpha
 * and we don't want to impact user mode with our change to ll64
 * in the kernel.
 *
 * However, some user programs are fine with this.  They can
 * flag __SANE_USERSPACE_TYPES__ to get int-ll64.h here.
 *
 * C preprocessor conditional preserved: when both __SANE_USERSPACE_TYPES__
 * and __KERNEL__ are absent, the asm-generic int-l64 declarations are
 * required; otherwise asm-generic int-ll64 declarations are required.
 */

#[cfg(all(not(feature = "__SANE_USERSPACE_TYPES__"), not(feature = "__KERNEL__")))]
use asm_generic_int_l64::*;

#[cfg(any(feature = "__SANE_USERSPACE_TYPES__", feature = "__KERNEL__"))]
use asm_generic_int_ll64::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
