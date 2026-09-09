/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * VMware VMCI Driver
 *
 * Copyright (C) 2012 VMware, Inc. All rights reserved.
 */

// Dependencies supplied by the corresponding VMCI definitions and uid/gid headers
// are intentionally referenced here rather than redefined.

pub const VMCI_KERNEL_API_VERSION_1: u32 = 1;
pub const VMCI_KERNEL_API_VERSION_2: u32 = 2;
pub const VMCI_KERNEL_API_VERSION: u32 = VMCI_KERNEL_API_VERSION_2;

pub type vmci_device_shutdown_fn = unsafe extern "C" fn(
    device_registration: *mut core::ffi::c_void,
    user_data: *mut core::ffi::c_void,
);
pub type vmci_vsock_cb = unsafe extern "C" fn(is_host: bool);

unsafe extern "C" {
    pub fn vmci_datagram_create_handle(
        resource_id: u32,
        flags: u32,
        recv_cb: vmci_datagram_recv_cb,
        client_data: *mut core::ffi::c_void,
        out_handle: *mut vmci_handle,
    ) -> i32;
    pub fn vmci_datagram_create_handle_priv(
        resource_id: u32,
        flags: u32,
        priv_flags: u32,
        recv_cb: vmci_datagram_recv_cb,
        client_data: *mut core::ffi::c_void,
        out_handle: *mut vmci_handle,
    ) -> i32;
    pub fn vmci_datagram_destroy_handle(handle: vmci_handle) -> i32;
    pub fn vmci_datagram_send(msg: *mut vmci_datagram) -> i32;
    pub fn vmci_doorbell_create(
        handle: *mut vmci_handle,
        flags: u32,
        priv_flags: u32,
        notify_cb: vmci_callback,
        client_data: *mut core::ffi::c_void,
    ) -> i32;
    pub fn vmci_doorbell_destroy(handle: vmci_handle) -> i32;
    pub fn vmci_get_context_id() -> u32;
    pub fn vmci_is_context_owner(context_id: u32, uid: kuid_t) -> bool;
    pub fn vmci_register_vsock_callback(callback: vmci_vsock_cb) -> i32;

    pub fn vmci_event_subscribe(
        event: u32,
        callback: vmci_event_cb,
        callback_data: *mut core::ffi::c_void,
        subid: *mut u32,
    ) -> i32;
    pub fn vmci_event_unsubscribe(subid: u32) -> i32;
    pub fn vmci_context_get_priv_flags(context_id: u32) -> u32;
    pub fn vmci_qpair_alloc(
        qpair: *mut *mut vmci_qp,
        handle: *mut vmci_handle,
        produce_qsize: u64,
        consume_qsize: u64,
        peer: u32,
        flags: u32,
        priv_flags: u32,
    ) -> i32;
    pub fn vmci_qpair_detach(qpair: *mut *mut vmci_qp) -> i32;
    pub fn vmci_qpair_get_produce_indexes(
        qpair: *const vmci_qp,
        producer_tail: *mut u64,
        consumer_head: *mut u64,
    ) -> i32;
    pub fn vmci_qpair_get_consume_indexes(
        qpair: *const vmci_qp,
        consumer_tail: *mut u64,
        producer_head: *mut u64,
    ) -> i32;
    pub fn vmci_qpair_produce_free_space(qpair: *const vmci_qp) -> i64;
    pub fn vmci_qpair_produce_buf_ready(qpair: *const vmci_qp) -> i64;
    pub fn vmci_qpair_consume_free_space(qpair: *const vmci_qp) -> i64;
    pub fn vmci_qpair_consume_buf_ready(qpair: *const vmci_qp) -> i64;
    pub fn vmci_qpair_enquev(
        qpair: *mut vmci_qp,
        msg: *mut msghdr,
        iov_size: usize,
        mode: i32,
    ) -> isize;
    pub fn vmci_qpair_dequev(
        qpair: *mut vmci_qp,
        msg: *mut msghdr,
        iov_size: usize,
        mode: i32,
    ) -> isize;
    pub fn vmci_qpair_peekv(
        qpair: *mut vmci_qp,
        msg: *mut msghdr,
        iov_size: usize,
        mode: i32,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
