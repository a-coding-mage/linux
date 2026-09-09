/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017 Glider bvba
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A77995_PD_CA53_CPU0: i32 = 5;
pub const R8A77995_PD_CA53_SCU: i32 = 21;

/* Always-on power area */
pub const R8A77995_PD_ALWAYS_ON: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
