/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: ___ASM_SPARC_PGALLOC_H
// The source includes the 64-bit implementation when compiling for SPARC64;
// otherwise it includes the 32-bit implementation. The corresponding
// external declarations are supplied by those dependency headers.
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
use crate::pgalloc_64::*;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
use crate::pgalloc_32::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
