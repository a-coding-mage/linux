/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes asm/elf_64.h when both __sparc__ and __arch64__ are
// defined; otherwise it includes asm/elf_32.h. The corresponding Rust
// declarations are supplied by those external dependencies.
#[cfg(all(target_arch = "sparc64", target_pointer_width = "64"))]
use crate::elf_64::*;

#[cfg(not(all(target_arch = "sparc64", target_pointer_width = "64")))]
use crate::elf_32::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
