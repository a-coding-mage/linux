/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2011  Intel Corporation. All rights reserved.
 */

/* Dependencies supplied by the surrounding kernel/NFC translation. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct nfc_hci_dev;
#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct nfc_target;
#[repr(C)]
pub struct nfc_dev;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct work_struct;
#[repr(C)]
pub struct timer_list;
#[repr(C)]
pub struct hci_msg;
#[repr(C)]
pub struct sk_buff_head;
#[repr(C)]
pub struct nfc_llc;
#[repr(C)]
pub struct nfc_vendor_cmd;

pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type size_t = usize;
pub type c_ulong = core::ffi::c_ulong;
pub type data_exchange_cb_t = unsafe extern "C" fn(*mut c_void, *mut sk_buff);
pub type se_io_cb_t = unsafe extern "C" fn(*mut c_void, *mut sk_buff);

#[repr(C)]
pub struct nfc_hci_ops {
    pub open: Option<unsafe extern "C" fn(*mut nfc_hci_dev) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut nfc_hci_dev)>,
    pub load_session: Option<unsafe extern "C" fn(*mut nfc_hci_dev) -> c_int>,
    pub hci_ready: Option<unsafe extern "C" fn(*mut nfc_hci_dev) -> c_int>,
    /* xmit must always send the complete buffer before returning. */
    pub xmit: Option<unsafe extern "C" fn(*mut nfc_hci_dev, *mut sk_buff) -> c_int>,
    pub start_poll: Option<unsafe extern "C" fn(*mut nfc_hci_dev, u32, u32) -> c_int>,
    pub stop_poll: Option<unsafe extern "C" fn(*mut nfc_hci_dev)>,
    pub dep_link_up: Option<unsafe extern "C" fn(*mut nfc_hci_dev, *mut nfc_target, u8, *mut u8, size_t) -> c_int>,
    pub dep_link_down: Option<unsafe extern "C" fn(*mut nfc_hci_dev) -> c_int>,
    pub target_from_gate: Option<unsafe extern "C" fn(*mut nfc_hci_dev, u8, *mut nfc_target) -> c_int>,
    pub complete_target_discovered: Option<unsafe extern "C" fn(*mut nfc_hci_dev, u8, *mut nfc_target) -> c_int>,
    pub im_transceive: Option<unsafe extern "C" fn(*mut nfc_hci_dev, *mut nfc_target, *mut sk_buff, data_exchange_cb_t, *mut c_void) -> c_int>,
    pub tm_send: Option<unsafe extern "C" fn(*mut nfc_hci_dev, *mut sk_buff) -> c_int>,
    pub check_presence: Option<unsafe extern "C" fn(*mut nfc_hci_dev, *mut nfc_target) -> c_int>,
    pub event_received: Option<unsafe extern "C" fn(*mut nfc_hci_dev, u8, u8, *mut sk_buff) -> c_int>,
    pub cmd_received: Option<unsafe extern "C" fn(*mut nfc_hci_dev, u8, u8, *mut sk_buff)>,
    pub fw_download: Option<unsafe extern "C" fn(*mut nfc_hci_dev, *const c_char) -> c_int>,
    pub discover_se: Option<unsafe extern "C" fn(*mut nfc_hci_dev) -> c_int>,
    pub enable_se: Option<unsafe extern "C" fn(*mut nfc_hci_dev, u32) -> c_int>,
    pub disable_se: Option<unsafe extern "C" fn(*mut nfc_hci_dev, u32) -> c_int>,
    pub se_io: Option<unsafe extern "C" fn(*mut nfc_hci_dev, u32, *mut u8, size_t, se_io_cb_t, *mut c_void) -> c_int>,
}

pub const NFC_HCI_DO_NOT_CREATE_PIPE: u8 = 0x81;
pub const NFC_HCI_INVALID_PIPE: u8 = 0x80;
pub const NFC_HCI_INVALID_GATE: u8 = 0xff;
pub const NFC_HCI_INVALID_HOST: u8 = 0x80;
pub const NFC_HCI_LINK_MGMT_PIPE: u8 = 0x00;
pub const NFC_HCI_ADMIN_PIPE: u8 = 0x01;

#[repr(C)]
pub struct nfc_hci_gate { pub gate: u8, pub pipe: u8 }
#[repr(C)]
pub struct nfc_hci_pipe { pub gate: u8, pub dest_host: u8 }

pub const NFC_HCI_MAX_CUSTOM_GATES: usize = 50;
pub const NFC_HCI_MAX_PIPES: usize = 128;

#[repr(C)]
pub struct nfc_hci_init_data {
    pub gate_count: u8,
    pub gates: [nfc_hci_gate; NFC_HCI_MAX_CUSTOM_GATES],
    pub session_id: [c_char; 9],
}

pub type xmit = unsafe extern "C" fn(*mut sk_buff, *mut c_void) -> c_int;
pub const NFC_HCI_MAX_GATES: usize = 256;
pub const NFC_HCI_QUIRK_SHORT_CLEAR: c_int = 0;

#[repr(C)]
pub struct nfc_hci_dev {
    pub ndev: *mut nfc_dev,
    pub max_data_link_payload: u32,
    pub shutting_down: bool,
    pub msg_tx_mutex: mutex,
    pub msg_tx_queue: list_head,
    pub msg_tx_work: work_struct,
    pub cmd_timer: timer_list,
    pub cmd_pending_msg: *mut hci_msg,
    pub rx_hcp_frags: sk_buff_head,
    pub msg_rx_work: work_struct,
    pub msg_rx_queue: sk_buff_head,
    pub ops: *const nfc_hci_ops,
    pub llc: *mut nfc_llc,
    pub init_data: nfc_hci_init_data,
    pub clientdata: *mut c_void,
    pub gate2pipe: [u8; NFC_HCI_MAX_GATES],
    pub pipes: [nfc_hci_pipe; NFC_HCI_MAX_PIPES],
    pub sw_romlib: u8, pub sw_patch: u8, pub sw_flashlib_major: u8, pub sw_flashlib_minor: u8,
    pub hw_derivative: u8, pub hw_version: u8, pub hw_mpw: u8, pub hw_software: u8, pub hw_bsid: u8,
    pub async_cb_type: c_int,
    pub async_cb: Option<data_exchange_cb_t>,
    pub async_cb_context: *mut c_void,
    pub gb: *mut u8,
    pub gb_len: size_t,
    pub quirks: c_ulong,
}

extern "C" {
    pub fn nfc_hci_allocate_device(ops: *const nfc_hci_ops, init_data: *mut nfc_hci_init_data, quirks: c_ulong, protocols: u32, llc_name: *const c_char, tx_headroom: c_int, tx_tailroom: c_int, max_link_payload: c_int) -> *mut nfc_hci_dev;
    pub fn nfc_hci_free_device(hdev: *mut nfc_hci_dev);
    pub fn nfc_hci_register_device(hdev: *mut nfc_hci_dev) -> c_int;
    pub fn nfc_hci_unregister_device(hdev: *mut nfc_hci_dev);
    pub fn nfc_hci_set_clientdata(hdev: *mut nfc_hci_dev, clientdata: *mut c_void);
    pub fn nfc_hci_get_clientdata(hdev: *mut nfc_hci_dev) -> *mut c_void;
    pub fn nfc_set_vendor_cmds(ndev: *mut nfc_dev, cmds: *const nfc_vendor_cmd, n_cmds: c_int) -> c_int;
    pub fn nfc_hci_driver_failure(hdev: *mut nfc_hci_dev, err: c_int);
    pub fn nfc_hci_result_to_errno(result: u8) -> c_int;
    pub fn nfc_hci_reset_pipes(dev: *mut nfc_hci_dev);
    pub fn nfc_hci_reset_pipes_per_host(hdev: *mut nfc_hci_dev, host: u8);
}

#[inline]
pub unsafe fn nfc_hci_set_vendor_cmds(hdev: *mut nfc_hci_dev, cmds: *const nfc_vendor_cmd, n_cmds: c_int) -> c_int {
    nfc_set_vendor_cmds((*hdev).ndev, cmds, n_cmds)
}

pub const NFC_HCI_HOST_CONTROLLER_ID: u8 = 0x00;
pub const NFC_HCI_TERMINAL_HOST_ID: u8 = 0x01;
pub const NFC_HCI_UICC_HOST_ID: u8 = 0x02;
pub const NFC_HCI_ADMIN_GATE: u8 = 0x00;
pub const NFC_HCI_ADMIN_SESSION_IDENTITY: u8 = 0x01;
pub const NFC_HCI_ADMIN_MAX_PIPE: u8 = 0x02;
pub const NFC_HCI_ADMIN_WHITELIST: u8 = 0x03;
pub const NFC_HCI_ADMIN_HOST_LIST: u8 = 0x04;
pub const NFC_HCI_LOOPBACK_GATE: u8 = 0x04;
pub const NFC_HCI_ID_MGMT_GATE: u8 = 0x05;
pub const NFC_HCI_ID_MGMT_VERSION_SW: u8 = 0x01;
pub const NFC_HCI_ID_MGMT_VERSION_HW: u8 = 0x03;
pub const NFC_HCI_ID_MGMT_VENDOR_NAME: u8 = 0x04;
pub const NFC_HCI_ID_MGMT_MODEL_ID: u8 = 0x05;
pub const NFC_HCI_ID_MGMT_HCI_VERSION: u8 = 0x02;
pub const NFC_HCI_ID_MGMT_GATES_LIST: u8 = 0x06;
pub const NFC_HCI_LINK_MGMT_GATE: u8 = 0x06;
pub const NFC_HCI_LINK_MGMT_REC_ERROR: u8 = 0x01;
pub const NFC_HCI_RF_READER_B_GATE: u8 = 0x11;
pub const NFC_HCI_RF_READER_B_PUPI: u8 = 0x03;
pub const NFC_HCI_RF_READER_B_APPLICATION_DATA: u8 = 0x04;
pub const NFC_HCI_RF_READER_B_AFI: u8 = 0x02;
pub const NFC_HCI_RF_READER_B_HIGHER_LAYER_RESPONSE: u8 = 0x01;
pub const NFC_HCI_RF_READER_B_HIGHER_LAYER_DATA: u8 = 0x05;
pub const NFC_HCI_RF_READER_A_GATE: u8 = 0x13;
pub const NFC_HCI_RF_READER_A_UID: u8 = 0x02;
pub const NFC_HCI_RF_READER_A_ATQA: u8 = 0x04;
pub const NFC_HCI_RF_READER_A_APPLICATION_DATA: u8 = 0x05;
pub const NFC_HCI_RF_READER_A_SAK: u8 = 0x03;
pub const NFC_HCI_RF_READER_A_FWI_SFGT: u8 = 0x06;
pub const NFC_HCI_RF_READER_A_DATARATE_MAX: u8 = 0x01;
#[inline] pub const fn NFC_HCI_TYPE_A_SEL_PROT(x: u8) -> u8 { (x & 0x60) >> 5 }
pub const NFC_HCI_TYPE_A_SEL_PROT_MIFARE: u8 = 0;
pub const NFC_HCI_TYPE_A_SEL_PROT_ISO14443: u8 = 1;
pub const NFC_HCI_TYPE_A_SEL_PROT_DEP: u8 = 2;
pub const NFC_HCI_TYPE_A_SEL_PROT_ISO14443_DEP: u8 = 3;
pub const NFC_HCI_EVT_HCI_END_OF_OPERATION: u8 = 0x01;
pub const NFC_HCI_EVT_POST_DATA: u8 = 0x02;
pub const NFC_HCI_EVT_HOT_PLUG: u8 = 0x03;
pub const NFC_HCI_ANY_SET_PARAMETER: u8 = 0x01;
pub const NFC_HCI_ANY_GET_PARAMETER: u8 = 0x02;
pub const NFC_HCI_ANY_OPEN_PIPE: u8 = 0x03;
pub const NFC_HCI_ANY_CLOSE_PIPE: u8 = 0x04;
pub const NFC_HCI_EVT_READER_REQUESTED: u8 = 0x10;
pub const NFC_HCI_EVT_END_OPERATION: u8 = 0x11;
pub const NFC_HCI_EVT_TARGET_DISCOVERED: u8 = 0x10;

extern "C" {
    pub fn nfc_hci_resp_received(hdev: *mut nfc_hci_dev, result: u8, skb: *mut sk_buff);
    pub fn nfc_hci_cmd_received(hdev: *mut nfc_hci_dev, pipe: u8, cmd: u8, skb: *mut sk_buff);
    pub fn nfc_hci_event_received(hdev: *mut nfc_hci_dev, pipe: u8, event: u8, skb: *mut sk_buff);
    pub fn nfc_hci_recv_frame(hdev: *mut nfc_hci_dev, skb: *mut sk_buff);
    pub fn nfc_hci_connect_gate(hdev: *mut nfc_hci_dev, dest_host: u8, dest_gate: u8, pipe: u8) -> c_int;
    pub fn nfc_hci_disconnect_gate(hdev: *mut nfc_hci_dev, gate: u8) -> c_int;
    pub fn nfc_hci_disconnect_all_gates(hdev: *mut nfc_hci_dev) -> c_int;
    pub fn nfc_hci_get_param(hdev: *mut nfc_hci_dev, gate: u8, idx: u8, skb: *mut *mut sk_buff) -> c_int;
    pub fn nfc_hci_set_param(hdev: *mut nfc_hci_dev, gate: u8, idx: u8, param: *const u8, param_len: size_t) -> c_int;
    pub fn nfc_hci_send_cmd(hdev: *mut nfc_hci_dev, gate: u8, cmd: u8, param: *const u8, param_len: size_t, skb: *mut *mut sk_buff) -> c_int;
    pub fn nfc_hci_send_cmd_async(hdev: *mut nfc_hci_dev, gate: u8, cmd: u8, param: *const u8, param_len: size_t, cb: data_exchange_cb_t, cb_context: *mut c_void) -> c_int;
    pub fn nfc_hci_send_event(hdev: *mut nfc_hci_dev, gate: u8, event: u8, param: *const u8, param_len: size_t) -> c_int;
    pub fn nfc_hci_target_discovered(hdev: *mut nfc_hci_dev, gate: u8) -> c_int;
    pub fn nfc_hci_sak_to_protocol(sak: u8) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
