/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Cogent Embedded Inc.
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A7792_PD_CA15_CPU0: u32 = 0;
pub const R8A7792_PD_CA15_CPU1: u32 = 1;
pub const R8A7792_PD_CA15_SCU: u32 = 12;
pub const R8A7792_PD_SGX: u32 = 20;
pub const R8A7792_PD_IMP: u32 = 24;

/* Always-on power area */
pub const R8A7792_PD_ALWAYS_ON: u32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
