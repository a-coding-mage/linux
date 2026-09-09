/* SPDX-License-Identifier: GPL-2.0 */
/*
 * MV-643XX ethernet platform device data definition file.
 */

/* Dependencies supplied by the surrounding kernel translation. */
use crate::{phy_interface_t, ETH_ALEN};

pub const MV643XX_ETH_SHARED_NAME: &str = "mv643xx_eth";
pub const MV643XX_ETH_NAME: &str = "mv643xx_eth_port";
pub const MV643XX_ETH_SHARED_REGS: u32 = 0x2000;
pub const MV643XX_ETH_SHARED_REGS_SIZE: u32 = 0x2000;
pub const MV643XX_ETH_BAR_4: u32 = 0x2220;
pub const MV643XX_ETH_SIZE_REG_4: u32 = 0x2224;
pub const MV643XX_ETH_BASE_ADDR_ENABLE_REG: u32 = 0x2290;

pub const MV643XX_TX_CSUM_DEFAULT_LIMIT: i32 = 0;

#[repr(C)]
pub struct mbus_dram_target_info;

#[repr(C)]
pub struct mv643xx_eth_shared_platform_data {
    pub dram: *mut mbus_dram_target_info,
    /*
     * Max packet size for Tx IP/Layer 4 checksum, when set to 0, default
     * limit of 9KiB will be used.
     */
    pub tx_csum_limit: i32,
}

pub const MV643XX_ETH_PHY_ADDR_DEFAULT: i32 = 0;

#[inline]
pub const fn MV643XX_ETH_PHY_ADDR(x: i32) -> i32 {
    0x80 | x
}

pub const MV643XX_ETH_PHY_NONE: i32 = 0xff;

#[repr(C)]
pub struct device_node;

#[repr(C)]
pub struct platform_device;

#[repr(C)]
pub struct mv643xx_eth_platform_data {
    /*
     * Pointer back to our parent instance, and our port number.
     */
    pub shared: *mut platform_device,
    pub port_number: i32,

    /*
     * Whether a PHY is present, and if yes, at which address.
     */
    pub phy_addr: i32,
    pub phy_node: *mut device_node,

    /*
     * Use this MAC address if it is valid, overriding the
     * address that is already in the hardware.
     */
    pub mac_addr: [u8; ETH_ALEN],

    /*
     * If speed is 0, autonegotiation is enabled.
     *   Valid values for speed: 0, SPEED_10, SPEED_100, SPEED_1000.
     *   Valid values for duplex: DUPLEX_HALF, DUPLEX_FULL.
     */
    pub speed: i32,
    pub duplex: i32,
    pub interface: phy_interface_t,

    /*
     * How many RX/TX queues to use.
     */
    pub rx_queue_count: i32,
    pub tx_queue_count: i32,

    /*
     * Override default RX/TX queue sizes if nonzero.
     */
    pub rx_queue_size: i32,
    pub tx_queue_size: i32,

    /*
     * Use on-chip SRAM for RX/TX descriptors if size is nonzero
     * and sufficient to contain all descriptors for the requested
     * ring sizes.
     */
    pub rx_sram_addr: ::core::ffi::c_ulong,
    pub rx_sram_size: i32,
    pub tx_sram_addr: ::core::ffi::c_ulong,
    pub tx_sram_size: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
