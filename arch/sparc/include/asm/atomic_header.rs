/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the C header guard:
// #ifndef ___ASM_SPARC_ATOMIC_H
// #define ___ASM_SPARC_ATOMIC_H

// On 64-bit SPARC, this header supplies the declarations and definitions from
// <asm/atomic_64.h>. On other configurations, it supplies those from
// <asm/atomic_32.h>. Those external headers are dependencies of this file and
// are intentionally not implemented here.
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
// External dependency: asm/atomic_64.h
const _ASM_ATOMIC_64_DEPENDENCY: () = ();

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
// External dependency: asm/atomic_32.h
const _ASM_ATOMIC_32_DEPENDENCY: () = ();

// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
