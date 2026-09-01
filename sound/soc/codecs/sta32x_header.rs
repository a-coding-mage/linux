/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Codec driver for ST STA32x 2.1-channel high-efficiency digital audio system
 *
 * Copyright: 2011 Raumfeld GmbH
 * Author: Johannes Stezenbach <js@sig21.net>
 *
 * based on code from:
 *	Wolfson Microelectronics PLC.
 *	Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

/* STA326 register addresses */

pub const STA32X_REGISTER_COUNT: u32 = 0x2d;
pub const STA32X_COEF_COUNT: u32 = 62;

pub const STA32X_CONFA: u32 = 0x00;
pub const STA32X_CONFB: u32 = 0x01;
pub const STA32X_CONFC: u32 = 0x02;
pub const STA32X_CONFD: u32 = 0x03;
pub const STA32X_CONFE: u32 = 0x04;
pub const STA32X_CONFF: u32 = 0x05;
pub const STA32X_MMUTE: u32 = 0x06;
pub const STA32X_MVOL: u32 = 0x07;
pub const STA32X_C1VOL: u32 = 0x08;
pub const STA32X_C2VOL: u32 = 0x09;
pub const STA32X_C3VOL: u32 = 0x0a;
pub const STA32X_AUTO1: u32 = 0x0b;
pub const STA32X_AUTO2: u32 = 0x0c;
pub const STA32X_AUTO3: u32 = 0x0d;
pub const STA32X_C1CFG: u32 = 0x0e;
pub const STA32X_C2CFG: u32 = 0x0f;
pub const STA32X_C3CFG: u32 = 0x10;
pub const STA32X_TONE: u32 = 0x11;
pub const STA32X_L1AR: u32 = 0x12;
pub const STA32X_L1ATRT: u32 = 0x13;
pub const STA32X_L2AR: u32 = 0x14;
pub const STA32X_L2ATRT: u32 = 0x15;
pub const STA32X_CFADDR2: u32 = 0x16;
pub const STA32X_B1CF1: u32 = 0x17;
pub const STA32X_B1CF2: u32 = 0x18;
pub const STA32X_B1CF3: u32 = 0x19;
pub const STA32X_B2CF1: u32 = 0x1a;
pub const STA32X_B2CF2: u32 = 0x1b;
pub const STA32X_B2CF3: u32 = 0x1c;
pub const STA32X_A1CF1: u32 = 0x1d;
pub const STA32X_A1CF2: u32 = 0x1e;
pub const STA32X_A1CF3: u32 = 0x1f;
pub const STA32X_A2CF1: u32 = 0x20;
pub const STA32X_A2CF2: u32 = 0x21;
pub const STA32X_A2CF3: u32 = 0x22;
pub const STA32X_B0CF1: u32 = 0x23;
pub const STA32X_B0CF2: u32 = 0x24;
pub const STA32X_B0CF3: u32 = 0x25;
pub const STA32X_CFUD: u32 = 0x26;
pub const STA32X_MPCC1: u32 = 0x27;
pub const STA32X_MPCC2: u32 = 0x28;
/* Reserved 0x29 */
/* Reserved 0x2a */
pub const STA32X_Reserved: u32 = 0x2a;
pub const STA32X_FDRC1: u32 = 0x2b;
pub const STA32X_FDRC2: u32 = 0x2c;
/* Reserved 0x2d */

/* STA326 register field definitions */

/* 0x00 CONFA */
pub const STA32X_CONFA_MCS_MASK: u32 = 0x03;
pub const STA32X_CONFA_MCS_SHIFT: u32 = 0;
pub const STA32X_CONFA_IR_MASK: u32 = 0x18;
pub const STA32X_CONFA_IR_SHIFT: u32 = 3;
pub const STA32X_CONFA_TWRB: u32 = 0x20;
pub const STA32X_CONFA_TWAB: u32 = 0x40;
pub const STA32X_CONFA_FDRB: u32 = 0x80;

/* 0x01 CONFB */
pub const STA32X_CONFB_SAI_MASK: u32 = 0x0f;
pub const STA32X_CONFB_SAI_SHIFT: u32 = 0;
pub const STA32X_CONFB_SAIFB: u32 = 0x10;
pub const STA32X_CONFB_DSCKE: u32 = 0x20;
pub const STA32X_CONFB_C1IM: u32 = 0x40;
pub const STA32X_CONFB_C2IM: u32 = 0x80;

/* 0x02 CONFC */
pub const STA32X_CONFC_OM_MASK: u32 = 0x03;
pub const STA32X_CONFC_OM_SHIFT: u32 = 0;
pub const STA32X_CONFC_CSZ_MASK: u32 = 0x7c;
pub const STA32X_CONFC_CSZ_SHIFT: u32 = 2;

/* 0x03 CONFD */
pub const STA32X_CONFD_HPB: u32 = 0x01;
pub const STA32X_CONFD_HPB_SHIFT: u32 = 0;
pub const STA32X_CONFD_DEMP: u32 = 0x02;
pub const STA32X_CONFD_DEMP_SHIFT: u32 = 1;
pub const STA32X_CONFD_DSPB: u32 = 0x04;
pub const STA32X_CONFD_DSPB_SHIFT: u32 = 2;
pub const STA32X_CONFD_PSL: u32 = 0x08;
pub const STA32X_CONFD_PSL_SHIFT: u32 = 3;
pub const STA32X_CONFD_BQL: u32 = 0x10;
pub const STA32X_CONFD_BQL_SHIFT: u32 = 4;
pub const STA32X_CONFD_DRC: u32 = 0x20;
pub const STA32X_CONFD_DRC_SHIFT: u32 = 5;
pub const STA32X_CONFD_ZDE: u32 = 0x40;
pub const STA32X_CONFD_ZDE_SHIFT: u32 = 6;
pub const STA32X_CONFD_MME: u32 = 0x80;
pub const STA32X_CONFD_MME_SHIFT: u32 = 7;

/* 0x04 CONFE */
pub const STA32X_CONFE_MPCV: u32 = 0x01;
pub const STA32X_CONFE_MPCV_SHIFT: u32 = 0;
pub const STA32X_CONFE_MPC: u32 = 0x02;
pub const STA32X_CONFE_MPC_SHIFT: u32 = 1;
pub const STA32X_CONFE_AME: u32 = 0x08;
pub const STA32X_CONFE_AME_SHIFT: u32 = 3;
pub const STA32X_CONFE_PWMS: u32 = 0x10;
pub const STA32X_CONFE_PWMS_SHIFT: u32 = 4;
pub const STA32X_CONFE_ZCE: u32 = 0x40;
pub const STA32X_CONFE_ZCE_SHIFT: u32 = 6;
pub const STA32X_CONFE_SVE: u32 = 0x80;
pub const STA32X_CONFE_SVE_SHIFT: u32 = 7;

/* 0x05 CONFF */
pub const STA32X_CONFF_OCFG_MASK: u32 = 0x03;
pub const STA32X_CONFF_OCFG_SHIFT: u32 = 0;
pub const STA32X_CONFF_IDE: u32 = 0x04;
pub const STA32X_CONFF_IDE_SHIFT: u32 = 2;
pub const STA32X_CONFF_BCLE: u32 = 0x08;
pub const STA32X_CONFF_ECLE: u32 = 0x20;
pub const STA32X_CONFF_PWDN: u32 = 0x40;
pub const STA32X_CONFF_EAPD: u32 = 0x80;

/* 0x06 MMUTE */
pub const STA32X_MMUTE_MMUTE: u32 = 0x01;

/* 0x0b AUTO1 */
pub const STA32X_AUTO1_AMEQ_MASK: u32 = 0x03;
pub const STA32X_AUTO1_AMEQ_SHIFT: u32 = 0;
pub const STA32X_AUTO1_AMV_MASK: u32 = 0xc0;
pub const STA32X_AUTO1_AMV_SHIFT: u32 = 2;
pub const STA32X_AUTO1_AMGC_MASK: u32 = 0x30;
pub const STA32X_AUTO1_AMGC_SHIFT: u32 = 4;
pub const STA32X_AUTO1_AMPS: u32 = 0x80;

/* 0x0c AUTO2 */
pub const STA32X_AUTO2_AMAME: u32 = 0x01;
pub const STA32X_AUTO2_AMAM_MASK: u32 = 0x0e;
pub const STA32X_AUTO2_AMAM_SHIFT: u32 = 1;
pub const STA32X_AUTO2_XO_MASK: u32 = 0xf0;
pub const STA32X_AUTO2_XO_SHIFT: u32 = 4;

/* 0x0d AUTO3 */
pub const STA32X_AUTO3_PEQ_MASK: u32 = 0x1f;
pub const STA32X_AUTO3_PEQ_SHIFT: u32 = 0;

/* 0x0e 0x0f 0x10 CxCFG */
pub const STA32X_CxCFG_TCB: u32 = 0x01; /* only C1 and C2 */
pub const STA32X_CxCFG_TCB_SHIFT: u32 = 0;
pub const STA32X_CxCFG_EQBP: u32 = 0x02; /* only C1 and C2 */
pub const STA32X_CxCFG_EQBP_SHIFT: u32 = 1;
pub const STA32X_CxCFG_VBP: u32 = 0x03;
pub const STA32X_CxCFG_VBP_SHIFT: u32 = 2;
pub const STA32X_CxCFG_BO: u32 = 0x04;
pub const STA32X_CxCFG_LS_MASK: u32 = 0x30;
pub const STA32X_CxCFG_LS_SHIFT: u32 = 4;
pub const STA32X_CxCFG_OM_MASK: u32 = 0xc0;
pub const STA32X_CxCFG_OM_SHIFT: u32 = 6;

/* 0x11 TONE */
pub const STA32X_TONE_BTC_SHIFT: u32 = 0;
pub const STA32X_TONE_TTC_SHIFT: u32 = 4;

/* 0x12 0x13 0x14 0x15 limiter attack/release */
pub const STA32X_LxA_SHIFT: u32 = 0;
pub const STA32X_LxR_SHIFT: u32 = 4;

/* 0x26 CFUD */
pub const STA32X_CFUD_W1: u32 = 0x01;
pub const STA32X_CFUD_WA: u32 = 0x02;
pub const STA32X_CFUD_R1: u32 = 0x04;
pub const STA32X_CFUD_RA: u32 = 0x08;

/* biquad filter coefficient table offsets */
pub const STA32X_C1_BQ_BASE: u32 = 0;
pub const STA32X_C2_BQ_BASE: u32 = 20;
pub const STA32X_CH_BQ_NUM: u32 = 4;
pub const STA32X_BQ_NUM_COEF: u32 = 5;
pub const STA32X_XO_HP_BQ_BASE: u32 = 40;
pub const STA32X_XO_LP_BQ_BASE: u32 = 45;
pub const STA32X_C1_PRESCALE: u32 = 50;
pub const STA32X_C2_PRESCALE: u32 = 51;
pub const STA32X_C1_POSTSCALE: u32 = 52;
pub const STA32X_C2_POSTSCALE: u32 = 53;
pub const STA32X_C3_POSTSCALE: u32 = 54;
pub const STA32X_TW_POSTSCALE: u32 = 55;
pub const STA32X_C1_MIX1: u32 = 56;
pub const STA32X_C1_MIX2: u32 = 57;
pub const STA32X_C2_MIX1: u32 = 58;
pub const STA32X_C2_MIX2: u32 = 59;
pub const STA32X_C3_MIX1: u32 = 60;
pub const STA32X_C3_MIX2: u32 = 61;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
