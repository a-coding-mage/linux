/* SPDX-License-Identifier: GPL-2.0 */

// The original header selects the architecture-specific fault-info
// declarations: faultinfo_32.h when compiling for i386, otherwise
// faultinfo_64.h. Those external declarations are supplied by the
// corresponding Rust translation unit/dependency.
#[cfg(target_arch = "x86")]
pub use crate::faultinfo_32::*;

#[cfg(not(target_arch = "x86"))]
pub use crate::faultinfo_64::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
