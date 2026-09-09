/* SPDX-License-Identifier: GPL-2.0 */

// Source dependency dispatch:
// #if defined(__sparc__) && defined(__arch64__)
// #include <asm/futex_64.h>
// #else
// #include <asm/futex_32.h>
// #endif
//
// The included architecture-specific declarations are supplied by the
// surrounding translation unit.

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
use crate::futex_64::*;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
use crate::futex_32::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
