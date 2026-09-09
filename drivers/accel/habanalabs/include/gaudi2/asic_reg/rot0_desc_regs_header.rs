/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2020 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

/************************************
 ** This is an auto-generated file **
 **       DO NOT EDIT BELOW        **
 ************************************/

/*
 *****************************************
 *   ROT0_DESC
 *   (Prototype: ROT_DESC)
 *****************************************
 */

pub const mmROT0_DESC_CONTEXT_ID: u32 = 0x4E0B100;
pub const mmROT0_DESC_IN_IMG_START_ADDR_L: u32 = 0x4E0B104;
pub const mmROT0_DESC_IN_IMG_START_ADDR_H: u32 = 0x4E0B108;
pub const mmROT0_DESC_OUT_IMG_START_ADDR_L: u32 = 0x4E0B10C;
pub const mmROT0_DESC_OUT_IMG_START_ADDR_H: u32 = 0x4E0B110;
pub const mmROT0_DESC_CFG: u32 = 0x4E0B114;
pub const mmROT0_DESC_IM_READ_SLOPE: u32 = 0x4E0B118;
pub const mmROT0_DESC_SIN_D: u32 = 0x4E0B11C;
pub const mmROT0_DESC_COS_D: u32 = 0x4E0B120;
pub const mmROT0_DESC_IN_IMG: u32 = 0x4E0B124;
pub const mmROT0_DESC_IN_STRIDE: u32 = 0x4E0B128;
pub const mmROT0_DESC_IN_STRIPE: u32 = 0x4E0B12C;
pub const mmROT0_DESC_IN_CENTER: u32 = 0x4E0B130;
pub const mmROT0_DESC_OUT_IMG: u32 = 0x4E0B134;
pub const mmROT0_DESC_OUT_STRIDE: u32 = 0x4E0B138;
pub const mmROT0_DESC_OUT_STRIPE: u32 = 0x4E0B13C;
pub const mmROT0_DESC_OUT_CENTER: u32 = 0x4E0B140;
pub const mmROT0_DESC_BACKGROUND: u32 = 0x4E0B144;
pub const mmROT0_DESC_CPL_MSG_EN: u32 = 0x4E0B148;
pub const mmROT0_DESC_IDLE_STATE: u32 = 0x4E0B14C;
pub const mmROT0_DESC_CPL_MSG_ADDR: u32 = 0x4E0B150;
pub const mmROT0_DESC_CPL_MSG_DATA: u32 = 0x4E0B154;
pub const mmROT0_DESC_CPL_MSG_AWUSER: u32 = 0x4E0B158;
pub const mmROT0_DESC_X_I_START_OFFSET: u32 = 0x4E0B15C;
pub const mmROT0_DESC_X_I_START_OFFSET_FLIP: u32 = 0x4E0B160;
pub const mmROT0_DESC_X_I_FIRST: u32 = 0x4E0B164;
pub const mmROT0_DESC_Y_I_FIRST: u32 = 0x4E0B168;
pub const mmROT0_DESC_Y_I: u32 = 0x4E0B16C;
pub const mmROT0_DESC_OUT_STRIPE_SIZE: u32 = 0x4E0B170;
pub const mmROT0_DESC_RSB_CFG_0: u32 = 0x4E0B174;
pub const mmROT0_DESC_RSB_PAD_VAL: u32 = 0x4E0B178;
pub const mmROT0_DESC_HBW_ARUSER_HI: u32 = 0x4E0B17C;
pub const mmROT0_DESC_HBW_ARUSER_LO: u32 = 0x4E0B180;
pub const mmROT0_DESC_HBW_AWUSER_HI: u32 = 0x4E0B184;
pub const mmROT0_DESC_HBW_AWUSER_LO: u32 = 0x4E0B188;
pub const mmROT0_DESC_OWM_CFG: u32 = 0x4E0B18C;
pub const mmROT0_DESC_CTRL_CFG: u32 = 0x4E0B190;
pub const mmROT0_DESC_PIXEL_PAD: u32 = 0x4E0B194;
pub const mmROT0_DESC_PREC_SHIFT: u32 = 0x4E0B198;
pub const mmROT0_DESC_MAX_VAL: u32 = 0x4E0B19C;
pub const mmROT0_DESC_A0_M11: u32 = 0x4E0B1A0;
pub const mmROT0_DESC_A1_M12: u32 = 0x4E0B1A4;
pub const mmROT0_DESC_A2: u32 = 0x4E0B1A8;
pub const mmROT0_DESC_B0_M21: u32 = 0x4E0B1AC;
pub const mmROT0_DESC_B1_M22: u32 = 0x4E0B1B0;
pub const mmROT0_DESC_B2: u32 = 0x4E0B1B4;
pub const mmROT0_DESC_C0: u32 = 0x4E0B1B8;
pub const mmROT0_DESC_C1: u32 = 0x4E0B1BC;
pub const mmROT0_DESC_C2: u32 = 0x4E0B1C0;
pub const mmROT0_DESC_D0: u32 = 0x4E0B1C4;
pub const mmROT0_DESC_D1: u32 = 0x4E0B1C8;
pub const mmROT0_DESC_D2: u32 = 0x4E0B1CC;
pub const mmROT0_DESC_INV_PROC_SIZE_M_1: u32 = 0x4E0B1D0;
pub const mmROT0_DESC_MESH_IMG_START_ADDR_L: u32 = 0x4E0B1D4;
pub const mmROT0_DESC_MESH_IMG_START_ADDR_H: u32 = 0x4E0B1D8;
pub const mmROT0_DESC_MESH_IMG: u32 = 0x4E0B1DC;
pub const mmROT0_DESC_MESH_STRIDE: u32 = 0x4E0B1E0;
pub const mmROT0_DESC_MESH_STRIPE: u32 = 0x4E0B1E4;
pub const mmROT0_DESC_MESH_CTRL: u32 = 0x4E0B1E8;
pub const mmROT0_DESC_MESH_GH: u32 = 0x4E0B1EC;
pub const mmROT0_DESC_MESH_GV: u32 = 0x4E0B1F0;
pub const mmROT0_DESC_MRSB_CFG_0: u32 = 0x4E0B1F4;
pub const mmROT0_DESC_MRSB_PAD_VAL: u32 = 0x4E0B1F8;
pub const mmROT0_DESC_BUF_CFG: u32 = 0x4E0B1FC;
pub const mmROT0_DESC_CID_OFFSET: u32 = 0x4E0B200;
pub const mmROT0_DESC_PUSH_DESC: u32 = 0x4E0B204;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
