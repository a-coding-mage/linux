/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor conditional preserved: on SPARC64, include the 64-bit
// thread-info declarations; otherwise include the 32-bit declarations.
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
use crate::asm::thread_info_64::*;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
use crate::asm::thread_info_32::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
