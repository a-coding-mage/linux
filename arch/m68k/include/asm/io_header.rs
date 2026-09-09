/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <asm/io_no.h> when __uClinux__ or CONFIG_COLDFIRE is
// defined; otherwise it includes <asm/io_mm.h>. These dependencies are
// supplied externally.

// #if defined(__uClinux__) || defined(CONFIG_COLDFIRE)
// Dependency: asm/io_no.h
// #else
// Dependency: asm/io_mm.h
// #endif

// #define gf_ioread32 ioread32be
// #define gf_iowrite32 iowrite32be
pub use ioread32be as gf_ioread32;
pub use iowrite32be as gf_iowrite32;

// Dependency: asm-generic/io.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
