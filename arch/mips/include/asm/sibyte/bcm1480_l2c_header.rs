/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * BCM1280/BCM1480 Board Support Package
 * L2 Cache constants and macros
 *
 * Translated from bcm1480_l2c.h.  The original include supplied the
 * _SB_MAKEMASK, _SB_MAKEMASK1, _SB_MAKEVALUE, and _SB_GETVALUE helpers;
 * their direct bit-operation equivalents are kept local here.
 */

#[inline(always)]
pub const fn _sb_makemask(width: u32, shift: u32) -> u64 {
    ((1u64 << width) - 1) << shift
}

#[inline(always)]
pub const fn _sb_makemask1(bit: u32) -> u64 {
    1u64 << bit
}

#[inline(always)]
pub const fn _sb_makevalue(value: u64, shift: u32) -> u64 {
    value << shift
}

#[inline(always)]
pub const fn _sb_getvalue(value: u64, shift: u32, mask: u64) -> u64 {
    (value & mask) >> shift
}

pub const S_BCM1480_L2C_MGMT_INDEX: u32 = 5;
pub const M_BCM1480_L2C_MGMT_INDEX: u64 = _sb_makemask(12, S_BCM1480_L2C_MGMT_INDEX);
#[inline(always)] pub const fn V_BCM1480_L2C_MGMT_INDEX(x: u64) -> u64 { _sb_makevalue(x, S_BCM1480_L2C_MGMT_INDEX) }
#[inline(always)] pub const fn G_BCM1480_L2C_MGMT_INDEX(x: u64) -> u64 { _sb_getvalue(x, S_BCM1480_L2C_MGMT_INDEX, M_BCM1480_L2C_MGMT_INDEX) }

pub const S_BCM1480_L2C_MGMT_WAY: u32 = 17;
pub const M_BCM1480_L2C_MGMT_WAY: u64 = _sb_makemask(3, S_BCM1480_L2C_MGMT_WAY);
#[inline(always)] pub const fn V_BCM1480_L2C_MGMT_WAY(x: u64) -> u64 { _sb_makevalue(x, S_BCM1480_L2C_MGMT_WAY) }
#[inline(always)] pub const fn G_BCM1480_L2C_MGMT_WAY(x: u64) -> u64 { _sb_getvalue(x, S_BCM1480_L2C_MGMT_WAY, M_BCM1480_L2C_MGMT_WAY) }
pub const M_BCM1480_L2C_MGMT_DIRTY: u64 = _sb_makemask1(20);
pub const M_BCM1480_L2C_MGMT_VALID: u64 = _sb_makemask1(21);

pub const S_BCM1480_L2C_MGMT_ECC_DIAG: u32 = 22;
pub const M_BCM1480_L2C_MGMT_ECC_DIAG: u64 = _sb_makemask(2, S_BCM1480_L2C_MGMT_ECC_DIAG);
#[inline(always)] pub const fn V_BCM1480_L2C_MGMT_ECC_DIAG(x: u64) -> u64 { _sb_makevalue(x, S_BCM1480_L2C_MGMT_ECC_DIAG) }
#[inline(always)] pub const fn G_BCM1480_L2C_MGMT_ECC_DIAG(x: u64) -> u64 { _sb_getvalue(x, S_BCM1480_L2C_MGMT_ECC_DIAG, M_BCM1480_L2C_MGMT_ECC_DIAG) }
pub const A_BCM1480_L2C_MGMT_TAG_BASE: u64 = 0x00D0000000;
pub const BCM1480_L2C_ENTRIES_PER_WAY: u32 = 4096;
pub const BCM1480_L2C_NUM_WAYS: u32 = 8;

pub const S_BCM1480_L2C_TAG_MBZ: u32 = 0;
pub const M_BCM1480_L2C_TAG_MBZ: u64 = _sb_makemask(5, S_BCM1480_L2C_TAG_MBZ);
pub const S_BCM1480_L2C_TAG_INDEX: u32 = 5;
pub const M_BCM1480_L2C_TAG_INDEX: u64 = _sb_makemask(12, S_BCM1480_L2C_TAG_INDEX);
#[inline(always)] pub const fn V_BCM1480_L2C_TAG_INDEX(x: u64) -> u64 { _sb_makevalue(x, S_BCM1480_L2C_TAG_INDEX) }
#[inline(always)] pub const fn G_BCM1480_L2C_TAG_INDEX(x: u64) -> u64 { _sb_getvalue(x, S_BCM1480_L2C_TAG_INDEX, M_BCM1480_L2C_TAG_INDEX) }
pub const S_BCM1480_L2C_TAG_TAG: u32 = 17;
pub const M_BCM1480_L2C_TAG_TAG: u64 = _sb_makemask(23, S_BCM1480_L2C_TAG_TAG);
#[inline(always)] pub const fn V_BCM1480_L2C_TAG_TAG(x: u64) -> u64 { _sb_makevalue(x, S_BCM1480_L2C_TAG_TAG) }
#[inline(always)] pub const fn G_BCM1480_L2C_TAG_TAG(x: u64) -> u64 { _sb_getvalue(x, S_BCM1480_L2C_TAG_TAG, M_BCM1480_L2C_TAG_TAG) }
pub const S_BCM1480_L2C_TAG_ECC: u32 = 40;
pub const M_BCM1480_L2C_TAG_ECC: u64 = _sb_makemask(6, S_BCM1480_L2C_TAG_ECC);
#[inline(always)] pub const fn V_BCM1480_L2C_TAG_ECC(x: u64) -> u64 { _sb_makevalue(x, S_BCM1480_L2C_TAG_ECC) }
#[inline(always)] pub const fn G_BCM1480_L2C_TAG_ECC(x: u64) -> u64 { _sb_getvalue(x, S_BCM1480_L2C_TAG_ECC, M_BCM1480_L2C_TAG_ECC) }
pub const S_BCM1480_L2C_TAG_WAY: u32 = 46;
pub const M_BCM1480_L2C_TAG_WAY: u64 = _sb_makemask(3, S_BCM1480_L2C_TAG_WAY);
#[inline(always)] pub const fn V_BCM1480_L2C_TAG_WAY(x: u64) -> u64 { _sb_makevalue(x, S_BCM1480_L2C_TAG_WAY) }
#[inline(always)] pub const fn G_BCM1480_L2C_TAG_WAY(x: u64) -> u64 { _sb_getvalue(x, S_BCM1480_L2C_TAG_WAY, M_BCM1480_L2C_TAG_WAY) }
pub const M_BCM1480_L2C_TAG_DIRTY: u64 = _sb_makemask1(49);
pub const M_BCM1480_L2C_TAG_VALID: u64 = _sb_makemask1(50);
pub const S_BCM1480_L2C_DATA_ECC: u32 = 51;
pub const M_BCM1480_L2C_DATA_ECC: u64 = _sb_makemask(10, S_BCM1480_L2C_DATA_ECC);
#[inline(always)] pub const fn V_BCM1480_L2C_DATA_ECC(x: u64) -> u64 { _sb_makevalue(x, S_BCM1480_L2C_DATA_ECC) }
#[inline(always)] pub const fn G_BCM1480_L2C_DATA_ECC(x: u64) -> u64 { _sb_getvalue(x, S_BCM1480_L2C_DATA_ECC, M_BCM1480_L2C_DATA_ECC) }

pub const S_BCM1480_L2C_MISC0_WAY_REMOTE: u32 = 0;
pub const M_BCM1480_L2C_MISC0_WAY_REMOTE: u64 = _sb_makemask(8, 0);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC0_WAY_REMOTE(x: u64) -> u64 { _sb_getvalue(x, 0, M_BCM1480_L2C_MISC0_WAY_REMOTE) }
pub const S_BCM1480_L2C_MISC0_WAY_LOCAL: u32 = 8;
pub const M_BCM1480_L2C_MISC0_WAY_LOCAL: u64 = _sb_makemask(8, 8);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC0_WAY_LOCAL(x: u64) -> u64 { _sb_getvalue(x, 8, M_BCM1480_L2C_MISC0_WAY_LOCAL) }
pub const S_BCM1480_L2C_MISC0_WAY_ENABLE: u32 = 16;
pub const M_BCM1480_L2C_MISC0_WAY_ENABLE: u64 = _sb_makemask(8, 16);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC0_WAY_ENABLE(x: u64) -> u64 { _sb_getvalue(x, 16, M_BCM1480_L2C_MISC0_WAY_ENABLE) }
pub const S_BCM1480_L2C_MISC0_CACHE_DISABLE: u32 = 24;
pub const M_BCM1480_L2C_MISC0_CACHE_DISABLE: u64 = _sb_makemask(2, 24);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC0_CACHE_DISABLE(x: u64) -> u64 { _sb_getvalue(x, 24, M_BCM1480_L2C_MISC0_CACHE_DISABLE) }
pub const S_BCM1480_L2C_MISC0_CACHE_QUAD: u32 = 26;
pub const M_BCM1480_L2C_MISC0_CACHE_QUAD: u64 = _sb_makemask(2, 26);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC0_CACHE_QUAD(x: u64) -> u64 { _sb_getvalue(x, 26, M_BCM1480_L2C_MISC0_CACHE_QUAD) }
pub const S_BCM1480_L2C_MISC0_MC_PRIORITY: u32 = 30;
pub const M_BCM1480_L2C_MISC0_MC_PRIORITY: u64 = _sb_makemask1(30);
pub const S_BCM1480_L2C_MISC0_ECC_CLEANUP: u32 = 31;
pub const M_BCM1480_L2C_MISC0_ECC_CLEANUP: u64 = _sb_makemask1(31);

pub const S_BCM1480_L2C_MISC1_WAY_AGENT_0: u32 = 0;
pub const M_BCM1480_L2C_MISC1_WAY_AGENT_0: u64 = _sb_makemask(8, 0);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC1_WAY_AGENT_0(x: u64) -> u64 { _sb_getvalue(x, 0, M_BCM1480_L2C_MISC1_WAY_AGENT_0) }
pub const S_BCM1480_L2C_MISC1_WAY_AGENT_1: u32 = 8;
pub const M_BCM1480_L2C_MISC1_WAY_AGENT_1: u64 = _sb_makemask(8, 8);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC1_WAY_AGENT_1(x: u64) -> u64 { _sb_getvalue(x, 8, M_BCM1480_L2C_MISC1_WAY_AGENT_1) }
pub const S_BCM1480_L2C_MISC1_WAY_AGENT_2: u32 = 16;
pub const M_BCM1480_L2C_MISC1_WAY_AGENT_2: u64 = _sb_makemask(8, 16);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC1_WAY_AGENT_2(x: u64) -> u64 { _sb_getvalue(x, 16, M_BCM1480_L2C_MISC1_WAY_AGENT_2) }
pub const S_BCM1480_L2C_MISC1_WAY_AGENT_3: u32 = 24;
pub const M_BCM1480_L2C_MISC1_WAY_AGENT_3: u64 = _sb_makemask(8, 24);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC1_WAY_AGENT_3(x: u64) -> u64 { _sb_getvalue(x, 24, M_BCM1480_L2C_MISC1_WAY_AGENT_3) }
pub const S_BCM1480_L2C_MISC1_WAY_AGENT_4: u32 = 32;
pub const M_BCM1480_L2C_MISC1_WAY_AGENT_4: u64 = _sb_makemask(8, 32);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC1_WAY_AGENT_4(x: u64) -> u64 { _sb_getvalue(x, 32, M_BCM1480_L2C_MISC1_WAY_AGENT_4) }

pub const S_BCM1480_L2C_MISC2_WAY_AGENT_8: u32 = 0;
pub const M_BCM1480_L2C_MISC2_WAY_AGENT_8: u64 = _sb_makemask(8, 0);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC2_WAY_AGENT_8(x: u64) -> u64 { _sb_getvalue(x, 0, M_BCM1480_L2C_MISC2_WAY_AGENT_8) }
pub const S_BCM1480_L2C_MISC2_WAY_AGENT_9: u32 = 8;
pub const M_BCM1480_L2C_MISC2_WAY_AGENT_9: u64 = _sb_makemask(8, 8);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC2_WAY_AGENT_9(x: u64) -> u64 { _sb_getvalue(x, 8, M_BCM1480_L2C_MISC2_WAY_AGENT_9) }
pub const S_BCM1480_L2C_MISC2_WAY_AGENT_A: u32 = 16;
pub const M_BCM1480_L2C_MISC2_WAY_AGENT_A: u64 = _sb_makemask(8, 16);
#[inline(always)] pub const fn G_BCM1480_L2C_MISC2_WAY_AGENT_A(x: u64) -> u64 { _sb_getvalue(x, 16, M_BCM1480_L2C_MISC2_WAY_AGENT_A) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
