/* SPDX-License-Identifier: GPL-2.0 */

// C translation of the header guard and architecture-dependent include:
//
// #if defined(__sparc__) && defined(__arch64__)
// #include <asm/mmu_context_64.h>
// #else
// #include <asm/mmu_context_32.h>
// #endif
//
// The selected declarations are supplied by the corresponding external
// dependency; this translation does not provide or implement that dependency.

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[allow(unused)]
const _MMU_CONTEXT_HEADER_ARCH: &str = "asm/mmu_context_64.h";

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
#[allow(unused)]
const _MMU_CONTEXT_HEADER_ARCH: &str = "asm/mmu_context_32.h";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
