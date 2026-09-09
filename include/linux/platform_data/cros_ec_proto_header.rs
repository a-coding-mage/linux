/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ChromeOS Embedded Controller protocol interface.
 *
 * Copyright (C) 2012 Google, Inc
 */

// Dependencies supplied by the corresponding kernel interfaces are intentionally
// left external to this translation.

pub const CROS_EC_DEV_NAME: &str = "cros_ec";
pub const CROS_EC_DEV_FP_NAME: &str = "cros_fp";
pub const CROS_EC_DEV_ISH_NAME: &str = "cros_ish";
pub const CROS_EC_DEV_PD_NAME: &str = "cros_pd";
pub const CROS_EC_DEV_SCP_NAME: &str = "cros_scp";
pub const CROS_EC_DEV_TP_NAME: &str = "cros_tp";

pub const CROS_EC_DEV_EC_INDEX: i32 = 0;
pub const CROS_EC_DEV_PD_INDEX: i32 = 1;
pub const EC_REBOOT_DELAY_MS: i32 = 50;
pub const EC_PROTO_VERSION_UNKNOWN: i32 = 0;
pub const EC_MAX_REQUEST_OVERHEAD: i32 = 4;
pub const EC_MAX_RESPONSE_OVERHEAD: i32 = 32;
pub const ACPI_NOTIFY_CROS_EC_MKBP: i32 = 0x80;
pub const ACPI_NOTIFY_CROS_EC_PANIC: i32 = 0xB0;

pub const EC_MSG_TX_HEADER_BYTES: i32 = 3;
pub const EC_MSG_TX_TRAILER_BYTES: i32 = 1;
pub const EC_MSG_TX_PROTO_BYTES: i32 = EC_MSG_TX_HEADER_BYTES + EC_MSG_TX_TRAILER_BYTES;
pub const EC_MSG_RX_PROTO_BYTES: i32 = 3;
pub const EC_PROTO2_MSG_BYTES: i32 = EC_PROTO2_MAX_PARAM_SIZE + EC_MSG_TX_PROTO_BYTES;
pub const EC_MAX_MSG_BYTES: i32 = 64 * 1024;

#[repr(C)]
pub struct cros_ec_command {
    pub version: u32,
    pub command: u32,
    pub outsize: u32,
    pub insize: u32,
    pub result: u32,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct cros_ec_device {
    pub phys_name: *const core::ffi::c_char,
    pub dev: *mut device,
    pub cros_class: *mut class,
    pub cmd_readmem: Option<unsafe extern "C" fn(*mut cros_ec_device, u32, u32, *mut core::ffi::c_void) -> i32>,
    pub max_request: u16,
    pub max_response: u16,
    pub max_passthru: u16,
    pub proto_version: u16,
    pub priv_: *mut core::ffi::c_void,
    pub irq: i32,
    pub din: *mut u8,
    pub dout: *mut u8,
    pub din_size: i32,
    pub dout_size: i32,
    pub wake_enabled: bool,
    pub suspended: bool,
    pub registered: bool,
    pub cmd_xfer: Option<unsafe extern "C" fn(*mut cros_ec_device, *mut cros_ec_command) -> i32>,
    pub pkt_xfer: Option<unsafe extern "C" fn(*mut cros_ec_device, *mut cros_ec_command) -> i32>,
    pub lockdep_key: lock_class_key,
    pub lock: mutex,
    pub mkbp_event_supported: u8,
    pub host_sleep_v1: bool,
    pub event_notifier: blocking_notifier_head,
    pub event_data: ec_response_get_next_event_v3,
    pub event_size: i32,
    pub host_event_wake_mask: u32,
    pub last_resume_result: u32,
    pub suspend_timeout_ms: u16,
    pub last_event_time: ktime_t,
    pub notifier_ready: notifier_block,
    pub ec: *mut platform_device,
    pub pd: *mut platform_device,
    pub panic_notifier: blocking_notifier_head,
}

#[repr(C)]
pub struct cros_ec_platform {
    pub ec_name: *const core::ffi::c_char,
    pub cmd_offset: u16,
}

#[repr(C)]
pub struct cros_ec_dev {
    pub class_dev: device,
    pub group: *const attribute_group,
    pub ec_dev: *mut cros_ec_device,
    pub dev: *mut device,
    pub debug_info: *mut cros_ec_debugfs,
    pub has_kb_wake_angle: bool,
    pub cmd_offset: u16,
    pub features: ec_response_get_features,
}

#[macro_export]
macro_rules! to_cros_ec_dev {
    ($dev:expr) => { container_of!($dev, cros_ec_dev, class_dev) };
}

extern "C" {
    pub fn cros_ec_prepare_tx(ec_dev: *mut cros_ec_device, msg: *mut cros_ec_command) -> i32;
    pub fn cros_ec_check_result(ec_dev: *mut cros_ec_device, msg: *mut cros_ec_command) -> i32;
    pub fn cros_ec_cmd_xfer(ec_dev: *mut cros_ec_device, msg: *mut cros_ec_command) -> i32;
    pub fn cros_ec_cmd_xfer_status(ec_dev: *mut cros_ec_device, msg: *mut cros_ec_command) -> i32;
    pub fn cros_ec_rwsig_continue(ec_dev: *mut cros_ec_device) -> i32;
    pub fn cros_ec_query_all(ec_dev: *mut cros_ec_device) -> i32;
    pub fn cros_ec_get_next_event(ec_dev: *mut cros_ec_device, wake_event: *mut bool, has_more_events: *mut bool) -> i32;
    pub fn cros_ec_get_host_event(ec_dev: *mut cros_ec_device) -> u32;
    pub fn cros_ec_read_features(ec: *mut cros_ec_dev) -> i32;
    pub fn cros_ec_check_features(ec: *mut cros_ec_dev, feature: i32) -> bool;
    pub fn cros_ec_get_sensor_count(ec: *mut cros_ec_dev) -> i32;
    pub fn cros_ec_cmd(ec_dev: *mut cros_ec_device, version: u32, command: i32, outdata: *const core::ffi::c_void, outsize: usize, indata: *mut core::ffi::c_void, insize: usize) -> i32;
    pub fn cros_ec_cmd_readmem(ec_dev: *mut cros_ec_device, offset: u8, size: u8, dest: *mut core::ffi::c_void) -> i32;
    pub fn cros_ec_get_cmd_versions(ec_dev: *mut cros_ec_device, cmd: u16) -> i32;
    pub fn cros_ec_device_registered(ec_dev: *mut cros_ec_device) -> bool;
}

/** Return time in ns. */
#[inline]
pub unsafe fn cros_ec_get_time_ns() -> ktime_t {
    ktime_get_boottime_ns()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
