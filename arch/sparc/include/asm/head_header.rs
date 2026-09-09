/* SPDX-License-Identifier: GPL-2.0 */

// The C header selects `asm/head_64.h` on SPARC 64-bit targets and
// `asm/head_32.h` otherwise. The corresponding Rust declarations are supplied
// by the surrounding translation unit.
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
// Dependency corresponding to <asm/head_64.h>.
#[allow(dead_code)]
const _ASM_HEAD_64_DEPENDENCY: () = ();

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
// Dependency corresponding to <asm/head_32.h>.
#[allow(dead_code)]
const _ASM_HEAD_32_DEPENDENCY: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
