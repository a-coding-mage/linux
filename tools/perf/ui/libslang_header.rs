// SPDX-License-Identifier: GPL-2.0

/*
 * Translated from perf/ui/libslang.h.
 *
 * Original C dependency intent:
 * - include <features.h>
 * - slang versions <= 2.0.6 have a "#if HAVE_LONG_LONG" that breaks the build
 *   if it isn't defined. The C header defines HAVE_LONG_LONG from
 *   __GLIBC_HAVE_LONG_LONG when missing.
 * - define ENABLE_SLFUTURE_CONST and ENABLE_SLFUTURE_VOID before including
 *   <slang.h> to enable future slang's corrected function prototypes.
 * - include <slang.h>
 */

/* Enable future slang's corrected function prototypes. */
pub const ENABLE_SLFUTURE_CONST: i32 = 1;
pub const ENABLE_SLFUTURE_VOID: i32 = 1;

pub const SL_KEY_UNTAB: i32 = 0x1000;
