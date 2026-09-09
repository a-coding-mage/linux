/*
 * include/asm-xtensa/signal.h
 *
 * Swiped from SH.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// C header guard: _XTENSA_SIGNAL_H

// Dependency supplied by <uapi/asm/signal.h>.

// The following declaration is omitted when compiling for the assembler
// (__ASSEMBLER__). Rust-side users should apply the corresponding build-time
// condition when importing the dependent sigcontext declarations.
#[cfg(not(assembler))]
pub const __ARCH_HAS_SA_RESTORER: bool = true;

// Dependency supplied by <asm/sigcontext.h>, available when not __ASSEMBLER__.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
