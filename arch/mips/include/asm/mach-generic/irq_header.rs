/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 by Ralf Baechle
 */

// C header guard: __ASM_MACH_GENERIC_IRQ_H

// C preprocessor condition: define NR_IRQS only when it is not already defined.
// Rust has no file-local macro-definition test; this is the header's default.
pub const NR_IRQS: u32 = 256;

// C preprocessor condition: CONFIG_I8259
#[cfg(feature = "CONFIG_I8259")]
pub const I8259A_IRQ_BASE: u32 = 0;

// C preprocessor condition: CONFIG_IRQ_MIPS_CPU
#[cfg(all(feature = "CONFIG_IRQ_MIPS_CPU", feature = "CONFIG_I8259"))]
pub const MIPS_CPU_IRQ_BASE: u32 = 16;

#[cfg(all(feature = "CONFIG_IRQ_MIPS_CPU", not(feature = "CONFIG_I8259")))]
pub const MIPS_CPU_IRQ_BASE: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
