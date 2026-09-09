/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * OPEN Alliance 10BASE-T1x MAC-PHY Serial Interface framework
 *
 * Link: https://opensig.org/download/document/OPEN_Alliance_10BASET1x_MAC-PHY_Serial_Interface_V1.1.pdf
 *
 * Author: Parthiban Veerasooran <parthiban.veerasooran@microchip.com>
 */

/* OPEN Alliance TC6 registers */
/* Standard Capabilities Register */
pub const OA_TC6_REG_STDCAP: u32 = 0x0002;
pub const OA_TC6_STDCAP_DIRECT_PHY_REG_ACCESS: u32 = 1 << 8;

/* Reset Control and Status Register */
pub const OA_TC6_REG_RESET: u32 = 0x0003;
pub const OA_TC6_RESET_SWRESET: u32 = 1 << 0; /* Software Reset */

/* Configuration Register #0 */
pub const OA_TC6_REG_CONFIG0: u32 = 0x0004;
pub const OA_TC6_CONFIG0_SYNC: u32 = 1 << 15;
pub const OA_TC6_CONFIG0_ZARFE_ENABLE: u32 = 1 << 12;
pub const OA_TC6_CONFIG0_PROTE: u32 = 1 << 5;

/* Configuration Register #2 */
pub const OA_TC6_REG_CONFIG2: u32 = 0x0006;

/* Status Register #0 */
pub const OA_TC6_REG_STATUS0: u32 = 0x0008;
pub const OA_TC6_STATUS0_RESETC: u32 = 1 << 6; /* Reset Complete */
pub const OA_TC6_STATUS0_HEADER_ERROR: u32 = 1 << 5;
pub const OA_TC6_STATUS0_LOSS_OF_FRAME_ERROR: u32 = 1 << 4;
pub const OA_TC6_STATUS0_RX_BUFFER_OVERFLOW_ERROR: u32 = 1 << 3;
pub const OA_TC6_STATUS0_TX_PROTOCOL_ERROR: u32 = 1 << 0;

/* Buffer Status Register */
pub const OA_TC6_REG_BUFFER_STATUS: u32 = 0x000B;
pub const OA_TC6_BUFFER_STATUS_TX_CREDITS_AVAILABLE: u32 = 0xFF00;
pub const OA_TC6_BUFFER_STATUS_RX_CHUNKS_AVAILABLE: u32 = 0x00FF;

/* Interrupt Mask Register #0 */
pub const OA_TC6_REG_INT_MASK0: u32 = 0x000C;
pub const OA_TC6_INT_MASK0_HEADER_ERR_MASK: u32 = 1 << 5;
pub const OA_TC6_INT_MASK0_LOSS_OF_FRAME_ERR_MASK: u32 = 1 << 4;
pub const OA_TC6_INT_MASK0_RX_BUFFER_OVERFLOW_ERR_MASK: u32 = 1 << 3;
pub const OA_TC6_INT_MASK0_TX_PROTOCOL_ERR_MASK: u32 = 1 << 0;
pub const OA_TC6_INT_MASK0_ALL_INTERRUPTS: u32 = 0x1F << 0 | 0x7F << 7;

/* PHY Clause 22 registers base address and mask */
pub const OA_TC6_PHY_STD_REG_ADDR_BASE: u32 = 0xFF00;
pub const OA_TC6_PHY_STD_REG_ADDR_MASK: u32 = 0x1F;

/* Memory map selector (MMS) values as per table 6 in the
 * OPEN Alliance specification.
 */
pub const OA_TC6_MAC_MMS1: u8 = 1;
pub const OA_TC6_PHY_C45_PCS_MMS2: u8 = 2; /* MMD 3 */
pub const OA_TC6_PHY_C45_PMA_PMD_MMS3: u8 = 3; /* MMD 1 */
pub const OA_TC6_PHY_C45_VS_PLCA_MMS4: u8 = 4; /* MMD 31 */
pub const OA_TC6_PHY_C45_AUTO_NEG_MMS5: u8 = 5; /* MMD 7 */
pub const OA_TC6_PHY_C45_POWER_UNIT_MMS6: u8 = 6; /* MMD 13 */

/* External kernel types supplied by other translation units. */
#[repr(C)]
pub struct oa_tc6 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spi_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mii_bus {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}
pub type netdev_tx_t = i32;

#[repr(u32)]
pub enum oa_tc6_quirk_flag {
    OA_TC6_BROKEN_PHY = 1 << 0,
}

#[repr(C)]
pub struct oa_tc6_quirks {
    pub quirk_flags: oa_tc6_quirk_flag,
}

extern "C" {
    pub fn oa_tc6_init(
        spi: *mut spi_device,
        netdev: *mut net_device,
        quirks: *mut oa_tc6_quirks,
    ) -> *mut oa_tc6;
    pub fn oa_tc6_exit(tc6: *mut oa_tc6);
    pub fn oa_tc6_write_register(tc6: *mut oa_tc6, address: u32, value: u32) -> i32;
    pub fn oa_tc6_write_register_mms(
        tc6: *mut oa_tc6,
        mms: u8,
        address: u16,
        value: u32,
    ) -> i32;
    pub fn oa_tc6_write_registers(
        tc6: *mut oa_tc6,
        address: u32,
        value: *mut u32,
        length: u8,
    ) -> i32;
    pub fn oa_tc6_read_register(tc6: *mut oa_tc6, address: u32, value: *mut u32) -> i32;
    pub fn oa_tc6_read_register_mms(
        tc6: *mut oa_tc6,
        mms: u8,
        address: u16,
        value: *mut u32,
    ) -> i32;
    pub fn oa_tc6_read_registers(
        tc6: *mut oa_tc6,
        address: u32,
        value: *mut u32,
        length: u8,
    ) -> i32;
    pub fn oa_tc6_start_xmit(tc6: *mut oa_tc6, skb: *mut sk_buff) -> netdev_tx_t;
    pub fn oa_tc6_zero_align_receive_frame_enable(tc6: *mut oa_tc6) -> i32;
    pub fn oa_tc6_mdiobus_read_c45(bus: *mut mii_bus, addr: i32, devnum: i32, regnum: i32) -> i32;
    pub fn oa_tc6_mdiobus_write_c45(
        bus: *mut mii_bus,
        addr: i32,
        devnum: i32,
        regnum: i32,
        val: u16,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
