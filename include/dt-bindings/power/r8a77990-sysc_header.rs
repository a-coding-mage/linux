/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 Renesas Electronics Corp.
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A77990_PD_CA53_CPU0: u32 = 5;
pub const R8A77990_PD_CA53_CPU1: u32 = 6;
pub const R8A77990_PD_CR7: u32 = 13;
pub const R8A77990_PD_A3VC: u32 = 14;
pub const R8A77990_PD_3DG_A: u32 = 17;
pub const R8A77990_PD_3DG_B: u32 = 18;
pub const R8A77990_PD_CA53_SCU: u32 = 21;
pub const R8A77990_PD_A2VC1: u32 = 26;

/* Always-on power area */
pub const R8A77990_PD_ALWAYS_ON: u32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
