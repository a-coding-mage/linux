/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2012  Intel Corporation. All rights reserved.
 */

// Original dependency: <net/nfc/hci.h>

#[repr(C)]
pub struct gate_pipe_map {
    pub gate: u8,
    pub pipe: u8,
}

#[repr(C, packed)]
pub struct hcp_message {
    pub header: u8, // type -cmd,evt,rsp- + instruction
    pub data: [u8; 0],
}

#[repr(C, packed)]
pub struct hcp_packet {
    pub header: u8, // cbit+pipe
    pub message: hcp_message,
}

#[repr(C)]
pub struct hcp_exec_waiter {
    pub wq: *mut wait_queue_head_t,
    pub exec_complete: bool,
    pub exec_result: i32,
    pub result_skb: *mut sk_buff,
}

#[repr(C)]
pub struct hci_msg {
    pub msg_l: list_head,
    pub msg_frags: sk_buff_head,
    pub wait_response: bool,
    pub cb: data_exchange_cb_t,
    pub cb_context: *mut core::ffi::c_void,
    pub completion_delay: c_ulong,
}

#[repr(C, packed)]
pub struct hci_create_pipe_params {
    pub src_gate: u8,
    pub dest_host: u8,
    pub dest_gate: u8,
}

#[repr(C, packed)]
pub struct hci_create_pipe_resp {
    pub src_host: u8,
    pub src_gate: u8,
    pub dest_host: u8,
    pub dest_gate: u8,
    pub pipe: u8,
}

#[repr(C, packed)]
pub struct hci_delete_pipe_noti {
    pub pipe: u8,
}

#[repr(C, packed)]
pub struct hci_all_pipe_cleared_noti {
    pub host: u8,
}

pub const NFC_HCI_FRAGMENT: u8 = 0x7f;

#[inline]
pub const fn HCP_HEADER(type_: u8, instr: u8) -> u8 {
    ((type_ & 0x03) << 6) | (instr & 0x3f)
}

#[inline]
pub const fn HCP_MSG_GET_TYPE(header: u8) -> u8 {
    (header & 0xc0) >> 6
}

#[inline]
pub const fn HCP_MSG_GET_CMD(header: u8) -> u8 {
    header & 0x3f
}

extern "C" {
    pub fn nfc_hci_hcp_message_tx(
        hdev: *mut nfc_hci_dev,
        pipe: u8,
        type_: u8,
        instruction: u8,
        payload: *const u8,
        payload_len: usize,
        cb: data_exchange_cb_t,
        cb_context: *mut core::ffi::c_void,
        completion_delay: c_ulong,
    ) -> i32;

    pub fn nfc_hci_hcp_message_rx(
        hdev: *mut nfc_hci_dev,
        pipe: u8,
        type_: u8,
        instruction: u8,
        skb: *mut sk_buff,
    );
}

pub const NFC_HCI_HCP_PACKET_HEADER_LEN: u32 = 1;
pub const NFC_HCI_HCP_MESSAGE_HEADER_LEN: u32 = 1;
pub const NFC_HCI_HCP_HEADER_LEN: u32 = 2;

pub const NFC_HCI_HCP_COMMAND: u32 = 0x00;
pub const NFC_HCI_HCP_EVENT: u32 = 0x01;
pub const NFC_HCI_HCP_RESPONSE: u32 = 0x02;

pub const NFC_HCI_ANY_SET_PARAMETER: u32 = 0x01;
pub const NFC_HCI_ANY_GET_PARAMETER: u32 = 0x02;
pub const NFC_HCI_ANY_OPEN_PIPE: u32 = 0x03;
pub const NFC_HCI_ANY_CLOSE_PIPE: u32 = 0x04;

pub const NFC_HCI_WR_XCHG_DATA: u32 = 0x10;

pub const NFC_HCI_ADM_CREATE_PIPE: u32 = 0x10;
pub const NFC_HCI_ADM_DELETE_PIPE: u32 = 0x11;
pub const NFC_HCI_ADM_NOTIFY_PIPE_CREATED: u32 = 0x12;
pub const NFC_HCI_ADM_NOTIFY_PIPE_DELETED: u32 = 0x13;
pub const NFC_HCI_ADM_CLEAR_ALL_PIPE: u32 = 0x14;
pub const NFC_HCI_ADM_NOTIFY_ALL_PIPE_CLEARED: u32 = 0x15;

pub const NFC_HCI_ANY_OK: u32 = 0x00;
pub const NFC_HCI_ANY_E_NOT_CONNECTED: u32 = 0x01;
pub const NFC_HCI_ANY_E_CMD_PAR_UNKNOWN: u32 = 0x02;
pub const NFC_HCI_ANY_E_NOK: u32 = 0x03;
pub const NFC_HCI_ANY_E_PIPES_FULL: u32 = 0x04;
pub const NFC_HCI_ANY_E_REG_PAR_UNKNOWN: u32 = 0x05;
pub const NFC_HCI_ANY_E_PIPE_NOT_OPENED: u32 = 0x06;
pub const NFC_HCI_ANY_E_CMD_NOT_SUPPORTED: u32 = 0x07;
pub const NFC_HCI_ANY_E_INHIBITED: u32 = 0x08;
pub const NFC_HCI_ANY_E_TIMEOUT: u32 = 0x09;
pub const NFC_HCI_ANY_E_REG_ACCESS_DENIED: u32 = 0x0a;
pub const NFC_HCI_ANY_E_PIPE_ACCESS_DENIED: u32 = 0x0b;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
