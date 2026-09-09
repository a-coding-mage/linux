/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SB1250 Board Support Package
 * L2 Cache constants and macros (translated from sb1250_l2c.h)
 */

// Dependency supplied by the translated sb1250_defs header:
// use crate::sb1250_defs::*;

/* Level 2 Cache Tag register (Table 5-3) */
pub const S_L2C_TAG_MBZ: u32 = 0;
pub const M_L2C_TAG_MBZ: u64 = _SB_MAKEMASK(5, S_L2C_TAG_MBZ);
pub const S_L2C_TAG_INDEX: u32 = 5;
pub const M_L2C_TAG_INDEX: u64 = _SB_MAKEMASK(12, S_L2C_TAG_INDEX);
pub fn V_L2C_TAG_INDEX(x: u64) -> u64 { _SB_MAKEVALUE(x, S_L2C_TAG_INDEX) }
pub fn G_L2C_TAG_INDEX(x: u64) -> u64 { _SB_GETVALUE(x, S_L2C_TAG_INDEX, M_L2C_TAG_INDEX) }
pub const S_L2C_TAG_TAG: u32 = 17;
pub const M_L2C_TAG_TAG: u64 = _SB_MAKEMASK(23, S_L2C_TAG_TAG);
pub fn V_L2C_TAG_TAG(x: u64) -> u64 { _SB_MAKEVALUE(x, S_L2C_TAG_TAG) }
pub fn G_L2C_TAG_TAG(x: u64) -> u64 { _SB_GETVALUE(x, S_L2C_TAG_TAG, M_L2C_TAG_TAG) }
pub const S_L2C_TAG_ECC: u32 = 40;
pub const M_L2C_TAG_ECC: u64 = _SB_MAKEMASK(6, S_L2C_TAG_ECC);
pub fn V_L2C_TAG_ECC(x: u64) -> u64 { _SB_MAKEVALUE(x, S_L2C_TAG_ECC) }
pub fn G_L2C_TAG_ECC(x: u64) -> u64 { _SB_GETVALUE(x, S_L2C_TAG_ECC, M_L2C_TAG_ECC) }
pub const S_L2C_TAG_WAY: u32 = 46;
pub const M_L2C_TAG_WAY: u64 = _SB_MAKEMASK(2, S_L2C_TAG_WAY);
pub fn V_L2C_TAG_WAY(x: u64) -> u64 { _SB_MAKEVALUE(x, S_L2C_TAG_WAY) }
pub fn G_L2C_TAG_WAY(x: u64) -> u64 { _SB_GETVALUE(x, S_L2C_TAG_WAY, M_L2C_TAG_WAY) }
pub const M_L2C_TAG_DIRTY: u64 = _SB_MAKEMASK1(48);
pub const M_L2C_TAG_VALID: u64 = _SB_MAKEMASK1(49);

/* Format of level 2 cache management address (table 5-2) */
pub const S_L2C_MGMT_INDEX: u32 = 5;
pub const M_L2C_MGMT_INDEX: u64 = _SB_MAKEMASK(12, S_L2C_MGMT_INDEX);
pub fn V_L2C_MGMT_INDEX(x: u64) -> u64 { _SB_MAKEVALUE(x, S_L2C_MGMT_INDEX) }
pub fn G_L2C_MGMT_INDEX(x: u64) -> u64 { _SB_GETVALUE(x, S_L2C_MGMT_INDEX, M_L2C_MGMT_INDEX) }
pub const S_L2C_MGMT_QUADRANT: u32 = 15;
pub const M_L2C_MGMT_QUADRANT: u64 = _SB_MAKEMASK(2, S_L2C_MGMT_QUADRANT);
pub fn V_L2C_MGMT_QUADRANT(x: u64) -> u64 { _SB_MAKEVALUE(x, S_L2C_MGMT_QUADRANT) }
pub fn G_L2C_MGMT_QUADRANT(x: u64) -> u64 { _SB_GETVALUE(x, S_L2C_MGMT_QUADRANT, M_L2C_MGMT_QUADRANT) }
pub const S_L2C_MGMT_HALF: u32 = 16;
pub const M_L2C_MGMT_HALF: u64 = _SB_MAKEMASK(1, S_L2C_MGMT_HALF);
pub const S_L2C_MGMT_WAY: u32 = 17;
pub const M_L2C_MGMT_WAY: u64 = _SB_MAKEMASK(2, S_L2C_MGMT_WAY);
pub fn V_L2C_MGMT_WAY(x: u64) -> u64 { _SB_MAKEVALUE(x, S_L2C_MGMT_WAY) }
pub fn G_L2C_MGMT_WAY(x: u64) -> u64 { _SB_GETVALUE(x, S_L2C_MGMT_WAY, M_L2C_MGMT_WAY) }
pub const S_L2C_MGMT_ECC_DIAG: u32 = 21;
pub const M_L2C_MGMT_ECC_DIAG: u64 = _SB_MAKEMASK(2, S_L2C_MGMT_ECC_DIAG);
pub fn V_L2C_MGMT_ECC_DIAG(x: u64) -> u64 { _SB_MAKEVALUE(x, S_L2C_MGMT_ECC_DIAG) }
pub fn G_L2C_MGMT_ECC_DIAG(x: u64) -> u64 { _SB_GETVALUE(x, S_L2C_MGMT_ECC_DIAG, M_L2C_MGMT_ECC_DIAG) }
pub const S_L2C_MGMT_TAG: u32 = 23;
pub const M_L2C_MGMT_TAG: u64 = _SB_MAKEMASK(4, S_L2C_MGMT_TAG);
pub fn V_L2C_MGMT_TAG(x: u64) -> u64 { _SB_MAKEVALUE(x, S_L2C_MGMT_TAG) }
pub fn G_L2C_MGMT_TAG(x: u64) -> u64 { _SB_GETVALUE(x, S_L2C_MGMT_TAG, M_L2C_MGMT_TAG) }
pub const M_L2C_MGMT_DIRTY: u64 = _SB_MAKEMASK1(19);
pub const M_L2C_MGMT_VALID: u64 = _SB_MAKEMASK1(20);
pub const A_L2C_MGMT_TAG_BASE: u64 = 0x00D0000000;
pub const L2C_ENTRIES_PER_WAY: u32 = 4096;
pub const L2C_NUM_WAYS: u32 = 4;

// C preprocessor condition: SIBYTE_HDR_FEATURE(1250, PASS3) || SIBYTE_HDR_FEATURE(112x, PASS1)
pub const S_L2C_MISC_NO_WAY: u32 = 10;
pub const M_L2C_MISC_NO_WAY: u64 = _SB_MAKEMASK(4, S_L2C_MISC_NO_WAY);
pub fn V_L2C_MISC_NO_WAY(x: u64) -> u64 { _SB_MAKEVALUE(x, S_L2C_MISC_NO_WAY) }
pub fn G_L2C_MISC_NO_WAY(x: u64) -> u64 { _SB_GETVALUE(x, S_L2C_MISC_NO_WAY, M_L2C_MISC_NO_WAY) }
pub const M_L2C_MISC_ECC_CLEANUP_DIS: u64 = _SB_MAKEMASK1(9);
pub const M_L2C_MISC_MC_PRIO_LOW: u64 = _SB_MAKEMASK1(8);
pub const M_L2C_MISC_SOFT_DISABLE_T: u64 = _SB_MAKEMASK1(7);
pub const M_L2C_MISC_SOFT_DISABLE_B: u64 = _SB_MAKEMASK1(6);
pub const M_L2C_MISC_SOFT_DISABLE_R: u64 = _SB_MAKEMASK1(5);
pub const M_L2C_MISC_SOFT_DISABLE_L: u64 = _SB_MAKEMASK1(4);
pub const M_L2C_MISC_SCACHE_DISABLE_T: u64 = _SB_MAKEMASK1(3);
pub const M_L2C_MISC_SCACHE_DISABLE_B: u64 = _SB_MAKEMASK1(2);
pub const M_L2C_MISC_SCACHE_DISABLE_R: u64 = _SB_MAKEMASK1(1);
pub const M_L2C_MISC_SCACHE_DISABLE_L: u64 = _SB_MAKEMASK1(0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
