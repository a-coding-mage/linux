/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2018 Renesas Electronics Corp.
 * Copyright (C) 2018 Cogent Embedded, Inc.
 */

/*
 * These power domain indices match the numbers of the interrupt bits
 * representing the power areas in the various Interrupt Registers
 * (e.g. SYSCISR, Interrupt Status Register)
 */

pub const R8A77980_PD_A2SC2: u32 = 0;
pub const R8A77980_PD_A2SC3: u32 = 1;
pub const R8A77980_PD_A2SC4: u32 = 2;
pub const R8A77980_PD_A2DP0: u32 = 3;
pub const R8A77980_PD_A2DP1: u32 = 4;
pub const R8A77980_PD_CA53_CPU0: u32 = 5;
pub const R8A77980_PD_CA53_CPU1: u32 = 6;
pub const R8A77980_PD_CA53_CPU2: u32 = 7;
pub const R8A77980_PD_CA53_CPU3: u32 = 8;
pub const R8A77980_PD_A2CN: u32 = 10;
pub const R8A77980_PD_A3VIP0: u32 = 11;
pub const R8A77980_PD_A2IR5: u32 = 12;
pub const R8A77980_PD_CR7: u32 = 13;
pub const R8A77980_PD_A2IR4: u32 = 15;
pub const R8A77980_PD_CA53_SCU: u32 = 21;
pub const R8A77980_PD_A2IR0: u32 = 23;
pub const R8A77980_PD_A3IR: u32 = 24;
pub const R8A77980_PD_A3VIP1: u32 = 25;
pub const R8A77980_PD_A3VIP2: u32 = 26;
pub const R8A77980_PD_A2IR1: u32 = 27;
pub const R8A77980_PD_A2IR2: u32 = 28;
pub const R8A77980_PD_A2IR3: u32 = 29;
pub const R8A77980_PD_A2SC0: u32 = 30;
pub const R8A77980_PD_A2SC1: u32 = 31;

/* Always-on power area */
pub const R8A77980_PD_ALWAYS_ON: u32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
