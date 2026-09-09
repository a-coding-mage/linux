/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2025 Ventana Micro Systems Inc. */

// Dependencies supplied by the surrounding kernel translation:
// linux::errno, linux::mailbox_client, linux::types, linux::wordpart.

#[inline]
pub const fn RPMI_VER_MAJOR(ver: u32) -> u32 { ver >> 16 }
#[inline]
pub const fn RPMI_VER_MINOR(ver: u32) -> u32 { ver & 0xffff }
#[inline]
pub const fn RPMI_MKVER(maj: u32, min: u32) -> u32 { (maj << 16) | (min as u16 as u32) }

#[repr(C)]
pub struct rpmi_message_header {
    pub servicegroup_id: u16,
    pub service_id: u8,
    pub flags: u8,
    pub datalen: u16,
    pub token: u16,
}

#[repr(C)]
pub struct rpmi_message {
    pub header: rpmi_message_header,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct rpmi_notification_event {
    pub event_datalen: u16,
    pub event_id: u8,
    pub reserved: u8,
    pub event_data: [u8; 0],
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmi_error_codes {
    RPMI_SUCCESS = 0,
    RPMI_ERR_FAILED = -1,
    RPMI_ERR_NOTSUPP = -2,
    RPMI_ERR_INVALID_PARAM = -3,
    RPMI_ERR_DENIED = -4,
    RPMI_ERR_INVALID_ADDR = -5,
    RPMI_ERR_ALREADY = -6,
    RPMI_ERR_EXTENSION = -7,
    RPMI_ERR_HW_FAULT = -8,
    RPMI_ERR_BUSY = -9,
    RPMI_ERR_INVALID_STATE = -10,
    RPMI_ERR_BAD_RANGE = -11,
    RPMI_ERR_TIMEOUT = -12,
    RPMI_ERR_IO = -13,
    RPMI_ERR_NO_DATA = -14,
    RPMI_ERR_RESERVED_START = -15,
    RPMI_ERR_RESERVED_END = -127,
    RPMI_ERR_VENDOR_START = -128,
}

#[inline]
pub unsafe fn rpmi_to_linux_error(rpmi_error: i32) -> i32 {
    match rpmi_error {
        0 => 0,
        -3 | -11 | -10 => -EINVAL,
        -4 => -EPERM,
        -5 | -8 => -EFAULT,
        -6 => -EALREADY,
        -9 => -EBUSY,
        -12 => -ETIMEDOUT,
        -13 => -ECOMM,
        _ => -EOPNOTSUPP,
    }
}

pub const RPMI_SRVGRP_SYSTEM_MSI: u32 = 0x00002;
pub const RPMI_SRVGRP_CLOCK: u32 = 0x00008;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmi_clock_service_id {
    RPMI_CLK_SRV_ENABLE_NOTIFICATION = 0x01,
    RPMI_CLK_SRV_GET_NUM_CLOCKS = 0x02,
    RPMI_CLK_SRV_GET_ATTRIBUTES = 0x03,
    RPMI_CLK_SRV_GET_SUPPORTED_RATES = 0x04,
    RPMI_CLK_SRV_SET_CONFIG = 0x05,
    RPMI_CLK_SRV_GET_CONFIG = 0x06,
    RPMI_CLK_SRV_SET_RATE = 0x07,
    RPMI_CLK_SRV_GET_RATE = 0x08,
    RPMI_CLK_SRV_ID_MAX_COUNT = 0x09,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmi_sysmsi_service_id {
    RPMI_SYSMSI_SRV_ENABLE_NOTIFICATION = 0x01,
    RPMI_SYSMSI_SRV_GET_ATTRIBUTES = 0x02,
    RPMI_SYSMSI_SRV_GET_MSI_ATTRIBUTES = 0x03,
    RPMI_SYSMSI_SRV_SET_MSI_STATE = 0x04,
    RPMI_SYSMSI_SRV_GET_MSI_STATE = 0x05,
    RPMI_SYSMSI_SRV_SET_MSI_TARGET = 0x06,
    RPMI_SYSMSI_SRV_GET_MSI_TARGET = 0x07,
    RPMI_SYSMSI_SRV_ID_MAX_COUNT = 0x08,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmi_mbox_attribute_id {
    RPMI_MBOX_ATTR_SPEC_VERSION = 0,
    RPMI_MBOX_ATTR_MAX_MSG_DATA_SIZE,
    RPMI_MBOX_ATTR_SERVICEGROUP_ID,
    RPMI_MBOX_ATTR_SERVICEGROUP_VERSION,
    RPMI_MBOX_ATTR_IMPL_ID,
    RPMI_MBOX_ATTR_IMPL_VERSION,
    RPMI_MBOX_ATTR_MAX_ID,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpmi_mbox_message_type {
    RPMI_MBOX_MSG_TYPE_GET_ATTRIBUTE = 0,
    RPMI_MBOX_MSG_TYPE_SET_ATTRIBUTE,
    RPMI_MBOX_MSG_TYPE_SEND_WITH_RESPONSE,
    RPMI_MBOX_MSG_TYPE_SEND_WITHOUT_RESPONSE,
    RPMI_MBOX_MSG_TYPE_NOTIFICATION_EVENT,
    RPMI_MBOX_MSG_MAX_TYPE,
}

#[repr(C)]
pub struct rpmi_mbox_message_attr {
    pub id: rpmi_mbox_attribute_id,
    pub value: u32,
}
#[repr(C)]
pub struct rpmi_mbox_message_data {
    pub service_id: u32,
    pub request: *mut core::ffi::c_void,
    pub request_len: usize,
    pub response: *mut core::ffi::c_void,
    pub max_response_len: usize,
    pub out_response_len: usize,
}
#[repr(C)]
pub struct rpmi_mbox_message_notif {
    pub event_datalen: u16,
    pub event_id: u8,
    pub event_data: *mut u8,
}
#[repr(C)]
pub union rpmi_mbox_message_payload {
    pub attr: rpmi_mbox_message_attr,
    pub data: rpmi_mbox_message_data,
    pub notif: rpmi_mbox_message_notif,
}
#[repr(C)]
pub struct rpmi_mbox_message {
    pub type_: rpmi_mbox_message_type,
    pub payload: rpmi_mbox_message_payload,
    pub error: i32,
}

#[inline]
pub unsafe fn rpmi_mbox_init_get_attribute(msg: *mut rpmi_mbox_message, id: rpmi_mbox_attribute_id) {
    (*msg).type_ = rpmi_mbox_message_type::RPMI_MBOX_MSG_TYPE_GET_ATTRIBUTE;
    (*msg).payload.attr.id = id;
    (*msg).payload.attr.value = 0;
    (*msg).error = 0;
}

#[inline]
pub unsafe fn rpmi_mbox_init_set_attribute(msg: *mut rpmi_mbox_message, id: rpmi_mbox_attribute_id, value: u32) {
    (*msg).type_ = rpmi_mbox_message_type::RPMI_MBOX_MSG_TYPE_SET_ATTRIBUTE;
    (*msg).payload.attr.id = id;
    (*msg).payload.attr.value = value;
    (*msg).error = 0;
}

#[inline]
pub unsafe fn rpmi_mbox_init_send_with_response(msg: *mut rpmi_mbox_message, service_id: u32, request: *mut core::ffi::c_void, request_len: usize, response: *mut core::ffi::c_void, max_response_len: usize) {
    (*msg).type_ = rpmi_mbox_message_type::RPMI_MBOX_MSG_TYPE_SEND_WITH_RESPONSE;
    (*msg).payload.data.service_id = service_id;
    (*msg).payload.data.request = request;
    (*msg).payload.data.request_len = request_len;
    (*msg).payload.data.response = response;
    (*msg).payload.data.max_response_len = max_response_len;
    (*msg).payload.data.out_response_len = 0;
    (*msg).error = 0;
}

#[inline]
pub unsafe fn rpmi_mbox_init_send_without_response(msg: *mut rpmi_mbox_message, service_id: u32, request: *mut core::ffi::c_void, request_len: usize) {
    (*msg).type_ = rpmi_mbox_message_type::RPMI_MBOX_MSG_TYPE_SEND_WITHOUT_RESPONSE;
    (*msg).payload.data.service_id = service_id;
    (*msg).payload.data.request = request;
    (*msg).payload.data.request_len = request_len;
    (*msg).payload.data.response = core::ptr::null_mut();
    (*msg).payload.data.max_response_len = 0;
    (*msg).payload.data.out_response_len = 0;
    (*msg).error = 0;
}

#[inline]
pub unsafe fn rpmi_mbox_get_msg_response(msg: *mut rpmi_mbox_message) -> *mut core::ffi::c_void {
    if msg.is_null() { core::ptr::null_mut() } else { (*msg).payload.data.response }
}

#[inline]
pub unsafe fn rpmi_mbox_send_message(chan: *mut mbox_chan, msg: *mut rpmi_mbox_message) -> i32 {
    let ret = mbox_send_message(chan, msg);
    if ret < 0 { return ret; }
    let ret = (*msg).error;
    mbox_client_txdone(chan, ret);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
