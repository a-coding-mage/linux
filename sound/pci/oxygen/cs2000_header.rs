// SPDX-License-Identifier: GPL-2.0

pub const CS2000_DEV_ID: u8 = 0x01;
pub const CS2000_DEV_CTRL: u8 = 0x02;
pub const CS2000_DEV_CFG_1: u8 = 0x03;
pub const CS2000_DEV_CFG_2: u8 = 0x04;
pub const CS2000_GLOBAL_CFG: u8 = 0x05;
pub const CS2000_RATIO_0: u8 = 0x06; // 32 bits, big endian
pub const CS2000_RATIO_1: u8 = 0x0a;
pub const CS2000_RATIO_2: u8 = 0x0e;
pub const CS2000_RATIO_3: u8 = 0x12;
pub const CS2000_FUN_CFG_1: u8 = 0x16;
pub const CS2000_FUN_CFG_2: u8 = 0x17;
pub const CS2000_FUN_CFG_3: u8 = 0x1e;

// DEV_ID
pub const CS2000_DEVICE_MASK: u8 = 0xf8;
pub const CS2000_REVISION_MASK: u8 = 0x07;

// DEV_CTRL
pub const CS2000_UNLOCK: u8 = 0x80;
pub const CS2000_AUX_OUT_DIS: u8 = 0x02;
pub const CS2000_CLK_OUT_DIS: u8 = 0x01;

// DEV_CFG_1
pub const CS2000_R_MOD_SEL_MASK: u8 = 0xe0;
pub const CS2000_R_MOD_SEL_1: u8 = 0x00;
pub const CS2000_R_MOD_SEL_2: u8 = 0x20;
pub const CS2000_R_MOD_SEL_4: u8 = 0x40;
pub const CS2000_R_MOD_SEL_8: u8 = 0x60;
pub const CS2000_R_MOD_SEL_1_2: u8 = 0x80;
pub const CS2000_R_MOD_SEL_1_4: u8 = 0xa0;
pub const CS2000_R_MOD_SEL_1_8: u8 = 0xc0;
pub const CS2000_R_MOD_SEL_1_16: u8 = 0xe0;
pub const CS2000_R_SEL_MASK: u8 = 0x18;
pub const CS2000_R_SEL_SHIFT: u8 = 3;
pub const CS2000_AUX_OUT_SRC_MASK: u8 = 0x06;
pub const CS2000_AUX_OUT_SRC_REF_CLK: u8 = 0x00;
pub const CS2000_AUX_OUT_SRC_CLK_IN: u8 = 0x02;
pub const CS2000_AUX_OUT_SRC_CLK_OUT: u8 = 0x04;
pub const CS2000_AUX_OUT_SRC_PLL_LOCK: u8 = 0x06;
pub const CS2000_EN_DEV_CFG_1: u8 = 0x01;

// DEV_CFG_2
pub const CS2000_LOCK_CLK_MASK: u8 = 0x06;
pub const CS2000_LOCK_CLK_SHIFT: u8 = 1;
pub const CS2000_FRAC_N_SRC_MASK: u8 = 0x01;
pub const CS2000_FRAC_N_SRC_STATIC: u8 = 0x00;
pub const CS2000_FRAC_N_SRC_DYNAMIC: u8 = 0x01;

// GLOBAL_CFG
pub const CS2000_FREEZE: u8 = 0x08;
pub const CS2000_EN_DEV_CFG_2: u8 = 0x01;

// FUN_CFG_1
pub const CS2000_CLK_SKIP_EN: u8 = 0x80;
pub const CS2000_AUX_LOCK_CFG_MASK: u8 = 0x40;
pub const CS2000_AUX_LOCK_CFG_PP_HIGH: u8 = 0x00;
pub const CS2000_AUX_LOCK_CFG_OD_LOW: u8 = 0x40;
pub const CS2000_REF_CLK_DIV_MASK: u8 = 0x18;
pub const CS2000_REF_CLK_DIV_4: u8 = 0x00;
pub const CS2000_REF_CLK_DIV_2: u8 = 0x08;
pub const CS2000_REF_CLK_DIV_1: u8 = 0x10;

// FUN_CFG_2
pub const CS2000_CLK_OUT_UNL: u8 = 0x10;
pub const CS2000_L_F_RATIO_CFG_MASK: u8 = 0x08;
pub const CS2000_L_F_RATIO_CFG_20_12: u8 = 0x00;
pub const CS2000_L_F_RATIO_CFG_12_20: u8 = 0x08;

// FUN_CFG_3
pub const CS2000_CLK_IN_BW_MASK: u8 = 0x70;
pub const CS2000_CLK_IN_BW_1: u8 = 0x00;
pub const CS2000_CLK_IN_BW_2: u8 = 0x10;
pub const CS2000_CLK_IN_BW_4: u8 = 0x20;
pub const CS2000_CLK_IN_BW_8: u8 = 0x30;
pub const CS2000_CLK_IN_BW_16: u8 = 0x40;
pub const CS2000_CLK_IN_BW_32: u8 = 0x50;
pub const CS2000_CLK_IN_BW_64: u8 = 0x60;
pub const CS2000_CLK_IN_BW_128: u8 = 0x70;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
