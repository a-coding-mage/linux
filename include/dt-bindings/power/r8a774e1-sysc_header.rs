/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2020 Renesas Electronics Corp.
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A774E1_PD_CA57_CPU0: u32 = 0;
pub const R8A774E1_PD_CA57_CPU1: u32 = 1;
pub const R8A774E1_PD_CA57_CPU2: u32 = 2;
pub const R8A774E1_PD_CA57_CPU3: u32 = 3;
pub const R8A774E1_PD_CA53_CPU0: u32 = 5;
pub const R8A774E1_PD_CA53_CPU1: u32 = 6;
pub const R8A774E1_PD_CA53_CPU2: u32 = 7;
pub const R8A774E1_PD_CA53_CPU3: u32 = 8;
pub const R8A774E1_PD_A3VP: u32 = 9;
pub const R8A774E1_PD_CA57_SCU: u32 = 12;
pub const R8A774E1_PD_A3VC: u32 = 14;
pub const R8A774E1_PD_3DG_A: u32 = 17;
pub const R8A774E1_PD_3DG_B: u32 = 18;
pub const R8A774E1_PD_3DG_C: u32 = 19;
pub const R8A774E1_PD_3DG_D: u32 = 20;
pub const R8A774E1_PD_CA53_SCU: u32 = 21;
pub const R8A774E1_PD_3DG_E: u32 = 22;
pub const R8A774E1_PD_A2VC1: u32 = 26;

/* Always-on power area */
pub const R8A774E1_PD_ALWAYS_ON: u32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
