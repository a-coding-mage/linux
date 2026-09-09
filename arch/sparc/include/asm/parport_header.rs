/* SPDX-License-Identifier: GPL-2.0 */

// Include the architecture-specific parport declarations when compiling
// 64-bit SPARC; otherwise use the generic parport declarations.
//
// C source condition:
// #if defined(__sparc__) && defined(__arch64__)
// #include <asm/parport_64.h>
// #else
// #include <asm-generic/parport.h>
// #endif

#[cfg(all(target_arch = "sparc64"))]
pub use crate::asm::parport_64::*;

#[cfg(not(all(target_arch = "sparc64")))]
pub use crate::asm_generic::parport::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
