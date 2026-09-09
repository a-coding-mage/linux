/* SPDX-License-Identifier: GPL-2.0 */

// The C header selects the architecture-specific shared-memory parameters:
// `asm/shmparam_64.h` when compiling for 64-bit SPARC, otherwise
// `asm/shmparam_32.h`. Those external declarations are supplied by the
// corresponding translated dependency.
#[cfg(all(target_arch = "sparc64", target_pointer_width = "64"))]
// Dependency: asm/shmparam_64.h

#[cfg(not(all(target_arch = "sparc64", target_pointer_width = "64")))]
// Dependency: asm/shmparam_32.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
