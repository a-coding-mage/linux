// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.

pub const UAUDIO_STREAM_SERVICE_ID_V01: u32 = 0x41D;
pub const UAUDIO_STREAM_SERVICE_VERS_V01: u32 = 0x01;

pub const QMI_UAUDIO_STREAM_RESP_V01: u32 = 0x0001;
pub const QMI_UAUDIO_STREAM_REQ_V01: u32 = 0x0001;
pub const QMI_UAUDIO_STREAM_IND_V01: u32 = 0x0001;

#[repr(C)]
pub struct mem_info_v01 {
    pub iova: u64,      // mapped into sysdev
    pub dma: u64,       // mapped into usb host
    pub size: u32,
}

#[repr(C)]
pub struct apps_mem_info_v01 {
    pub evt_ring: mem_info_v01,
    pub tr_data: mem_info_v01,
    pub tr_sync: mem_info_v01,
    pub xfer_buff: mem_info_v01,
    pub dcba: mem_info_v01,
}

#[repr(C)]
pub struct usb_endpoint_descriptor_v01 {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
    pub bRefresh: u8,
    pub bSynchAddress: u8,
}

#[repr(C)]
pub struct usb_interface_descriptor_v01 {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}

#[repr(i32)]
pub enum usb_qmi_audio_stream_status_enum_v01 {
    USB_QMI_STREAM_STATUS_ENUM_MIN_VAL_V01 = i32::MIN,
    USB_QMI_STREAM_REQ_SUCCESS_V01 = 0,
    USB_QMI_STREAM_REQ_FAILURE_V01 = 1,
    USB_QMI_STREAM_REQ_FAILURE_NOT_FOUND_V01 = 2,
    USB_QMI_STREAM_REQ_FAILURE_INVALID_PARAM_V01 = 3,
    USB_QMI_STREAM_REQ_FAILURE_MEMALLOC_V01 = 4,
    USB_QMI_STREAM_STATUS_ENUM_MAX_VAL_V01 = i32::MAX,
}

#[repr(i32)]
pub enum usb_qmi_audio_device_indication_enum_v01 {
    USB_QMI_DEVICE_INDICATION_ENUM_MIN_VAL_V01 = i32::MIN,
    USB_QMI_DEV_CONNECT_V01 = 0,
    USB_QMI_DEV_DISCONNECT_V01 = 1,
    USB_QMI_DEV_SUSPEND_V01 = 2,
    USB_QMI_DEV_RESUME_V01 = 3,
    USB_QMI_DEVICE_INDICATION_ENUM_MAX_VAL_V01 = i32::MAX,
}

#[repr(i32)]
pub enum usb_qmi_audio_device_speed_enum_v01 {
    USB_QMI_DEVICE_SPEED_ENUM_MIN_VAL_V01 = i32::MIN,
    USB_QMI_DEVICE_SPEED_INVALID_V01 = 0,
    USB_QMI_DEVICE_SPEED_LOW_V01 = 1,
    USB_QMI_DEVICE_SPEED_FULL_V01 = 2,
    USB_QMI_DEVICE_SPEED_HIGH_V01 = 3,
    USB_QMI_DEVICE_SPEED_SUPER_V01 = 4,
    USB_QMI_DEVICE_SPEED_SUPER_PLUS_V01 = 5,
    USB_QMI_DEVICE_SPEED_ENUM_MAX_VAL_V01 = i32::MAX,
}

#[repr(C)]
pub struct qmi_uaudio_stream_req_msg_v01 {
    pub enable: u8,
    pub usb_token: u32,
    pub audio_format_valid: u8,
    pub audio_format: u32,
    pub number_of_ch_valid: u8,
    pub number_of_ch: u32,
    pub bit_rate_valid: u8,
    pub bit_rate: u32,
    pub xfer_buff_size_valid: u8,
    pub xfer_buff_size: u32,
    pub service_interval_valid: u8,
    pub service_interval: u32,
}

pub const QMI_UAUDIO_STREAM_REQ_MSG_V01_MAX_MSG_LEN: u32 = 46;

extern "C" {
    pub static qmi_uaudio_stream_req_msg_v01_ei: qmi_elem_info;
}

#[repr(C)]
pub struct qmi_uaudio_stream_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub status_valid: u8,
    pub status: usb_qmi_audio_stream_status_enum_v01,
    pub internal_status_valid: u8,
    pub internal_status: u32,
    pub slot_id_valid: u8,
    pub slot_id: u32,
    pub usb_token_valid: u8,
    pub usb_token: u32,
    pub std_as_opr_intf_desc_valid: u8,
    pub std_as_opr_intf_desc: usb_interface_descriptor_v01,
    pub std_as_data_ep_desc_valid: u8,
    pub std_as_data_ep_desc: usb_endpoint_descriptor_v01,
    pub std_as_sync_ep_desc_valid: u8,
    pub std_as_sync_ep_desc: usb_endpoint_descriptor_v01,
    pub usb_audio_spec_revision_valid: u8,
    pub usb_audio_spec_revision: u16,
    pub data_path_delay_valid: u8,
    pub data_path_delay: u8,
    pub usb_audio_subslot_size_valid: u8,
    pub usb_audio_subslot_size: u8,
    pub xhci_mem_info_valid: u8,
    pub xhci_mem_info: apps_mem_info_v01,
    pub interrupter_num_valid: u8,
    pub interrupter_num: u8,
    pub speed_info_valid: u8,
    pub speed_info: usb_qmi_audio_device_speed_enum_v01,
    pub controller_num_valid: u8,
    pub controller_num: u8,
}

pub const QMI_UAUDIO_STREAM_RESP_MSG_V01_MAX_MSG_LEN: u32 = 202;

extern "C" {
    pub static qmi_uaudio_stream_resp_msg_v01_ei: qmi_elem_info;
}

#[repr(C)]
pub struct qmi_uaudio_stream_ind_msg_v01 {
    pub dev_event: usb_qmi_audio_device_indication_enum_v01,
    pub slot_id: u32,
    pub usb_token_valid: u8,
    pub usb_token: u32,
    pub std_as_opr_intf_desc_valid: u8,
    pub std_as_opr_intf_desc: usb_interface_descriptor_v01,
    pub std_as_data_ep_desc_valid: u8,
    pub std_as_data_ep_desc: usb_endpoint_descriptor_v01,
    pub std_as_sync_ep_desc_valid: u8,
    pub std_as_sync_ep_desc: usb_endpoint_descriptor_v01,
    pub usb_audio_spec_revision_valid: u8,
    pub usb_audio_spec_revision: u16,
    pub data_path_delay_valid: u8,
    pub data_path_delay: u8,
    pub usb_audio_subslot_size_valid: u8,
    pub usb_audio_subslot_size: u8,
    pub xhci_mem_info_valid: u8,
    pub xhci_mem_info: apps_mem_info_v01,
    pub interrupter_num_valid: u8,
    pub interrupter_num: u8,
    pub controller_num_valid: u8,
    pub controller_num: u8,
}

pub const QMI_UAUDIO_STREAM_IND_MSG_V01_MAX_MSG_LEN: u32 = 181;

extern "C" {
    pub static qmi_uaudio_stream_ind_msg_v01_ei: qmi_elem_info;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
