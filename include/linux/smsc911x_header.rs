/* SPDX-License-Identifier: GPL-2.0-or-later */
/***************************************************************************
 *
 * Copyright (C) 2004-2008 SMSC
 * Copyright (C) 2005-2008 ARM
 *
 ***************************************************************************/

// External C dependencies represented by this header:
// - `phy_interface_t` is supplied by <linux/phy.h>.
// - `ETH_ALEN` is supplied by <linux/if_ether.h>.

/* platform_device configuration data, should be assigned to
 * the platform_device's dev.platform_data */
#[repr(C)]
pub struct smsc911x_platform_config {
    pub irq_polarity: u32,
    pub irq_type: u32,
    pub flags: u32,
    pub shift: u32,
    pub phy_interface: phy_interface_t,
    pub mac: [u8; ETH_ALEN],
}

/* Constants for platform_device irq polarity configuration */
pub const SMSC911X_IRQ_POLARITY_ACTIVE_LOW: u32 = 0;
pub const SMSC911X_IRQ_POLARITY_ACTIVE_HIGH: u32 = 1;

/* Constants for platform_device irq type configuration */
pub const SMSC911X_IRQ_TYPE_OPEN_DRAIN: u32 = 0;
pub const SMSC911X_IRQ_TYPE_PUSH_PULL: u32 = 1;

/* Constants for flags */
pub const SMSC911X_USE_16BIT: u32 = 1u32 << 0;
pub const SMSC911X_USE_32BIT: u32 = 1u32 << 1;
pub const SMSC911X_FORCE_INTERNAL_PHY: u32 = 1u32 << 2;
pub const SMSC911X_FORCE_EXTERNAL_PHY: u32 = 1u32 << 3;
pub const SMSC911X_SAVE_MAC_ADDRESS: u32 = 1u32 << 4;

/*
 * SMSC911X_SWAP_FIFO:
 * Enables software byte swap for fifo data. Should only be used as a
 * "last resort" in the case of big endian mode on boards with incorrectly
 * routed data bus to older devices such as LAN9118. Newer devices such as
 * LAN9221 can handle this in hardware, there are registers to control
 * this swapping but the driver doesn't currently use them.
 */
pub const SMSC911X_SWAP_FIFO: u32 = 1u32 << 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
