/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * This file is never included by application software unless
 * explicitly requested (e.g., via linux/types.h) in which case the
 * application is Linux specific so (user-) name space pollution is
 * not a major issue.  However, for interoperability, libraries still
 * need to be careful to avoid a name clashes.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

/*
 * The selected integer type definitions are supplied by the corresponding
 * asm-generic header.  The original build-time condition is preserved here;
 * those external definitions are expected to be available to this translation.
 *
 * #if !defined(__SANE_USERSPACE_TYPES__) && defined(__powerpc64__) && !defined(__KERNEL__)
 *     asm-generic/int-l64.h
 * #else
 *     asm-generic/int-ll64.h
 * #endif
 */

#[repr(C, align(16))]
pub struct __vector128 {
	pub u: [__u32; 4],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
