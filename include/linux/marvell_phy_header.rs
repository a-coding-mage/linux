/* SPDX-License-Identifier: GPL-2.0 */

/* Mask used for ID comparisons */
pub const MARVELL_PHY_ID_MASK: u32 = 0xfffffff0;

/* Known PHY IDs */
pub const MARVELL_PHY_ID_88E1101: u32 = 0x01410c60;
pub const MARVELL_PHY_ID_88E3082: u32 = 0x01410c80;
pub const MARVELL_PHY_ID_88E1112: u32 = 0x01410c90;
pub const MARVELL_PHY_ID_88E1111: u32 = 0x01410cc0;
pub const MARVELL_PHY_ID_88E1118: u32 = 0x01410e10;
pub const MARVELL_PHY_ID_88E1121R: u32 = 0x01410cb0;
pub const MARVELL_PHY_ID_88E1145: u32 = 0x01410cd0;
pub const MARVELL_PHY_ID_88E1149R: u32 = 0x01410e50;
pub const MARVELL_PHY_ID_88E1240: u32 = 0x01410e30;
pub const MARVELL_PHY_ID_88E1318S: u32 = 0x01410e90;
pub const MARVELL_PHY_ID_88E1340S: u32 = 0x01410dc0;
pub const MARVELL_PHY_ID_88E1116R: u32 = 0x01410e40;
pub const MARVELL_PHY_ID_88E1510: u32 = 0x01410dd0;
pub const MARVELL_PHY_ID_88E1540: u32 = 0x01410eb0;
pub const MARVELL_PHY_ID_88E1545: u32 = 0x01410ea0;
pub const MARVELL_PHY_ID_88E1548P: u32 = 0x01410ec0;
pub const MARVELL_PHY_ID_88E3016: u32 = 0x01410e60;
pub const MARVELL_PHY_ID_88X3310: u32 = 0x002b09a0;
pub const MARVELL_PHY_ID_88E2110: u32 = 0x002b09b0;
pub const MARVELL_PHY_ID_88X2222: u32 = 0x01410f10;
pub const MARVELL_PHY_ID_88Q2110: u32 = 0x002b0980;
pub const MARVELL_PHY_ID_88Q2220: u32 = 0x002b0b20;

/* Marvel 88E1111 in Finisar SFP module with modified PHY ID */
pub const MARVELL_PHY_ID_88E1111_FINISAR: u32 = 0x01ff0cc0;

/* ID from 88E6020, assumed to be the same for the whole 6250 family */
pub const MARVELL_PHY_ID_88E6250_FAMILY: u32 = 0x01410db0;
/* These Ethernet switch families contain embedded PHYs, but they do
 * not have a model ID. So the switch driver traps reads to the ID2
 * register and returns the switch family ID
 */
pub const MARVELL_PHY_ID_88E6341_FAMILY: u32 = 0x01410f41;
pub const MARVELL_PHY_ID_88E6390_FAMILY: u32 = 0x01410f90;
pub const MARVELL_PHY_ID_88E6393_FAMILY: u32 = 0x002b0b9b;

#[inline]
pub const fn MARVELL_PHY_FAMILY_ID(id: u32) -> u32 {
    id >> 4
}

/* struct phy_device dev_flags definitions */
pub const MARVELL_PHY_M1145_FLAGS_RESISTANCE: u32 = 0x00000001;
pub const MARVELL_PHY_M1118_DNS323_LEDS: u32 = 0x00000002;
pub const MARVELL_PHY_LED0_LINK_LED1_ACTIVE: u32 = 0x00000004;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
