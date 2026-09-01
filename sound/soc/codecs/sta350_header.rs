/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Codec driver for ST STA350 2.1-channel high-efficiency digital audio system
 *
 * Copyright: 2011 Raumfeld GmbH
 * Author: Sven Brandau <info@brandau.biz>
 *
 * based on code from:
 *      Raumfeld GmbH
 *        Johannes Stezenbach <js@sig21.net>
 *	Wolfson Microelectronics PLC.
 *	  Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

/* STA350 register addresses */

pub const STA350_REGISTER_COUNT: u32 = 0x4D;
pub const STA350_COEF_COUNT: u32 = 62;

pub const STA350_CONFA: u32 = 0x00;
pub const STA350_CONFB: u32 = 0x01;
pub const STA350_CONFC: u32 = 0x02;
pub const STA350_CONFD: u32 = 0x03;
pub const STA350_CONFE: u32 = 0x04;
pub const STA350_CONFF: u32 = 0x05;
pub const STA350_MMUTE: u32 = 0x06;
pub const STA350_MVOL: u32 = 0x07;
pub const STA350_C1VOL: u32 = 0x08;
pub const STA350_C2VOL: u32 = 0x09;
pub const STA350_C3VOL: u32 = 0x0a;
pub const STA350_AUTO1: u32 = 0x0b;
pub const STA350_AUTO2: u32 = 0x0c;
pub const STA350_AUTO3: u32 = 0x0d;
pub const STA350_C1CFG: u32 = 0x0e;
pub const STA350_C2CFG: u32 = 0x0f;
pub const STA350_C3CFG: u32 = 0x10;
pub const STA350_TONE: u32 = 0x11;
pub const STA350_L1AR: u32 = 0x12;
pub const STA350_L1ATRT: u32 = 0x13;
pub const STA350_L2AR: u32 = 0x14;
pub const STA350_L2ATRT: u32 = 0x15;
pub const STA350_CFADDR2: u32 = 0x16;
pub const STA350_B1CF1: u32 = 0x17;
pub const STA350_B1CF2: u32 = 0x18;
pub const STA350_B1CF3: u32 = 0x19;
pub const STA350_B2CF1: u32 = 0x1a;
pub const STA350_B2CF2: u32 = 0x1b;
pub const STA350_B2CF3: u32 = 0x1c;
pub const STA350_A1CF1: u32 = 0x1d;
pub const STA350_A1CF2: u32 = 0x1e;
pub const STA350_A1CF3: u32 = 0x1f;
pub const STA350_A2CF1: u32 = 0x20;
pub const STA350_A2CF2: u32 = 0x21;
pub const STA350_A2CF3: u32 = 0x22;
pub const STA350_B0CF1: u32 = 0x23;
pub const STA350_B0CF2: u32 = 0x24;
pub const STA350_B0CF3: u32 = 0x25;
pub const STA350_CFUD: u32 = 0x26;
pub const STA350_MPCC1: u32 = 0x27;
pub const STA350_MPCC2: u32 = 0x28;
pub const STA350_DCC1: u32 = 0x29;
pub const STA350_DCC2: u32 = 0x2a;
pub const STA350_FDRC1: u32 = 0x2b;
pub const STA350_FDRC2: u32 = 0x2c;
pub const STA350_STATUS: u32 = 0x2d;
/* reserved: 0x2d - 0x30 */
pub const STA350_EQCFG: u32 = 0x31;
pub const STA350_EATH1: u32 = 0x32;
pub const STA350_ERTH1: u32 = 0x33;
pub const STA350_EATH2: u32 = 0x34;
pub const STA350_ERTH2: u32 = 0x35;
pub const STA350_CONFX: u32 = 0x36;
pub const STA350_SVCA: u32 = 0x37;
pub const STA350_SVCB: u32 = 0x38;
pub const STA350_RMS0A: u32 = 0x39;
pub const STA350_RMS0B: u32 = 0x3a;
pub const STA350_RMS0C: u32 = 0x3b;
pub const STA350_RMS1A: u32 = 0x3c;
pub const STA350_RMS1B: u32 = 0x3d;
pub const STA350_RMS1C: u32 = 0x3e;
pub const STA350_EVOLRES: u32 = 0x3f;
/* reserved: 0x40 - 0x47 */
pub const STA350_NSHAPE: u32 = 0x48;
pub const STA350_CTXB4B1: u32 = 0x49;
pub const STA350_CTXB7B5: u32 = 0x4a;
pub const STA350_MISC1: u32 = 0x4b;
pub const STA350_MISC2: u32 = 0x4c;

/* 0x00 CONFA */
pub const STA350_CONFA_MCS_MASK: u32 = 0x03;
pub const STA350_CONFA_MCS_SHIFT: u32 = 0;
pub const STA350_CONFA_IR_MASK: u32 = 0x18;
pub const STA350_CONFA_IR_SHIFT: u32 = 3;
pub const STA350_CONFA_TWRB: u32 = 1u32 << 5;
pub const STA350_CONFA_TWAB: u32 = 1u32 << 6;
pub const STA350_CONFA_FDRB: u32 = 1u32 << 7;

/* 0x01 CONFB */
pub const STA350_CONFB_SAI_MASK: u32 = 0x0f;
pub const STA350_CONFB_SAI_SHIFT: u32 = 0;
pub const STA350_CONFB_SAIFB: u32 = 1u32 << 4;
pub const STA350_CONFB_DSCKE: u32 = 1u32 << 5;
pub const STA350_CONFB_C1IM: u32 = 1u32 << 6;
pub const STA350_CONFB_C2IM: u32 = 1u32 << 7;

/* 0x02 CONFC */
pub const STA350_CONFC_OM_MASK: u32 = 0x03;
pub const STA350_CONFC_OM_SHIFT: u32 = 0;
pub const STA350_CONFC_CSZ_MASK: u32 = 0x3c;
pub const STA350_CONFC_CSZ_SHIFT: u32 = 2;
pub const STA350_CONFC_OCRB: u32 = 1u32 << 7;

/* 0x03 CONFD */
pub const STA350_CONFD_HPB_SHIFT: u32 = 0;
pub const STA350_CONFD_DEMP_SHIFT: u32 = 1;
pub const STA350_CONFD_DSPB_SHIFT: u32 = 2;
pub const STA350_CONFD_PSL_SHIFT: u32 = 3;
pub const STA350_CONFD_BQL_SHIFT: u32 = 4;
pub const STA350_CONFD_DRC_SHIFT: u32 = 5;
pub const STA350_CONFD_ZDE_SHIFT: u32 = 6;
pub const STA350_CONFD_SME_SHIFT: u32 = 7;

/* 0x04 CONFE */
pub const STA350_CONFE_MPCV: u32 = 1u32 << 0;
pub const STA350_CONFE_MPCV_SHIFT: u32 = 0;
pub const STA350_CONFE_MPC: u32 = 1u32 << 1;
pub const STA350_CONFE_MPC_SHIFT: u32 = 1;
pub const STA350_CONFE_NSBW: u32 = 1u32 << 2;
pub const STA350_CONFE_NSBW_SHIFT: u32 = 2;
pub const STA350_CONFE_AME: u32 = 1u32 << 3;
pub const STA350_CONFE_AME_SHIFT: u32 = 3;
pub const STA350_CONFE_PWMS: u32 = 1u32 << 4;
pub const STA350_CONFE_PWMS_SHIFT: u32 = 4;
pub const STA350_CONFE_DCCV: u32 = 1u32 << 5;
pub const STA350_CONFE_DCCV_SHIFT: u32 = 5;
pub const STA350_CONFE_ZCE: u32 = 1u32 << 6;
pub const STA350_CONFE_ZCE_SHIFT: u32 = 6;
pub const STA350_CONFE_SVE: u32 = 1u32 << 7;
pub const STA350_CONFE_SVE_SHIFT: u32 = 7;

/* 0x05 CONFF */
pub const STA350_CONFF_OCFG_MASK: u32 = 0x03;
pub const STA350_CONFF_OCFG_SHIFT: u32 = 0;
pub const STA350_CONFF_IDE: u32 = 1u32 << 2;
pub const STA350_CONFF_BCLE: u32 = 1u32 << 3;
pub const STA350_CONFF_LDTE: u32 = 1u32 << 4;
pub const STA350_CONFF_ECLE: u32 = 1u32 << 5;
pub const STA350_CONFF_PWDN: u32 = 1u32 << 6;
pub const STA350_CONFF_EAPD: u32 = 1u32 << 7;

/* 0x06 MMUTE */
pub const STA350_MMUTE_MMUTE: u32 = 0x01;
pub const STA350_MMUTE_MMUTE_SHIFT: u32 = 0;
pub const STA350_MMUTE_C1M: u32 = 0x02;
pub const STA350_MMUTE_C1M_SHIFT: u32 = 1;
pub const STA350_MMUTE_C2M: u32 = 0x04;
pub const STA350_MMUTE_C2M_SHIFT: u32 = 2;
pub const STA350_MMUTE_C3M: u32 = 0x08;
pub const STA350_MMUTE_C3M_SHIFT: u32 = 3;
pub const STA350_MMUTE_LOC_MASK: u32 = 0xC0;
pub const STA350_MMUTE_LOC_SHIFT: u32 = 6;

/* 0x0b AUTO1 */
pub const STA350_AUTO1_AMGC_MASK: u32 = 0x30;
pub const STA350_AUTO1_AMGC_SHIFT: u32 = 4;

/* 0x0c AUTO2 */
pub const STA350_AUTO2_AMAME: u32 = 0x01;
pub const STA350_AUTO2_AMAM_MASK: u32 = 0x0e;
pub const STA350_AUTO2_AMAM_SHIFT: u32 = 1;
pub const STA350_AUTO2_XO_MASK: u32 = 0xf0;
pub const STA350_AUTO2_XO_SHIFT: u32 = 4;

/* 0x0d AUTO3 */
pub const STA350_AUTO3_PEQ_MASK: u32 = 0x1f;
pub const STA350_AUTO3_PEQ_SHIFT: u32 = 0;

/* 0x0e 0x0f 0x10 CxCFG */
pub const STA350_CxCFG_TCB_SHIFT: u32 = 0;
pub const STA350_CxCFG_EQBP_SHIFT: u32 = 1;
pub const STA350_CxCFG_VBP_SHIFT: u32 = 2;
pub const STA350_CxCFG_BO_SHIFT: u32 = 3;
pub const STA350_CxCFG_LS_SHIFT: u32 = 4;
pub const STA350_CxCFG_OM_MASK: u32 = 0xc0;
pub const STA350_CxCFG_OM_SHIFT: u32 = 6;

/* 0x11 TONE */
pub const STA350_TONE_BTC_SHIFT: u32 = 0;
pub const STA350_TONE_TTC_SHIFT: u32 = 4;

/* 0x12 0x13 0x14 0x15 limiter attack/release */
pub const STA350_LxA_SHIFT: u32 = 0;
pub const STA350_LxR_SHIFT: u32 = 4;

/* 0x26 CFUD */
pub const STA350_CFUD_W1: u32 = 0x01;
pub const STA350_CFUD_WA: u32 = 0x02;
pub const STA350_CFUD_R1: u32 = 0x04;
pub const STA350_CFUD_RA: u32 = 0x08;

/* biquad filter coefficient table offsets */
pub const STA350_C1_BQ_BASE: u32 = 0;
pub const STA350_C2_BQ_BASE: u32 = 20;
pub const STA350_CH_BQ_NUM: u32 = 4;
pub const STA350_BQ_NUM_COEF: u32 = 5;
pub const STA350_XO_HP_BQ_BASE: u32 = 40;
pub const STA350_XO_LP_BQ_BASE: u32 = 45;
pub const STA350_C1_PRESCALE: u32 = 50;
pub const STA350_C2_PRESCALE: u32 = 51;
pub const STA350_C1_POSTSCALE: u32 = 52;
pub const STA350_C2_POSTSCALE: u32 = 53;
pub const STA350_C3_POSTSCALE: u32 = 54;
pub const STA350_TW_POSTSCALE: u32 = 55;
pub const STA350_C1_MIX1: u32 = 56;
pub const STA350_C1_MIX2: u32 = 57;
pub const STA350_C2_MIX1: u32 = 58;
pub const STA350_C2_MIX2: u32 = 59;
pub const STA350_C3_MIX1: u32 = 60;
pub const STA350_C3_MIX2: u32 = 61;

/* miscellaneous register 1 */
pub const STA350_MISC1_CPWMEN: u32 = 1u32 << 2;
pub const STA350_MISC1_BRIDGOFF: u32 = 1u32 << 5;
pub const STA350_MISC1_NSHHPEN: u32 = 1u32 << 6;
pub const STA350_MISC1_RPDNEN: u32 = 1u32 << 7;

/* miscellaneous register 2 */
pub const STA350_MISC2_PNDLSL_MASK: u32 = 0x1c;
pub const STA350_MISC2_PNDLSL_SHIFT: u32 = 2;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
