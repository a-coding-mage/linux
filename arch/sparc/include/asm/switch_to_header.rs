/* SPDX-License-Identifier: GPL-2.0 */

// Architecture-specific dependency selection from the original header:
// - on SPARC with 64-bit architecture support, use asm/switch_to_64.h;
// - otherwise, use asm/switch_to_32.h.
// These dependencies are supplied by other translated files.

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
use crate::asm::switch_to_64::*;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
use crate::asm::switch_to_32::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
