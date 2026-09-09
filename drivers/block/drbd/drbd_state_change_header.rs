/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from drbd_state_change.h.
// External types are supplied by other translation units.

#[repr(C)]
pub struct drbd_resource_state_change {
    pub resource: *mut drbd_resource,
    pub role: [drbd_role; 2],
    pub susp: [bool; 2],
    pub susp_nod: [bool; 2],
    pub susp_fen: [bool; 2],
}

#[repr(C)]
pub struct drbd_device_state_change {
    pub device: *mut drbd_device,
    pub disk_state: [drbd_disk_state; 2],
}

#[repr(C)]
pub struct drbd_connection_state_change {
    pub connection: *mut drbd_connection,
    // drbd9: enum drbd_conn_state
    pub cstate: [drbd_conns; 2],
    pub peer_role: [drbd_role; 2],
}

#[repr(C)]
pub struct drbd_peer_device_state_change {
    pub peer_device: *mut drbd_peer_device,
    pub disk_state: [drbd_disk_state; 2],
    // drbd9: enum drbd_repl_state
    pub repl_state: [drbd_conns; 2],
    pub resync_susp_user: [bool; 2],
    pub resync_susp_peer: [bool; 2],
    pub resync_susp_dependency: [bool; 2],
}

#[repr(C)]
pub struct drbd_state_change {
    pub list: list_head,
    pub n_devices: core::ffi::c_uint,
    pub n_connections: core::ffi::c_uint,
    pub resource: [drbd_resource_state_change; 1],
    pub devices: *mut drbd_device_state_change,
    pub connections: *mut drbd_connection_state_change,
    pub peer_devices: *mut drbd_peer_device_state_change,
}

extern "C" {
    pub fn remember_old_state(
        resource: *mut drbd_resource,
        gfp: gfp_t,
    ) -> *mut drbd_state_change;
    pub fn copy_old_to_new_state_change(state_change: *mut drbd_state_change);
    pub fn forget_state_change(state_change: *mut drbd_state_change);

    pub fn notify_resource_state_change(
        skb: *mut sk_buff,
        group: core::ffi::c_uint,
        arg: *mut core::ffi::c_void,
        type_: drbd_notification_type,
    ) -> core::ffi::c_int;
    pub fn notify_connection_state_change(
        skb: *mut sk_buff,
        group: core::ffi::c_uint,
        arg: *mut core::ffi::c_void,
        type_: drbd_notification_type,
    ) -> core::ffi::c_int;
    pub fn notify_device_state_change(
        skb: *mut sk_buff,
        group: core::ffi::c_uint,
        arg: *mut core::ffi::c_void,
        type_: drbd_notification_type,
    ) -> core::ffi::c_int;
    pub fn notify_peer_device_state_change(
        skb: *mut sk_buff,
        group: core::ffi::c_uint,
        arg: *mut core::ffi::c_void,
        type_: drbd_notification_type,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
