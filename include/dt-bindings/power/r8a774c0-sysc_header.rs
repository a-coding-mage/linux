/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2018 Renesas Electronics Corp.
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A774C0_PD_CA53_CPU0: i32 = 5;
pub const R8A774C0_PD_CA53_CPU1: i32 = 6;
pub const R8A774C0_PD_A3VC: i32 = 14;
pub const R8A774C0_PD_3DG_A: i32 = 17;
pub const R8A774C0_PD_3DG_B: i32 = 18;
pub const R8A774C0_PD_CA53_SCU: i32 = 21;
pub const R8A774C0_PD_A2VC1: i32 = 26;

/* Always-on power area */
pub const R8A774C0_PD_ALWAYS_ON: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
