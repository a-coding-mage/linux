/* SPDX-License-Identifier: GPL-2.0-only */

// The original header requires compilation with:
// '-march=armv7-a -mfloat-abi=softfp -mfpu=neon'
// This Rust translation likewise assumes an ARM NEON target.

// Dependency supplied by the surrounding kernel translation:
// #include <asm-generic/int-ll64.h>

/*
 * The C99 types uintXX_t that are usually defined in 'stdint.h' are not as
 * unambiguous on ARM as you would expect. For the types below, there is a
 * difference on ARM between GCC built for bare metal ARM, GCC built for glibc
 * and the kernel itself, which results in build errors if you try to build
 * with -ffreestanding and include 'stdint.h' (such as when you include
 * 'arm_neon.h' in order to use NEON intrinsics)
 *
 * As the typedefs for these types in 'stdint.h' are based on builtin defines
 * supplied by GCC, we can tweak these to align with the kernel's idea of those
 * types, so 'linux/types.h' and 'stdint.h' can be safely included from the
 * same source file (provided that -ffreestanding is used).
 *
 *                    int32_t     uint32_t          intptr_t     uintptr_t
 * bare metal GCC     long        unsigned long     int          unsigned int
 * glibc GCC          int         unsigned int      int          unsigned int
 * kernel             int         unsigned int      long         unsigned long
 */

// Equivalent target-level Rust type intent of the original compiler builtin
// overrides:
// __INT32_TYPE__   = int
// __UINT32_TYPE__  = unsigned int
// __INTPTR_TYPE__  = long
// __UINTPTR_TYPE__ = unsigned long

/*
 * genksyms chokes on the ARM NEON instrinsics system header, but we
 * don't export anything it defines anyway, so just disregard when
 * genksyms execute.
 */
// The original header includes <arm_neon.h> unless __GENKSYMS__ is defined.
// The NEON intrinsic declarations are supplied by the surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
