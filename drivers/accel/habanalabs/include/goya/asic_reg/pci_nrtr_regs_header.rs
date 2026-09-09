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
 *   PCI_NRTR (Prototype: IF_NRTR)
 *****************************************
 */

pub const mmPCI_NRTR_HBW_MAX_CRED: u32 = 0x100;
pub const mmPCI_NRTR_LBW_MAX_CRED: u32 = 0x120;
pub const mmPCI_NRTR_DBG_E_ARB: u32 = 0x300;
pub const mmPCI_NRTR_DBG_W_ARB: u32 = 0x304;
pub const mmPCI_NRTR_DBG_N_ARB: u32 = 0x308;
pub const mmPCI_NRTR_DBG_S_ARB: u32 = 0x30C;
pub const mmPCI_NRTR_DBG_L_ARB: u32 = 0x310;
pub const mmPCI_NRTR_DBG_E_ARB_MAX: u32 = 0x320;
pub const mmPCI_NRTR_DBG_W_ARB_MAX: u32 = 0x324;
pub const mmPCI_NRTR_DBG_N_ARB_MAX: u32 = 0x328;
pub const mmPCI_NRTR_DBG_S_ARB_MAX: u32 = 0x32C;
pub const mmPCI_NRTR_DBG_L_ARB_MAX: u32 = 0x330;
pub const mmPCI_NRTR_SPLIT_COEF_0: u32 = 0x400;
pub const mmPCI_NRTR_SPLIT_COEF_1: u32 = 0x404;
pub const mmPCI_NRTR_SPLIT_COEF_2: u32 = 0x408;
pub const mmPCI_NRTR_SPLIT_COEF_3: u32 = 0x40C;
pub const mmPCI_NRTR_SPLIT_COEF_4: u32 = 0x410;
pub const mmPCI_NRTR_SPLIT_COEF_5: u32 = 0x414;
pub const mmPCI_NRTR_SPLIT_COEF_6: u32 = 0x418;
pub const mmPCI_NRTR_SPLIT_COEF_7: u32 = 0x41C;
pub const mmPCI_NRTR_SPLIT_COEF_8: u32 = 0x420;
pub const mmPCI_NRTR_SPLIT_COEF_9: u32 = 0x424;
pub const mmPCI_NRTR_SPLIT_CFG: u32 = 0x440;
pub const mmPCI_NRTR_SPLIT_RD_SAT: u32 = 0x444;
pub const mmPCI_NRTR_SPLIT_RD_RST_TOKEN: u32 = 0x448;
pub const mmPCI_NRTR_SPLIT_RD_TIMEOUT_0: u32 = 0x44C;
pub const mmPCI_NRTR_SPLIT_RD_TIMEOUT_1: u32 = 0x450;
pub const mmPCI_NRTR_SPLIT_WR_SAT: u32 = 0x454;
pub const mmPCI_NRTR_WPLIT_WR_TST_TOLEN: u32 = 0x458;
pub const mmPCI_NRTR_SPLIT_WR_TIMEOUT_0: u32 = 0x45C;
pub const mmPCI_NRTR_SPLIT_WR_TIMEOUT_1: u32 = 0x460;
pub const mmPCI_NRTR_HBW_RANGE_HIT: u32 = 0x470;

pub const mmPCI_NRTR_HBW_RANGE_MASK_L_0: u32 = 0x480;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_1: u32 = 0x484;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_2: u32 = 0x488;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_3: u32 = 0x48C;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_4: u32 = 0x490;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_5: u32 = 0x494;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_6: u32 = 0x498;
pub const mmPCI_NRTR_HBW_RANGE_MASK_L_7: u32 = 0x49C;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_0: u32 = 0x4A0;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_1: u32 = 0x4A4;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_2: u32 = 0x4A8;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_3: u32 = 0x4AC;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_4: u32 = 0x4B0;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_5: u32 = 0x4B4;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_6: u32 = 0x4B8;
pub const mmPCI_NRTR_HBW_RANGE_MASK_H_7: u32 = 0x4BC;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_0: u32 = 0x4C0;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_1: u32 = 0x4C4;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_2: u32 = 0x4C8;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_3: u32 = 0x4CC;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_4: u32 = 0x4D0;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_5: u32 = 0x4D4;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_6: u32 = 0x4D8;
pub const mmPCI_NRTR_HBW_RANGE_BASE_L_7: u32 = 0x4DC;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_0: u32 = 0x4E0;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_1: u32 = 0x4E4;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_2: u32 = 0x4E8;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_3: u32 = 0x4EC;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_4: u32 = 0x4F0;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_5: u32 = 0x4F4;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_6: u32 = 0x4F8;
pub const mmPCI_NRTR_HBW_RANGE_BASE_H_7: u32 = 0x4FC;
pub const mmPCI_NRTR_LBW_RANGE_HIT: u32 = 0x500;

pub const mmPCI_NRTR_LBW_RANGE_MASK_0: u32 = 0x510;
pub const mmPCI_NRTR_LBW_RANGE_MASK_1: u32 = 0x514;
pub const mmPCI_NRTR_LBW_RANGE_MASK_2: u32 = 0x518;
pub const mmPCI_NRTR_LBW_RANGE_MASK_3: u32 = 0x51C;
pub const mmPCI_NRTR_LBW_RANGE_MASK_4: u32 = 0x520;
pub const mmPCI_NRTR_LBW_RANGE_MASK_5: u32 = 0x524;
pub const mmPCI_NRTR_LBW_RANGE_MASK_6: u32 = 0x528;
pub const mmPCI_NRTR_LBW_RANGE_MASK_7: u32 = 0x52C;
pub const mmPCI_NRTR_LBW_RANGE_MASK_8: u32 = 0x530;
pub const mmPCI_NRTR_LBW_RANGE_MASK_9: u32 = 0x534;
pub const mmPCI_NRTR_LBW_RANGE_MASK_10: u32 = 0x538;
pub const mmPCI_NRTR_LBW_RANGE_MASK_11: u32 = 0x53C;
pub const mmPCI_NRTR_LBW_RANGE_MASK_12: u32 = 0x540;
pub const mmPCI_NRTR_LBW_RANGE_MASK_13: u32 = 0x544;
pub const mmPCI_NRTR_LBW_RANGE_MASK_14: u32 = 0x548;
pub const mmPCI_NRTR_LBW_RANGE_MASK_15: u32 = 0x54C;
pub const mmPCI_NRTR_LBW_RANGE_BASE_0: u32 = 0x550;
pub const mmPCI_NRTR_LBW_RANGE_BASE_1: u32 = 0x554;
pub const mmPCI_NRTR_LBW_RANGE_BASE_2: u32 = 0x558;
pub const mmPCI_NRTR_LBW_RANGE_BASE_3: u32 = 0x55C;
pub const mmPCI_NRTR_LBW_RANGE_BASE_4: u32 = 0x560;
pub const mmPCI_NRTR_LBW_RANGE_BASE_5: u32 = 0x564;
pub const mmPCI_NRTR_LBW_RANGE_BASE_6: u32 = 0x568;
pub const mmPCI_NRTR_LBW_RANGE_BASE_7: u32 = 0x56C;
pub const mmPCI_NRTR_LBW_RANGE_BASE_8: u32 = 0x570;
pub const mmPCI_NRTR_LBW_RANGE_BASE_9: u32 = 0x574;
pub const mmPCI_NRTR_LBW_RANGE_BASE_10: u32 = 0x578;
pub const mmPCI_NRTR_LBW_RANGE_BASE_11: u32 = 0x57C;
pub const mmPCI_NRTR_LBW_RANGE_BASE_12: u32 = 0x580;
pub const mmPCI_NRTR_LBW_RANGE_BASE_13: u32 = 0x584;
pub const mmPCI_NRTR_LBW_RANGE_BASE_14: u32 = 0x588;
pub const mmPCI_NRTR_LBW_RANGE_BASE_15: u32 = 0x58C;
pub const mmPCI_NRTR_RGLTR: u32 = 0x590;
pub const mmPCI_NRTR_RGLTR_WR_RESULT: u32 = 0x594;
pub const mmPCI_NRTR_RGLTR_RD_RESULT: u32 = 0x598;
pub const mmPCI_NRTR_SCRAMB_EN: u32 = 0x600;
pub const mmPCI_NRTR_NON_LIN_SCRAMB: u32 = 0x604;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
