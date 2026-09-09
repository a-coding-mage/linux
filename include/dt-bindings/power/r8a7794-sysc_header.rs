/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Glider bvba
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A7794_PD_CA7_CPU0: u32 = 5;
pub const R8A7794_PD_CA7_CPU1: u32 = 6;
pub const R8A7794_PD_SH_4A: u32 = 16;
pub const R8A7794_PD_SGX: u32 = 20;
pub const R8A7794_PD_CA7_SCU: u32 = 21;

/* Always-on power area */
pub const R8A7794_PD_ALWAYS_ON: u32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
