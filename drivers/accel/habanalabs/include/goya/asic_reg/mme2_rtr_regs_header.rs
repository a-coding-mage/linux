/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2018 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

/************************************
 ** This is an auto-generated file **
 **       DO NOT EDIT BELOW        **
 ************************************/

// #ifndef ASIC_REG_MME2_RTR_REGS_H_
// #define ASIC_REG_MME2_RTR_REGS_H_

/*
 *****************************************
 *   MME2_RTR (Prototype: MME_RTR)
 *****************************************
 */

pub const mmMME2_RTR_HBW_RD_RQ_E_ARB: u32 = 0x80100;

pub const mmMME2_RTR_HBW_RD_RQ_W_ARB: u32 = 0x80104;

pub const mmMME2_RTR_HBW_RD_RQ_N_ARB: u32 = 0x80108;

pub const mmMME2_RTR_HBW_RD_RQ_S_ARB: u32 = 0x8010C;

pub const mmMME2_RTR_HBW_RD_RQ_L_ARB: u32 = 0x80110;

pub const mmMME2_RTR_HBW_E_ARB_MAX: u32 = 0x80120;

pub const mmMME2_RTR_HBW_W_ARB_MAX: u32 = 0x80124;

pub const mmMME2_RTR_HBW_N_ARB_MAX: u32 = 0x80128;

pub const mmMME2_RTR_HBW_S_ARB_MAX: u32 = 0x8012C;

pub const mmMME2_RTR_HBW_L_ARB_MAX: u32 = 0x80130;

pub const mmMME2_RTR_HBW_RD_RS_MAX_CREDIT: u32 = 0x80140;

pub const mmMME2_RTR_HBW_WR_RQ_MAX_CREDIT: u32 = 0x80144;

pub const mmMME2_RTR_HBW_RD_RQ_MAX_CREDIT: u32 = 0x80148;

pub const mmMME2_RTR_HBW_RD_RS_E_ARB: u32 = 0x80150;

pub const mmMME2_RTR_HBW_RD_RS_W_ARB: u32 = 0x80154;

pub const mmMME2_RTR_HBW_RD_RS_N_ARB: u32 = 0x80158;

pub const mmMME2_RTR_HBW_RD_RS_S_ARB: u32 = 0x8015C;

pub const mmMME2_RTR_HBW_RD_RS_L_ARB: u32 = 0x80160;

pub const mmMME2_RTR_HBW_WR_RQ_E_ARB: u32 = 0x80170;

pub const mmMME2_RTR_HBW_WR_RQ_W_ARB: u32 = 0x80174;

pub const mmMME2_RTR_HBW_WR_RQ_N_ARB: u32 = 0x80178;

pub const mmMME2_RTR_HBW_WR_RQ_S_ARB: u32 = 0x8017C;

pub const mmMME2_RTR_HBW_WR_RQ_L_ARB: u32 = 0x80180;

pub const mmMME2_RTR_HBW_WR_RS_E_ARB: u32 = 0x80190;

pub const mmMME2_RTR_HBW_WR_RS_W_ARB: u32 = 0x80194;

pub const mmMME2_RTR_HBW_WR_RS_N_ARB: u32 = 0x80198;

pub const mmMME2_RTR_HBW_WR_RS_S_ARB: u32 = 0x8019C;

pub const mmMME2_RTR_HBW_WR_RS_L_ARB: u32 = 0x801A0;

pub const mmMME2_RTR_LBW_RD_RQ_E_ARB: u32 = 0x80200;

pub const mmMME2_RTR_LBW_RD_RQ_W_ARB: u32 = 0x80204;

pub const mmMME2_RTR_LBW_RD_RQ_N_ARB: u32 = 0x80208;

pub const mmMME2_RTR_LBW_RD_RQ_S_ARB: u32 = 0x8020C;

pub const mmMME2_RTR_LBW_RD_RQ_L_ARB: u32 = 0x80210;

pub const mmMME2_RTR_LBW_E_ARB_MAX: u32 = 0x80220;

pub const mmMME2_RTR_LBW_W_ARB_MAX: u32 = 0x80224;

pub const mmMME2_RTR_LBW_N_ARB_MAX: u32 = 0x80228;

pub const mmMME2_RTR_LBW_S_ARB_MAX: u32 = 0x8022C;

pub const mmMME2_RTR_LBW_L_ARB_MAX: u32 = 0x80230;

pub const mmMME2_RTR_LBW_SRAM_MAX_CREDIT: u32 = 0x80240;

pub const mmMME2_RTR_LBW_RD_RS_E_ARB: u32 = 0x80250;

pub const mmMME2_RTR_LBW_RD_RS_W_ARB: u32 = 0x80254;

pub const mmMME2_RTR_LBW_RD_RS_N_ARB: u32 = 0x80258;

pub const mmMME2_RTR_LBW_RD_RS_S_ARB: u32 = 0x8025C;

pub const mmMME2_RTR_LBW_RD_RS_L_ARB: u32 = 0x80260;

pub const mmMME2_RTR_LBW_WR_RQ_E_ARB: u32 = 0x80270;

pub const mmMME2_RTR_LBW_WR_RQ_W_ARB: u32 = 0x80274;

pub const mmMME2_RTR_LBW_WR_RQ_N_ARB: u32 = 0x80278;

pub const mmMME2_RTR_LBW_WR_RQ_S_ARB: u32 = 0x8027C;

pub const mmMME2_RTR_LBW_WR_RQ_L_ARB: u32 = 0x80280;

pub const mmMME2_RTR_LBW_WR_RS_E_ARB: u32 = 0x80290;

pub const mmMME2_RTR_LBW_WR_RS_W_ARB: u32 = 0x80294;

pub const mmMME2_RTR_LBW_WR_RS_N_ARB: u32 = 0x80298;

pub const mmMME2_RTR_LBW_WR_RS_S_ARB: u32 = 0x8029C;

pub const mmMME2_RTR_LBW_WR_RS_L_ARB: u32 = 0x802A0;

pub const mmMME2_RTR_DBG_E_ARB: u32 = 0x80300;

pub const mmMME2_RTR_DBG_W_ARB: u32 = 0x80304;

pub const mmMME2_RTR_DBG_N_ARB: u32 = 0x80308;

pub const mmMME2_RTR_DBG_S_ARB: u32 = 0x8030C;

pub const mmMME2_RTR_DBG_L_ARB: u32 = 0x80310;

pub const mmMME2_RTR_DBG_E_ARB_MAX: u32 = 0x80320;

pub const mmMME2_RTR_DBG_W_ARB_MAX: u32 = 0x80324;

pub const mmMME2_RTR_DBG_N_ARB_MAX: u32 = 0x80328;

pub const mmMME2_RTR_DBG_S_ARB_MAX: u32 = 0x8032C;

pub const mmMME2_RTR_DBG_L_ARB_MAX: u32 = 0x80330;

pub const mmMME2_RTR_SPLIT_COEF_0: u32 = 0x80400;

pub const mmMME2_RTR_SPLIT_COEF_1: u32 = 0x80404;

pub const mmMME2_RTR_SPLIT_COEF_2: u32 = 0x80408;

pub const mmMME2_RTR_SPLIT_COEF_3: u32 = 0x8040C;

pub const mmMME2_RTR_SPLIT_COEF_4: u32 = 0x80410;

pub const mmMME2_RTR_SPLIT_COEF_5: u32 = 0x80414;

pub const mmMME2_RTR_SPLIT_COEF_6: u32 = 0x80418;

pub const mmMME2_RTR_SPLIT_COEF_7: u32 = 0x8041C;

pub const mmMME2_RTR_SPLIT_COEF_8: u32 = 0x80420;

pub const mmMME2_RTR_SPLIT_COEF_9: u32 = 0x80424;

pub const mmMME2_RTR_SPLIT_CFG: u32 = 0x80440;

pub const mmMME2_RTR_SPLIT_RD_SAT: u32 = 0x80444;

pub const mmMME2_RTR_SPLIT_RD_RST_TOKEN: u32 = 0x80448;

pub const mmMME2_RTR_SPLIT_RD_TIMEOUT_0: u32 = 0x8044C;

pub const mmMME2_RTR_SPLIT_RD_TIMEOUT_1: u32 = 0x80450;

pub const mmMME2_RTR_SPLIT_WR_SAT: u32 = 0x80454;

pub const mmMME2_RTR_WPLIT_WR_TST_TOLEN: u32 = 0x80458;

pub const mmMME2_RTR_SPLIT_WR_TIMEOUT_0: u32 = 0x8045C;

pub const mmMME2_RTR_SPLIT_WR_TIMEOUT_1: u32 = 0x80460;

pub const mmMME2_RTR_HBW_RANGE_HIT: u32 = 0x80470;

pub const mmMME2_RTR_HBW_RANGE_MASK_L_0: u32 = 0x80480;

pub const mmMME2_RTR_HBW_RANGE_MASK_L_1: u32 = 0x80484;

pub const mmMME2_RTR_HBW_RANGE_MASK_L_2: u32 = 0x80488;

pub const mmMME2_RTR_HBW_RANGE_MASK_L_3: u32 = 0x8048C;

pub const mmMME2_RTR_HBW_RANGE_MASK_L_4: u32 = 0x80490;

pub const mmMME2_RTR_HBW_RANGE_MASK_L_5: u32 = 0x80494;

pub const mmMME2_RTR_HBW_RANGE_MASK_L_6: u32 = 0x80498;

pub const mmMME2_RTR_HBW_RANGE_MASK_L_7: u32 = 0x8049C;

pub const mmMME2_RTR_HBW_RANGE_MASK_H_0: u32 = 0x804A0;

pub const mmMME2_RTR_HBW_RANGE_MASK_H_1: u32 = 0x804A4;

pub const mmMME2_RTR_HBW_RANGE_MASK_H_2: u32 = 0x804A8;

pub const mmMME2_RTR_HBW_RANGE_MASK_H_3: u32 = 0x804AC;

pub const mmMME2_RTR_HBW_RANGE_MASK_H_4: u32 = 0x804B0;

pub const mmMME2_RTR_HBW_RANGE_MASK_H_5: u32 = 0x804B4;

pub const mmMME2_RTR_HBW_RANGE_MASK_H_6: u32 = 0x804B8;

pub const mmMME2_RTR_HBW_RANGE_MASK_H_7: u32 = 0x804BC;

pub const mmMME2_RTR_HBW_RANGE_BASE_L_0: u32 = 0x804C0;

pub const mmMME2_RTR_HBW_RANGE_BASE_L_1: u32 = 0x804C4;

pub const mmMME2_RTR_HBW_RANGE_BASE_L_2: u32 = 0x804C8;

pub const mmMME2_RTR_HBW_RANGE_BASE_L_3: u32 = 0x804CC;

pub const mmMME2_RTR_HBW_RANGE_BASE_L_4: u32 = 0x804D0;

pub const mmMME2_RTR_HBW_RANGE_BASE_L_5: u32 = 0x804D4;

pub const mmMME2_RTR_HBW_RANGE_BASE_L_6: u32 = 0x804D8;

pub const mmMME2_RTR_HBW_RANGE_BASE_L_7: u32 = 0x804DC;

pub const mmMME2_RTR_HBW_RANGE_BASE_H_0: u32 = 0x804E0;

pub const mmMME2_RTR_HBW_RANGE_BASE_H_1: u32 = 0x804E4;

pub const mmMME2_RTR_HBW_RANGE_BASE_H_2: u32 = 0x804E8;

pub const mmMME2_RTR_HBW_RANGE_BASE_H_3: u32 = 0x804EC;

pub const mmMME2_RTR_HBW_RANGE_BASE_H_4: u32 = 0x804F0;

pub const mmMME2_RTR_HBW_RANGE_BASE_H_5: u32 = 0x804F4;

pub const mmMME2_RTR_HBW_RANGE_BASE_H_6: u32 = 0x804F8;

pub const mmMME2_RTR_HBW_RANGE_BASE_H_7: u32 = 0x804FC;

pub const mmMME2_RTR_LBW_RANGE_HIT: u32 = 0x80500;

pub const mmMME2_RTR_LBW_RANGE_MASK_0: u32 = 0x80510;

pub const mmMME2_RTR_LBW_RANGE_MASK_1: u32 = 0x80514;

pub const mmMME2_RTR_LBW_RANGE_MASK_2: u32 = 0x80518;

pub const mmMME2_RTR_LBW_RANGE_MASK_3: u32 = 0x8051C;

pub const mmMME2_RTR_LBW_RANGE_MASK_4: u32 = 0x80520;

pub const mmMME2_RTR_LBW_RANGE_MASK_5: u32 = 0x80524;

pub const mmMME2_RTR_LBW_RANGE_MASK_6: u32 = 0x80528;

pub const mmMME2_RTR_LBW_RANGE_MASK_7: u32 = 0x8052C;

pub const mmMME2_RTR_LBW_RANGE_MASK_8: u32 = 0x80530;

pub const mmMME2_RTR_LBW_RANGE_MASK_9: u32 = 0x80534;

pub const mmMME2_RTR_LBW_RANGE_MASK_10: u32 = 0x80538;

pub const mmMME2_RTR_LBW_RANGE_MASK_11: u32 = 0x8053C;

pub const mmMME2_RTR_LBW_RANGE_MASK_12: u32 = 0x80540;

pub const mmMME2_RTR_LBW_RANGE_MASK_13: u32 = 0x80544;

pub const mmMME2_RTR_LBW_RANGE_MASK_14: u32 = 0x80548;

pub const mmMME2_RTR_LBW_RANGE_MASK_15: u32 = 0x8054C;

pub const mmMME2_RTR_LBW_RANGE_BASE_0: u32 = 0x80550;

pub const mmMME2_RTR_LBW_RANGE_BASE_1: u32 = 0x80554;

pub const mmMME2_RTR_LBW_RANGE_BASE_2: u32 = 0x80558;

pub const mmMME2_RTR_LBW_RANGE_BASE_3: u32 = 0x8055C;

pub const mmMME2_RTR_LBW_RANGE_BASE_4: u32 = 0x80560;

pub const mmMME2_RTR_LBW_RANGE_BASE_5: u32 = 0x80564;

pub const mmMME2_RTR_LBW_RANGE_BASE_6: u32 = 0x80568;

pub const mmMME2_RTR_LBW_RANGE_BASE_7: u32 = 0x8056C;

pub const mmMME2_RTR_LBW_RANGE_BASE_8: u32 = 0x80570;

pub const mmMME2_RTR_LBW_RANGE_BASE_9: u32 = 0x80574;

pub const mmMME2_RTR_LBW_RANGE_BASE_10: u32 = 0x80578;

pub const mmMME2_RTR_LBW_RANGE_BASE_11: u32 = 0x8057C;

pub const mmMME2_RTR_LBW_RANGE_BASE_12: u32 = 0x80580;

pub const mmMME2_RTR_LBW_RANGE_BASE_13: u32 = 0x80584;

pub const mmMME2_RTR_LBW_RANGE_BASE_14: u32 = 0x80588;

pub const mmMME2_RTR_LBW_RANGE_BASE_15: u32 = 0x8058C;

pub const mmMME2_RTR_RGLTR: u32 = 0x80590;

pub const mmMME2_RTR_RGLTR_WR_RESULT: u32 = 0x80594;

pub const mmMME2_RTR_RGLTR_RD_RESULT: u32 = 0x80598;

pub const mmMME2_RTR_SCRAMB_EN: u32 = 0x80600;

pub const mmMME2_RTR_NON_LIN_SCRAMB: u32 = 0x80604;

// #endif /* ASIC_REG_MME2_RTR_REGS_H_ */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
