/* SPDX-License-Identifier: GPL-2.0 */
/*
 * System Control and Management Interface (SCMI) Message Protocol
 * notification header file containing some definitions, structures
 * and function prototypes related to SCMI Notification handling.
 *
 * Copyright (C) 2020-2021 ARM Ltd.
 */

// Dependencies supplied by the surrounding Linux/SCMI environment:
// `ktime_t` and `scmi_handle` are intentionally not defined here.

pub const SCMI_PROTO_QUEUE_SZ: usize = 4096;

/**
 * Describes an event to be supported.
 *
 * Each SCMI protocol, during its initialization phase, can describe the events
 * it wishes to support in a few `scmi_event` and pass them to the core
 * using `scmi_register_protocol_events()`.
 */
#[repr(C)]
pub struct scmi_event {
    /// Event ID.
    pub id: u8,
    /// Max possible size for the payload of a notification message.
    pub max_payld_sz: usize,
    /// Max possible size for the report of a notification message.
    pub max_report_sz: usize,
}

pub struct scmi_protocol_handle;

/**
 * Protocol helpers called by the notification core.
 *
 * Context: Helpers described in `scmi_event_ops` are called only in
 * process context.
 */
#[repr(C)]
pub struct scmi_event_ops {
    /// Return false if the specified notification is not supported.
    pub is_notify_supported: Option<unsafe extern "C" fn(
        ph: *const scmi_protocol_handle,
        evt_id: u8,
        src_id: u32,
    ) -> bool>,
    /// Returns the number of possible events' sources for this protocol.
    pub get_num_sources:
        Option<unsafe extern "C" fn(ph: *const scmi_protocol_handle) -> i32>,
    /// Enable/disable the required event/source notifications.
    pub set_notify_enabled: Option<unsafe extern "C" fn(
        ph: *const scmi_protocol_handle,
        evt_id: u8,
        src_id: u32,
        enabled: bool,
    ) -> i32>,
    /// Fill a custom event report from the event message payload.
    pub fill_custom_report: Option<unsafe extern "C" fn(
        ph: *const scmi_protocol_handle,
        evt_id: u8,
        timestamp: ktime_t,
        payld: *const core::ffi::c_void,
        payld_sz: usize,
        report: *mut core::ffi::c_void,
        src_id: *mut u32,
    ) -> *mut core::ffi::c_void>,
}

/**
 * Per-protocol description of available events.
 */
#[repr(C)]
pub struct scmi_protocol_events {
    /// Size in bytes of the per-protocol queue to use.
    pub queue_sz: usize,
    /// Array of protocol-specific event operations.
    pub ops: *const scmi_event_ops,
    /// Array of supported protocol events.
    pub evts: *const scmi_event,
    /// Number of supported protocol events described in `evts`.
    pub num_events: core::ffi::c_uint,
    /// Number of protocol sources.
    pub num_sources: core::ffi::c_uint,
}

pub fn scmi_notification_init(handle: *mut scmi_handle) -> i32;
pub fn scmi_notification_quiesce(handle: *mut scmi_handle);
pub fn scmi_notification_exit(handle: *mut scmi_handle);
pub fn scmi_register_protocol_events(
    handle: *const scmi_handle,
    proto_id: u8,
    ph: *const scmi_protocol_handle,
    ee: *const scmi_protocol_events,
) -> i32;
pub fn scmi_deregister_protocol_events(handle: *const scmi_handle, proto_id: u8);
pub fn scmi_notify(
    handle: *const scmi_handle,
    proto_id: u8,
    evt_id: u8,
    buf: *const core::ffi::c_void,
    len: usize,
    ts: ktime_t,
) -> i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
