/* SPDX-License-Identifier: GPL-2.0 */

/*
 * tools/ copied this from include/asm-generic/bitops.h, bit by bit as it needed
 * some functions.
 *
 * For the benefit of those who are trying to port Linux to another
 * architecture, here are some C-language equivalents.  You should
 * recode these in the native assembly language, if at all possible.
 *
 * C language equivalents written by Theodore Ts'o, 9/26/92
 */

/*
 * C header dependency intent:
 * - <asm-generic/bitops/__ffs.h>
 * - <asm-generic/bitops/__ffz.h>
 * - <asm-generic/bitops/fls.h>
 * - <asm-generic/bitops/__fls.h>
 * - <asm-generic/bitops/fls64.h>
 *
 * Original C preprocessor check:
 * #ifndef _TOOLS_LINUX_BITOPS_H_
 * #error only <linux/bitops.h> can be included directly
 * #endif
 *
 * Additional C header dependency intent:
 * - <asm-generic/bitops/hweight.h>
 * - <asm-generic/bitops/atomic.h>
 * - <asm-generic/bitops/non-atomic.h>
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
