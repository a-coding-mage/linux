/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2017-2018, Intel Corporation
 * Copyright (C) 2025, Altera Corporation
 */

// Service layer driver supports client names.
pub const SVC_CLIENT_FPGA: &str = "fpga";
pub const SVC_CLIENT_RSU: &str = "rsu";
pub const SVC_CLIENT_FCS: &str = "fcs";
pub const SVC_CLIENT_HWMON: &str = "hwmon";

pub const SVC_STATUS_OK: u32 = 0;
pub const SVC_STATUS_BUFFER_SUBMITTED: u32 = 1;
pub const SVC_STATUS_BUFFER_DONE: u32 = 2;
pub const SVC_STATUS_COMPLETED: u32 = 3;
pub const SVC_STATUS_BUSY: u32 = 4;
pub const SVC_STATUS_ERROR: u32 = 5;
pub const SVC_STATUS_NO_SUPPORT: u32 = 6;
pub const SVC_STATUS_INVALID_PARAM: u32 = 7;

pub const COMMAND_RECONFIG_FLAG_PARTIAL: u32 = 0;

pub const SVC_RECONFIG_REQUEST_TIMEOUT_MS: u32 = 5000;
pub const SVC_RECONFIG_BUFFER_TIMEOUT_MS: u32 = 5000;
pub const SVC_RSU_REQUEST_TIMEOUT_MS: u32 = 2000;
pub const SVC_FCS_REQUEST_TIMEOUT_MS: u32 = 2000;
pub const SVC_COMPLETED_TIMEOUT_MS: u32 = 30000;
pub const SVC_HWMON_REQUEST_TIMEOUT_MS: u32 = 2000;

use core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum stratix10_svc_command_code {
    COMMAND_NOOP = 0,
    COMMAND_RECONFIG,
    COMMAND_RECONFIG_DATA_SUBMIT,
    COMMAND_RECONFIG_DATA_CLAIM,
    COMMAND_RECONFIG_STATUS,
    COMMAND_RSU_STATUS = 10,
    COMMAND_RSU_UPDATE,
    COMMAND_RSU_NOTIFY,
    COMMAND_RSU_RETRY,
    COMMAND_RSU_MAX_RETRY,
    COMMAND_RSU_DCMF_VERSION,
    COMMAND_RSU_DCMF_STATUS,
    COMMAND_RSU_GET_DEVICE_INFO,
    COMMAND_FIRMWARE_VERSION,
    COMMAND_RSU_GET_SPT_TABLE,
    COMMAND_FCS_REQUEST_SERVICE = 20,
    COMMAND_FCS_SEND_CERTIFICATE,
    COMMAND_FCS_GET_PROVISION_DATA,
    COMMAND_FCS_DATA_ENCRYPTION,
    COMMAND_FCS_DATA_DECRYPTION,
    COMMAND_FCS_RANDOM_NUMBER_GEN,
    COMMAND_POLL_SERVICE_STATUS = 40,
    COMMAND_MBOX_SEND_CMD = 100,
    COMMAND_SMC_SVC_VERSION = 200,
    COMMAND_HWMON_READTEMP,
    COMMAND_HWMON_READVOLT,
    COMMAND_SMC_ATF_BUILD_VER,
}

#[repr(C)]
pub struct stratix10_svc_chan {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stratix10_svc_client_msg {
    pub payload: *mut c_void,
    pub payload_length: usize,
    pub payload_output: *mut c_void,
    pub payload_length_output: usize,
    pub command: stratix10_svc_command_code,
    pub arg: [u64; 3],
}

#[repr(C)]
pub struct stratix10_svc_command_config_type {
    pub flags: u32,
}

#[repr(C)]
pub struct stratix10_svc_cb_data {
    pub status: u32,
    pub kaddr1: *mut c_void,
    pub kaddr2: *mut c_void,
    pub kaddr3: *mut c_void,
}

#[repr(C)]
pub struct stratix10_svc_client {
    pub dev: *mut device,
    pub receive_cb: Option<unsafe extern "C" fn(
        client: *mut stratix10_svc_client,
        cb_data: *mut stratix10_svc_cb_data,
    )>,
    pub priv_: *mut c_void,
}

pub type async_callback_t = Option<unsafe extern "C" fn(cb_arg: *mut c_void)>;

extern "C" {
    pub fn stratix10_svc_request_channel_byname(
        client: *mut stratix10_svc_client,
        name: *const core::ffi::c_char,
    ) -> *mut stratix10_svc_chan;
    pub fn stratix10_svc_free_channel(chan: *mut stratix10_svc_chan);
    pub fn stratix10_svc_allocate_memory(
        chan: *mut stratix10_svc_chan,
        size: usize,
    ) -> *mut c_void;
    pub fn stratix10_svc_free_memory(chan: *mut stratix10_svc_chan, kaddr: *mut c_void);
    pub fn stratix10_svc_send(chan: *mut stratix10_svc_chan, msg: *mut c_void) -> i32;
    pub fn stratix10_svc_done(chan: *mut stratix10_svc_chan);
    pub fn stratix10_svc_add_async_client(
        chan: *mut stratix10_svc_chan,
        use_unique_clientid: bool,
    ) -> i32;
    pub fn stratix10_svc_remove_async_client(chan: *mut stratix10_svc_chan) -> i32;
    pub fn stratix10_svc_async_send(
        chan: *mut stratix10_svc_chan,
        msg: *mut c_void,
        handler: *mut *mut c_void,
        cb: async_callback_t,
        cb_arg: *mut c_void,
    ) -> i32;
    pub fn stratix10_svc_async_poll(
        chan: *mut stratix10_svc_chan,
        tx_handle: *mut c_void,
        data: *mut stratix10_svc_cb_data,
    ) -> i32;
    pub fn stratix10_svc_async_done(chan: *mut stratix10_svc_chan, tx_handle: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
