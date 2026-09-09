/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/* Copyright 2019 NXP */

/* Translated from enetc_mdio.h.  Types and symbols supplied by Linux headers
 * remain external dependencies. */

use core::ffi::c_void;

/* PCS registers */
pub const ENETC_PCS_LINK_TIMER1: u32 = 0x12;
pub const ENETC_PCS_LINK_TIMER1_VAL: u32 = 0x06a0;
pub const ENETC_PCS_LINK_TIMER2: u32 = 0x13;
pub const ENETC_PCS_LINK_TIMER2_VAL: u32 = 0x0003;
pub const ENETC_PCS_IF_MODE: u32 = 0x14;
pub const ENETC_PCS_IF_MODE_SGMII_EN: u32 = 1 << 0;
pub const ENETC_PCS_IF_MODE_USE_SGMII_AN: u32 = 1 << 1;

#[inline]
pub const fn ENETC_PCS_IF_MODE_SGMII_SPEED(x: u32) -> u32 {
    (x << 2) & ((1 << 3) | (1 << 2))
}

pub const ENETC_PCS_IF_MODE_DUPLEX_HALF: u32 = 1 << 3;

/* Not a mistake, the SerDes PLL needs to be set at 3.125 GHz by Reset
 * Configuration Word (RCW, outside Linux control) for 2.5G SGMII mode. The PCS
 * still thinks it's at gigabit.
 */
pub type EnetcPcsSpeed = i32;
pub const ENETC_PCS_SPEED_10: EnetcPcsSpeed = 0;
pub const ENETC_PCS_SPEED_100: EnetcPcsSpeed = 1;
pub const ENETC_PCS_SPEED_1000: EnetcPcsSpeed = 2;
pub const ENETC_PCS_SPEED_2500: EnetcPcsSpeed = 2;

#[repr(C)]
pub struct enetc_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mii_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct enetc_mdio_priv {
    pub hw: *mut enetc_hw,
    pub mdio_base: i32,
}

/* IS_REACHABLE(CONFIG_FSL_ENETC_MDIO) */

unsafe extern "C" {
    pub fn enetc_mdio_read_c22(bus: *mut mii_bus, phy_id: i32, regnum: i32) -> i32;
    pub fn enetc_mdio_write_c22(
        bus: *mut mii_bus,
        phy_id: i32,
        regnum: i32,
        value: u16,
    ) -> i32;
    pub fn enetc_mdio_read_c45(
        bus: *mut mii_bus,
        phy_id: i32,
        devad: i32,
        regnum: i32,
    ) -> i32;
    pub fn enetc_mdio_write_c45(
        bus: *mut mii_bus,
        phy_id: i32,
        devad: i32,
        regnum: i32,
        value: u16,
    ) -> i32;
    pub fn enetc_hw_alloc(dev: *mut device, port_regs: *mut c_void) -> *mut enetc_hw;
}

/* When CONFIG_FSL_ENETC_MDIO is not reachable, the declarations above have
 * the following inline definitions instead:
 *
 * static inline int enetc_mdio_read_c22(...) { return -EINVAL; }
 * static inline int enetc_mdio_write_c22(...) { return -EINVAL; }
 * static inline int enetc_mdio_read_c45(...) { return -EINVAL; }
 * static inline int enetc_mdio_write_c45(...) { return -EINVAL; }
 * static inline struct enetc_hw *enetc_hw_alloc(...) { return ERR_PTR(-EINVAL); }
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
