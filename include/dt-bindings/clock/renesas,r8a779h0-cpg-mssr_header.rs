/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (C) 2023 Renesas Electronics Corp.
 */

// Dependency: <dt-bindings/clock/renesas-cpg-mssr.h>

/* r8a779h0 CPG Core Clocks */

pub const R8A779H0_CLK_ZX: u32 = 0;
pub const R8A779H0_CLK_ZD: u32 = 1;
pub const R8A779H0_CLK_ZS: u32 = 2;
pub const R8A779H0_CLK_ZT: u32 = 3;
pub const R8A779H0_CLK_ZTR: u32 = 4;
pub const R8A779H0_CLK_S0D2: u32 = 5;
pub const R8A779H0_CLK_S0D3: u32 = 6;
pub const R8A779H0_CLK_S0D4: u32 = 7;
pub const R8A779H0_CLK_S0D1_VIO: u32 = 8;
pub const R8A779H0_CLK_S0D2_VIO: u32 = 9;
pub const R8A779H0_CLK_S0D4_VIO: u32 = 10;
pub const R8A779H0_CLK_S0D8_VIO: u32 = 11;
pub const R8A779H0_CLK_VIOBUSD1: u32 = 12;
pub const R8A779H0_CLK_VIOBUSD2: u32 = 13;
pub const R8A779H0_CLK_S0D1_VC: u32 = 14;
pub const R8A779H0_CLK_S0D2_VC: u32 = 15;
pub const R8A779H0_CLK_S0D4_VC: u32 = 16;
pub const R8A779H0_CLK_VCBUSD1: u32 = 17;
pub const R8A779H0_CLK_VCBUSD2: u32 = 18;
pub const R8A779H0_CLK_S0D2_MM: u32 = 19;
pub const R8A779H0_CLK_S0D4_MM: u32 = 20;
pub const R8A779H0_CLK_S0D2_U3DG: u32 = 21;
pub const R8A779H0_CLK_S0D4_U3DG: u32 = 22;
pub const R8A779H0_CLK_S0D2_RT: u32 = 23;
pub const R8A779H0_CLK_S0D3_RT: u32 = 24;
pub const R8A779H0_CLK_S0D4_RT: u32 = 25;
pub const R8A779H0_CLK_S0D6_RT: u32 = 26;
pub const R8A779H0_CLK_S0D2_PER: u32 = 27;
pub const R8A779H0_CLK_S0D3_PER: u32 = 28;
pub const R8A779H0_CLK_S0D4_PER: u32 = 29;
pub const R8A779H0_CLK_S0D6_PER: u32 = 30;
pub const R8A779H0_CLK_S0D12_PER: u32 = 31;
pub const R8A779H0_CLK_S0D24_PER: u32 = 32;
pub const R8A779H0_CLK_S0D1_HSC: u32 = 33;
pub const R8A779H0_CLK_S0D2_HSC: u32 = 34;
pub const R8A779H0_CLK_S0D4_HSC: u32 = 35;
pub const R8A779H0_CLK_S0D8_HSC: u32 = 36;
pub const R8A779H0_CLK_SVD1_IR: u32 = 37;
pub const R8A779H0_CLK_SVD2_IR: u32 = 38;
pub const R8A779H0_CLK_IMPAD1: u32 = 39;
pub const R8A779H0_CLK_IMPAD4: u32 = 40;
pub const R8A779H0_CLK_IMPB: u32 = 41;
pub const R8A779H0_CLK_SVD1_VIP: u32 = 42;
pub const R8A779H0_CLK_SVD2_VIP: u32 = 43;
pub const R8A779H0_CLK_CL: u32 = 44;
pub const R8A779H0_CLK_CL16M: u32 = 45;
pub const R8A779H0_CLK_CL16M_MM: u32 = 46;
pub const R8A779H0_CLK_CL16M_RT: u32 = 47;
pub const R8A779H0_CLK_CL16M_PER: u32 = 48;
pub const R8A779H0_CLK_CL16M_HSC: u32 = 49;
pub const R8A779H0_CLK_ZC0: u32 = 50;
pub const R8A779H0_CLK_ZC1: u32 = 51;
pub const R8A779H0_CLK_ZC2: u32 = 52;
pub const R8A779H0_CLK_ZC3: u32 = 53;
pub const R8A779H0_CLK_ZB3: u32 = 54;
pub const R8A779H0_CLK_ZB3D2: u32 = 55;
pub const R8A779H0_CLK_ZB3D4: u32 = 56;
pub const R8A779H0_CLK_ZG: u32 = 57;
pub const R8A779H0_CLK_SD0H: u32 = 58;
pub const R8A779H0_CLK_SD0: u32 = 59;
pub const R8A779H0_CLK_RPC: u32 = 60;
pub const R8A779H0_CLK_RPCD2: u32 = 61;
pub const R8A779H0_CLK_MSO: u32 = 62;
pub const R8A779H0_CLK_CANFD: u32 = 63;
pub const R8A779H0_CLK_CSI: u32 = 64;
pub const R8A779H0_CLK_FRAY: u32 = 65;
pub const R8A779H0_CLK_IPC: u32 = 66;
pub const R8A779H0_CLK_SASYNCRT: u32 = 67;
pub const R8A779H0_CLK_SASYNCPERD1: u32 = 68;
pub const R8A779H0_CLK_SASYNCPERD2: u32 = 69;
pub const R8A779H0_CLK_SASYNCPERD4: u32 = 70;
pub const R8A779H0_CLK_DSIEXT: u32 = 71;
pub const R8A779H0_CLK_DSIREF: u32 = 72;
pub const R8A779H0_CLK_ADGH: u32 = 73;
pub const R8A779H0_CLK_OSC: u32 = 74;
pub const R8A779H0_CLK_ZR0: u32 = 75;
pub const R8A779H0_CLK_ZR1: u32 = 76;
pub const R8A779H0_CLK_ZR2: u32 = 77;
pub const R8A779H0_CLK_RGMII: u32 = 78;
pub const R8A779H0_CLK_CPEX: u32 = 79;
pub const R8A779H0_CLK_CP: u32 = 80;
pub const R8A779H0_CLK_CBFUSA: u32 = 81;
pub const R8A779H0_CLK_R: u32 = 82;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
