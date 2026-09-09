/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2021 MediaTek Inc.
 */

pub const MT6315_RP: u32 = 3;
pub const MT6315_PP: u32 = 6;
pub const MT6315_SP: u32 = 7;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mt6315Vbuck {
    MT6315_VBUCK1 = 0,
    MT6315_VBUCK2,
    MT6315_VBUCK3,
    MT6315_VBUCK4,
    MT6315_VBUCK_MAX,
}

/* Register */
pub const MT6315_TOP2_ELR7: u32 = 0x139;
pub const MT6315_TOP_TMA_KEY: u32 = 0x39F;
pub const MT6315_TOP_TMA_KEY_H: u32 = 0x3A0;
pub const MT6315_BUCK_TOP_CON0: u32 = 0x1440;
pub const MT6315_BUCK_TOP_CON1: u32 = 0x1443;
pub const MT6315_BUCK_TOP_ELR0: u32 = 0x1449;
pub const MT6315_BUCK_TOP_ELR2: u32 = 0x144B;
pub const MT6315_BUCK_TOP_ELR4: u32 = 0x144D;
pub const MT6315_BUCK_TOP_ELR6: u32 = 0x144F;
pub const MT6315_VBUCK1_DBG0: u32 = 0x1499;
pub const MT6315_VBUCK1_DBG4: u32 = 0x149D;
pub const MT6315_VBUCK2_DBG0: u32 = 0x1519;
pub const MT6315_VBUCK2_DBG4: u32 = 0x151D;
pub const MT6315_VBUCK3_DBG0: u32 = 0x1599;
pub const MT6315_VBUCK3_DBG4: u32 = 0x159D;
pub const MT6315_VBUCK4_DBG0: u32 = 0x1619;
pub const MT6315_VBUCK4_DBG4: u32 = 0x161D;
pub const MT6315_BUCK_TOP_4PHASE_ANA_CON42: u32 = 0x16B1;

pub const PROTECTION_KEY_H: u32 = 0x9C;
pub const PROTECTION_KEY: u32 = 0xEA;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
