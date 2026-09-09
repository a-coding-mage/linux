/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __SPARC_SIGNAL_H

// The following dependencies are supplied by other headers in the source
// environment:
// #include <linux/personality.h>
// #include <linux/types.h>
// #include <uapi/asm/signal.h>

// In the C source these declarations are excluded when __ASSEMBLER__ is
// defined. Rust has no equivalent assembler preprocessing mode here.

pub const __ARCH_HAS_KA_RESTORER: bool = true;
pub const __ARCH_HAS_SA_RESTORER: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
