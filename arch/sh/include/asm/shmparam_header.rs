/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/shmparam.h
 *
 * Copyright (C) 1999 Niibe Yutaka
 * Copyright (C) 2006 Paul Mundt
 */

/*
 * SH-4 and SH-3 7705 have an aliasing dcache. Bump this up to a sensible value
 * for everyone, and work out the specifics from the probed cache descriptor.
 */
pub const SHMLBA: usize = 0x4000; /* attach addr a multiple of this */

/* Corresponds to the presence-only C preprocessor macro. */
pub const __ARCH_FORCE_SHMLBA: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
