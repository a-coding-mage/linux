/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017 Cogent Embedded Inc.
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A77970_PD_CA53_CPU0: u32 = 5;
pub const R8A77970_PD_CA53_CPU1: u32 = 6;
pub const R8A77970_PD_CA53_SCU: u32 = 21;
pub const R8A77970_PD_A2IR0: u32 = 23;
pub const R8A77970_PD_A3IR: u32 = 24;
pub const R8A77970_PD_A2IR1: u32 = 27;
pub const R8A77970_PD_A2DP: u32 = 28;
pub const R8A77970_PD_A2CN: u32 = 29;
pub const R8A77970_PD_A2SC0: u32 = 30;
pub const R8A77970_PD_A2SC1: u32 = 31;

/* Always-on power area */
pub const R8A77970_PD_ALWAYS_ON: u32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
