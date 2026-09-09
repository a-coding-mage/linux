/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Glider bvba
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A7779_PD_ARM1: i32 = 1;
pub const R8A7779_PD_ARM2: i32 = 2;
pub const R8A7779_PD_ARM3: i32 = 3;
pub const R8A7779_PD_SGX: i32 = 20;
pub const R8A7779_PD_VDP: i32 = 21;
pub const R8A7779_PD_IMP: i32 = 24;

/* Always-on power area */
pub const R8A7779_PD_ALWAYS_ON: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
