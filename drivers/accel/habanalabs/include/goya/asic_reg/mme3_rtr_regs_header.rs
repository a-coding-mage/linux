/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2018 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

/************************************
 ** This is an auto-generated file **
 **       DO NOT EDIT BELOW        **
 ************************************/

/*
 *****************************************
 *   MME3_RTR (Prototype: MME_RTR)
 *****************************************
 */

pub const mmMME3_RTR_HBW_RD_RQ_E_ARB: u32 = 0xC0100;
pub const mmMME3_RTR_HBW_RD_RQ_W_ARB: u32 = 0xC0104;
pub const mmMME3_RTR_HBW_RD_RQ_N_ARB: u32 = 0xC0108;
pub const mmMME3_RTR_HBW_RD_RQ_S_ARB: u32 = 0xC010C;
pub const mmMME3_RTR_HBW_RD_RQ_L_ARB: u32 = 0xC0110;
pub const mmMME3_RTR_HBW_E_ARB_MAX: u32 = 0xC0120;
pub const mmMME3_RTR_HBW_W_ARB_MAX: u32 = 0xC0124;
pub const mmMME3_RTR_HBW_N_ARB_MAX: u32 = 0xC0128;
pub const mmMME3_RTR_HBW_S_ARB_MAX: u32 = 0xC012C;
pub const mmMME3_RTR_HBW_L_ARB_MAX: u32 = 0xC0130;
pub const mmMME3_RTR_HBW_RD_RS_MAX_CREDIT: u32 = 0xC0140;
pub const mmMME3_RTR_HBW_WR_RQ_MAX_CREDIT: u32 = 0xC0144;
pub const mmMME3_RTR_HBW_RD_RQ_MAX_CREDIT: u32 = 0xC0148;
pub const mmMME3_RTR_HBW_RD_RS_E_ARB: u32 = 0xC0150;
pub const mmMME3_RTR_HBW_RD_RS_W_ARB: u32 = 0xC0154;
pub const mmMME3_RTR_HBW_RD_RS_N_ARB: u32 = 0xC0158;
pub const mmMME3_RTR_HBW_RD_RS_S_ARB: u32 = 0xC015C;
pub const mmMME3_RTR_HBW_RD_RS_L_ARB: u32 = 0xC0160;
pub const mmMME3_RTR_HBW_WR_RQ_E_ARB: u32 = 0xC0170;
pub const mmMME3_RTR_HBW_WR_RQ_W_ARB: u32 = 0xC0174;
pub const mmMME3_RTR_HBW_WR_RQ_N_ARB: u32 = 0xC0178;
pub const mmMME3_RTR_HBW_WR_RQ_S_ARB: u32 = 0xC017C;
pub const mmMME3_RTR_HBW_WR_RQ_L_ARB: u32 = 0xC0180;
pub const mmMME3_RTR_HBW_WR_RS_E_ARB: u32 = 0xC0190;
pub const mmMME3_RTR_HBW_WR_RS_W_ARB: u32 = 0xC0194;
pub const mmMME3_RTR_HBW_WR_RS_N_ARB: u32 = 0xC0198;
pub const mmMME3_RTR_HBW_WR_RS_S_ARB: u32 = 0xC019C;
pub const mmMME3_RTR_HBW_WR_RS_L_ARB: u32 = 0xC01A0;
pub const mmMME3_RTR_LBW_RD_RQ_E_ARB: u32 = 0xC0200;
pub const mmMME3_RTR_LBW_RD_RQ_W_ARB: u32 = 0xC0204;
pub const mmMME3_RTR_LBW_RD_RQ_N_ARB: u32 = 0xC0208;
pub const mmMME3_RTR_LBW_RD_RQ_S_ARB: u32 = 0xC020C;
pub const mmMME3_RTR_LBW_RD_RQ_L_ARB: u32 = 0xC0210;
pub const mmMME3_RTR_LBW_E_ARB_MAX: u32 = 0xC0220;
pub const mmMME3_RTR_LBW_W_ARB_MAX: u32 = 0xC0224;
pub const mmMME3_RTR_LBW_N_ARB_MAX: u32 = 0xC0228;
pub const mmMME3_RTR_LBW_S_ARB_MAX: u32 = 0xC022C;
pub const mmMME3_RTR_LBW_L_ARB_MAX: u32 = 0xC0230;
pub const mmMME3_RTR_LBW_SRAM_MAX_CREDIT: u32 = 0xC0240;
pub const mmMME3_RTR_LBW_RD_RS_E_ARB: u32 = 0xC0250;
pub const mmMME3_RTR_LBW_RD_RS_W_ARB: u32 = 0xC0254;
pub const mmMME3_RTR_LBW_RD_RS_N_ARB: u32 = 0xC0258;
pub const mmMME3_RTR_LBW_RD_RS_S_ARB: u32 = 0xC025C;
pub const mmMME3_RTR_LBW_RD_RS_L_ARB: u32 = 0xC0260;
pub const mmMME3_RTR_LBW_WR_RQ_E_ARB: u32 = 0xC0270;
pub const mmMME3_RTR_LBW_WR_RQ_W_ARB: u32 = 0xC0274;
pub const mmMME3_RTR_LBW_WR_RQ_N_ARB: u32 = 0xC0278;
pub const mmMME3_RTR_LBW_WR_RQ_S_ARB: u32 = 0xC027C;
pub const mmMME3_RTR_LBW_WR_RQ_L_ARB: u32 = 0xC0280;
pub const mmMME3_RTR_LBW_WR_RS_E_ARB: u32 = 0xC0290;
pub const mmMME3_RTR_LBW_WR_RS_W_ARB: u32 = 0xC0294;
pub const mmMME3_RTR_LBW_WR_RS_N_ARB: u32 = 0xC0298;
pub const mmMME3_RTR_LBW_WR_RS_S_ARB: u32 = 0xC029C;
pub const mmMME3_RTR_LBW_WR_RS_L_ARB: u32 = 0xC02A0;
pub const mmMME3_RTR_DBG_E_ARB: u32 = 0xC0300;
pub const mmMME3_RTR_DBG_W_ARB: u32 = 0xC0304;
pub const mmMME3_RTR_DBG_N_ARB: u32 = 0xC0308;
pub const mmMME3_RTR_DBG_S_ARB: u32 = 0xC030C;
pub const mmMME3_RTR_DBG_L_ARB: u32 = 0xC0310;
pub const mmMME3_RTR_DBG_E_ARB_MAX: u32 = 0xC0320;
pub const mmMME3_RTR_DBG_W_ARB_MAX: u32 = 0xC0324;
pub const mmMME3_RTR_DBG_N_ARB_MAX: u32 = 0xC0328;
pub const mmMME3_RTR_DBG_S_ARB_MAX: u32 = 0xC032C;
pub const mmMME3_RTR_DBG_L_ARB_MAX: u32 = 0xC0330;
pub const mmMME3_RTR_SPLIT_COEF_0: u32 = 0xC0400;
pub const mmMME3_RTR_SPLIT_COEF_1: u32 = 0xC0404;
pub const mmMME3_RTR_SPLIT_COEF_2: u32 = 0xC0408;
pub const mmMME3_RTR_SPLIT_COEF_3: u32 = 0xC040C;
pub const mmMME3_RTR_SPLIT_COEF_4: u32 = 0xC0410;
pub const mmMME3_RTR_SPLIT_COEF_5: u32 = 0xC0414;
pub const mmMME3_RTR_SPLIT_COEF_6: u32 = 0xC0418;
pub const mmMME3_RTR_SPLIT_COEF_7: u32 = 0xC041C;
pub const mmMME3_RTR_SPLIT_COEF_8: u32 = 0xC0420;
pub const mmMME3_RTR_SPLIT_COEF_9: u32 = 0xC0424;
pub const mmMME3_RTR_SPLIT_CFG: u32 = 0xC0440;
pub const mmMME3_RTR_SPLIT_RD_SAT: u32 = 0xC0444;
pub const mmMME3_RTR_SPLIT_RD_RST_TOKEN: u32 = 0xC0448;
pub const mmMME3_RTR_SPLIT_RD_TIMEOUT_0: u32 = 0xC044C;
pub const mmMME3_RTR_SPLIT_RD_TIMEOUT_1: u32 = 0xC0450;
pub const mmMME3_RTR_SPLIT_WR_SAT: u32 = 0xC0454;
pub const mmMME3_RTR_WPLIT_WR_TST_TOLEN: u32 = 0xC0458;
pub const mmMME3_RTR_SPLIT_WR_TIMEOUT_0: u32 = 0xC045C;
pub const mmMME3_RTR_SPLIT_WR_TIMEOUT_1: u32 = 0xC0460;
pub const mmMME3_RTR_HBW_RANGE_HIT: u32 = 0xC0470;
pub const mmMME3_RTR_HBW_RANGE_MASK_L_0: u32 = 0xC0480;
pub const mmMME3_RTR_HBW_RANGE_MASK_L_1: u32 = 0xC0484;
pub const mmMME3_RTR_HBW_RANGE_MASK_L_2: u32 = 0xC0488;
pub const mmMME3_RTR_HBW_RANGE_MASK_L_3: u32 = 0xC048C;
pub const mmMME3_RTR_HBW_RANGE_MASK_L_4: u32 = 0xC0490;
pub const mmMME3_RTR_HBW_RANGE_MASK_L_5: u32 = 0xC0494;
pub const mmMME3_RTR_HBW_RANGE_MASK_L_6: u32 = 0xC0498;
pub const mmMME3_RTR_HBW_RANGE_MASK_L_7: u32 = 0xC049C;
pub const mmMME3_RTR_HBW_RANGE_MASK_H_0: u32 = 0xC04A0;
pub const mmMME3_RTR_HBW_RANGE_MASK_H_1: u32 = 0xC04A4;
pub const mmMME3_RTR_HBW_RANGE_MASK_H_2: u32 = 0xC04A8;
pub const mmMME3_RTR_HBW_RANGE_MASK_H_3: u32 = 0xC04AC;
pub const mmMME3_RTR_HBW_RANGE_MASK_H_4: u32 = 0xC04B0;
pub const mmMME3_RTR_HBW_RANGE_MASK_H_5: u32 = 0xC04B4;
pub const mmMME3_RTR_HBW_RANGE_MASK_H_6: u32 = 0xC04B8;
pub const mmMME3_RTR_HBW_RANGE_MASK_H_7: u32 = 0xC04BC;
pub const mmMME3_RTR_HBW_RANGE_BASE_L_0: u32 = 0xC04C0;
pub const mmMME3_RTR_HBW_RANGE_BASE_L_1: u32 = 0xC04C4;
pub const mmMME3_RTR_HBW_RANGE_BASE_L_2: u32 = 0xC04C8;
pub const mmMME3_RTR_HBW_RANGE_BASE_L_3: u32 = 0xC04CC;
pub const mmMME3_RTR_HBW_RANGE_BASE_L_4: u32 = 0xC04D0;
pub const mmMME3_RTR_HBW_RANGE_BASE_L_5: u32 = 0xC04D4;
pub const mmMME3_RTR_HBW_RANGE_BASE_L_6: u32 = 0xC04D8;
pub const mmMME3_RTR_HBW_RANGE_BASE_L_7: u32 = 0xC04DC;
pub const mmMME3_RTR_HBW_RANGE_BASE_H_0: u32 = 0xC04E0;
pub const mmMME3_RTR_HBW_RANGE_BASE_H_1: u32 = 0xC04E4;
pub const mmMME3_RTR_HBW_RANGE_BASE_H_2: u32 = 0xC04E8;
pub const mmMME3_RTR_HBW_RANGE_BASE_H_3: u32 = 0xC04EC;
pub const mmMME3_RTR_HBW_RANGE_BASE_H_4: u32 = 0xC04F0;
pub const mmMME3_RTR_HBW_RANGE_BASE_H_5: u32 = 0xC04F4;
pub const mmMME3_RTR_HBW_RANGE_BASE_H_6: u32 = 0xC04F8;
pub const mmMME3_RTR_HBW_RANGE_BASE_H_7: u32 = 0xC04FC;
pub const mmMME3_RTR_LBW_RANGE_HIT: u32 = 0xC0500;
pub const mmMME3_RTR_LBW_RANGE_MASK_0: u32 = 0xC0510;
pub const mmMME3_RTR_LBW_RANGE_MASK_1: u32 = 0xC0514;
pub const mmMME3_RTR_LBW_RANGE_MASK_2: u32 = 0xC0518;
pub const mmMME3_RTR_LBW_RANGE_MASK_3: u32 = 0xC051C;
pub const mmMME3_RTR_LBW_RANGE_MASK_4: u32 = 0xC0520;
pub const mmMME3_RTR_LBW_RANGE_MASK_5: u32 = 0xC0524;
pub const mmMME3_RTR_LBW_RANGE_MASK_6: u32 = 0xC0528;
pub const mmMME3_RTR_LBW_RANGE_MASK_7: u32 = 0xC052C;
pub const mmMME3_RTR_LBW_RANGE_MASK_8: u32 = 0xC0530;
pub const mmMME3_RTR_LBW_RANGE_MASK_9: u32 = 0xC0534;
pub const mmMME3_RTR_LBW_RANGE_MASK_10: u32 = 0xC0538;
pub const mmMME3_RTR_LBW_RANGE_MASK_11: u32 = 0xC053C;
pub const mmMME3_RTR_LBW_RANGE_MASK_12: u32 = 0xC0540;
pub const mmMME3_RTR_LBW_RANGE_MASK_13: u32 = 0xC0544;
pub const mmMME3_RTR_LBW_RANGE_MASK_14: u32 = 0xC0548;
pub const mmMME3_RTR_LBW_RANGE_MASK_15: u32 = 0xC054C;
pub const mmMME3_RTR_LBW_RANGE_BASE_0: u32 = 0xC0550;
pub const mmMME3_RTR_LBW_RANGE_BASE_1: u32 = 0xC0554;
pub const mmMME3_RTR_LBW_RANGE_BASE_2: u32 = 0xC0558;
pub const mmMME3_RTR_LBW_RANGE_BASE_3: u32 = 0xC055C;
pub const mmMME3_RTR_LBW_RANGE_BASE_4: u32 = 0xC0560;
pub const mmMME3_RTR_LBW_RANGE_BASE_5: u32 = 0xC0564;
pub const mmMME3_RTR_LBW_RANGE_BASE_6: u32 = 0xC0568;
pub const mmMME3_RTR_LBW_RANGE_BASE_7: u32 = 0xC056C;
pub const mmMME3_RTR_LBW_RANGE_BASE_8: u32 = 0xC0570;
pub const mmMME3_RTR_LBW_RANGE_BASE_9: u32 = 0xC0574;
pub const mmMME3_RTR_LBW_RANGE_BASE_10: u32 = 0xC0578;
pub const mmMME3_RTR_LBW_RANGE_BASE_11: u32 = 0xC057C;
pub const mmMME3_RTR_LBW_RANGE_BASE_12: u32 = 0xC0580;
pub const mmMME3_RTR_LBW_RANGE_BASE_13: u32 = 0xC0584;
pub const mmMME3_RTR_LBW_RANGE_BASE_14: u32 = 0xC0588;
pub const mmMME3_RTR_LBW_RANGE_BASE_15: u32 = 0xC058C;
pub const mmMME3_RTR_RGLTR: u32 = 0xC0590;
pub const mmMME3_RTR_RGLTR_WR_RESULT: u32 = 0xC0594;
pub const mmMME3_RTR_RGLTR_RD_RESULT: u32 = 0xC0598;
pub const mmMME3_RTR_SCRAMB_EN: u32 = 0xC0600;
pub const mmMME3_RTR_NON_LIN_SCRAMB: u32 = 0xC0604;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
