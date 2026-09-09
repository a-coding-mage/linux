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

/*
 *****************************************
 *   DMA_NRTR (Prototype: IF_NRTR)
 *****************************************
 */

pub const mmDMA_NRTR_HBW_MAX_CRED: u32 = 0x1C0100;
pub const mmDMA_NRTR_LBW_MAX_CRED: u32 = 0x1C0120;
pub const mmDMA_NRTR_DBG_E_ARB: u32 = 0x1C0300;
pub const mmDMA_NRTR_DBG_W_ARB: u32 = 0x1C0304;
pub const mmDMA_NRTR_DBG_N_ARB: u32 = 0x1C0308;
pub const mmDMA_NRTR_DBG_S_ARB: u32 = 0x1C030C;
pub const mmDMA_NRTR_DBG_L_ARB: u32 = 0x1C0310;
pub const mmDMA_NRTR_DBG_E_ARB_MAX: u32 = 0x1C0320;
pub const mmDMA_NRTR_DBG_W_ARB_MAX: u32 = 0x1C0324;
pub const mmDMA_NRTR_DBG_N_ARB_MAX: u32 = 0x1C0328;
pub const mmDMA_NRTR_DBG_S_ARB_MAX: u32 = 0x1C032C;
pub const mmDMA_NRTR_DBG_L_ARB_MAX: u32 = 0x1C0330;
pub const mmDMA_NRTR_SPLIT_COEF_0: u32 = 0x1C0400;
pub const mmDMA_NRTR_SPLIT_COEF_1: u32 = 0x1C0404;
pub const mmDMA_NRTR_SPLIT_COEF_2: u32 = 0x1C0408;
pub const mmDMA_NRTR_SPLIT_COEF_3: u32 = 0x1C040C;
pub const mmDMA_NRTR_SPLIT_COEF_4: u32 = 0x1C0410;
pub const mmDMA_NRTR_SPLIT_COEF_5: u32 = 0x1C0414;
pub const mmDMA_NRTR_SPLIT_COEF_6: u32 = 0x1C0418;
pub const mmDMA_NRTR_SPLIT_COEF_7: u32 = 0x1C041C;
pub const mmDMA_NRTR_SPLIT_COEF_8: u32 = 0x1C0420;
pub const mmDMA_NRTR_SPLIT_COEF_9: u32 = 0x1C0424;
pub const mmDMA_NRTR_SPLIT_CFG: u32 = 0x1C0440;
pub const mmDMA_NRTR_SPLIT_RD_SAT: u32 = 0x1C0444;
pub const mmDMA_NRTR_SPLIT_RD_RST_TOKEN: u32 = 0x1C0448;
pub const mmDMA_NRTR_SPLIT_RD_TIMEOUT_0: u32 = 0x1C044C;
pub const mmDMA_NRTR_SPLIT_RD_TIMEOUT_1: u32 = 0x1C0450;
pub const mmDMA_NRTR_SPLIT_WR_SAT: u32 = 0x1C0454;
pub const mmDMA_NRTR_WPLIT_WR_TST_TOLEN: u32 = 0x1C0458;
pub const mmDMA_NRTR_SPLIT_WR_TIMEOUT_0: u32 = 0x1C045C;
pub const mmDMA_NRTR_SPLIT_WR_TIMEOUT_1: u32 = 0x1C0460;
pub const mmDMA_NRTR_HBW_RANGE_HIT: u32 = 0x1C0470;

pub const mmDMA_NRTR_HBW_RANGE_MASK_L_0: u32 = 0x1C0480;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_1: u32 = 0x1C0484;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_2: u32 = 0x1C0488;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_3: u32 = 0x1C048C;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_4: u32 = 0x1C0490;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_5: u32 = 0x1C0494;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_6: u32 = 0x1C0498;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_7: u32 = 0x1C049C;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_0: u32 = 0x1C04A0;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_1: u32 = 0x1C04A4;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_2: u32 = 0x1C04A8;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_3: u32 = 0x1C04AC;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_4: u32 = 0x1C04B0;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_5: u32 = 0x1C04B4;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_6: u32 = 0x1C04B8;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_7: u32 = 0x1C04BC;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_0: u32 = 0x1C04C0;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_1: u32 = 0x1C04C4;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_2: u32 = 0x1C04C8;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_3: u32 = 0x1C04CC;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_4: u32 = 0x1C04D0;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_5: u32 = 0x1C04D4;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_6: u32 = 0x1C04D8;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_7: u32 = 0x1C04DC;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_0: u32 = 0x1C04E0;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_1: u32 = 0x1C04E4;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_2: u32 = 0x1C04E8;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_3: u32 = 0x1C04EC;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_4: u32 = 0x1C04F0;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_5: u32 = 0x1C04F4;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_6: u32 = 0x1C04F8;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_7: u32 = 0x1C04FC;
pub const mmDMA_NRTR_LBW_RANGE_HIT: u32 = 0x1C0500;
pub const mmDMA_NRTR_LBW_RANGE_MASK_0: u32 = 0x1C0510;
pub const mmDMA_NRTR_LBW_RANGE_MASK_1: u32 = 0x1C0514;
pub const mmDMA_NRTR_LBW_RANGE_MASK_2: u32 = 0x1C0518;
pub const mmDMA_NRTR_LBW_RANGE_MASK_3: u32 = 0x1C051C;
pub const mmDMA_NRTR_LBW_RANGE_MASK_4: u32 = 0x1C0520;
pub const mmDMA_NRTR_LBW_RANGE_MASK_5: u32 = 0x1C0524;
pub const mmDMA_NRTR_LBW_RANGE_MASK_6: u32 = 0x1C0528;
pub const mmDMA_NRTR_LBW_RANGE_MASK_7: u32 = 0x1C052C;
pub const mmDMA_NRTR_LBW_RANGE_MASK_8: u32 = 0x1C0530;
pub const mmDMA_NRTR_LBW_RANGE_MASK_9: u32 = 0x1C0534;
pub const mmDMA_NRTR_LBW_RANGE_MASK_10: u32 = 0x1C0538;
pub const mmDMA_NRTR_LBW_RANGE_MASK_11: u32 = 0x1C053C;
pub const mmDMA_NRTR_LBW_RANGE_MASK_12: u32 = 0x1C0540;
pub const mmDMA_NRTR_LBW_RANGE_MASK_13: u32 = 0x1C0544;
pub const mmDMA_NRTR_LBW_RANGE_MASK_14: u32 = 0x1C0548;
pub const mmDMA_NRTR_LBW_RANGE_MASK_15: u32 = 0x1C054C;
pub const mmDMA_NRTR_LBW_RANGE_BASE_0: u32 = 0x1C0550;
pub const mmDMA_NRTR_LBW_RANGE_BASE_1: u32 = 0x1C0554;
pub const mmDMA_NRTR_LBW_RANGE_BASE_2: u32 = 0x1C0558;
pub const mmDMA_NRTR_LBW_RANGE_BASE_3: u32 = 0x1C055C;
pub const mmDMA_NRTR_LBW_RANGE_BASE_4: u32 = 0x1C0560;
pub const mmDMA_NRTR_LBW_RANGE_BASE_5: u32 = 0x1C0564;
pub const mmDMA_NRTR_LBW_RANGE_BASE_6: u32 = 0x1C0568;
pub const mmDMA_NRTR_LBW_RANGE_BASE_7: u32 = 0x1C056C;
pub const mmDMA_NRTR_LBW_RANGE_BASE_8: u32 = 0x1C0570;
pub const mmDMA_NRTR_LBW_RANGE_BASE_9: u32 = 0x1C0574;
pub const mmDMA_NRTR_LBW_RANGE_BASE_10: u32 = 0x1C0578;
pub const mmDMA_NRTR_LBW_RANGE_BASE_11: u32 = 0x1C057C;
pub const mmDMA_NRTR_LBW_RANGE_BASE_12: u32 = 0x1C0580;
pub const mmDMA_NRTR_LBW_RANGE_BASE_13: u32 = 0x1C0584;
pub const mmDMA_NRTR_LBW_RANGE_BASE_14: u32 = 0x1C0588;
pub const mmDMA_NRTR_LBW_RANGE_BASE_15: u32 = 0x1C058C;
pub const mmDMA_NRTR_RGLTR: u32 = 0x1C0590;
pub const mmDMA_NRTR_RGLTR_WR_RESULT: u32 = 0x1C0594;
pub const mmDMA_NRTR_RGLTR_RD_RESULT: u32 = 0x1C0598;
pub const mmDMA_NRTR_SCRAMB_EN: u32 = 0x1C0600;
pub const mmDMA_NRTR_NON_LIN_SCRAMB: u32 = 0x1C0604;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
