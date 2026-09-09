/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * This file is never included by application software unless
 * explicitly requested (e.g., via linux/types.h) in which case the
 * application is Linux specific so (user-) name space pollution is
 * not a major issue.  However, for interoperability, libraries still
 * need to be careful to avoid a name clashes.
 */

// Dependency provided by uapi/asm/types.h.

// The C declaration is excluded when assembling (__ASSEMBLER__).
pub type vector128 = __vector128;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
