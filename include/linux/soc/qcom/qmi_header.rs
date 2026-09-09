// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2012-2014, The Linux Foundation. All rights reserved.
 * Copyright (c) 2017, Linaro Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C, packed)]
pub struct qmi_header {
    pub type_: u8,
    pub txn_id: __le16,
    pub msg_id: __le16,
    pub msg_len: __le16,
}

pub const QMI_REQUEST: i32 = 0;
pub const QMI_RESPONSE: i32 = 2;
pub const QMI_INDICATION: i32 = 4;
pub const QMI_COMMON_TLV_TYPE: i32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qmi_elem_type {
    QMI_EOTI,
    QMI_OPT_FLAG,
    QMI_DATA_LEN,
    QMI_UNSIGNED_1_BYTE,
    QMI_UNSIGNED_2_BYTE,
    QMI_UNSIGNED_4_BYTE,
    QMI_UNSIGNED_8_BYTE,
    QMI_SIGNED_2_BYTE_ENUM,
    QMI_SIGNED_4_BYTE_ENUM,
    QMI_STRUCT,
    QMI_STRING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qmi_array_type {
    NO_ARRAY,
    STATIC_ARRAY,
    VAR_LEN_ARRAY,
}

#[repr(C)]
pub struct qmi_elem_info {
    pub data_type: qmi_elem_type,
    pub elem_len: u32,
    pub elem_size: u32,
    pub array_type: qmi_array_type,
    pub tlv_type: u8,
    pub offset: u32,
    pub ei_array: *const qmi_elem_info,
}

pub const QMI_RESULT_SUCCESS_V01: i32 = 0;
pub const QMI_RESULT_FAILURE_V01: i32 = 1;
pub const QMI_ERR_NONE_V01: i32 = 0;
pub const QMI_ERR_MALFORMED_MSG_V01: i32 = 1;
pub const QMI_ERR_NO_MEMORY_V01: i32 = 2;
pub const QMI_ERR_INTERNAL_V01: i32 = 3;
pub const QMI_ERR_CLIENT_IDS_EXHAUSTED_V01: i32 = 5;
pub const QMI_ERR_INVALID_ID_V01: i32 = 41;
pub const QMI_ERR_ENCODING_V01: i32 = 58;
pub const QMI_ERR_DISABLED_V01: i32 = 69;
pub const QMI_ERR_INCOMPATIBLE_STATE_V01: i32 = 90;
pub const QMI_ERR_NOT_SUPPORTED_V01: i32 = 94;

pub const QMI_SERVICE_ID_TEST: i32 = 0x0f;
pub const QMI_SERVICE_ID_SSCTL: i32 = 0x2b;
pub const QMI_SERVICE_ID_IPA: i32 = 0x31;
pub const QMI_SERVICE_ID_SERVREG_LOC: i32 = 0x40;
pub const QMI_SERVICE_ID_SERVREG_NOTIF: i32 = 0x42;
pub const QMI_SERVICE_ID_WLFW: i32 = 0x45;
pub const QMI_SERVICE_ID_SLIMBUS: i32 = 0x301;
pub const QMI_SERVICE_ID_USB_AUDIO_STREAM: i32 = 0x41d;

#[repr(C)]
pub struct qmi_response_type_v01 { pub result: u16, pub error: u16 }

unsafe extern "C" {
    pub static qmi_response_type_v01_ei: [qmi_elem_info; 0];
}

#[repr(C)]
pub struct qmi_service {
    pub service: c_uint,
    pub version: c_uint,
    pub instance: c_uint,
    pub node: c_uint,
    pub port: c_uint,
    pub priv_: *mut c_void,
    pub list_node: list_head,
}

#[repr(C)]
pub struct qmi_handle;

#[repr(C)]
pub struct qmi_ops {
    pub new_server: Option<unsafe extern "C" fn(*mut qmi_handle, *mut qmi_service) -> c_int>,
    pub del_server: Option<unsafe extern "C" fn(*mut qmi_handle, *mut qmi_service)>,
    pub net_reset: Option<unsafe extern "C" fn(*mut qmi_handle)>,
    pub msg_handler: Option<unsafe extern "C" fn(*mut qmi_handle, *mut sockaddr_qrtr, *const c_void, size_t)>,
    pub bye: Option<unsafe extern "C" fn(*mut qmi_handle, c_uint)>,
    pub del_client: Option<unsafe extern "C" fn(*mut qmi_handle, c_uint, c_uint)>,
}

#[repr(C)]
pub struct qmi_txn {
    pub qmi: *mut qmi_handle,
    pub id: u16,
    pub lock: mutex,
    pub completion: completion,
    pub result: c_int,
    pub ei: *const qmi_elem_info,
    pub dest: *mut c_void,
}

#[repr(C)]
pub struct qmi_msg_handler {
    pub type_: c_uint,
    pub msg_id: c_uint,
    pub ei: *const qmi_elem_info,
    pub decoded_size: size_t,
    pub fn_: Option<unsafe extern "C" fn(*mut qmi_handle, *mut sockaddr_qrtr, *mut qmi_txn, *const c_void)>,
}

#[repr(C)]
pub struct qmi_handle {
    pub sock: *mut socket,
    pub sock_lock: mutex,
    pub sq: sockaddr_qrtr,
    pub work: work_struct,
    pub wq: *mut workqueue_struct,
    pub recv_buf: *mut c_void,
    pub recv_buf_size: size_t,
    pub lookups: list_head,
    pub lookup_results: list_head,
    pub services: list_head,
    pub ops: qmi_ops,
    pub txns: idr,
    pub txn_lock: mutex,
    pub handlers: *const qmi_msg_handler,
}

unsafe extern "C" {
    pub fn qmi_add_lookup(qmi: *mut qmi_handle, service: c_uint, version: c_uint, instance: c_uint) -> c_int;
    pub fn qmi_add_server(qmi: *mut qmi_handle, service: c_uint, version: c_uint, instance: c_uint) -> c_int;
    pub fn qmi_handle_init(qmi: *mut qmi_handle, max_msg_len: size_t, ops: *const qmi_ops, handlers: *const qmi_msg_handler) -> c_int;
    pub fn qmi_handle_release(qmi: *mut qmi_handle);
    pub fn qmi_send_request(qmi: *mut qmi_handle, sq: *mut sockaddr_qrtr, txn: *mut qmi_txn, msg_id: c_int, len: size_t, ei: *const qmi_elem_info, c_struct: *const c_void) -> ssize_t;
    pub fn qmi_send_response(qmi: *mut qmi_handle, sq: *mut sockaddr_qrtr, txn: *mut qmi_txn, msg_id: c_int, len: size_t, ei: *const qmi_elem_info, c_struct: *const c_void) -> ssize_t;
    pub fn qmi_send_indication(qmi: *mut qmi_handle, sq: *mut sockaddr_qrtr, msg_id: c_int, len: size_t, ei: *const qmi_elem_info, c_struct: *const c_void) -> ssize_t;
    pub fn qmi_encode_message(type_: c_int, msg_id: c_uint, len: *mut size_t, txn_id: c_uint, ei: *const qmi_elem_info, c_struct: *const c_void) -> *mut c_void;
    pub fn qmi_decode_message(buf: *const c_void, len: size_t, ei: *const qmi_elem_info, c_struct: *mut c_void) -> c_int;
    pub fn qmi_txn_init(qmi: *mut qmi_handle, txn: *mut qmi_txn, ei: *const qmi_elem_info, c_struct: *mut c_void) -> c_int;
    pub fn qmi_txn_wait(txn: *mut qmi_txn, timeout: c_ulong) -> c_int;
    pub fn qmi_txn_cancel(txn: *mut qmi_txn);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
