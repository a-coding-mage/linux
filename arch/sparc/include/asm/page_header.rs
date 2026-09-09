/* SPDX-License-Identifier: GPL-2.0 */

// For SPARC targets with the 64-bit architecture enabled, use the declarations
// and definitions corresponding to asm/page_64.h; otherwise use asm/page_32.h.
// The referenced headers are supplied as external dependencies.

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[path = "page_64.rs"]
mod page_64;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
#[path = "page_32.rs"]
mod page_32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
