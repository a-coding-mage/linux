/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/if_ether.h, linux/phy.h, and linux/spinlock.h.

#[repr(C)]
pub struct ll_temac_platform_data {
    pub txcsum: bool,                 /* Enable/disable TX checksum */
    pub rxcsum: bool,                 /* Enable/disable RX checksum */
    pub mac_addr: [u8; ETH_ALEN],     /* MAC address (6 bytes) */
    /* Clock frequency for input to MDIO clock generator */
    pub mdio_clk_freq: u32,
    pub mdio_bus_id: u64,             /* Unique id for MDIO bus */
    pub phy_addr: i32,                /* Address of the PHY to connect to */
    pub phy_interface: phy_interface_t, /* PHY interface mode */
    pub reg_little_endian: bool,      /* Little endian TEMAC register access  */
    pub dma_little_endian: bool,      /* Little endian DMA register access  */
    /* Pre-initialized mutex to use for synchronizing indirect
     * register access.  When using both interfaces of a single
     * TEMAC IP block, the same mutex should be passed here, as
     * they share the same DCR bus bridge.
     */
    pub indirect_lock: *mut spinlock_t,
    /* DMA channel control setup */
    pub tx_irq_timeout: u8,           /* TX Interrupt Delay Time-out */
    pub tx_irq_count: u8,             /* TX Interrupt Coalescing Threshold Count */
    pub rx_irq_timeout: u8,           /* RX Interrupt Delay Time-out */
    pub rx_irq_count: u8,             /* RX Interrupt Coalescing Threshold Count */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
