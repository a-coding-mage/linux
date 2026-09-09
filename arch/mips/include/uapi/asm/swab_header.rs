/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 99, 2003 by Ralf Baechle
 */

// The C header includes <linux/compiler.h> and <linux/types.h>.

/// Equivalent of the C preprocessor definition `__SWAB_64_THRU_32__`.
pub const __SWAB_64_THRU_32__: () = ();

/*
 * The following definitions are enabled in C when !defined(__mips16) and
 * ((defined(__mips_isa_rev) && (__mips_isa_rev >= 2)) ||
 *  defined(_MIPS_ARCH_LOONGSON3A)).  The inline assembly is represented by
 * the corresponding byte-swap operation; the source assembly uses MIPS R2
 * `wsbh`/`rotr` instructions.
 */

#[inline]
pub const fn __arch_swab16(x: __u16) -> __u16 {
    x.swap_bytes()
}

// #define __arch_swab16 __arch_swab16

#[inline]
pub const fn __arch_swab32(x: __u32) -> __u32 {
    x.swap_bytes()
}

// #define __arch_swab32 __arch_swab32

/*
 * Having already checked for MIPS R2, enable the optimized version for
 * 64-bit kernel on r2 CPUs.
 *
 * This definition is enabled in C only when __mips64 is defined.  The source
 * assembly uses the MIPS R2 `dsbh`/`dshd` instructions.
 */
#[inline]
pub const fn __arch_swab64(x: __u64) -> __u64 {
    x.swap_bytes()
}

// #define __arch_swab64 __arch_swab64


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
