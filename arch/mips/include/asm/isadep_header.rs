/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Various ISA level dependent constants.
 * Most of the following constants reflect the different layout
 * of Coprocessor 0 registers.
 *
 * Copyright (c) 1998 Harald Koerfgen
 */

// The original header selects these constants at build time with
// CONFIG_CPU_R3000.

#[cfg(CONFIG_CPU_R3000)]
pub const KU_MASK: u32 = 0x08;
#[cfg(CONFIG_CPU_R3000)]
pub const KU_USER: u32 = 0x08;
#[cfg(CONFIG_CPU_R3000)]
pub const KU_KERN: u32 = 0x00;

#[cfg(not(CONFIG_CPU_R3000))]
pub const KU_MASK: u32 = 0x18;
#[cfg(not(CONFIG_CPU_R3000))]
pub const KU_USER: u32 = 0x10;
#[cfg(not(CONFIG_CPU_R3000))]
pub const KU_KERN: u32 = 0x00;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
