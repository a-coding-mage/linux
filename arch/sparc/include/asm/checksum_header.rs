/* SPDX-License-Identifier: GPL-2.0 */

// Architecture-provided checksum implementation markers.
pub const _HAVE_ARCH_CSUM_AND_COPY: bool = true;
pub const _HAVE_ARCH_COPY_AND_CSUM_FROM_USER: bool = true;
pub const HAVE_CSUM_COPY_USER: bool = true;

// The C header selects <asm/checksum_64.h> when compiling for SPARC64
// (__sparc__ && __arch64__), and <asm/checksum_32.h> otherwise. Those
// architecture-specific declarations are supplied by other translated files.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
