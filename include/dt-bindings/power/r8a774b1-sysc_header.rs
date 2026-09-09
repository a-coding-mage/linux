/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2019 Renesas Electronics Corp.
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A774B1_PD_CA57_CPU0: i32 = 0;
pub const R8A774B1_PD_CA57_CPU1: i32 = 1;
pub const R8A774B1_PD_A3VP: i32 = 9;
pub const R8A774B1_PD_CA57_SCU: i32 = 12;
pub const R8A774B1_PD_A3VC: i32 = 14;
pub const R8A774B1_PD_3DG_A: i32 = 17;
pub const R8A774B1_PD_3DG_B: i32 = 18;
pub const R8A774B1_PD_A2VC1: i32 = 26;

/* Always-on power area */
pub const R8A774B1_PD_ALWAYS_ON: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
