/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <asm-generic/vmlinux.lds.h> dependency context.
// The original declaration is a linker-script macro; it is represented here
// as a string because the linker-script syntax has no direct Rust equivalent.

#[cfg(feature = "CONFIG_DWARF_UNWINDER")]
pub const DWARF_EH_FRAME: &str = ".eh_frame : AT(ADDR(.eh_frame) - LOAD_OFFSET) {\n\
          __start_eh_frame = .;\n\
          *(.eh_frame)\n\
          __stop_eh_frame = .;\n\
        }";

#[cfg(not(feature = "CONFIG_DWARF_UNWINDER"))]
pub const DWARF_EH_FRAME: &str = "";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
