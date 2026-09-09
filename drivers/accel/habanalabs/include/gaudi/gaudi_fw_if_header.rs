/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2019-2020 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

pub const GAUDI_EVENT_QUEUE_MSI_IDX: u32 = 8;
pub const GAUDI_NIC_PORT1_MSI_IDX: u32 = 10;
pub const GAUDI_NIC_PORT3_MSI_IDX: u32 = 12;
pub const GAUDI_NIC_PORT5_MSI_IDX: u32 = 14;
pub const GAUDI_NIC_PORT7_MSI_IDX: u32 = 16;
pub const GAUDI_NIC_PORT9_MSI_IDX: u32 = 18;

pub const UBOOT_FW_OFFSET: u32 = 0x100000; // 1MB in SRAM
pub const LINUX_FW_OFFSET: u32 = 0x800000; // 8MB in HBM

// HBM thermal delta in [Deg] added to composite (CTemp)
pub const HBM_TEMP_ADJUST_COEFF: u32 = 6;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum gaudi_nic_axi_error {
    RXB,
    RXE,
    TXS,
    TXE,
    QPC_RESP,
    NON_AXI_ERR,
    TMR,
}

/*
 * struct eq_nic_sei_event - describes an AXI error cause.
 * @axi_error_cause: one of the events defined in enum gaudi_nic_axi_error.
 * @id: can be either 0 or 1, to further describe unit with interrupt cause
 *      (i.e. TXE0 or TXE1).
 * @pad[6]: padding structure to 64bit.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct eq_nic_sei_event {
    pub axi_error_cause: u8,
    pub id: u8,
    pub pad: [u8; 6],
}

#[repr(C)]
pub union gaudi_cold_rst_data_bits {
    // spsram_init_done: bit 0; reserved: bits 1..31
    pub bits: u32,
    pub data: u32, // __le32
}

#[repr(C)]
pub struct gaudi_cold_rst_data {
    pub value: gaudi_cold_rst_data_bits,
}

pub const GAUDI_PLL_FREQ_LOW: u32 = 200000000; // 200 MHz

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
