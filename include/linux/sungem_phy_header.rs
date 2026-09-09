/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux type definitions.

use core::ffi::{c_char, c_int, c_void};

pub struct mii_phy;
pub struct net_device;

/* Operations supported by any kind of PHY */
#[repr(C)]
pub struct mii_phy_ops {
    pub init: Option<unsafe extern "C" fn(phy: *mut mii_phy) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(phy: *mut mii_phy) -> c_int>,
    pub setup_aneg: Option<unsafe extern "C" fn(phy: *mut mii_phy, advertise: u32) -> c_int>,
    pub setup_forced:
        Option<unsafe extern "C" fn(phy: *mut mii_phy, speed: c_int, fd: c_int) -> c_int>,
    pub poll_link: Option<unsafe extern "C" fn(phy: *mut mii_phy) -> c_int>,
    pub read_link: Option<unsafe extern "C" fn(phy: *mut mii_phy) -> c_int>,
    pub enable_fiber: Option<unsafe extern "C" fn(phy: *mut mii_phy, autoneg: c_int) -> c_int>,
}

/* Structure used to statically define an mii/gii based PHY */
#[repr(C)]
pub struct mii_phy_def {
    pub phy_id: u32,             /* Concatenated ID1 << 16 | ID2 */
    pub phy_id_mask: u32,        /* Significant bits */
    pub features: u32,           /* Ethtool SUPPORTED_* defines */
    pub magic_aneg: c_int,        /* Autoneg does all speed test for us */
    pub name: *const c_char,
    pub ops: *const mii_phy_ops,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bcm54xx_phy_kind {
    BCM54XX_COPPER,
    BCM54XX_FIBER,
    BCM54XX_GBIC,
    BCM54XX_SGMII,
    BCM54XX_UNKNOWN,
}

/* An instance of a PHY, partially borrowed from mii_if_info */
#[repr(C)]
pub struct mii_phy {
    pub def: *const mii_phy_def,
    pub advertising: u32,
    pub mii_id: c_int,

    /* 1: autoneg enabled, 0: disabled */
    pub autoneg: c_int,

    /* forced speed & duplex (no autoneg)
     * partner speed & duplex & pause (autoneg)
     */
    pub speed: c_int,
    pub duplex: c_int,
    pub pause: c_int,

    /* Provided by host chip */
    pub dev: *mut net_device,
    pub mdio_read:
        Option<unsafe extern "C" fn(dev: *mut net_device, mii_id: c_int, reg: c_int) -> c_int>,
    pub mdio_write: Option<
        unsafe extern "C" fn(dev: *mut net_device, mii_id: c_int, reg: c_int, val: c_int),
    >,
    pub platform_data: *mut c_void,
}

/* Pass in a struct mii_phy with dev, mdio_read and mdio_write
 * filled, the remaining fields will be filled on return
 */
unsafe extern "C" {
    pub fn sungem_phy_probe(phy: *mut mii_phy, mii_id: c_int) -> c_int;
}

/* MII definitions missing from mii.h */

pub const BMCR_SPD2: u32 = 0x0040; /* Gigabit enable (bcm54xx) */
pub const LPA_PAUSE: u32 = 0x0400;

/* More PHY registers (model specific) */

/* MII BCM5201 MULTIPHY interrupt register */
pub const MII_BCM5201_INTERRUPT: u32 = 0x1A;
pub const MII_BCM5201_INTERRUPT_INTENABLE: u32 = 0x4000;

pub const MII_BCM5201_AUXMODE2: u32 = 0x1B;
pub const MII_BCM5201_AUXMODE2_LOWPOWER: u32 = 0x0008;

pub const MII_BCM5201_MULTIPHY: u32 = 0x1E;

/* MII BCM5201 MULTIPHY register bits */
pub const MII_BCM5201_MULTIPHY_SERIALMODE: u32 = 0x0002;
pub const MII_BCM5201_MULTIPHY_SUPERISOLATE: u32 = 0x0008;

/* MII BCM5221 Additional registers */
pub const MII_BCM5221_TEST: u32 = 0x1f;
pub const MII_BCM5221_TEST_ENABLE_SHADOWS: u32 = 0x0080;
pub const MII_BCM5221_SHDOW_AUX_STAT2: u32 = 0x1b;
pub const MII_BCM5221_SHDOW_AUX_STAT2_APD: u32 = 0x0020;
pub const MII_BCM5221_SHDOW_AUX_MODE4: u32 = 0x1a;
pub const MII_BCM5221_SHDOW_AUX_MODE4_IDDQMODE: u32 = 0x0001;
pub const MII_BCM5221_SHDOW_AUX_MODE4_CLKLOPWR: u32 = 0x0004;

/* MII BCM5241 Additional registers */
pub const MII_BCM5241_SHDOW_AUX_MODE4_STANDBYPWR: u32 = 0x0008;

/* MII BCM5400 1000-BASET Control register */
pub const MII_BCM5400_GB_CONTROL: u32 = 0x09;
pub const MII_BCM5400_GB_CONTROL_FULLDUPLEXCAP: u32 = 0x0200;

/* MII BCM5400 AUXCONTROL register */
pub const MII_BCM5400_AUXCONTROL: u32 = 0x18;
pub const MII_BCM5400_AUXCONTROL_PWR10BASET: u32 = 0x0004;

/* MII BCM5400 AUXSTATUS register */
pub const MII_BCM5400_AUXSTATUS: u32 = 0x19;
pub const MII_BCM5400_AUXSTATUS_LINKMODE_MASK: u32 = 0x0700;
pub const MII_BCM5400_AUXSTATUS_LINKMODE_SHIFT: u32 = 8;

/* 1000BT control (Marvell & BCM54xx at least) */
pub const MII_1000BASETCONTROL: u32 = 0x09;
pub const MII_1000BASETCONTROL_FULLDUPLEXCAP: u32 = 0x0200;
pub const MII_1000BASETCONTROL_HALFDUPLEXCAP: u32 = 0x0100;

/* Marvell 88E1011 PHY control */
pub const MII_M1011_PHY_SPEC_CONTROL: u32 = 0x10;
pub const MII_M1011_PHY_SPEC_CONTROL_MANUAL_MDIX: u32 = 0x20;
pub const MII_M1011_PHY_SPEC_CONTROL_AUTO_MDIX: u32 = 0x40;

/* Marvell 88E1011 PHY status */
pub const MII_M1011_PHY_SPEC_STATUS: u32 = 0x11;
pub const MII_M1011_PHY_SPEC_STATUS_1000: u32 = 0x8000;
pub const MII_M1011_PHY_SPEC_STATUS_100: u32 = 0x4000;
pub const MII_M1011_PHY_SPEC_STATUS_SPD_MASK: u32 = 0xc000;
pub const MII_M1011_PHY_SPEC_STATUS_FULLDUPLEX: u32 = 0x2000;
pub const MII_M1011_PHY_SPEC_STATUS_RESOLVED: u32 = 0x0800;
pub const MII_M1011_PHY_SPEC_STATUS_TX_PAUSE: u32 = 0x0008;
pub const MII_M1011_PHY_SPEC_STATUS_RX_PAUSE: u32 = 0x0004;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
