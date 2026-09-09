/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/micrel_phy.h
 *
 * Micrel PHY IDs
 */

pub const MICREL_OUI: u32 = 0x0885;

pub const MICREL_PHY_ID_MASK: u32 = 0x00fffff0;

pub const PHY_ID_KSZ8873MLL: u32 = 0x000e7237;
pub const PHY_ID_KSZ9021: u32 = 0x00221610;
pub const PHY_ID_KSZ9021RLRN: u32 = 0x00221611;
pub const PHY_ID_KS8737: u32 = 0x00221720;
pub const PHY_ID_KSZ8021: u32 = 0x00221555;
pub const PHY_ID_KSZ8031: u32 = 0x00221556;
pub const PHY_ID_KSZ8041: u32 = 0x00221510;
/* undocumented */
pub const PHY_ID_KSZ8041RNLI: u32 = 0x00221537;
pub const PHY_ID_KSZ8051: u32 = 0x00221550;
/* same id: ks8001 Rev. A/B, and ks8721 Rev 3. */
pub const PHY_ID_KSZ8001: u32 = 0x0022161A;
/* same id: KS8081, KS8091 */
pub const PHY_ID_KSZ8081: u32 = 0x00221560;
pub const PHY_ID_KSZ8061: u32 = 0x00221570;
pub const PHY_ID_KSZ9031: u32 = 0x00221620;
pub const PHY_ID_KSZ9131: u32 = 0x00221640;
pub const PHY_ID_LAN8814: u32 = 0x00221660;
pub const PHY_ID_LAN8804: u32 = 0x00221670;
pub const PHY_ID_LAN8841: u32 = 0x00221650;
pub const PHY_ID_LAN8842: u32 = 0x002216C0;
pub const PHY_ID_LAN9645X: u32 = 0x002216D0;

pub const PHY_ID_KSZ886X: u32 = 0x00221430;
pub const PHY_ID_KSZ8863: u32 = 0x00221435;

pub const PHY_ID_KSZ87XX: u32 = 0x00221550;

pub const PHY_ID_KSZ9477: u32 = 0x00221631;

/* struct phy_device dev_flags definitions */
pub const MICREL_PHY_50MHZ_CLK: u32 = 1 << 0;
pub const MICREL_PHY_FXEN: u32 = 1 << 1;
pub const MICREL_KSZ8_P1_ERRATA: u32 = 1 << 2;

pub const MICREL_KSZ9021_EXTREG_CTRL: u32 = 0xB;
pub const MICREL_KSZ9021_EXTREG_DATA_WRITE: u32 = 0xC;
pub const MICREL_KSZ9021_RGMII_CLK_CTRL_PAD_SCEW: u32 = 0x104;
pub const MICREL_KSZ9021_RGMII_RX_DATA_PAD_SCEW: u32 = 0x105;

/* Device specific MII_BMCR (Reg 0) bits */
/* 1 = HP Auto MDI/MDI-X mode, 0 = Microchip Auto MDI/MDI-X mode */
pub const KSZ886X_BMCR_HP_MDIX: u32 = 1 << 5;
/* 1 = Force MDI (transmit on RXP/RXM pins), 0 = Normal operation
 * (transmit on TXP/TXM pins)
 */
pub const KSZ886X_BMCR_FORCE_MDI: u32 = 1 << 4;
/* 1 = Disable auto MDI-X */
pub const KSZ886X_BMCR_DISABLE_AUTO_MDIX: u32 = 1 << 3;
pub const KSZ886X_BMCR_DISABLE_FAR_END_FAULT: u32 = 1 << 2;
pub const KSZ886X_BMCR_DISABLE_TRANSMIT: u32 = 1 << 1;
pub const KSZ886X_BMCR_DISABLE_LED: u32 = 1 << 0;

/* PHY Special Control/Status Register (Reg 31) */
pub const KSZ886X_CTRL_MDIX_STAT: u32 = 1 << 4;
pub const KSZ886X_CTRL_FORCE_LINK: u32 = 1 << 3;
pub const KSZ886X_CTRL_PWRSAVE: u32 = 1 << 2;
pub const KSZ886X_CTRL_REMOTE_LOOPBACK: u32 = 1 << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
