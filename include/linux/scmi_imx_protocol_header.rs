/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SCMI Message Protocol driver NXP extension header
 *
 * Copyright 2024 NXP.
 */

/* External Linux types and protocol declarations are supplied by dependencies. */

pub const SCMI_PROTOCOL_IMX_LMM: u32 = 0x80;
pub const SCMI_PROTOCOL_IMX_BBM: u32 = 0x81;
pub const SCMI_PROTOCOL_IMX_CPU: u32 = 0x82;
pub const SCMI_PROTOCOL_IMX_MISC: u32 = 0x84;

pub const SCMI_IMX_VENDOR: &str = "NXP";
pub const SCMI_IMX_SUBVENDOR: &str = "IMX";

#[repr(C)]
pub struct scmi_imx_bbm_proto_ops {
    pub rtc_time_set: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, id: u32, sec: u64) -> i32>,
    pub rtc_time_get: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, id: u32, val: *mut u64) -> i32>,
    pub rtc_alarm_set: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, id: u32, enable: bool, sec: u64) -> i32>,
    pub button_get: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, state: *mut u32) -> i32>,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum scmi_nxp_notification_events {
    SCMI_EVENT_IMX_BBM_RTC = 0x0,
    SCMI_EVENT_IMX_BBM_BUTTON = 0x1,
    SCMI_EVENT_IMX_MISC_CONTROL = 0x0,
}

#[repr(C)]
pub struct scmi_imx_bbm_notif_report {
    pub is_rtc: bool,
    pub is_button: bool,
    pub timestamp: ktime_t,
    pub rtc_id: u32,
    pub rtc_evt: u32,
}

#[repr(C)]
pub struct scmi_imx_misc_ctrl_notify_report {
    pub timestamp: ktime_t,
    pub ctrl_id: u32,
    pub flags: u32,
}

pub const MISC_EXT_INFO_LEN_MAX: usize = 4;

#[repr(C)]
pub struct scmi_imx_misc_reset_reason {
    pub valid: bool,
    pub orig_valid: bool,
    pub err_valid: bool,
    pub reason: u32,
    pub origin: u32,
    pub errid: u32,
}

#[repr(C)]
pub struct scmi_imx_misc_proto_ops {
    pub misc_ctrl_set: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, id: u32, num: u32, val: *mut u32) -> i32>,
    pub misc_ctrl_get: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, id: u32, num: *mut u32, val: *mut u32) -> i32>,
    pub misc_ctrl_req_notify: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, ctrl_id: u32, evt_id: u32, flags: u32) -> i32>,
    pub misc_syslog: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, size: *mut u16, array: *mut core::ffi::c_void) -> i32>,
    pub misc_reset_reason: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, system: bool, boot_r: *mut scmi_imx_misc_reset_reason, shut_r: *mut scmi_imx_misc_reset_reason, extinfo: *mut u32) -> i32>,
}

/* See LMM_ATTRIBUTES in imx95.rst */
pub const LMM_ID_DISCOVER: u32 = 0xFFFFFFFF;
pub const LMM_MAX_NAME: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum scmi_imx_lmm_state {
    LMM_STATE_LM_OFF,
    LMM_STATE_LM_ON,
    LMM_STATE_LM_SUSPEND,
    LMM_STATE_LM_POWERED,
}

#[repr(C)]
pub struct scmi_imx_lmm_info {
    pub lmid: u32,
    pub state: scmi_imx_lmm_state,
    pub errstatus: u32,
    pub name: [u8; LMM_MAX_NAME],
}

#[repr(C)]
pub struct scmi_imx_lmm_proto_ops {
    pub lmm_power_boot: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, lmid: u32, boot: bool) -> i32>,
    pub lmm_info: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, lmid: u32, info: *mut scmi_imx_lmm_info) -> i32>,
    pub lmm_reset_vector_set: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, lmid: u32, cpuid: u32, flags: u32, vector: u64) -> i32>,
    pub lmm_shutdown: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, lmid: u32, flags: u32) -> i32>,
}

#[repr(C)]
pub struct scmi_imx_cpu_proto_ops {
    pub cpu_reset_vector_set: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, cpuid: u32, vector: u64, start: bool, boot: bool, resume: bool) -> i32>,
    pub cpu_start: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, cpuid: u32, start: bool) -> i32>,
    pub cpu_started: Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle, cpuid: u32, started: *mut bool) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
