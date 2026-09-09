/* SPDX-License-Identifier: GPL-2.0+ */
/* Applied Micro X-Gene SoC MDIO Driver
 *
 * Copyright (c) 2016, Applied Micro Circuits Corporation
 * Author: Iyappan Subramanian <isubramanian@apm.com>
 */

// C header dependencies: linux/bits.h, linux/spinlock.h, linux/types.h

pub const BLOCK_XG_MDIO_CSR_OFFSET: u32 = 0x5000;
pub const BLOCK_DIAG_CSR_OFFSET: u32 = 0xd000;
pub const XGENET_CONFIG_REG_ADDR: u32 = 0x20;

pub const MAC_ADDR_REG_OFFSET: u32 = 0x00;
pub const MAC_COMMAND_REG_OFFSET: u32 = 0x04;
pub const MAC_WRITE_REG_OFFSET: u32 = 0x08;
pub const MAC_READ_REG_OFFSET: u32 = 0x0c;
pub const MAC_COMMAND_DONE_REG_OFFSET: u32 = 0x10;

pub const CLKEN_OFFSET: u32 = 0x08;
pub const SRST_OFFSET: u32 = 0x00;

pub const MENET_CFG_MEM_RAM_SHUTDOWN_ADDR: u32 = 0x70;
pub const MENET_BLOCK_MEM_RDY_ADDR: u32 = 0x74;

pub const MAC_CONFIG_1_ADDR: u32 = 0x00;
pub const MII_MGMT_CONFIG_ADDR: u32 = 0x20;
pub const MII_MGMT_COMMAND_ADDR: u32 = 0x24;
pub const MII_MGMT_ADDRESS_ADDR: u32 = 0x28;
pub const MII_MGMT_CONTROL_ADDR: u32 = 0x2c;
pub const MII_MGMT_STATUS_ADDR: u32 = 0x30;
pub const MII_MGMT_INDICATORS_ADDR: u32 = 0x34;
pub const SOFT_RESET: u32 = 1u32 << 31;

pub const MIIM_COMMAND_ADDR: u32 = 0x20;
pub const MIIM_FIELD_ADDR: u32 = 0x24;
pub const MIIM_CONFIGURATION_ADDR: u32 = 0x28;
pub const MIIM_LINKFAILVECTOR_ADDR: u32 = 0x2c;
pub const MIIM_INDICATOR_ADDR: u32 = 0x30;
pub const MIIMRD_FIELD_ADDR: u32 = 0x34;

pub const MDIO_CSR_OFFSET: u32 = 0x5000;

pub const REG_ADDR_POS: i32 = 0;
pub const REG_ADDR_LEN: i32 = 5;
pub const PHY_ADDR_POS: i32 = 8;
pub const PHY_ADDR_LEN: i32 = 5;

pub const HSTMIIMWRDAT_POS: i32 = 0;
pub const HSTMIIMWRDAT_LEN: i32 = 16;
pub const HSTPHYADX_POS: i32 = 23;
pub const HSTPHYADX_LEN: i32 = 5;
pub const HSTREGADX_POS: i32 = 18;
pub const HSTREGADX_LEN: i32 = 5;
pub const HSTLDCMD: u32 = 1u32 << 3;
pub const HSTMIIMCMD_POS: i32 = 0;
pub const HSTMIIMCMD_LEN: i32 = 3;

pub const BUSY_MASK: u32 = 1;
pub const READ_CYCLE_MASK: u32 = 1;

#[repr(i32)]
pub enum xgene_enet_cmd {
    XGENE_ENET_WR_CMD = 1i32 << 31,
    XGENE_ENET_RD_CMD = 1i32 << 30,
}

pub const MIIM_CMD_IDLE: i32 = 0;
pub const MIIM_CMD_LEGACY_WRITE: i32 = 1;
pub const MIIM_CMD_LEGACY_READ: i32 = 2;

#[repr(i32)]
pub enum xgene_mdio_id {
    XGENE_MDIO_RGMII = 1,
    XGENE_MDIO_XFI = 2,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mii_bus {
    _private: [u8; 0],
}
#[repr(C)]
pub struct phy_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xgene_mdio_pdata {
    pub clk: *mut clk,
    pub dev: *mut device,
    pub mac_csr_addr: *mut core::ffi::c_void,
    pub diag_csr_addr: *mut core::ffi::c_void,
    pub mdio_csr_addr: *mut core::ffi::c_void,
    pub mdio_bus: *mut mii_bus,
    pub mdio_id: i32,
    pub mac_lock: spinlock_t, /* mac lock */
}

/* Set the specified value into a bit-field defined by its starting position
 * and length within a single u64.
 */
#[inline]
pub const fn xgene_enet_set_field_value(pos: i32, len: i32, val: u64) -> u64 {
    (val & ((1u64 << (len as u32)) - 1)) << (pos as u32)
}

#[macro_export]
macro_rules! SET_VAL {
    ($field_pos:expr, $field_len:expr, $val:expr) => {
        $crate::xgene_enet_set_field_value($field_pos, $field_len, $val)
    };
}

#[macro_export]
macro_rules! SET_BIT {
    ($field_pos:expr, $val:expr) => {
        $crate::xgene_enet_set_field_value($field_pos, 1, $val)
    };
}

/* Get the value from a bit-field defined by its starting position
 * and length within the specified u64.
 */
#[inline]
pub const fn xgene_enet_get_field_value(pos: i32, len: i32, src: u64) -> u64 {
    (src >> (pos as u32)) & ((1u64 << (len as u32)) - 1)
}

#[macro_export]
macro_rules! GET_VAL {
    ($field_pos:expr, $field_len:expr, $src:expr) => {
        $crate::xgene_enet_get_field_value($field_pos, $field_len, $src)
    };
}

#[macro_export]
macro_rules! GET_BIT {
    ($field_pos:expr, $src:expr) => {
        $crate::xgene_enet_get_field_value($field_pos, 1, $src)
    };
}

extern "C" {
    pub fn xgene_mdio_rd_mac(pdata: *mut xgene_mdio_pdata, rd_addr: u32) -> u32;
    pub fn xgene_mdio_wr_mac(pdata: *mut xgene_mdio_pdata, wr_addr: u32, data: u32);
    pub fn xgene_mdio_rgmii_read(bus: *mut mii_bus, phy_id: i32, reg: i32) -> i32;
    pub fn xgene_mdio_rgmii_write(bus: *mut mii_bus, phy_id: i32, reg: i32, data: u16) -> i32;
    pub fn xgene_enet_phy_register(bus: *mut mii_bus, phy_addr: i32) -> *mut phy_device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
