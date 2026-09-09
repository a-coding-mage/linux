/* SPDX-License-Identifier: GPL-2.0 */

// On SPARC 64-bit targets, this header includes <asm/timex_64.h>.
// Otherwise, it includes <asm-generic/timex.h>.
// These dependencies are supplied by other translated files.

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub use crate::asm::timex_64::*;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub use crate::asm_generic::timex::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
