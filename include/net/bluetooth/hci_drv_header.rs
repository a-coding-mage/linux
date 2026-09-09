/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2025 Google Corporation
 */

// Translated from hci_drv.h. Linux and Bluetooth types are supplied by the
// surrounding translation unit.

#[repr(C, packed)]
pub struct hci_drv_cmd_hdr {
    pub opcode: u16,
    pub len: u16,
}

#[repr(C, packed)]
pub struct hci_drv_ev_hdr {
    pub opcode: u16,
    pub len: u16,
}

pub const HCI_DRV_EV_CMD_STATUS: u16 = 0x0000;

#[repr(C, packed)]
pub struct hci_drv_ev_cmd_status {
    pub opcode: u16,
    pub status: u8,
}

pub const HCI_DRV_EV_CMD_COMPLETE: u16 = 0x0001;

#[repr(C, packed)]
pub struct hci_drv_ev_cmd_complete {
    pub opcode: u16,
    pub status: u8,
    pub data: [u8; 0],
}

pub const HCI_DRV_STATUS_SUCCESS: u8 = 0x00;
pub const HCI_DRV_STATUS_UNSPECIFIED_ERROR: u8 = 0x01;
pub const HCI_DRV_STATUS_UNKNOWN_COMMAND: u8 = 0x02;
pub const HCI_DRV_STATUS_INVALID_PARAMETERS: u8 = 0x03;

pub const HCI_DRV_MAX_DRIVER_NAME_LENGTH: usize = 32;

/* Common commands that make sense on all drivers start from 0x0000 */
pub const HCI_DRV_OP_READ_INFO: u16 = 0x0000;
pub const HCI_DRV_READ_INFO_SIZE: usize = 0;

#[repr(C, packed)]
pub struct hci_drv_rp_read_info {
    pub driver_name: [u8; HCI_DRV_MAX_DRIVER_NAME_LENGTH],
    pub num_supported_commands: u16,
    // __counted_by_le(num_supported_commands): flexible array member.
    pub supported_commands: [u16; 0],
}

/* Driver specific OGF (Opcode Group Field)
 * Commands in this group may have different meanings across different drivers.
 */
pub const HCI_DRV_OGF_DRIVER_SPECIFIC: u8 = 0x01;

extern "C" {
    pub fn hci_drv_cmd_status(hdev: *mut hci_dev, cmd: u16, status: u8) -> i32;
    pub fn hci_drv_cmd_complete(
        hdev: *mut hci_dev,
        cmd: u16,
        status: u8,
        rp: *mut core::ffi::c_void,
        rp_len: usize,
    ) -> i32;
    pub fn hci_drv_process_cmd(hdev: *mut hci_dev, cmd_skb: *mut sk_buff) -> i32;
}

#[repr(C)]
pub struct hci_drv_handler {
    pub func: Option<unsafe extern "C" fn(
        hdev: *mut hci_dev,
        data: *mut core::ffi::c_void,
        data_len: u16,
    ) -> i32>,
    pub data_len: usize,
}

#[repr(C)]
pub struct hci_drv {
    pub common_handler_count: usize,
    pub common_handlers: *const hci_drv_handler,
    pub specific_handler_count: usize,
    pub specific_handlers: *const hci_drv_handler,
}

// Opaque types supplied by the Bluetooth subsystem.
pub enum hci_dev {}
pub enum sk_buff {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
