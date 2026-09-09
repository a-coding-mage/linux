/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Greybus connections
 *
 * Copyright 2014 Google Inc.
 * Copyright 2014 Linaro Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const GB_CONNECTION_FLAG_CSD: usize = 1usize << 0;
pub const GB_CONNECTION_FLAG_NO_FLOWCTRL: usize = 1usize << 1;
pub const GB_CONNECTION_FLAG_OFFLOADED: usize = 1usize << 2;
pub const GB_CONNECTION_FLAG_CDSI1: usize = 1usize << 3;
pub const GB_CONNECTION_FLAG_CONTROL: usize = 1usize << 4;
pub const GB_CONNECTION_FLAG_HIGH_PRIO: usize = 1usize << 5;

pub const GB_CONNECTION_FLAG_CORE_MASK: usize = GB_CONNECTION_FLAG_CONTROL;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gb_connection_state {
    GB_CONNECTION_STATE_DISABLED = 0,
    GB_CONNECTION_STATE_ENABLED_TX = 1,
    GB_CONNECTION_STATE_ENABLED = 2,
    GB_CONNECTION_STATE_DISCONNECTING = 3,
}

pub enum gb_operation {}

pub type gb_request_handler_t =
    Option<unsafe extern "C" fn(operation: *mut gb_operation) -> i32>;

#[repr(C)]
pub struct gb_connection {
    pub hd: *mut gb_host_device,
    pub intf: *mut gb_interface,
    pub bundle: *mut gb_bundle,
    pub kref: kref,
    pub hd_cport_id: u16,
    pub intf_cport_id: u16,

    pub hd_links: list_head,
    pub bundle_links: list_head,

    pub handler: gb_request_handler_t,
    pub flags: usize,

    pub mutex: mutex,
    pub lock: spinlock_t,
    pub state: gb_connection_state,
    pub operations: list_head,

    pub name: [core::ffi::c_char; 16],
    pub wq: *mut workqueue_struct,

    pub op_cycle: atomic_t,

    pub private: *mut core::ffi::c_void,

    pub mode_switch: bool,
}

unsafe extern "C" {
    pub fn gb_connection_create_static(
        hd: *mut gb_host_device,
        hd_cport_id: u16,
        handler: gb_request_handler_t,
    ) -> *mut gb_connection;
    pub fn gb_connection_create_control(intf: *mut gb_interface) -> *mut gb_connection;
    pub fn gb_connection_create(
        bundle: *mut gb_bundle,
        cport_id: u16,
        handler: gb_request_handler_t,
    ) -> *mut gb_connection;
    pub fn gb_connection_create_flags(
        bundle: *mut gb_bundle,
        cport_id: u16,
        handler: gb_request_handler_t,
        flags: usize,
    ) -> *mut gb_connection;
    pub fn gb_connection_create_offloaded(
        bundle: *mut gb_bundle,
        cport_id: u16,
        flags: usize,
    ) -> *mut gb_connection;
    pub fn gb_connection_destroy(connection: *mut gb_connection);

    pub fn gb_connection_enable(connection: *mut gb_connection) -> i32;
    pub fn gb_connection_enable_tx(connection: *mut gb_connection) -> i32;
    pub fn gb_connection_disable_rx(connection: *mut gb_connection);
    pub fn gb_connection_disable(connection: *mut gb_connection);
    pub fn gb_connection_disable_forced(connection: *mut gb_connection);

    pub fn gb_connection_mode_switch_prepare(connection: *mut gb_connection);
    pub fn gb_connection_mode_switch_complete(connection: *mut gb_connection);

    pub fn greybus_data_rcvd(
        hd: *mut gb_host_device,
        cport_id: u16,
        data: *mut u8,
        length: usize,
    );

    pub fn gb_connection_latency_tag_enable(connection: *mut gb_connection);
    pub fn gb_connection_latency_tag_disable(connection: *mut gb_connection);
}

#[inline]
pub unsafe fn gb_connection_is_static(connection: *mut gb_connection) -> bool {
    unsafe { (*connection).intf.is_null() }
}

#[inline]
pub unsafe fn gb_connection_e2efc_enabled(connection: *mut gb_connection) -> bool {
    unsafe { ((*connection).flags & GB_CONNECTION_FLAG_CSD) == 0 }
}

#[inline]
pub unsafe fn gb_connection_flow_control_disabled(connection: *mut gb_connection) -> bool {
    unsafe { ((*connection).flags & GB_CONNECTION_FLAG_NO_FLOWCTRL) != 0 }
}

#[inline]
pub unsafe fn gb_connection_is_offloaded(connection: *mut gb_connection) -> bool {
    unsafe { ((*connection).flags & GB_CONNECTION_FLAG_OFFLOADED) != 0 }
}

#[inline]
pub unsafe fn gb_connection_is_control(connection: *mut gb_connection) -> bool {
    unsafe { ((*connection).flags & GB_CONNECTION_FLAG_CONTROL) != 0 }
}

#[inline]
pub unsafe fn gb_connection_get_data(connection: *mut gb_connection) -> *mut core::ffi::c_void {
    unsafe { (*connection).private }
}

#[inline]
pub unsafe fn gb_connection_set_data(
    connection: *mut gb_connection,
    data: *mut core::ffi::c_void,
) {
    unsafe { (*connection).private = data; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
