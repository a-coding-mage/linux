/* SPDX-License-Identifier: GPL-2.0 */

// The C header selects the architecture-specific Open Firmware interface:
// `asm/oplib_64.h` when compiling for 64-bit SPARC, otherwise `asm/oplib_32.h`.
// Those external declarations are supplied by the corresponding Rust
// translation units.
#[cfg(all(target_arch = "sparc64", target_pointer_width = "64"))]
pub mod oplib {
    // Declarations from asm/oplib_64.h are provided externally.
}

#[cfg(not(all(target_arch = "sparc64", target_pointer_width = "64")))]
pub mod oplib {
    // Declarations from asm/oplib_32.h are provided externally.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
