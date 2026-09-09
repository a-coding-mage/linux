/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2005 Network Appliance, Inc. All rights reserved.
 * Copyright (c) 2005 Open Grid Computing, Inc. All rights reserved.
 */

// Dependencies supplied by the corresponding Linux/RDMA headers:
// linux/in.h and rdma/ib_cm.h

use core::ffi::c_void;

pub struct iw_cm_id;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iw_cm_event_type {
    IW_CM_EVENT_CONNECT_REQUEST = 1, // connect request received
    IW_CM_EVENT_CONNECT_REPLY,        // reply from active connect request
    IW_CM_EVENT_ESTABLISHED,          // passive side accept successful
    IW_CM_EVENT_DISCONNECT,            // orderly shutdown
    IW_CM_EVENT_CLOSE,                // close complete
}

#[repr(C)]
pub struct iw_cm_event {
    pub event: iw_cm_event_type,
    pub status: core::ffi::c_int,
    pub local_addr: sockaddr_storage,
    pub remote_addr: sockaddr_storage,
    pub private_data: *mut c_void,
    pub provider_data: *mut c_void,
    pub private_data_len: u8,
    pub ord: u8,
    pub ird: u8,
}

/// Function to be called by the IW CM when delivering events to the client.
pub type iw_cm_handler = unsafe extern "C" fn(
    cm_id: *mut iw_cm_id,
    event: *mut iw_cm_event,
) -> core::ffi::c_int;

/// Function called by the provider when delivering provider events to the IW CM.
pub type iw_event_handler = unsafe extern "C" fn(
    cm_id: *mut iw_cm_id,
    event: *mut iw_cm_event,
) -> core::ffi::c_int;

#[repr(C)]
pub struct iw_cm_id {
    pub cm_handler: iw_cm_handler, // client callback function
    pub context: *mut c_void,       // client cb context
    pub device: *mut ib_device,
    pub local_addr: sockaddr_storage,  // local addr
    pub remote_addr: sockaddr_storage,
    pub m_local_addr: sockaddr_storage, // nmapped local addr
    pub m_remote_addr: sockaddr_storage, // nmapped rem addr
    pub provider_data: *mut c_void,      // provider private data
    pub event_handler: iw_event_handler, // cb for provider events
    // Used by provider to add and remove refs on IW cm_id
    pub add_ref: unsafe extern "C" fn(*mut iw_cm_id),
    pub rem_ref: unsafe extern "C" fn(*mut iw_cm_id),
    pub tos: u8,
    // C bitfields: tos_set:1, mapped:1, afonly:1. Packed in one byte.
    pub tos_set: u8,
    pub mapped: u8,
    pub afonly: u8,
}

#[repr(C)]
pub struct iw_cm_conn_param {
    pub private_data: *const c_void,
    pub private_data_len: u16,
    pub ord: u32,
    pub ird: u32,
    pub qpn: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iw_flags {
    /*
     * This flag allows the iwcm and iwpmd to still advertise
     * mappings but the real and mapped port numbers are the
     * same.  Further, iwpmd will not bind any user socket to
     * reserve the port.  This is required for soft iwarp
     * to play in the port mapped iwarp space.
     */
    IW_F_NO_PORT_MAP = 1 << 0,
}

extern "C" {
    pub fn iw_create_cm_id(
        device: *mut ib_device,
        cm_handler: iw_cm_handler,
        context: *mut c_void,
    ) -> *mut iw_cm_id;
    pub fn iw_destroy_cm_id(cm_id: *mut iw_cm_id);
    pub fn iw_cm_listen(cm_id: *mut iw_cm_id, backlog: core::ffi::c_int) -> core::ffi::c_int;
    pub fn iw_cm_accept(cm_id: *mut iw_cm_id, iw_param: *mut iw_cm_conn_param) -> core::ffi::c_int;
    pub fn iw_cm_reject(
        cm_id: *mut iw_cm_id,
        private_data: *const c_void,
        private_data_len: u8,
    ) -> core::ffi::c_int;
    pub fn iw_cm_connect(cm_id: *mut iw_cm_id, iw_param: *mut iw_cm_conn_param) -> core::ffi::c_int;
    pub fn iw_cm_disconnect(cm_id: *mut iw_cm_id, abrupt: core::ffi::c_int) -> core::ffi::c_int;
    pub fn iw_cm_init_qp_attr(
        cm_id: *mut iw_cm_id,
        qp_attr: *mut ib_qp_attr,
        qp_attr_mask: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn iwcm_reject_msg(reason: core::ffi::c_int) -> *const core::ffi::c_char;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
