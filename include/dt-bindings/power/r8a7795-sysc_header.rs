/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Glider bvba
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A7795_PD_CA57_CPU0: i32 = 0;
pub const R8A7795_PD_CA57_CPU1: i32 = 1;
pub const R8A7795_PD_CA57_CPU2: i32 = 2;
pub const R8A7795_PD_CA57_CPU3: i32 = 3;
pub const R8A7795_PD_CA53_CPU0: i32 = 5;
pub const R8A7795_PD_CA53_CPU1: i32 = 6;
pub const R8A7795_PD_CA53_CPU2: i32 = 7;
pub const R8A7795_PD_CA53_CPU3: i32 = 8;
pub const R8A7795_PD_A3VP: i32 = 9;
pub const R8A7795_PD_CA57_SCU: i32 = 12;
pub const R8A7795_PD_CR7: i32 = 13;
pub const R8A7795_PD_A3VC: i32 = 14;
pub const R8A7795_PD_3DG_A: i32 = 17;
pub const R8A7795_PD_3DG_B: i32 = 18;
pub const R8A7795_PD_3DG_C: i32 = 19;
pub const R8A7795_PD_3DG_D: i32 = 20;
pub const R8A7795_PD_CA53_SCU: i32 = 21;
pub const R8A7795_PD_3DG_E: i32 = 22;
pub const R8A7795_PD_A3IR: i32 = 24;
pub const R8A7795_PD_A2VC1: i32 = 26;

/* Always-on power area */
pub const R8A7795_PD_ALWAYS_ON: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
