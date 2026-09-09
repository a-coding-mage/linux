/* SPDX-License-Identifier: GPL-2.0 */

// When CONFIG_GENERIC_CSUM is enabled, declarations from
// <asm-generic/checksum.h> are required here.
// Otherwise, the architecture-specific checksum implementation is selected:
// <asm/checksum_32.h> when CONFIG_X86_32 is enabled, or
// <asm/checksum_64.h> otherwise.

// These constants correspond to the C preprocessor definitions in the
// non-generic-checksum configuration.
pub const _HAVE_ARCH_COPY_AND_CSUM_FROM_USER: i32 = 1;
pub const HAVE_CSUM_COPY_USER: bool = true;
pub const _HAVE_ARCH_CSUM_AND_COPY: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
