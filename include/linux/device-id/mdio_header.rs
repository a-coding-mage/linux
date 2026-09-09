/* SPDX-License-Identifier: GPL-2.0 */

// The C header's __KERNEL__ include supplies the fixed-width integer types.

pub const MDIO_MODULE_PREFIX: &str = "mdio:";

pub const MDIO_ID_FMT: &str = "%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u%u";

macro_rules! MDIO_ID_ARGS {
    ($id:expr) => {
        (($id >> 31) & 1), (($id >> 30) & 1), (($id >> 29) & 1), (($id >> 28) & 1),
        (($id >> 27) & 1), (($id >> 26) & 1), (($id >> 25) & 1), (($id >> 24) & 1),
        (($id >> 23) & 1), (($id >> 22) & 1), (($id >> 21) & 1), (($id >> 20) & 1),
        (($id >> 19) & 1), (($id >> 18) & 1), (($id >> 17) & 1), (($id >> 16) & 1),
        (($id >> 15) & 1), (($id >> 14) & 1), (($id >> 13) & 1), (($id >> 12) & 1),
        (($id >> 11) & 1), (($id >> 10) & 1), (($id >> 9) & 1), (($id >> 8) & 1),
        (($id >> 7) & 1), (($id >> 6) & 1), (($id >> 5) & 1), (($id >> 4) & 1),
        (($id >> 3) & 1), (($id >> 2) & 1), (($id >> 1) & 1), (($id) & 1)
    };
}

/**
 * struct mdio_device_id - identifies PHY devices on an MDIO/MII bus
 * @phy_id: The result of
 *     (mdio_read(&MII_PHYSID1) << 16 | mdio_read(&MII_PHYSID2)) & @phy_id_mask
 *     for this PHY type
 * @phy_id_mask: Defines the significant bits of @phy_id.  A value of 0
 *     is used to terminate an array of struct mdio_device_id.
 */
#[repr(C)]
pub struct mdio_device_id {
    pub phy_id: u32,
    pub phy_id_mask: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
