/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * MPC85xx cpu type detection
 *
 * Copyright 2011-2012 Freescale Semiconductor, Inc.
 */

// C header guard: __ASM_PPC_MPC85XX_H

pub const fn SVR_REV(svr: u32) -> u32 { svr & 0xFF } // SOC design resision
pub const fn SVR_MAJ(svr: u32) -> u32 { (svr >> 4) & 0xF } // Major revision field
pub const fn SVR_MIN(svr: u32) -> u32 { (svr >> 0) & 0xF } // Minor revision field

/* Some parts define SVR[0:23] as the SOC version */
pub const fn SVR_SOC_VER(svr: u32) -> u32 { (svr >> 8) & 0xFFF7FF } // SOC Version fields

pub const SVR_8533: u32 = 0x803400;
pub const SVR_8535: u32 = 0x803701;
pub const SVR_8536: u32 = 0x803700;
pub const SVR_8540: u32 = 0x803000;
pub const SVR_8541: u32 = 0x807200;
pub const SVR_8543: u32 = 0x803200;
pub const SVR_8544: u32 = 0x803401;
pub const SVR_8545: u32 = 0x803102;
pub const SVR_8547: u32 = 0x803101;
pub const SVR_8548: u32 = 0x803100;
pub const SVR_8555: u32 = 0x807100;
pub const SVR_8560: u32 = 0x807000;
pub const SVR_8567: u32 = 0x807501;
pub const SVR_8568: u32 = 0x807500;
pub const SVR_8569: u32 = 0x808000;
pub const SVR_8572: u32 = 0x80E000;
pub const SVR_P1010: u32 = 0x80F100;
pub const SVR_P1011: u32 = 0x80E500;
pub const SVR_P1012: u32 = 0x80E501;
pub const SVR_P1013: u32 = 0x80E700;
pub const SVR_P1014: u32 = 0x80F101;
pub const SVR_P1017: u32 = 0x80F700;
pub const SVR_P1020: u32 = 0x80E400;
pub const SVR_P1021: u32 = 0x80E401;
pub const SVR_P1022: u32 = 0x80E600;
pub const SVR_P1023: u32 = 0x80F600;
pub const SVR_P1024: u32 = 0x80E402;
pub const SVR_P1025: u32 = 0x80E403;
pub const SVR_P2010: u32 = 0x80E300;
pub const SVR_P2020: u32 = 0x80E200;
pub const SVR_P2040: u32 = 0x821000;
pub const SVR_P2041: u32 = 0x821001;
pub const SVR_P3041: u32 = 0x821103;
pub const SVR_P4040: u32 = 0x820100;
pub const SVR_P4080: u32 = 0x820000;
pub const SVR_P5010: u32 = 0x822100;
pub const SVR_P5020: u32 = 0x822000;
pub const SVR_P5021: u32 = 0x820500;
pub const SVR_P5040: u32 = 0x820400;
pub const SVR_T4240: u32 = 0x824000;
pub const SVR_T4120: u32 = 0x824001;
pub const SVR_T4160: u32 = 0x824100;
pub const SVR_T4080: u32 = 0x824102;
pub const SVR_C291: u32 = 0x850000;
pub const SVR_C292: u32 = 0x850020;
pub const SVR_C293: u32 = 0x850030;
pub const SVR_B4860: u32 = 0x868000;
pub const SVR_G4860: u32 = 0x868001;
pub const SVR_G4060: u32 = 0x868003;
pub const SVR_B4440: u32 = 0x868100;
pub const SVR_G4440: u32 = 0x868101;
pub const SVR_B4420: u32 = 0x868102;
pub const SVR_B4220: u32 = 0x868103;
pub const SVR_T1040: u32 = 0x852000;
pub const SVR_T1041: u32 = 0x852001;
pub const SVR_T1042: u32 = 0x852002;
pub const SVR_T1020: u32 = 0x852100;
pub const SVR_T1021: u32 = 0x852101;
pub const SVR_T1022: u32 = 0x852102;
pub const SVR_T2080: u32 = 0x853000;
pub const SVR_T2081: u32 = 0x853100;

pub const SVR_8610: u32 = 0x80A000;
pub const SVR_8641: u32 = 0x809000;
pub const SVR_8641D: u32 = 0x809001;

pub const SVR_9130: u32 = 0x860001;
pub const SVR_9131: u32 = 0x860000;
pub const SVR_9132: u32 = 0x861000;
pub const SVR_9232: u32 = 0x861400;

pub const SVR_Unknown: u32 = 0xFFFFFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
