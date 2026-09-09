/* SPDX-License-Identifier: GPL-2.0 */

// On 64-bit SPARC, use the 64-bit bit operations implementation.
#[cfg(all(target_arch = "sparc64"))]
pub use crate::asm::bitops_64::*;

// Otherwise, use the 32-bit bit operations implementation.
#[cfg(not(all(target_arch = "sparc64")))]
pub use crate::asm::bitops_32::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
